use super::tests::{setup, Setup};
use crate::game_data::fixture;

/// Fixture data where only `kind` "Rods" unlocks the rod recipe, and screws and ingots are owned from the start.
fn rods_locked_behind(kind: crate::game_data::UnlockKind, tree: &str) -> Setup {
    let mut s = setup(fixture());
    let rods = crate::game_data::Unlock {
        id: "Schematic_Rods_C".into(),
        name: "Rods".into(),
        kind,
        tier: 2,
        tree: tree.into(),
        recipes: vec!["Recipe_IronRod_C".into()],
    };
    s.data.unlocks = crate::game_data::UnlockBook { unlocks: vec![rods], always: ["Recipe_Screw_C", "Recipe_IngotIron_C"].map(String::from).into() };
    s.alternates = s.data.unlocks.owned_recipes(&Default::default());
    s
}

#[test]
fn plan_rate_never_uses_a_recipe_from_an_unticked_milestone() {
    let err = rods_locked_behind(crate::game_data::UnlockKind::Milestone, "").planner().plan_rate("Desc_IronScrew_C", 40.0).unwrap_err();
    assert_eq!(err, "No recipe you own makes Iron Rod. Tick one of these in My recipes: Rods (Tier 2).");
}

#[test]
fn plan_rate_locked_mam_recipe_names_the_mam_node_to_tick() {
    let err = rods_locked_behind(crate::game_data::UnlockKind::Mam, "Sulfur").planner().plan_rate("Desc_IronScrew_C", 40.0).unwrap_err();
    assert_eq!(err, "No recipe you own makes Iron Rod. Tick one of these in My recipes: Rods (MAM Sulfur).");
}

#[test]
fn plan_rate_ticked_milestone_recipe_is_used() {
    let mut s = rods_locked_behind(crate::game_data::UnlockKind::Milestone, "");
    s.alternates = s.data.unlocks.owned_recipes(&["Schematic_Rods_C".to_string()].into());
    assert!(s.planner().plan_rate("Desc_IronScrew_C", 40.0).is_ok());
}
