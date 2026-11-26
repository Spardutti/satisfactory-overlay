mod card_text;
mod checklist;
mod fill;
mod model;

pub use checklist::checklist;
pub use model::{ChecklistStep, Mode, Plan, PlanRequest, PlanStore};

/// Where built-machine counts come from; a game-API source can fill them in later.
pub trait ProgressSource {
    fn sync(&self, plan: &mut Plan);
}

/// Progress is whatever the user ticked, already stored in the plan.
pub struct ManualProgress;

impl ProgressSource for ManualProgress {
    fn sync(&self, _plan: &mut Plan) {}
}

#[cfg(test)]
mod fill_tests;
#[cfg(test)]
mod tests;
