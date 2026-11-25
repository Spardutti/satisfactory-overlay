use super::nodes::NodeSupply;
use super::transport::BeltGroup;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub item: String,
    pub item_name: String,
    pub recipe_id: String,
    pub recipe_name: String,
    pub building_name: String,
    pub machines_exact: f64,
    pub machines: u32,
    /// clock of the last machine, 100 when the count is exact
    pub last_machine_pct: f64,
    pub output_per_min: f64,
    /// 1 for machines fed only by raw resources, +1 per layer above
    pub level: u32,
    /// what one machine makes per minute at 100%
    pub per_machine: f64,
    pub inputs: Vec<Flow>,
    pub output: Flow,
    pub goes_to: Vec<Usage>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawNeed {
    pub item: String,
    pub item_name: String,
    pub per_min: f64,
    pub fluid: bool,
    pub miners: Option<u32>,
    /// each miner makes more than one belt carries, so it is held back
    pub capped: bool,
    /// miners per output belt; for fluids and gathered items, one group per line
    pub belts: Vec<BeltGroup>,
    pub goes_to: Vec<Usage>,
    pub nodes: Option<NodeSupply>,
}

/// Where an item goes next: which product's machines take how much.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub item: String,
    pub item_name: String,
    pub per_min: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Surplus {
    pub item: String,
    pub item_name: String,
    pub per_min: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanResult {
    pub target: String,
    pub target_name: String,
    pub rate: f64,
    pub raw: Vec<RawNeed>,
    /// in build order: extraction side first, final product last
    pub steps: Vec<Step>,
    pub surplus: Vec<Surplus>,
}

/// One item moving into or out of a step, and how its machines share belt or pipe lines.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Flow {
    pub item: String,
    pub item_name: String,
    pub per_min: f64,
    pub fluid: bool,
    pub belts: Vec<BeltGroup>,
}

/// Whole machines needed, ignoring float noise like 3.0000000001.
pub fn whole_machines(exact: f64) -> u32 {
    (exact - 1e-9).ceil().max(0.0) as u32
}

pub fn last_machine_pct(exact: f64, machines: u32) -> f64 {
    if machines == 0 {
        return 0.0;
    }
    (exact - f64::from(machines - 1)) * 100.0
}
