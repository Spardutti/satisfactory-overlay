use super::flows::link_destinations;
use super::result::{last_machine_pct, whole_machines, PlanResult, RawNeed, Step, Surplus};
use super::walk::Walk;
use super::Planner;
use crate::game_data::Recipe;
use std::collections::HashMap;

#[derive(Default)]
struct Flow {
    demand: HashMap<String, f64>,
    surplus: HashMap<String, f64>,
    steps: Vec<Step>,
}

impl Planner<'_> {
    /// Mode A: the machines needed to make `rate` of `target` per minute.
    pub fn plan_rate(&self, target: &str, rate: f64) -> Result<PlanResult, String> {
        if !rate.is_finite() || rate < 0.0 {
            return Err("The rate must be 0 or more".into());
        }
        let walk = self.walk(target)?;
        let mut flow = Flow::default();
        flow.demand.insert(target.to_string(), rate);
        for item in walk.order.iter().rev() {
            if let Some(recipe) = walk.chosen.get(item) {
                self.produce(item, recipe, &mut flow);
            }
        }
        let levels = levels(&walk);
        flow.steps.sort_by_key(|s| levels[&s.item]);
        flow.steps.iter_mut().for_each(|s| s.level = levels[&s.item]);
        let mut raw = self.raw_needs(&walk.order, &flow.demand);
        link_destinations(&mut flow.steps, &mut raw);
        Ok(PlanResult {
            target: target.to_string(),
            target_name: self.data.item_name(target),
            rate,
            raw,
            steps: flow.steps,
            surplus: self.surplus_list(flow.surplus),
        })
    }

    fn produce(&self, item: &str, recipe: &Recipe, flow: &mut Flow) {
        let needed = flow.demand.get(item).copied().unwrap_or(0.0);
        let made = recipe.products.iter().find(|p| p.item == item).map_or(0.0, |p| p.amount);
        let per_machine = recipe.rate_per_min(made);
        let machines_exact = needed / per_machine;
        for input in &recipe.ingredients {
            *flow.demand.entry(input.item.clone()).or_default() +=
                machines_exact * recipe.rate_per_min(input.amount);
        }
        for extra in recipe.products.iter().filter(|p| p.item != item) {
            *flow.surplus.entry(extra.item.clone()).or_default() +=
                machines_exact * recipe.rate_per_min(extra.amount);
        }
        let machines = whole_machines(machines_exact);
        let (output, inputs) = self.step_flows(item, recipe, machines_exact);
        flow.steps.push(Step {
            item: item.to_string(),
            item_name: self.data.item_name(item),
            recipe_id: recipe.id.clone(),
            recipe_name: recipe.name.clone(),
            building_name: self.data.building_name(&recipe.building),
            machines_exact,
            machines,
            last_machine_pct: last_machine_pct(machines_exact, machines),
            output_per_min: needed,
            level: 0,
            per_machine,
            inputs,
            output,
            goes_to: vec![],
        });
    }

    fn raw_needs(&self, order: &[String], demand: &HashMap<String, f64>) -> Vec<RawNeed> {
        order
            .iter()
            .filter(|i| self.raw.contains(*i))
            .map(|item| {
                let per_min = demand.get(item).copied().unwrap_or(0.0);
                RawNeed {
                    item: item.clone(),
                    item_name: self.data.item_name(item),
                    per_min,
                    fluid: self.data.is_fluid(item),
                    miners: self
                        .data
                        .is_minable(item)
                        .then(|| whole_machines(per_min / self.miner_rate)),
                    belts: self.raw_belts(item, per_min),
                    goes_to: vec![],
                }
            })
            .collect()
    }

    fn surplus_list(&self, surplus: HashMap<String, f64>) -> Vec<Surplus> {
        let mut list: Vec<Surplus> = surplus
            .into_iter()
            .map(|(item, per_min)| Surplus { item_name: self.data.item_name(&item), item, per_min })
            .collect();
        list.sort_by(|a, b| a.item_name.cmp(&b.item_name));
        list
    }
}

/// 0 for raw resources, else one more than the highest ingredient.
fn levels(walk: &Walk) -> HashMap<String, u32> {
    let mut levels: HashMap<String, u32> = HashMap::new();
    for item in &walk.order {
        let level = walk.chosen.get(item).map_or(0, |recipe| {
            1 + recipe.ingredients.iter().map(|i| levels[&i.item]).max().unwrap_or(0)
        });
        levels.insert(item.clone(), level);
    }
    levels
}
