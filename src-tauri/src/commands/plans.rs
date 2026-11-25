use super::planning::calculate_request;
use crate::plans::{checklist, ManualProgress, Plan, PlanRequest, PlanStore, ProgressSource};
use crate::state::{lock, AppState};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager, State};

pub const PLANS_CHANGED: &str = "plans-changed";

#[tauri::command]
pub fn list_plans(state: State<AppState>) -> PlanStore {
    let mut store = lock(&state.plans).clone();
    store.plans.iter_mut().for_each(|p| ManualProgress.sync(p));
    store
}

#[derive(Deserialize)]
pub struct PlanDraft {
    id: Option<String>,
    name: String,
    request: PlanRequest,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepChange {
    plan_id: String,
    step: usize,
    delta: i32,
}

/// Saves a new plan, or recalculates an existing one keeping its progress.
#[tauri::command]
pub fn save_plan(app: AppHandle, state: State<AppState>, draft: PlanDraft) -> Result<Plan, String> {
    let PlanDraft { id, name, request } = draft;
    let calculation = calculate_request(&state, &request)?;
    let steps = checklist(&calculation.result, &request.miner_tier, &request.purity);
    let id = id.unwrap_or_else(new_plan_id);
    let mut saved = None;
    change_plans(&app, |store| {
        let mut plan = store.get_mut(&id).cloned().unwrap_or(Plan {
            id: id.clone(),
            name: name.clone(),
            request: request.clone(),
            steps: vec![],
            positions: Default::default(),
        });
        plan.name = name;
        plan.request = request;
        plan.replace_steps(steps);
        saved = Some(plan.clone());
        store.upsert(plan);
    })?;
    saved.ok_or("Plan was not saved".into())
}

#[tauri::command]
pub fn delete_plan(app: AppHandle, id: String) -> Result<(), String> {
    change_plans(&app, |store| store.remove(&id))
}

#[tauri::command]
pub fn set_active_plan(app: AppHandle, id: String) -> Result<(), String> {
    change_plans(&app, |store| store.set_active(&id))
}

#[tauri::command]
pub fn adjust_step(app: AppHandle, change: StepChange) -> Result<(), String> {
    change_plans(&app, |store| {
        if let Some(plan) = store.get_mut(&change.plan_id) {
            plan.adjust(change.step, change.delta);
        }
    })
}

#[tauri::command]
pub fn mark_next_step(app: AppHandle) -> Result<(), String> {
    change_plans(&app, |store| {
        if let Some(plan) = store.active_mut() {
            plan.mark_next();
        }
    })
}

pub fn change_plans(app: &AppHandle, change: impl FnOnce(&mut PlanStore)) -> Result<(), String> {
    let state = app.state::<AppState>();
    let mut store = lock(&state.plans);
    change(&mut store);
    state.save_plans(&store)?;
    app.emit(PLANS_CHANGED, ()).map_err(|e| e.to_string())
}

fn new_plan_id() -> String {
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis());
    format!("plan-{millis}")
}
