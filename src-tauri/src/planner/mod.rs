mod book;
mod expand;
mod limit;
mod walk;
mod result;
mod targets;
mod transport;
mod flows;

pub use limit::{Limit, RawLeft};
pub use result::{Flow, PlanResult, RawNeed, Step, Surplus, Usage};
pub use targets::{targets, Target};
pub use transport::{BeltGroup, Transport};

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
    pub transport: Transport,
}

#[cfg(test)]
mod choice_tests;
#[cfg(test)]
mod flow_tests;
#[cfg(test)]
mod tests;
