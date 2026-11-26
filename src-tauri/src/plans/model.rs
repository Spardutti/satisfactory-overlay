use crate::planner::NodeCounts;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Mode {
    Rate { rate: f64 },
    Resources { available: HashMap<String, f64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    pub target: String,
    pub mode: Mode,
    pub miner_tier: String,
    #[serde(default = "first_tier")]
    pub belt_tier: String,
    #[serde(default = "first_tier")]
    pub pipe_tier: String,
    #[serde(default)]
    pub choices: HashMap<String, String>,
    #[serde(default)]
    pub nodes: BTreeMap<String, NodeCounts>,
    /// items the player already makes elsewhere: item → per minute on hand
    #[serde(default)]
    pub depot: BTreeMap<String, f64>,
}

impl PlanRequest {
    pub fn set_nodes(&mut self, item: String, counts: NodeCounts) {
        match counts == NodeCounts::default() {
            true => self.nodes.remove(&item),
            false => self.nodes.insert(item, counts),
        };
    }
}

impl PlanRequest {
    /// Sets what a depot sends of one item; None or 0 takes the depot away. Refuses negative or broken amounts.
    pub fn set_depot(&mut self, item: String, per_min: Option<f64>) -> Result<(), String> {
        match per_min {
            None | Some(0.0) => {
                self.depot.remove(&item);
            }
            Some(n) if n.is_finite() && n > 0.0 => {
                self.depot.insert(item, n);
            }
            Some(_) => return Err("Enter an amount above 0".into()),
        }
        Ok(())
    }
}

/// Plans saved before belts and pipes existed get the slowest tier.
fn first_tier() -> String {
    "Mk1".into()
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistStep {
    pub label: String,
    pub detail: String,
    pub total: u32,
    pub built: u32,
    /// one line per input, e.g. "Iron Ore: 3 belts · 2 each (60)"
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub goes_to: String,
    /// the item this card makes or mines
    #[serde(default)]
    pub item: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub request: PlanRequest,
    pub steps: Vec<ChecklistStep>,
    /// blueprint blocks the user moved by hand: item → [x, y]
    #[serde(default)]
    pub positions: BTreeMap<String, [f64; 2]>,
    /// machines ticked as built in the blueprint: item → machine rows
    #[serde(default)]
    pub built: BTreeMap<String, BTreeSet<u32>>,
}

impl Plan {
    /// Ticks or unticks one machine; a row its step does not have is ignored.
    pub fn toggle_built(&mut self, item: &str, row: u32) {
        let Some(step) = self.steps.iter().find(|s| s.item == item) else { return };
        if row >= step.total {
            return;
        }
        let rows = self.built.entry(item.to_string()).or_default();
        if !rows.remove(&row) {
            rows.insert(row);
        }
        self.count_built();
    }

    pub fn clear_built(&mut self) {
        self.built.clear();
        self.count_built();
    }

    /// Entering nodes reorders an ore's miners, so its ticks start over.
    pub fn set_nodes(&mut self, item: String, counts: NodeCounts) {
        self.built.remove(&item);
        self.request.set_nodes(item, counts);
        self.count_built();
    }

    /// Replaces the steps; ticks go away on machines that are gone or now another recipe.
    pub fn replace_steps(&mut self, steps: Vec<ChecklistStep>) {
        let old: HashMap<String, String> = self.steps.iter().map(|s| (s.item.clone(), s.label.clone())).collect();
        self.steps = steps;
        let kept: HashMap<&str, u32> = self
            .steps
            .iter()
            .filter(|s| old.get(&s.item) == Some(&s.label))
            .map(|s| (s.item.as_str(), s.total))
            .collect();
        self.built.retain(|item, rows| {
            let total = kept.get(item.as_str()).copied().unwrap_or(0);
            rows.retain(|row| *row < total);
            !rows.is_empty()
        });
        self.count_built();
    }

    fn count_built(&mut self) {
        for step in &mut self.steps {
            step.built = self.built.get(&step.item).map_or(0, |rows| rows.len() as u32);
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanStore {
    pub plans: Vec<Plan>,
}

impl PlanStore {
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Plan> {
        self.plans.iter_mut().find(|p| p.id == id)
    }

    pub fn upsert(&mut self, plan: Plan) {
        match self.plans.iter_mut().find(|p| p.id == plan.id) {
            Some(existing) => *existing = plan,
            None => self.plans.push(plan),
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.plans.retain(|p| p.id != id);
    }
}
