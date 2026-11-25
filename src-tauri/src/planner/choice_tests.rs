use super::tests::{recipe, setup};
use crate::game_data::{GameData, Item, Recipe};

fn fuel_loop_data() -> GameData {
    let mut data = GameData::default();
    data.recipes.push(recipe("Unpack", &[("Packaged", 1.0)], &[("Fuel", 1.0)]));
    data.recipes.push(recipe("Pack", &[("Fuel", 1.0)], &[("Packaged", 1.0)]));
    data.recipes.push(recipe("Refine", &[("Desc_OreIron_C", 1.0)], &[("Fuel", 1.0)]));
    data
}

#[test]
fn plan_rate_default_recipe_that_loops_falls_back_to_next_option() {
    let result = setup(fuel_loop_data()).planner().plan_rate("Fuel", 1.0).unwrap();
    let recipes: Vec<&str> = result.steps.iter().map(|s| s.recipe_id.as_str()).collect();
    assert_eq!(recipes, vec!["Refine"]);
}

#[test]
fn plan_rate_user_picked_recipe_that_loops_is_an_error() {
    let mut s = setup(fuel_loop_data());
    s.choices.insert("Fuel".into(), "Unpack".into());
    let err = s.planner().plan_rate("Fuel", 1.0).unwrap_err();
    assert_eq!(err, "Recipe loop: Fuel → Packaged → Fuel. Pick another recipe for one of them.");
}

#[test]
fn recipe_options_prefers_recipe_named_like_the_item() {
    let mut data = GameData::default();
    let item = Item { id: "Desc_Fuel_C".into(), name: "Fuel".into(), fluid: true, resource: false };
    data.items.insert(item.id.clone(), item);
    data.recipes.push(Recipe { name: "Residual Fuel".into(), ..recipe("Residual", &[], &[("Desc_Fuel_C", 1.0)]) });
    data.recipes.push(Recipe { name: "Fuel".into(), ..recipe("Main", &[], &[("Desc_Fuel_C", 1.0)]) });
    let s = setup(data);
    let ids: Vec<&str> = s.planner().recipe_options("Desc_Fuel_C").iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, vec!["Main", "Residual"]);
}

#[test]
fn plan_rate_every_default_recipe_looping_asks_for_an_alternate() {
    let mut data = fuel_loop_data();
    data.recipes.retain(|r| r.id != "Refine");
    let err = setup(data).planner().plan_rate("Fuel", 1.0).unwrap_err();
    assert_eq!(err, "Every recipe you own for Fuel loops back on itself. Tick an alternate that makes it.");
}

#[test]
fn plan_rate_user_pick_that_loops_is_not_silently_dropped() {
    let mut data = fuel_loop_data();
    data.recipes.push(recipe("TopFromFuel", &[("Fuel", 1.0)], &[("Top", 1.0)]));
    data.recipes.push(recipe("TopFromOre", &[("Desc_OreIron_C", 1.0)], &[("Top", 1.0)]));
    let mut s = setup(data);
    s.choices.insert("Fuel".into(), "Unpack".into());
    let err = s.planner().plan_rate("Top", 1.0).unwrap_err();
    assert_eq!(err, "Recipe loop: Fuel → Packaged → Fuel. Pick another recipe for one of them.");
}







#[test]
fn plan_rate_negative_rate_is_an_error() {
    let err = setup(crate::game_data::fixture()).planner().plan_rate("Desc_IronScrew_C", -10.0).unwrap_err();
    assert_eq!(err, "The rate must be 0 or more");
}
