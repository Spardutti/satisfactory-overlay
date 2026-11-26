mod docs;
mod encoding;
mod locate;
mod machine_power;
mod model;
mod power;
mod unlocks;
mod unreal;

pub use locate::find_docs_file;
pub use machine_power::clocked_mw;
pub use model::{GameData, Item, ItemAmount, Recipe};
pub use power::{power_options, PowerOption, POWER};
pub use unlocks::{Unlock, UnlockBook, UnlockKind};

use std::path::PathBuf;

/// Where recipes come from; a game-API source can replace the docs file later.
pub trait RecipeSource {
    fn load(&self) -> Result<GameData, String>;
}

pub struct GameDocsSource {
    pub path: PathBuf,
}

impl RecipeSource for GameDocsSource {
    fn load(&self) -> Result<GameData, String> {
        let bytes = std::fs::read(&self.path)
            .map_err(|e| format!("Cannot read {}: {e}", self.path.display()))?;
        docs::parse_docs(&encoding::decode_text(&bytes)?)
    }
}

#[cfg(test)]
pub fn fixture() -> GameData {
    docs::parse_docs(include_str!("../../tests/fixtures/docs_mini.json")).unwrap()
}

#[cfg(test)]
mod unlocks_tests;
#[cfg(test)]
mod power_tests;
#[cfg(test)]
mod machine_power_tests;
