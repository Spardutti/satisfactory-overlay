use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("Bad JSON in {}: {e}", path.display()))
}

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("Cannot create {}: {e}", dir.display()))?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| format!("Cannot write {}: {e}", path.display()))
}

/// Reads `path`, first writing `default_text` there if the file does not exist yet.
pub fn read_or_create<T: DeserializeOwned>(path: &Path, default_text: &str) -> Result<T, String> {
    if !path.exists() {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        std::fs::write(path, default_text).map_err(|e| format!("Cannot write {}: {e}", path.display()))?;
    }
    read_json(path)
}
