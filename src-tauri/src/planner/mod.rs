mod book;
mod expand;
mod fill;
mod limit;
mod nodes;
mod walk;
mod result;
mod targets;
mod transport;
mod flows;

pub use fill::miner_capacity;
pub use limit::{Limit, RawLeft};
pub use nodes::{apply_nodes, draw_from_nodes, node_limit, NodeCounts, NodeDraw, NodeRates, NodeSupply};
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
    /// items per minute one miner extracts on a normal node, for the chosen tier
    pub miner_rate: f64,
    pub transport: Transport,
}

#[cfg(test)]
mod choice_tests;
#[cfg(test)]
mod fill_tests;
#[cfg(test)]
mod flow_tests;
#[cfg(test)]
mod tests;
