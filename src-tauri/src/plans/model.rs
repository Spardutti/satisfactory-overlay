use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub purity: String,
    #[serde(default)]
    pub choices: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistStep {
    pub label: String,
    pub detail: String,
    pub total: u32,
    pub built: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub request: PlanRequest,
    pub steps: Vec<ChecklistStep>,
}

impl Plan {
    /// Builds one more machine of the first unfinished step; false when all are built.
    pub fn mark_next(&mut self) -> bool {
        match self.steps.iter_mut().find(|s| s.built < s.total) {
            Some(step) => {
                step.built += 1;
                true
            }
            None => false,
        }
    }

    pub fn adjust(&mut self, step: usize, delta: i32) {
        if let Some(s) = self.steps.get_mut(step) {
            s.built = s.built.saturating_add_signed(delta).min(s.total);
        }
    }

    /// Replaces the steps, keeping what was built on steps with the same label.
    pub fn replace_steps(&mut self, mut steps: Vec<ChecklistStep>) {
        for step in &mut steps {
            let old = self.steps.iter().find(|o| o.label == step.label);
            step.built = old.map_or(0, |o| o.built.min(step.total));
        }
        self.steps = steps;
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanStore {
    pub active: Option<String>,
    pub plans: Vec<Plan>,
}

impl PlanStore {
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Plan> {
        self.plans.iter_mut().find(|p| p.id == id)
    }

    pub fn active_mut(&mut self) -> Option<&mut Plan> {
        let id = self.active.clone()?;
        self.get_mut(&id)
    }

    pub fn upsert(&mut self, plan: Plan) {
        self.active.get_or_insert_with(|| plan.id.clone());
        match self.plans.iter_mut().find(|p| p.id == plan.id) {
            Some(existing) => *existing = plan,
            None => self.plans.push(plan),
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.plans.retain(|p| p.id != id);
        if self.active.as_deref() == Some(id) {
            self.active = self.plans.first().map(|p| p.id.clone());
        }
    }

    pub fn set_active(&mut self, id: &str) {
        if self.plans.iter().any(|p| p.id == id) {
            self.active = Some(id.to_string());
        }
    }
}
