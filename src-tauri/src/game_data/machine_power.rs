use super::power::{number, text};
use serde_json::Value;
use std::collections::HashMap;

const CLOCK_EXPONENT: f64 = 1.321929;

pub fn machine_power(building_classes: &[&Value]) -> HashMap<String, f64> {
    let steady = |c: &Value| number(c, "mPowerConsumption").filter(|mw| *mw > 0.0);
    building_classes.iter().filter_map(|c| Some((power_key(text(c, "mDisplayName")?), steady(c)?))).collect()
}

/// Settings name miners "Miner Mk1"; the game calls them "Miner Mk.1".
pub fn power_key(building_name: &str) -> String {
    building_name.replace('.', "")
}

pub fn recipe_power(recipe_classes: &[&Value]) -> HashMap<String, f64> {
    let average = |c: &Value| Some(number(c, "mVariablePowerConsumptionConstant")? + number(c, "mVariablePowerConsumptionFactor")? / 2.0);
    recipe_classes.iter().filter_map(|c| Some((text(c, "ClassName")?.to_string(), average(c).filter(|mw| *mw > 0.0)?))).collect()
}

pub fn clocked_mw(full_clock_mw: f64, clock_pct: f64) -> f64 {
    full_clock_mw * (clock_pct / 100.0).powf(CLOCK_EXPONENT)
}
