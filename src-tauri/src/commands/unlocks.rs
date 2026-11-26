use super::data::with_game;
use crate::game_data::{GameData, Unlock, UnlockKind};
use crate::state::{lock, AppState};
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockInfo {
    id: String,
    name: String,
    kind: UnlockKind,
    tier: u32,
    tree: String,
    /// names of the items its recipes make, e.g. ["Steel Ingot", "Steel Beam"]
    makes: Vec<String>,
    ticked: bool,
}

/// Every milestone and MAM node that gives a factory recipe, with whether the player ticked it.
#[tauri::command]
pub fn list_unlocks(state: State<AppState>) -> Result<Vec<UnlockInfo>, String> {
    let unlocked = lock(&state.config).unlocked.clone();
    with_game(&state, |data| {
        data.unlocks
            .unlocks
            .iter()
            .filter(|u| u.kind != UnlockKind::Alternate)
            .map(|u| UnlockInfo {
                id: u.id.clone(),
                name: u.name.clone(),
                kind: u.kind,
                tier: u.tier,
                tree: u.tree.clone(),
                makes: makes(data, u),
                ticked: unlocked.contains(&u.id),
            })
            .collect()
    })
}

#[tauri::command]
pub fn set_unlock(state: State<AppState>, id: String, ticked: bool) -> Result<(), String> {
    let mut config = lock(&state.config);
    match ticked {
        true => config.unlocked.insert(id),
        false => config.unlocked.remove(&id),
    };
    state.save_config(&config)
}

/// "I've finished up to Tier N": ticks every milestone of tier N or lower, unticks the ones above.
#[tauri::command]
pub fn set_tier(state: State<AppState>, tier: u32) -> Result<(), String> {
    let milestones: Vec<(String, u32)> = with_game(&state, |data| {
        data.unlocks.unlocks.iter().filter(|u| u.kind == UnlockKind::Milestone).map(|u| (u.id.clone(), u.tier)).collect()
    })?;
    let mut config = lock(&state.config);
    for (id, milestone_tier) in milestones {
        match milestone_tier <= tier {
            true => config.unlocked.insert(id),
            false => config.unlocked.remove(&id),
        };
    }
    state.save_config(&config)
}


/// Item names its recipes make, once each, in the order the recipes list them.
fn makes(data: &GameData, unlock: &Unlock) -> Vec<String> {
    let mut names: Vec<String> = vec![];
    let products = unlock.recipes.iter().filter_map(|id| data.recipes.iter().find(|r| &r.id == id)).map(|r| &r.products[0].item);
    for item in products.map(|i| data.item_name(i)) {
        if !names.contains(&item) {
            names.push(item);
        }
    }
    names
}
