mod book;
mod depot;
mod expand;
mod extractor;
mod fill;
mod hint;
mod limit;
mod nodes;
mod walk;
mod result;
mod targets;
mod transport;
mod flows;

pub use depot::{depot_cap, mark_depot, on_hand};
pub use extractor::{Extractor, ExtractorBook};
pub use fill::{fillable_ores, miner_capacity};
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
    /// recipes the player owns; without unlock data in the game file, just the ticked alternates
    pub enabled_alternates: &'a HashSet<String>,
    /// the extractor each raw item gets, for the chosen tiers
    pub extractors: &'a ExtractorBook,
    pub transport: Transport,
}

#[cfg(test)]
mod choice_tests;
#[cfg(test)]
mod depot_tests;
#[cfg(test)]
mod extractor_tests;
#[cfg(test)]
mod fill_tests;
#[cfg(test)]
mod flow_tests;
#[cfg(test)]
mod power_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod unlock_tests;
