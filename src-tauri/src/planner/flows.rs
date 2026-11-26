use super::result::{Flow, RawNeed, Step, Usage};
use super::transport::{belt_groups, line_groups, BeltGroup};
use super::Planner;
use crate::game_data::Recipe;
use std::collections::HashMap;

impl Planner<'_> {
    /// The step's output and inputs, with its machines grouped onto belt or pipe lines.
    pub(super) fn step_flows(&self, item: &str, recipe: &Recipe, machines_exact: f64) -> (Flow, Vec<Flow>) {
        let made = recipe.products.iter().find(|p| p.item == item).map_or(0.0, |p| p.amount);
        let output = self.flow(item, recipe.rate_per_min(made), machines_exact);
        let inputs = recipe
            .ingredients
            .iter()
            .map(|i| self.flow(&i.item, recipe.rate_per_min(i.amount), machines_exact))
            .collect();
        (output, inputs)
    }

    fn flow(&self, item: &str, per_machine: f64, machines_exact: f64) -> Flow {
        let fluid = self.data.is_fluid(item);
        Flow {
            item: item.to_string(),
            item_name: self.data.item_name(item),
            per_min: per_machine * machines_exact,
            fluid,
            belts: belt_groups(per_machine, machines_exact, self.transport.capacity(fluid)),
        }
    }

    /// Extractors grouped per output line; `each` is what one really delivers, None when brought in.
    pub(super) fn raw_belts(&self, item: &str, per_min: f64, each: Option<f64>) -> Vec<BeltGroup> {
        let capacity = self.transport.capacity(self.data.is_fluid(item));
        match each {
            Some(each) => belt_groups(each, per_min / each, capacity),
            None => line_groups(per_min, capacity),
        }
    }
}

/// Fills each step's and raw resource's `goes_to` from the inputs of the steps that use it.
pub(super) fn link_destinations(steps: &mut [Step], raw: &mut [RawNeed]) {
    let mut usage: HashMap<String, Vec<Usage>> = HashMap::new();
    for step in steps.iter() {
        for input in &step.inputs {
            let used = Usage { item: step.item.clone(), item_name: step.item_name.clone(), per_min: input.per_min };
            usage.entry(input.item.clone()).or_default().push(used);
        }
    }
    for step in steps.iter_mut() {
        step.goes_to = usage.remove(&step.item).unwrap_or_default();
    }
    for need in raw.iter_mut() {
        need.goes_to = usage.remove(&need.item).unwrap_or_default();
    }
}
