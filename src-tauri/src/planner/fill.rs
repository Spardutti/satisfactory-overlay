use super::{PlanResult, RawNeed};

/// Room smaller than this is float noise, not a slow miner.
const ROOM: f64 = 1e-9;

/// What an ore's current miners make when every one runs as fast as its belt lets it.
pub fn miner_capacity(raw: &RawNeed) -> f64 {
    let Some(rates) = raw.extractor.as_ref().map(|e| e.rates) else { return 0.0 };
    let full = |rate: f64| rate.min(rates.belt);
    match &raw.nodes {
        Some(supply) => supply.draws.iter().map(|d| full(rates.of(&d.purity))).sum(),
        None => f64::from(raw.miners.unwrap_or(0)) * full(rates.normal),
    }
}

/// Ores whose miners could make more than the plan takes from them now.
pub fn fillable_ores(result: &PlanResult) -> Vec<String> {
    result
        .raw
        .iter()
        .filter(|r| r.miners.is_some() && miner_capacity(r) - r.per_min > ROOM)
        .map(|r| r.item.clone())
        .collect()
}
