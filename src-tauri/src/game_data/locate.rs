use std::path::{Path, PathBuf};

const DOCS_FILES: [&str; 2] = ["en-US.json", "Docs.json"];
const WINDOWS_DRIVES: [&str; 5] = ["C", "D", "E", "F", "G"];

/// Looks for the docs file in the usual Steam and Epic install folders.
pub fn find_docs_file() -> Option<PathBuf> {
    first_docs_file(&game_dirs())
}

pub fn first_docs_file(game_dirs: &[PathBuf]) -> Option<PathBuf> {
    game_dirs
        .iter()
        .flat_map(|dir| DOCS_FILES.iter().map(move |f| docs_dir(dir).join(f)))
        .find(|path| path.is_file())
}

fn docs_dir(game_dir: &Path) -> PathBuf {
    game_dir.join("CommunityResources").join("Docs")
}

fn game_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common\Satisfactory"),
        PathBuf::from(r"C:\Program Files\Epic Games\Satisfactory"),
        PathBuf::from(r"C:\Program Files\Epic Games\SatisfactoryEarlyAccess"),
        PathBuf::from(r"C:\Program Files\Epic Games\SatisfactoryExperimental"),
    ];
    dirs.extend(
        WINDOWS_DRIVES
            .iter()
            .map(|d| PathBuf::from(format!(r"{d}:\SteamLibrary\steamapps\common\Satisfactory"))),
    );
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        dirs.push(home.join(".steam/steam/steamapps/common/Satisfactory"));
        dirs.push(home.join(".local/share/Steam/steamapps/common/Satisfactory"));
    }
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_game_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sf-locate-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(docs_dir(&dir)).unwrap();
        dir
    }

    #[test]
    fn first_docs_file_prefers_en_us_over_docs_json() {
        let dir = temp_game_dir("both");
        fs::write(docs_dir(&dir).join("Docs.json"), "[]").unwrap();
        fs::write(docs_dir(&dir).join("en-US.json"), "[]").unwrap();
        assert_eq!(first_docs_file(std::slice::from_ref(&dir)), Some(docs_dir(&dir).join("en-US.json")));
    }

    #[test]
    fn first_docs_file_falls_back_to_old_docs_json() {
        let dir = temp_game_dir("old");
        fs::write(docs_dir(&dir).join("Docs.json"), "[]").unwrap();
        assert_eq!(first_docs_file(std::slice::from_ref(&dir)), Some(docs_dir(&dir).join("Docs.json")));
    }

    #[test]
    fn first_docs_file_without_docs_returns_none() {
        assert_eq!(first_docs_file(&[temp_game_dir("empty")]), None);
    }
}
