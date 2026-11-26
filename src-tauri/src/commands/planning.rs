use crate::game_data::GameData;
use crate::planner::{apply_nodes, fillable_ores, node_limit, Limit, PlanResult, Planner, Transport};
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
    /// the most target per minute the listed nodes allow, when an ore runs short
    pub node_limit: Option<f64>,
    /// ores whose miners could run faster, so the blueprint offers ↑100%
    pub fillable: Vec<String>,
    /// build cards, in build order
    pub cards: Vec<ChecklistStep>,
    /// item id → recipes the user may pick for that step
    pub options: HashMap<String, Vec<RecipeOption>>,
}

#[tauri::command]
pub fn calculate(state: State<AppState>, request: PlanRequest) -> Result<Calculation, String> {
    calculate_request(&state, &request)
}

pub fn calculate_request(state: &AppState, request: &PlanRequest) -> Result<Calculation, String> {
    let extractors = &state.editable.extractors;
    let miner_rate = extractors.miner_rate(&request.miner_tier, "normal")?;
    let transport = transport_for(state, request)?;
    let node_rates = extractors.node_rates(&request.miner_tier, transport.belt)?;
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
    let (mut result, limit) = run_mode(&planner, request)?;
    apply_nodes(&mut result, &request.nodes, node_rates);
    let node_limit = node_limit(&result);
    let fillable = fillable_ores(&result, node_rates);
    let options = result.steps.iter().map(|s| (s.item.clone(), options_for(&planner, &s.item))).collect();
    let cards = checklist(&result, &request.miner_tier);
    Ok(Calculation { result, limit, node_limit, fillable, options, cards })
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
