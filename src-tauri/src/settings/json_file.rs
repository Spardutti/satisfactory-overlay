use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("Bad JSON in {}: {e}", path.display()))
}

/// Writes through a temp file and a rename, so a crash never leaves half a file.
pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("Cannot create {}: {e}", dir.display()))?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    let temp = with_suffix(path, "tmp");
    std::fs::write(&temp, text).map_err(|e| format!("Cannot write {}: {e}", temp.display()))?;
    std::fs::rename(&temp, path).map_err(|e| format!("Cannot replace {}: {e}", path.display()))
}

/// Reads a file the app writes; a damaged one is moved to `<name>.bad` and the default is used.
pub fn read_or_backup<T: DeserializeOwned + Default>(path: &Path, notices: &mut Vec<String>) -> T {
    if !path.exists() {
        return T::default();
    }
    read_json(path).unwrap_or_else(|error| {
        let backup = with_suffix(path, "bad");
        let _ = std::fs::rename(path, &backup);
        notices.push(format!("{error}. Started empty; the old file is {}", backup.display()));
        T::default()
    })
}

/// Reads a file the user edits, creating it from `default_text` first if it is missing.
/// A broken file is left alone for the user to fix, and the defaults are used.
pub fn read_user_file<T: DeserializeOwned>(path: &Path, default_text: &str, notices: &mut Vec<String>) -> T {
    if !path.exists() {
        let _ = std::fs::create_dir_all(path.parent().unwrap_or(Path::new(".")));
        if let Err(e) = std::fs::write(path, default_text) {
            notices.push(format!("Cannot write {}: {e}", path.display()));
        }
    }
    read_json(path).unwrap_or_else(|error| {
        notices.push(format!("{error}. Using the built-in defaults until it is fixed."));
        serde_json::from_str(default_text).expect("built-in default JSON is valid")
    })
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(format!(".{suffix}"));
    PathBuf::from(name)
}
