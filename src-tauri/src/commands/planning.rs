use crate::game_data::{GameData, UnlockKind};
use crate::planner::{apply_nodes, depot_cap, fillable_ores, mark_depot, node_limit, Limit, PlanResult, Planner, Transport};
use crate::plans::{checklist, ChecklistStep, Mode, PlanRequest};
use crate::state::{lock, AppState};
use serde::Serialize;
use std::collections::{BTreeSet, HashMap, HashSet};
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
    /// the most target per minute the depot allows, when it caps the plan
    pub depot_limit: Option<f64>,
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
    let transport = transport_for(state, request)?;
    let book = extractors.book(&request.miner_tier, transport)?;
    let (alternates, unlocked) = {
        let config = lock(&state.config);
        (config.enabled_alternates.clone(), config.unlocked.clone())
    };
    let game = lock(&state.game);
    let data: &GameData = game.as_ref().ok_or("Game data is not loaded yet")?;
    let owned = owned_recipes(data, &alternates, &unlocked)?;
    let raw: HashSet<String> = state.editable.raw.iter().chain(request.depot.keys()).cloned().collect();
    let planner = Planner {
        data,
        raw: &raw,
        choices: &request.choices,
        enabled_alternates: &owned,
        extractors: &book,
        transport,
    };
    let Planned { mut result, limit, depot_limit } = run_mode(&planner, request)?;
    mark_depot(&mut result, &request.depot);
    apply_nodes(&mut result, &request.nodes);
    let node_limit = node_limit(&result);
    // A plan the depot caps cannot grow, so ↑100% would do nothing.
    let fillable = if depot_limit.is_some() { vec![] } else { fillable_ores(&result) };
    let options = result.steps.iter().map(|s| (s.item.clone(), options_for(&planner, &s.item))).collect();
    let cards = checklist(&result);
    Ok(Calculation { result, limit, node_limit, fillable, depot_limit, options, cards })
}

/// Recipes a plan may use: ticked alternates, plus what ticked milestones and MAM nodes give.
fn owned_recipes(data: &GameData, alternates: &BTreeSet<String>, unlocked: &BTreeSet<String>) -> Result<HashSet<String>, String> {
    let mut owned: HashSet<String> = alternates.iter().cloned().collect();
    if data.unlocks.is_empty() {
        return Ok(owned);
    }
    let picked_tier = data.unlocks.unlocks.iter().any(|u| u.kind == UnlockKind::Milestone && unlocked.contains(&u.id));
    if !picked_tier {
        return Err("Pick your tier in My recipes first.".into());
    }
    owned.extend(data.unlocks.owned_recipes(unlocked));
    Ok(owned)
}

pub fn transport_for(state: &AppState, request: &PlanRequest) -> Result<Transport, String> {
    let logistics = &state.editable.logistics;
    Ok(Transport {
        belt: logistics.belt_speed(&request.belt_tier)?,
        pipe: logistics.pipe_speed(&request.pipe_tier)?,
    })
}

struct Planned {
    result: PlanResult,
    limit: Option<Limit>,
    depot_limit: Option<f64>,
}

fn run_mode(planner: &Planner, request: &PlanRequest) -> Result<Planned, String> {
    let target = &request.target;
    match &request.mode {
        Mode::Rate { rate } => {
            let result = planner.plan_rate(target, *rate)?;
            match depot_cap(&result, &request.depot) {
                Some(cap) => Ok(Planned { result: planner.plan_rate(target, cap)?, limit: None, depot_limit: Some(cap) }),
                None => Ok(Planned { result, limit: None, depot_limit: None }),
            }
        }
        Mode::Resources { available } => {
            let mut on_hand = available.clone();
            on_hand.extend(request.depot.iter().map(|(item, n)| (item.clone(), *n)));
            let limit = planner.max_rate(target, &on_hand)?;
            let result = planner.plan_rate(target, limit.max_rate)?;
            Ok(Planned { result, limit: Some(limit), depot_limit: None })
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
