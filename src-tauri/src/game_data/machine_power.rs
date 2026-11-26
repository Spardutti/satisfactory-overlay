use super::power::{number, text};
use serde_json::Value;
use std::collections::HashMap;

/// The game's mPowerConsumptionExponent, the same for every factory and extractor.
const CLOCK_EXPONENT: f64 = 1.321929;

/// MW each building draws at 100% clock, keyed by `power_key` of its name.
pub fn machine_power(building_classes: &[&Value]) -> HashMap<String, f64> {
    building_classes.iter().filter_map(|c| Some((power_key(text(c, "mDisplayName")?), full_clock_mw(c)?))).collect()
}

/// Settings name miners "Miner Mk1"; the game calls them "Miner Mk.1".
pub fn power_key(building_name: &str) -> String {
    building_name.replace('.', "")
}

pub fn clocked_mw(full_clock_mw: f64, clock_pct: f64) -> f64 {
    full_clock_mw * (clock_pct / 100.0).powf(CLOCK_EXPONENT)
}

/// A building whose draw swings (Particle Accelerator) counts at the middle of its range.
fn full_clock_mw(class: &Value) -> Option<f64> {
    let steady = number(class, "mPowerConsumption").filter(|mw| *mw > 0.0);
    let middle = || {
        let (low, high) = (number(class, "mEstimatedMininumPowerConsumption")?, number(class, "mEstimatedMaximumPowerConsumption")?);
        Some((low + high) / 2.0).filter(|mw| *mw > 0.0)
    };
    steady.or_else(middle)
}
