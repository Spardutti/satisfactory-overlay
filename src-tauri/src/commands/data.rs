use crate::game_data::{find_docs_file, GameData, GameDocsSource, RecipeSource};
use crate::state::{lock, AppState};
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::PathBuf;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSummary {
    path: String,
    items: usize,
    recipes: usize,
    warnings: Vec<String>,
}

#[derive(Serialize)]
pub struct Named {
    id: String,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateInfo {
    id: String,
    name: String,
    makes: String,
    enabled: bool,
}

/// Loads the docs file from the saved path or the usual install folders; None when not found.
#[tauri::command]
pub fn load_game_data(state: State<AppState>) -> Result<Option<DataSummary>, String> {
    let saved = lock(&state.config).docs_path.clone().filter(|p| p.is_file());
    match saved.or_else(find_docs_file) {
        Some(path) => load_from(&state, path).map(Some),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn set_docs_path(state: State<AppState>, path: String) -> Result<DataSummary, String> {
    let summary = load_from(&state, PathBuf::from(&path))?;
    let mut config = lock(&state.config);
    config.docs_path = Some(PathBuf::from(path));
    state.save_config(&config)?;
    Ok(summary)
}

fn load_from(state: &AppState, path: PathBuf) -> Result<DataSummary, String> {
    let data = GameDocsSource { path: path.clone() }.load()?;
    let summary = DataSummary {
        path: path.display().to_string(),
        items: data.items.len(),
        recipes: data.recipes.len(),
        warnings: data.warnings.clone(),
    };
    *lock(&state.game) = Some(data);
    Ok(summary)
}

pub fn with_game<T>(state: &AppState, f: impl FnOnce(&GameData) -> T) -> Result<T, String> {
    lock(&state.game).as_ref().map(f).ok_or("Game data is not loaded yet".into())
}

/// Every item some factory recipe makes, by name.
#[tauri::command]
pub fn list_targets(state: State<AppState>) -> Result<Vec<Named>, String> {
    with_game(&state, |data| {
        let ids: BTreeSet<&str> =
            data.recipes.iter().flat_map(|r| r.products.iter().map(|p| p.item.as_str())).collect();
        sorted_by_name(ids.into_iter().map(|id| Named { id: id.into(), name: data.item_name(id) }))
    })
}

#[tauri::command]
pub fn list_raw_resources(state: State<AppState>) -> Result<Vec<Named>, String> {
    with_game(&state, |data| {
        sorted_by_name(state.editable.raw.iter().map(|id| Named { id: id.clone(), name: data.item_name(id) }))
    })
}

#[tauri::command]
pub fn list_alternates(state: State<AppState>) -> Result<Vec<AlternateInfo>, String> {
    let enabled = lock(&state.config).enabled_alternates.clone();
    with_game(&state, |data| {
        let mut list: Vec<AlternateInfo> = data
            .recipes
            .iter()
            .filter(|r| r.alternate)
            .map(|r| AlternateInfo {
                id: r.id.clone(),
                name: r.name.trim_start_matches("Alternate:").trim().to_string(),
                makes: data.item_name(&r.products[0].item),
                enabled: enabled.contains(&r.id),
            })
            .collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    })
}

#[tauri::command]
pub fn set_alternate(state: State<AppState>, id: String, enabled: bool) -> Result<(), String> {
    let mut config = lock(&state.config);
    match enabled {
        true => config.enabled_alternates.insert(id),
        false => config.enabled_alternates.remove(&id),
    };
    state.save_config(&config)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractorOptions {
    tiers: Vec<String>,
    purities: Vec<String>,
}

#[tauri::command]
pub fn extractor_options(state: State<AppState>) -> ExtractorOptions {
    let extractors = &state.editable.extractors;
    ExtractorOptions {
        tiers: extractors.miners.keys().cloned().collect(),
        purities: extractors.purity.keys().cloned().collect(),
    }
}

fn sorted_by_name(items: impl Iterator<Item = Named>) -> Vec<Named> {
    let mut list: Vec<Named> = items.collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    list
}
