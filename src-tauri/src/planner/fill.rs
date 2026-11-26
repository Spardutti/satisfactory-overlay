use super::{NodeRates, RawNeed};

/// What an ore's current miners make when every one runs as fast as its belt lets it.
pub fn miner_capacity(raw: &RawNeed, rates: NodeRates) -> f64 {
    let full = |rate: f64| rate.min(rates.belt);
    match &raw.nodes {
        Some(supply) => supply.draws.iter().map(|d| full(rates.of(&d.purity))).sum(),
        None => f64::from(raw.miners.unwrap_or(0)) * full(rates.normal),
    }
}
