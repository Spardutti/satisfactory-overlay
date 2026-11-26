use super::PlanResult;
use std::collections::{BTreeMap, HashMap};

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

/// Marks the sources that come from the player's depot, which arrive on plain lines with nothing built.
pub fn mark_depot(result: &mut PlanResult, depot: &BTreeMap<String, f64>) {
    for raw in result.raw.iter_mut().filter(|r| depot.contains_key(&r.item)) {
        raw.from_depot = true;
        (raw.miners, raw.extractor, raw.nodes, raw.capped) = (None, None, None, false);
        raw.belts.iter_mut().for_each(|line| line.machines = 0);
    }
}

/// What the player has on hand in resources mode: typed amounts plus what the depot sends.
pub fn on_hand(available: &HashMap<String, f64>, depot: &BTreeMap<String, f64>) -> HashMap<String, f64> {
    let mut all = available.clone();
    for (item, n) in depot {
        *all.entry(item.clone()).or_default() += n;
    }
    all
}
