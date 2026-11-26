use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ItemAmount {
    pub item: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub fluid: bool,
    /// a node resource in the game data, the kind a miner or extractor pulls
    pub resource: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub alternate: bool,
    pub ingredients: Vec<ItemAmount>,
    pub products: Vec<ItemAmount>,
    pub duration_secs: f64,
    pub building: String,
}

impl Recipe {
    /// Items per minute of `item` one machine makes or uses at 100% clock.
    pub fn rate_per_min(&self, amount: f64) -> f64 {
        amount * 60.0 / self.duration_secs
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameData {
    pub items: HashMap<String, Item>,
    pub recipes: Vec<Recipe>,
    pub unlocks: super::unlocks::UnlockBook,
    pub buildings: HashMap<String, String>,
    pub machine_power: HashMap<String, f64>,
    pub recipe_power: HashMap<String, f64>,
    pub warnings: Vec<String>,
}

impl GameData {
    pub fn item_name(&self, id: &str) -> String {
        self.items.get(id).map_or_else(|| id.to_string(), |i| i.name.clone())
    }

    pub fn building_name(&self, id: &str) -> String {
        self.buildings.get(id).cloned().unwrap_or_else(|| id.to_string())
    }

    /// A swinging building (Particle Accelerator) has no steady draw, so its recipe's average counts.
    pub fn full_clock_mw(&self, building_name: &str, recipe_id: Option<&str>) -> f64 {
        let steady = self.machine_power.get(&super::machine_power::power_key(building_name));
        let swinging = || recipe_id.and_then(|id| self.recipe_power.get(id));
        steady.or_else(swinging).copied().unwrap_or(0.0)
    }

    pub fn is_fluid(&self, id: &str) -> bool {
        self.items.get(id).is_some_and(|i| i.fluid)
    }

    pub fn is_minable(&self, id: &str) -> bool {
        self.items.get(id).is_some_and(|i| i.resource && !i.fluid)
    }
}
