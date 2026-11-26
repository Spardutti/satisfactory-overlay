use super::model::{GameData, ItemAmount, Recipe};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// The made-up item every generator recipe produces; its rate is megawatts, not items per minute.
pub const POWER: &str = "Power";

const LITRES_PER_M3: f64 = 1000.0;
/// Listed first, so a new power plan starts on coal.
const DEFAULT_GENERATOR: &str = "Build_GeneratorCoal_C";

/// One generator + fuel pair the player can pick for a power plan.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerOption {
    pub recipe_id: String,
    pub generator: String,
    pub fuel: String,
    pub megawatts: f64,
}

/// Every generator + fuel recipe, in the order the planner ranks them (coal first).
pub fn power_options(data: &GameData) -> Vec<PowerOption> {
    data.recipes
        .iter()
        .filter(|r| r.products.first().is_some_and(|p| p.item == POWER))
        .map(|r| PowerOption {
            recipe_id: r.id.clone(),
            generator: data.building_name(&r.building),
            fuel: data.item_name(&r.ingredients[0].item),
            megawatts: r.products[0].amount,
        })
        .collect()
}

/// Fuel burned per minute: items for solids, m³ for fluids (whose energy the game gives per litre).
pub fn fuel_per_min(megawatts: f64, energy_mj: f64, fluid: bool) -> f64 {
    let per_min = 60.0 * megawatts / energy_mj;
    if fluid { per_min / LITRES_PER_M3 } else { per_min }
}

/// Water in m³ per minute for a generator that needs `ratio` litres per MW each second.
pub fn water_per_min(megawatts: f64, ratio: f64) -> f64 {
    megawatts * ratio * 60.0 / LITRES_PER_M3
}

/// Fuel energy in MJ per item (per litre for fluids), from every item class that has one.
pub fn energy_values(item_classes: &[&Value]) -> HashMap<String, f64> {
    item_classes
        .iter()
        .filter_map(|c| Some((text(c, "ClassName")?.to_string(), number(c, "mEnergyValue")?)))
        .filter(|(_, mj)| *mj > 0.0)
        .collect()
}

/// One recipe per fuel-burning generator and fuel, making `POWER` at the generator's MW per machine.
pub fn power_recipes(generator_classes: &[&Value], energy: &HashMap<String, f64>, data: &GameData) -> Vec<Recipe> {
    let mut recipes: Vec<Recipe> = generator_classes.iter().flat_map(|g| generator_recipes(g, energy, data)).collect();
    recipes.sort_by_key(|r| r.building != DEFAULT_GENERATOR);
    recipes
}

fn text<'a>(class: &'a Value, key: &str) -> Option<&'a str> {
    class.get(key)?.as_str()
}

fn number(class: &Value, key: &str) -> Option<f64> {
    text(class, key)?.parse().ok()
}

/// Every fuel one generator burns; fuels that leave waste behind (nuclear) are left out.
fn generator_recipes(generator: &Value, energy: &HashMap<String, f64>, data: &GameData) -> Vec<Recipe> {
    let (Some(id), Some(name), Some(megawatts)) = (text(generator, "ClassName"), text(generator, "mDisplayName"), number(generator, "mPowerProduction")) else {
        return vec![];
    };
    let ratio = number(generator, "mSupplementalToPowerRatio").unwrap_or(0.0);
    let fuels = generator.get("mFuel").and_then(Value::as_array).cloned().unwrap_or_default();
    fuels
        .iter()
        .filter(|f| text(f, "mByproduct").unwrap_or_default().is_empty())
        .filter_map(|f| {
            let fuel = text(f, "mFuelClass")?;
            let mut ingredients = vec![ItemAmount { item: fuel.into(), amount: fuel_per_min(megawatts, *energy.get(fuel)?, data.is_fluid(fuel)) }];
            if let Some(water) = text(f, "mSupplementalResourceClass").filter(|w| !w.is_empty()) {
                ingredients.push(ItemAmount { item: water.into(), amount: water_per_min(megawatts, ratio) });
            }
            Some(Recipe {
                id: format!("Power_{id}_{fuel}"),
                name: format!("{name} — {}", data.item_name(fuel)),
                alternate: false,
                ingredients,
                products: vec![ItemAmount { item: POWER.into(), amount: megawatts }],
                duration_secs: 60.0,
                building: id.into(),
            })
        })
        .collect()
}
