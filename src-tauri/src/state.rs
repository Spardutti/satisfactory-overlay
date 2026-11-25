use crate::game_data::GameData;
use crate::plans::PlanStore;
use crate::settings::{self, AppConfig, Editable};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct AppState {
    config_path: PathBuf,
    plans_path: PathBuf,
    pub editable: Editable,
    pub config: Mutex<AppConfig>,
    pub game: Mutex<Option<GameData>>,
    pub plans: Mutex<PlanStore>,
    pub notices: Mutex<Vec<String>>,
}

impl AppState {
    /// Damaged or broken files never stop the start; they become notices for the UI.
    pub fn load(config_dir: &Path, data_dir: &Path) -> Self {
        let config_path = config_dir.join("config.json");
        let plans_path = data_dir.join("plans.json");
        let mut notices = vec![];
        Self {
            editable: settings::load_editable(config_dir, &mut notices),
            config: Mutex::new(settings::read_or_backup(&config_path, &mut notices)),
            game: Mutex::new(None),
            plans: Mutex::new(settings::read_or_backup(&plans_path, &mut notices)),
            notices: Mutex::new(notices),
            config_path,
            plans_path,
        }
    }

    pub fn save_config(&self, config: &AppConfig) -> Result<(), String> {
        settings::write_json(&self.config_path, config)
    }

    pub fn save_plans(&self, plans: &PlanStore) -> Result<(), String> {
        settings::write_json(&self.plans_path, plans)
    }
}

pub fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sf-state-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn load_with_damaged_plans_file_keeps_a_backup() {
        let dir = temp_dir("plans");
        std::fs::write(dir.join("plans.json"), "{ broken").unwrap();
        let state = AppState::load(&dir, &dir);
        assert_eq!(std::fs::read_to_string(dir.join("plans.json.bad")).unwrap(), "{ broken");
        assert_eq!(lock(&state.notices).len(), 1);
    }

    #[test]
    fn load_with_bad_extractors_file_starts_with_defaults() {
        let dir = temp_dir("extractors");
        std::fs::write(dir.join("extractors.json"), "{ broken").unwrap();
        let state = AppState::load(&dir, &dir);
        assert_eq!(state.editable.extractors.miner_rate("Mk1", "normal"), Ok(60.0));
    }
}
