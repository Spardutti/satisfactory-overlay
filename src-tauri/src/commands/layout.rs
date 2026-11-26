use super::planning::{calculate_request, transport_for};
use super::plans::change_plans;
use crate::layout::{apply_positions, build_layout, Layout};
use crate::state::{lock, AppState};
use serde::Deserialize;
use tauri::{AppHandle, State};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockMove {
    plan_id: String,
    item: String,
    x: f64,
    y: f64,
}

/// The blueprint of a saved plan, recalculated from its request, with blocks where the user put them.
#[tauri::command]
pub fn plan_layout(state: State<AppState>, plan_id: String) -> Result<Layout, String> {
    let plan = lock(&state.plans)
        .plans
        .iter()
        .find(|p| p.id == plan_id)
        .cloned()
        .ok_or("That plan no longer exists")?;
    let calculation = calculate_request(&state, &plan.request)?;
    let transport = transport_for(&state, &plan.request)?;
    let miner_rate = state.editable.extractors.miner_rate(&plan.request.miner_tier, "normal")?;
    let mut layout = build_layout(&calculation.result, transport, miner_rate);
    apply_positions(&mut layout, &plan.positions);
    Ok(layout)
}

#[tauri::command]
pub fn move_block(app: AppHandle, change: BlockMove) -> Result<(), String> {
    change_plans(&app, |store| {
        if let Some(plan) = store.get_mut(&change.plan_id) {
            plan.positions.insert(change.item, [change.x, change.y]);
        }
    })
}

/// Forgets every hand-placed block, so the blueprint goes back to build order.
#[tauri::command]
pub fn reset_layout(app: AppHandle, plan_id: String) -> Result<(), String> {
    change_plans(&app, |store| {
        if let Some(plan) = store.get_mut(&plan_id) {
            plan.positions.clear();
        }
    })
}
