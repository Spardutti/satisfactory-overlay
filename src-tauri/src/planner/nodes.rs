use super::{BeltGroup, PlanResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeCounts {
    pub pure: u32,
    pub normal: u32,
    pub impure: u32,
}

/// What one miner makes per minute on each purity, and what one belt carries away from it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct NodeRates {
    pub pure: f64,
    pub normal: f64,
    pub impure: f64,
    pub belt: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDraw {
    pub purity: String,
    pub per_min: f64,
    pub clock: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSupply {
    pub draws: Vec<NodeDraw>,
    pub spare: NodeCounts,
    pub short: f64,
}

const EPS: f64 = 1e-9;

impl NodeRates {
    pub(super) fn of(&self, purity: &str) -> f64 {
        match purity {
            "pure" => self.pure,
            "normal" => self.normal,
            _ => self.impure,
        }
    }
}

/// Fills `need` from the best nodes first; the last miner used is slowed down to fit.
pub fn draw_from_nodes(need: f64, counts: NodeCounts, rates: NodeRates) -> NodeSupply {
    let mut left = need;
    let mut draws = vec![];
    let by_purity = [("pure", counts.pure, rates.pure), ("normal", counts.normal, rates.normal), ("impure", counts.impure, rates.impure)];
    for (purity, count, rate) in by_purity {
        for _ in 0..count {
            if left <= EPS {
                break;
            }
            let per_min = rate.min(rates.belt).min(left);
            draws.push(NodeDraw { purity: purity.into(), per_min, clock: per_min / rate * 100.0 });
            left -= per_min;
        }
    }
    let used = |purity: &str| draws.iter().filter(|d| d.purity == purity).count() as u32;
    let spare = NodeCounts { pure: counts.pure - used("pure"), normal: counts.normal - used("normal"), impure: counts.impure - used("impure") };
    NodeSupply { draws, spare, short: if left > EPS { left } else { 0.0 } }
}

/// Puts the player's nodes on each source they listed, so its extractors, lines and shortfall follow them.
pub fn apply_nodes(result: &mut PlanResult, nodes: &BTreeMap<String, NodeCounts>) {
    for raw in result.raw.iter_mut() {
        let Some(rates) = raw.extractor.as_ref().filter(|e| e.purity).map(|e| e.rates) else { continue };
        let Some(&counts) = nodes.get(&raw.item).filter(|c| **c != NodeCounts::default()) else { continue };
        let supply = draw_from_nodes(raw.per_min, counts, rates);
        raw.miners = Some(supply.draws.len() as u32);
        raw.capped = supply.draws.iter().any(|d| rates.of(&d.purity) > rates.belt);
        raw.belts = supply.draws.iter().map(|d| BeltGroup { machines: 1, per_min: d.per_min, over: false }).collect();
        raw.nodes = Some(supply);
    }
}

/// The most target per minute the listed nodes allow, when any ore runs short.
pub fn node_limit(result: &PlanResult) -> Option<f64> {
    result
        .raw
        .iter()
        .filter_map(|r| r.nodes.as_ref().filter(|n| n.short > 0.0).map(|n| (r.per_min - n.short) / r.per_min))
        .min_by(f64::total_cmp)
        .map(|share| result.rate * share)
}

#[cfg(test)]
#[path = "node_tests.rs"]
mod tests;
