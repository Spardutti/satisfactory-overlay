use super::power::*;
use super::GameData;
use serde_json::{json, Value};
use std::collections::HashMap;

fn coal_generator(fuel: &str) -> Value {
    json!({
        "ClassName": "Build_GeneratorCoal_C", "mDisplayName": "Coal-Powered Generator", "mPowerProduction": "75.000000",
        "mSupplementalToPowerRatio": "10.000000",
        "mFuel": [{ "mFuelClass": fuel, "mSupplementalResourceClass": "Desc_Water_C", "mByproduct": "", "mByproductAmount": "" }]
    })
}

fn amounts(recipe: &super::Recipe) -> (Vec<(String, f64)>, Vec<(String, f64)>) {
    let pairs = |list: &[super::ItemAmount]| list.iter().map(|a| (a.item.clone(), a.amount)).collect();
    (pairs(&recipe.ingredients), pairs(&recipe.products))
}

#[test]
fn fuel_per_min_coal_generator_burns_15_coal() {
    assert_eq!(fuel_per_min(75.0, 300.0, false), 15.0);
}

#[test]
fn fuel_per_min_fuel_generator_burns_20_m3_of_fuel() {
    assert_eq!(fuel_per_min(250.0, 0.75, true), 20.0);
}

#[test]
fn water_per_min_coal_generator_drinks_45() {
    assert_eq!(water_per_min(75.0, 10.0), 45.0);
}

#[test]
fn energy_values_keeps_only_items_with_energy() {
    let items = [json!({"ClassName": "Desc_Coal_C", "mEnergyValue": "300.000000"}), json!({"ClassName": "Desc_IronPlate_C", "mEnergyValue": "0.000000"})];
    assert_eq!(energy_values(&items.iter().collect::<Vec<_>>()), HashMap::from([("Desc_Coal_C".to_string(), 300.0)]));
}

#[test]
fn power_recipes_coal_generator_on_coal_needs_15_coal_and_45_water_for_75_mw() {
    let recipes = power_recipes(&[&coal_generator("Desc_Coal_C")], &HashMap::from([("Desc_Coal_C".into(), 300.0)]), &GameData::default());
    let expected = (vec![("Desc_Coal_C".to_string(), 15.0), ("Desc_Water_C".to_string(), 45.0)], vec![(POWER.to_string(), 75.0)]);
    assert_eq!(amounts(&recipes[0]), expected);
}

#[test]
fn power_recipes_name_the_generator_and_fuel() {
    let recipes = power_recipes(&[&coal_generator("Desc_Coal_C")], &HashMap::from([("Desc_Coal_C".into(), 300.0)]), &GameData::default());
    assert_eq!(recipes[0].name, "Coal-Powered Generator — Desc_Coal_C");
}

#[test]
fn power_recipes_skip_fuels_that_leave_waste() {
    let nuclear = json!({
        "ClassName": "Build_GeneratorNuclear_C", "mDisplayName": "Nuclear Power Plant", "mPowerProduction": "2500.000000",
        "mFuel": [{ "mFuelClass": "Desc_NuclearFuelRod_C", "mSupplementalResourceClass": "Desc_Water_C", "mByproduct": "Desc_NuclearWaste_C" }]
    });
    assert!(power_recipes(&[&nuclear], &HashMap::from([("Desc_NuclearFuelRod_C".into(), 750000.0)]), &GameData::default()).is_empty());
}


#[test]
fn power_options_name_the_generator_fuel_and_megawatts() {
    let mut data = GameData::default();
    data.buildings.insert("Build_GeneratorCoal_C".into(), "Coal-Powered Generator".into());
    data.recipes = power_recipes(&[&coal_generator("Desc_Coal_C")], &HashMap::from([("Desc_Coal_C".into(), 300.0)]), &data);
    let option = &power_options(&data)[0];
    assert_eq!((option.generator.as_str(), option.fuel.as_str(), option.megawatts), ("Coal-Powered Generator", "Desc_Coal_C", 75.0));
}
