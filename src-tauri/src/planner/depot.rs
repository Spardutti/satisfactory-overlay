use super::PlanResult;
use std::collections::BTreeMap;

/// A share this close to 1 is float noise, not a short depot.
const EPS: f64 = 1e-9;

/// The most target per minute the depot allows, when it cannot cover the plan's rate.
pub fn depot_cap(result: &PlanResult, depot: &BTreeMap<String, f64>) -> Option<f64> {
    result
        .raw
        .iter()
        .filter(|r| r.per_min > EPS)
        .filter_map(|r| depot.get(&r.item).map(|supply| supply / r.per_min))
        .min_by(f64::total_cmp)
        .filter(|share| *share < 1.0 - EPS)
        .map(|share| result.rate * share)
}

/// Marks the sources that come from the player's depot instead of a miner.
pub fn mark_depot(result: &mut PlanResult, depot: &BTreeMap<String, f64>) {
    for raw in &mut result.raw {
        raw.from_depot = depot.contains_key(&raw.item);
    }
}
