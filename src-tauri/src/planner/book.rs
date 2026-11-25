use super::Planner;
use crate::game_data::Recipe;

impl<'a> Planner<'a> {
    /// Standard recipes first (main product, then same name as the item), then owned alternates.
    pub fn recipe_options(&self, item: &str) -> Vec<&'a Recipe> {
        let item_name = self.data.item_name(item);
        let mut options: Vec<&'a Recipe> = self
            .data
            .recipes
            .iter()
            .filter(|r| r.products.iter().any(|p| p.item == item))
            .filter(|r| !r.alternate || self.enabled_alternates.contains(&r.id))
            .collect();
        options.sort_by_key(|r| (r.alternate, r.products[0].item != item, r.name != item_name));
        options
    }

    /// The recipe the user picked for `item`, if it is still one of its options.
    pub fn user_choice(&self, item: &str) -> Option<&'a Recipe> {
        let picked = self.choices.get(item)?;
        self.recipe_options(item).into_iter().find(|r| &r.id == picked)
    }
}
