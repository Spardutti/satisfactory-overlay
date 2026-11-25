use crate::game_data::GameData;
use crate::plans::PlanStore;
use crate::settings::{self, AppConfig, Editable};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct AppState {
    config_path: PathBuf,
    plans_path: PathBuf,
    pub editable: Editable,
    pub config: Mutex<AppConfig>,
    pub game: Mutex<Option<GameData>>,
    pub plans: Mutex<PlanStore>,
    pub overlay_interactive: AtomicBool,
    pub hotkey_errors: Mutex<Vec<String>>,
}

impl AppState {
    pub fn load(config_dir: &Path, data_dir: &Path) -> Result<Self, String> {
        let config_path = config_dir.join("config.json");
        let plans_path = data_dir.join("plans.json");
        let plans = settings::read_json(&plans_path).unwrap_or_default();
        Ok(Self {
            editable: settings::load_editable(config_dir)?,
            config: Mutex::new(settings::load_config(&config_path)),
            game: Mutex::new(None),
            plans: Mutex::new(plans),
            overlay_interactive: AtomicBool::new(true),
            hotkey_errors: Mutex::new(vec![]),
            config_path,
            plans_path,
        })
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
