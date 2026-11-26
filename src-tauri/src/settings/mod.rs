mod json_file;

pub use json_file::{read_or_backup, write_json};

use crate::planner::NodeRates;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extractors {
    pub miners: BTreeMap<String, f64>,
    pub purity: BTreeMap<String, f64>,
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
mod tests {
    use super::*;

    fn extractors() -> Extractors {
        serde_json::from_str(DEFAULT_EXTRACTORS).unwrap()
    }

    #[test]
    fn miner_rate_mk2_on_pure_node_is_240() {
        assert_eq!(extractors().miner_rate("Mk2", "pure"), Ok(240.0));
    }

    #[test]
    fn node_rates_mk2_on_mk1_belt_keeps_full_miner_rates() {
        let rates = extractors().node_rates("Mk2", 60.0).unwrap();
        assert_eq!(rates, NodeRates { pure: 240.0, normal: 120.0, impure: 60.0, belt: 60.0 });
    }

    #[test]
    fn miner_rate_mk1_on_impure_node_is_30() {
        assert_eq!(extractors().miner_rate("Mk1", "impure"), Ok(30.0));
    }

    #[test]
    fn miner_rate_of_zero_is_an_error() {
        let mut zero = extractors();
        zero.miners.insert("Mk1".into(), 0.0);
        assert_eq!(
            zero.miner_rate("Mk1", "normal"),
            Err("Miner Mk1 on normal nodes needs a rate above 0 in extractors.json".into())
        );
    }

    #[test]
    fn miner_rate_unknown_tier_is_an_error() {
        assert_eq!(extractors().miner_rate("Mk9", "pure"), Err("Unknown miner tier Mk9".into()));
    }

    #[test]
    fn load_editable_first_run_writes_default_files() {
        let dir = std::env::temp_dir().join(format!("sf-settings-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let editable = load_editable(&dir, &mut vec![]);
        assert!(editable.raw.contains("Desc_OreIron_C"));
        assert!(dir.join("raw_resources.json").is_file());
        assert!(dir.join("extractors.json").is_file());
    }

    fn logistics() -> Logistics {
        serde_json::from_str(DEFAULT_LOGISTICS).unwrap()
    }

    #[test]
    fn belt_speed_mk3_is_270() {
        assert_eq!(logistics().belt_speed("Mk3"), Ok(270.0));
    }

    #[test]
    fn belt_speed_unknown_tier_is_an_error_naming_it() {
        assert_eq!(logistics().belt_speed("Mk9"), Err("Unknown belt tier Mk9".into()));
    }

    #[test]
    fn belt_speed_of_zero_is_an_error() {
        let zero = Logistics { belts: BTreeMap::from([("Mk1".to_string(), 0.0)]), pipes: BTreeMap::new() };
        assert_eq!(zero.belt_speed("Mk1"), Err("The belt tier Mk1 needs a speed above 0 in logistics.json".into()));
    }

    #[test]
    fn pipe_speed_mk2_is_600() {
        assert_eq!(logistics().pipe_speed("Mk2"), Ok(600.0));
    }

    #[test]
    fn read_or_backup_missing_config_returns_defaults() {
        let config: AppConfig = read_or_backup(Path::new("/nonexistent/config.json"), &mut vec![]);
        assert_eq!((config.docs_path, config.enabled_alternates.len()), (None, 0));
    }
}
