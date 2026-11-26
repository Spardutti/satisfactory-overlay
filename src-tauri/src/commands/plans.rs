use super::planning::{calculate_request, transport_for};
use crate::planner::{miner_capacity, NodeCounts};
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
pub struct BuiltToggle {
    plan_id: String,
    item: String,
    row: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodesChange {
    plan_id: String,
    item: String,
    nodes: NodeCounts,
}

/// Records the nodes found for one ore of a saved plan and recalculates it, keeping its progress.
#[tauri::command]
pub fn set_nodes(app: AppHandle, state: State<AppState>, change: NodesChange) -> Result<(), String> {
    let mut plan = lock(&state.plans).get_mut(&change.plan_id).ok_or("That plan no longer exists")?.clone();
    plan.set_nodes(change.item, change.nodes);
    let steps = checklist(&calculate_request(&state, &plan.request)?.result, &plan.request.miner_tier);
    plan.replace_steps(steps);
    change_plans(&app, |store| store.upsert(plan))
}

/// Saves a new plan, or recalculates an existing one keeping its progress.
#[tauri::command]
pub fn save_plan(app: AppHandle, state: State<AppState>, draft: PlanDraft) -> Result<Plan, String> {
    let PlanDraft { id, name, request } = draft;
    let calculation = calculate_request(&state, &request)?;
    let steps = checklist(&calculation.result, &request.miner_tier);
    let id = id.unwrap_or_else(new_plan_id);
    let mut saved = None;
    change_plans(&app, |store| {
        let mut plan = store.get_mut(&id).cloned().unwrap_or(Plan {
            id: id.clone(),
            name: name.clone(),
            request: request.clone(),
            steps: vec![],
            positions: Default::default(),
            built: Default::default(),
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

/// Ticks or unticks one machine of a saved plan in the blueprint.
#[tauri::command]
pub fn toggle_built(app: AppHandle, change: BuiltToggle) -> Result<(), String> {
    change_plans(&app, |store| {
        if let Some(plan) = store.get_mut(&change.plan_id) {
            plan.toggle_built(&change.item, change.row);
        }
    })
}

/// Unticks every machine of a saved plan, to build it again.
#[tauri::command]
pub fn clear_built(app: AppHandle, plan_id: String) -> Result<(), String> {
    change_plans(&app, |store| {
        if let Some(plan) = store.get_mut(&plan_id) {
            plan.clear_built();
        }
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillChange {
    plan_id: String,
    item: String,
}

/// Grows a saved plan until this ore's miners all run full, keeping its progress.
#[tauri::command]
pub fn fill_miners(app: AppHandle, state: State<AppState>, change: FillChange) -> Result<(), String> {
    let mut plan = lock(&state.plans).get_mut(&change.plan_id).ok_or("That plan no longer exists")?.clone();
    let calculation = calculate_request(&state, &plan.request)?;
    let raw = calculation.result.raw.iter().find(|r| r.item == change.item).ok_or("This plan does not mine that")?;
    let belt = transport_for(&state, &plan.request)?.belt;
    let rates = state.editable.extractors.node_rates(&plan.request.miner_tier, belt)?;
    if !plan.request.grow_to_fill(raw, miner_capacity(raw, rates)) {
        return Err("These miners can't grow: they run full, or another ore limits the plan".into());
    }
    let steps = checklist(&calculate_request(&state, &plan.request)?.result, &plan.request.miner_tier);
    plan.replace_steps(steps);
    change_plans(&app, |store| store.upsert(plan))
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
