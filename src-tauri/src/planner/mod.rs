mod book;
mod expand;
mod limit;
mod walk;
mod result;

pub use limit::{Limit, RawLeft};
pub use result::{PlanResult, RawNeed, Step, Surplus};

use crate::game_data::GameData;
use std::collections::{HashMap, HashSet};

pub struct Planner<'a> {
    pub data: &'a GameData,
    pub raw: &'a HashSet<String>,
    /// item id → recipe id the user picked for it
    pub choices: &'a HashMap<String, String>,
    pub enabled_alternates: &'a HashSet<String>,
    /// items per minute one miner extracts, for the chosen tier and purity
    pub miner_rate: f64,
}

#[cfg(test)]
mod choice_tests;
#[cfg(test)]
mod tests;
