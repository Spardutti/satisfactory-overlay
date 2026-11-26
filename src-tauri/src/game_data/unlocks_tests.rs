use super::unlocks::*;
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashSet};

fn path(class: &str) -> String {
    format!("\"/Script/Engine.BlueprintGeneratedClass'/Game/X/{class}.{class}'\"")
}

fn schematic(id: &str, name: &str, kind: &str, tier: u32, recipes: &[&str], children: &[&str]) -> Value {
    let list = |ids: &[&str]| format!("({})", ids.iter().map(|i| path(i)).collect::<Vec<_>>().join(","));
    json!({
        "ClassName": id, "mDisplayName": name, "mType": kind, "mTechTier": tier.to_string(),
        "mUnlocks": [{ "Class": "BP_UnlockRecipe_C", "mRecipes": list(recipes) }, { "Class": "BP_UnlockSchematic_C", "mSchematics": list(children) }]
    })
}

fn factory() -> HashSet<String> {
    ["Recipe_Screw_C", "Recipe_IngotSteel_C", "Recipe_IngotCaterium_C", "Recipe_Alternate_Turbofuel_C", "Recipe_TurboHeavy_C"]
        .map(String::from)
        .into()
}

fn book(classes: &[Value]) -> UnlockBook {
    read_unlocks(&classes.iter().collect::<Vec<_>>(), &factory())
}

fn ticked(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|i| i.to_string()).collect()
}

#[test]
fn read_unlocks_milestone_gives_its_recipes() {
    let b = book(&[schematic("Schematic_3-4_C", "Basic Steel Production", "EST_Milestone", 3, &["Recipe_IngotSteel_C", "Recipe_Wall_C"], &[])]);
    let u = &b.unlocks[0];
    assert_eq!((u.kind, u.tier, u.recipes.clone()), (UnlockKind::Milestone, 3, vec!["Recipe_IngotSteel_C".to_string()]));
}

#[test]
fn read_unlocks_hidden_unlock_counts_for_the_milestone_that_hands_it_out() {
    let b = book(&[
        schematic("Schematic_5-5_C", "Petroleum Power", "EST_Milestone", 5, &[], &["Schematic_5-5-1_C"]),
        schematic("Schematic_5-5-1_C", "Petroleum Power 2", "EST_Custom", 5, &["Recipe_IngotCaterium_C"], &[]),
    ]);
    let listed: Vec<(&str, Vec<String>)> = b.unlocks.iter().map(|u| (u.id.as_str(), u.recipes.clone())).collect();
    assert_eq!((listed, b.always.len()), (vec![("Schematic_5-5_C", vec!["Recipe_IngotCaterium_C".to_string()])], 0));
}

#[test]
fn read_unlocks_hub_upgrade_recipes_are_always_owned() {
    let b = book(&[schematic("Schematic_Tutorial2_C", "HUB Upgrade 3", "EST_Tutorial", 0, &["Recipe_Screw_C"], &[])]);
    assert_eq!((b.unlocks.len(), b.always.contains("Recipe_Screw_C")), (0, true));
}

#[test]
fn read_unlocks_starting_blueprints_nobody_hands_out_are_always_owned() {
    let b = book(&[schematic("Schematic_StartingRecipes_C", "Starting Blueprints", "EST_Custom", 0, &["Recipe_Screw_C"], &[])]);
    assert!(b.always.contains("Recipe_Screw_C"));
}

#[test]
fn read_unlocks_mam_tree_comes_from_the_id() {
    let b = book(&[schematic("Research_Sulfur_TurboFuel_C", "Turbofuel", "EST_MAM", 3, &["Recipe_Alternate_Turbofuel_C"], &[])]);
    assert_eq!((b.unlocks[0].kind, b.unlocks[0].tree.as_str()), (UnlockKind::Mam, "Sulfur"));
}

#[test]
fn read_unlocks_leaves_out_ficsmas_research() {
    let b = book(&[schematic("Research_XMas_1_C", "FICSMAS Gift Tree", "EST_MAM", 1, &["Recipe_Screw_C"], &[])]);
    assert_eq!((b.unlocks.len(), b.always.len()), (0, 0));
}

#[test]
fn owned_recipes_with_nothing_ticked_is_only_the_always_set() {
    let b = book(&[
        schematic("Schematic_Tutorial2_C", "HUB Upgrade 3", "EST_Tutorial", 0, &["Recipe_Screw_C"], &[]),
        schematic("Schematic_3-4_C", "Basic Steel Production", "EST_Milestone", 3, &["Recipe_IngotSteel_C"], &[]),
    ]);
    assert_eq!(b.owned_recipes(&ticked(&[])), HashSet::from(["Recipe_Screw_C".to_string()]));
}

#[test]
fn owned_recipes_with_the_milestone_ticked_adds_its_recipes() {
    let b = book(&[schematic("Schematic_3-4_C", "Basic Steel Production", "EST_Milestone", 3, &["Recipe_IngotSteel_C"], &[])]);
    assert!(b.owned_recipes(&ticked(&["Schematic_3-4_C"])).contains("Recipe_IngotSteel_C"));
}

#[test]
fn givers_of_turbofuel_names_the_mam_node_not_the_other_alternate() {
    let b = book(&[
        schematic("Research_Sulfur_TurboFuel_C", "Turbofuel", "EST_MAM", 3, &["Recipe_Alternate_Turbofuel_C"], &[]),
        schematic("Schematic_Alternate_TurboHeavy_C", "Alternate: Turbo Heavy Fuel", "EST_Alternate", 0, &["Recipe_TurboHeavy_C"], &[]),
    ]);
    let names: Vec<&str> = b.givers_of(&["Recipe_Alternate_Turbofuel_C"], &ticked(&[])).iter().map(|u| u.name.as_str()).collect();
    assert_eq!(names, vec!["Turbofuel"]);
}

#[test]
fn givers_of_skips_what_is_already_ticked() {
    let b = book(&[schematic("Research_Sulfur_TurboFuel_C", "Turbofuel", "EST_MAM", 3, &["Recipe_Alternate_Turbofuel_C"], &[])]);
    assert!(b.givers_of(&["Recipe_Alternate_Turbofuel_C"], &ticked(&["Research_Sulfur_TurboFuel_C"])).is_empty());
}
