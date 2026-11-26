use super::{Mode, PlanRequest};
use crate::planner::RawNeed;

/// Room smaller than this is float noise, not a slow miner.
const EPS: f64 = 1e-9;

impl PlanRequest {
    /// Grows the plan so this ore's miners make `capacity`; false when they already do.
    pub fn grow_to_fill(&mut self, raw: &RawNeed, capacity: f64) -> bool {
        if raw.per_min <= EPS || capacity - raw.per_min <= EPS {
            return false;
        }
        match &mut self.mode {
            Mode::Rate { rate } => *rate *= capacity / raw.per_min,
            Mode::Resources { available } => {
                let amount = available.entry(raw.item.clone()).or_insert(0.0);
                if *amount >= capacity {
                    return false;
                }
                *amount = capacity;
            }
        }
        true
    }
}
