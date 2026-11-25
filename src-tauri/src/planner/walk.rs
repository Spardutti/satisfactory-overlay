use super::Planner;
use crate::game_data::Recipe;
use std::collections::{HashMap, HashSet};

/// The recipe picked for every item under a target, in build order.
#[derive(Clone, Default)]
pub struct Walk<'r> {
    path: Vec<String>,
    done: HashSet<String>,
    /// post-order: every item comes after all of its ingredients
    pub order: Vec<String>,
    pub chosen: HashMap<String, &'r Recipe>,
}

enum WalkError {
    /// `start` is the item the loop leads back to
    Loop { start: String, message: String },
    Stuck(String),
}

impl WalkError {
    fn message(self) -> String {
        match self {
            WalkError::Loop { message, .. } | WalkError::Stuck(message) => message,
        }
    }
}

impl<'a> Planner<'a> {
    pub fn walk(&self, target: &str) -> Result<Walk<'a>, String> {
        let mut walk = Walk::default();
        self.visit(target, &mut walk).map_err(WalkError::message)?;
        Ok(walk)
    }

    fn visit(&self, item: &str, walk: &mut Walk<'a>) -> Result<(), WalkError> {
        if walk.done.contains(item) {
            return Ok(());
        }
        if let Some(start) = walk.path.iter().position(|p| p == item) {
            let message = self.cycle_error(&walk.path[start..], item);
            return Err(WalkError::Loop { start: item.to_string(), message });
        }
        walk.path.push(item.to_string());
        if !self.raw.contains(item) {
            let recipe = match self.user_choice(item) {
                Some(picked) => self.try_recipe(picked, walk).map(|()| picked)?,
                None => self.first_working_default(item, walk)?,
            };
            walk.chosen.insert(item.to_string(), recipe);
        }
        walk.path.pop();
        walk.done.insert(item.to_string());
        walk.order.push(item.to_string());
        Ok(())
    }

    fn try_recipe(&self, recipe: &Recipe, walk: &mut Walk<'a>) -> Result<(), WalkError> {
        let mut trial = walk.clone();
        recipe.ingredients.iter().try_for_each(|i| self.visit(&i.item, &mut trial))?;
        *walk = trial;
        Ok(())
    }

    /// The first default option whose inputs resolve without looping back.
    fn first_working_default(&self, item: &str, walk: &mut Walk<'a>) -> Result<&'a Recipe, WalkError> {
        let name = self.data.item_name(item);
        let mut first_error = None;
        for recipe in self.recipe_options(item) {
            match self.try_recipe(recipe, walk) {
                Ok(()) => return Ok(recipe),
                Err(e) => {
                    first_error.get_or_insert(e);
                }
            }
        }
        Err(match first_error {
            None => WalkError::Stuck(format!("No recipe makes {name}")),
            Some(WalkError::Loop { start, .. }) if start == item => WalkError::Stuck(format!(
                "Every recipe you own for {name} loops back on itself. Tick an alternate that makes it."
            )),
            Some(other) => other,
        })
    }

    fn cycle_error(&self, loop_items: &[String], back_to: &str) -> String {
        let names: Vec<String> = loop_items.iter().map(|i| self.data.item_name(i)).collect();
        let first = self.data.item_name(back_to);
        format!("Recipe loop: {} → {first}. Pick another recipe for one of them.", names.join(" → "))
    }
}
