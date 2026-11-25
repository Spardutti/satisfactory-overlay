mod json_file;

pub use json_file::{read_or_backup, write_json};

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

const DEFAULT_RAW: &str = include_str!("../../../config/raw_resources.json");
const DEFAULT_EXTRACTORS: &str = include_str!("../../../config/extractors.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hotkeys {
    pub toggle_mode: String,
    pub toggle_visible: String,
    pub mark_next: String,
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self { toggle_mode: "F8".into(), toggle_visible: "F9".into(), mark_next: "Ctrl+F8".into() }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppConfig {
    pub docs_path: Option<PathBuf>,
    pub enabled_alternates: BTreeSet<String>,
    pub hotkeys: Hotkeys,
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
        Ok(base * factor)
    }
}

/// The user-editable files in the app config folder, created from defaults on first run.
pub struct Editable {
    pub raw: HashSet<String>,
    pub extractors: Extractors,
}

pub fn load_editable(config_dir: &Path, notices: &mut Vec<String>) -> Editable {
    let raw: Vec<String> =
        json_file::read_user_file(&config_dir.join("raw_resources.json"), DEFAULT_RAW, notices);
    let extractors =
        json_file::read_user_file(&config_dir.join("extractors.json"), DEFAULT_EXTRACTORS, notices);
    Editable { raw: raw.into_iter().collect(), extractors }
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
    fn miner_rate_mk1_on_impure_node_is_30() {
        assert_eq!(extractors().miner_rate("Mk1", "impure"), Ok(30.0));
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

    #[test]
    fn read_or_backup_missing_config_returns_default_hotkeys() {
        let config: AppConfig = read_or_backup(Path::new("/nonexistent/config.json"), &mut vec![]);
        assert_eq!(config.hotkeys.mark_next, "Ctrl+F8");
    }
}
