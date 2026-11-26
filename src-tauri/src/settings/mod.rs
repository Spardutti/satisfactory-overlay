mod json_file;

pub use json_file::{read_or_backup, write_json};

use crate::planner::{Extractor, ExtractorBook, NodeRates, Transport};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

const DEFAULT_RAW: &str = include_str!("../../../config/raw_resources.json");
const DEFAULT_EXTRACTORS: &str = include_str!("../../../config/extractors.json");
const DEFAULT_LOGISTICS: &str = include_str!("../../../config/logistics.json");

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppConfig {
    pub docs_path: Option<PathBuf>,
    pub enabled_alternates: BTreeSet<String>,
    /// milestones and MAM research the player finished, by schematic id
    pub unlocked: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extractors {
    pub miners: BTreeMap<String, f64>,
    pub purity: BTreeMap<String, f64>,
    /// older user files lack this, so they get the built-in fluid extractors
    #[serde(default = "default_fluids")]
    pub fluids: BTreeMap<String, FluidExtractor>,
}

/// One fluid's extractor from extractors.json, e.g. Water → Water Extractor at 120 m³/min.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluidExtractor {
    pub building: String,
    /// per minute on a normal node
    pub rate: f64,
    pub purity: bool,
}

fn default_fluids() -> BTreeMap<String, FluidExtractor> {
    let built_in: serde_json::Value = serde_json::from_str(DEFAULT_EXTRACTORS).unwrap_or_default();
    serde_json::from_value(built_in["fluids"].clone()).unwrap_or_default()
}

impl Extractors {
    pub fn miner_rate(&self, tier: &str, purity: &str) -> Result<f64, String> {
        let base = self.miners.get(tier).ok_or(format!("Unknown miner tier {tier}"))?;
        let factor = self.purity.get(purity).ok_or(format!("Unknown node purity {purity}"))?;
        match base * factor {
            rate if rate > 0.0 => Ok(rate),
            _ => Err(format!("Miner {tier} on {purity} nodes needs a rate above 0 in extractors.json")),
        }
    }

    /// The extractors one plan uses: the miner of `tier`, and each fluid's, capped by its line.
    pub fn book(&self, tier: &str, transport: Transport) -> Result<ExtractorBook, String> {
        let miner = Extractor { building: format!("Miner {tier}"), rates: self.node_rates(tier, transport.belt)?, purity: true };
        let fluids = self.fluids.iter().map(|(item, f)| Ok((item.clone(), self.fluid_extractor(f, transport.pipe)?))).collect::<Result<_, String>>()?;
        Ok(ExtractorBook { miner, fluids })
    }

    fn fluid_extractor(&self, fluid: &FluidExtractor, pipe: f64) -> Result<Extractor, String> {
        let factor = |purity: &str| if fluid.purity { self.purity.get(purity).copied().unwrap_or(0.0) } else { 1.0 };
        let rates = NodeRates { pure: fluid.rate * factor("pure"), normal: fluid.rate * factor("normal"), impure: fluid.rate * factor("impure"), belt: pipe };
        match rates.impure > 0.0 && rates.normal > 0.0 && rates.pure > 0.0 {
            true => Ok(Extractor { building: fluid.building.clone(), rates, purity: fluid.purity }),
            false => Err(format!("{} needs a rate above 0 in extractors.json", fluid.building)),
        }
    }

    /// What one miner of `tier` makes on each purity, with the `belt` that carries it.
    pub fn node_rates(&self, tier: &str, belt: f64) -> Result<NodeRates, String> {
        let rate = |purity: &str| self.miner_rate(tier, purity);
        Ok(NodeRates { pure: rate("pure")?, normal: rate("normal")?, impure: rate("impure")?, belt })
    }
}

/// Items per minute one belt or pipe line carries, per tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logistics {
    pub belts: BTreeMap<String, f64>,
    pub pipes: BTreeMap<String, f64>,
}

impl Logistics {
    pub fn belt_speed(&self, tier: &str) -> Result<f64, String> {
        positive_speed(&self.belts, "belt", tier)
    }

    pub fn pipe_speed(&self, tier: &str) -> Result<f64, String> {
        positive_speed(&self.pipes, "pipe", tier)
    }
}

/// A speed of 0 would ask for endless belt lines, so it is refused.
fn positive_speed(speeds: &BTreeMap<String, f64>, kind: &str, tier: &str) -> Result<f64, String> {
    match speeds.get(tier) {
        None => Err(format!("Unknown {kind} tier {tier}")),
        Some(speed) if *speed > 0.0 => Ok(*speed),
        Some(_) => Err(format!("The {kind} tier {tier} needs a speed above 0 in logistics.json")),
    }
}

/// The user-editable files in the app config folder, created from defaults on first run.
pub struct Editable {
    pub raw: HashSet<String>,
    pub extractors: Extractors,
    pub logistics: Logistics,
}

pub fn load_editable(config_dir: &Path, notices: &mut Vec<String>) -> Editable {
    let raw: Vec<String> =
        json_file::read_user_file(&config_dir.join("raw_resources.json"), DEFAULT_RAW, notices);
    let extractors =
        json_file::read_user_file(&config_dir.join("extractors.json"), DEFAULT_EXTRACTORS, notices);
    let logistics =
        json_file::read_user_file(&config_dir.join("logistics.json"), DEFAULT_LOGISTICS, notices);
    Editable { raw: raw.into_iter().collect(), extractors, logistics }
}

#[cfg(test)]
mod tests;
