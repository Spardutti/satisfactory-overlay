use super::Planner;
use crate::game_data::{GameData, Recipe};
use std::collections::HashSet;

/// Owned recipes for `item`: standard first (main product, then same name as the item), then alternates.
/// Without unlock data every standard recipe counts as owned; with it, only what `owned` holds.
pub fn ranked_recipes<'a>(data: &'a GameData, owned: &HashSet<String>, item: &str) -> Vec<&'a Recipe> {
    let usable = |r: &Recipe| match data.unlocks.is_empty() {
        true => !r.alternate || owned.contains(&r.id),
        false => owned.contains(&r.id),
    };
    rank(data, item, usable)
}

/// The recipe the game treats as the item's own, whatever the player has unlocked.
pub fn standard_recipe<'a>(data: &'a GameData, item: &str) -> Option<&'a Recipe> {
    rank(data, item, |r| !r.alternate).into_iter().next()
}

fn rank<'a>(data: &'a GameData, item: &str, keep: impl Fn(&Recipe) -> bool) -> Vec<&'a Recipe> {
    let item_name = data.item_name(item);
    let mut options: Vec<&'a Recipe> =
        data.recipes.iter().filter(|r| r.products.iter().any(|p| p.item == item)).filter(|r| keep(r)).collect();
    options.sort_by_key(|r| (r.alternate, r.products[0].item != item, r.name != item_name));
    options
}

impl<'a> Planner<'a> {
    pub fn recipe_options(&self, item: &str) -> Vec<&'a Recipe> {
        ranked_recipes(self.data, self.enabled_alternates, item)
    }

    /// The recipe the user picked for `item`, if it is still one of its options.
    pub fn user_choice(&self, item: &str) -> Option<&'a Recipe> {
        let picked = self.choices.get(item)?;
        self.recipe_options(item).into_iter().find(|r| &r.id == picked)
    }
}
