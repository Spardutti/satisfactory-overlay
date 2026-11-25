use crate::game_data::GameData;
use crate::planner::{Limit, PlanResult, Planner, Transport};
use crate::plans::{checklist, ChecklistStep, Mode, PlanRequest};
use crate::state::{lock, AppState};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeOption {
    id: String,
    name: String,
    alternate: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Calculation {
    pub result: PlanResult,
    pub limit: Option<Limit>,
    /// the same build cards the overlay shows
    pub cards: Vec<ChecklistStep>,
    /// item id → recipes the user may pick for that step
    pub options: HashMap<String, Vec<RecipeOption>>,
}

#[tauri::command]
pub fn calculate(state: State<AppState>, request: PlanRequest) -> Result<Calculation, String> {
    calculate_request(&state, &request)
}

pub fn calculate_request(state: &AppState, request: &PlanRequest) -> Result<Calculation, String> {
    let miner_rate = state.editable.extractors.miner_rate(&request.miner_tier, &request.purity)?;
    let transport = transport_for(state, request)?;
    let alternates: HashSet<String> = lock(&state.config).enabled_alternates.iter().cloned().collect();
    let game = lock(&state.game);
    let data: &GameData = game.as_ref().ok_or("Game data is not loaded yet")?;
    let planner = Planner {
        data,
        raw: &state.editable.raw,
        choices: &request.choices,
        enabled_alternates: &alternates,
        miner_rate,
        transport,
    };
    let (result, limit) = run_mode(&planner, request)?;
    let options = result.steps.iter().map(|s| (s.item.clone(), options_for(&planner, &s.item))).collect();
    let cards = checklist(&result, &request.miner_tier, &request.purity);
    Ok(Calculation { result, limit, options, cards })
}

pub fn transport_for(state: &AppState, request: &PlanRequest) -> Result<Transport, String> {
    let logistics = &state.editable.logistics;
    Ok(Transport {
        belt: logistics.belt_speed(&request.belt_tier)?,
        pipe: logistics.pipe_speed(&request.pipe_tier)?,
    })
}

fn run_mode(planner: &Planner, request: &PlanRequest) -> Result<(PlanResult, Option<Limit>), String> {
    match &request.mode {
        Mode::Rate { rate } => Ok((planner.plan_rate(&request.target, *rate)?, None)),
        Mode::Resources { available } => {
            let limit = planner.max_rate(&request.target, available)?;
            Ok((planner.plan_rate(&request.target, limit.max_rate)?, Some(limit)))
        }
    }
}

fn options_for(planner: &Planner, item: &str) -> Vec<RecipeOption> {
    planner
        .recipe_options(item)
        .into_iter()
        .map(|r| RecipeOption { id: r.id.clone(), name: r.name.clone(), alternate: r.alternate })
        .collect()
}
