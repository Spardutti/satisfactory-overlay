use super::NodeRates;
use crate::game_data::GameData;
use serde::Serialize;
use std::collections::HashMap;

/// The building that pulls a raw item out of the ground, and how fast on each node purity.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Extractor {
    /// e.g. "Miner Mk2", "Water Extractor", "Oil Extractor"
    pub building: String,
    /// `belt` holds the belt or pipe limit of the line it fills
    pub rates: NodeRates,
    /// false when every spot gives the same, like water from a lake
    pub purity: bool,
}

/// Which extractor each raw item gets for one plan's tiers.
#[derive(Debug, Clone)]
pub struct ExtractorBook {
    pub miner: Extractor,
    /// fluid item id → its extractor
    pub fluids: HashMap<String, Extractor>,
}

impl ExtractorBook {
    /// A Mk1-style miner of `normal` per minute on a `belt` line, and no fluid extractors.
    #[cfg(test)]
    pub fn miner_only(normal: f64, belt: f64) -> Self {
        let rates = NodeRates { pure: normal * 2.0, normal, impure: normal / 2.0, belt };
        ExtractorBook { miner: Extractor { building: "Miner Mk1".into(), rates, purity: true }, fluids: HashMap::new() }
    }

    /// The miner for solid ore, the configured extractor for a fluid, or None to bring it in.
    pub fn for_item(&self, data: &GameData, item: &str) -> Option<&Extractor> {
        match data.is_minable(item) {
            true => Some(&self.miner),
            false => self.fluids.get(item),
        }
    }
}
