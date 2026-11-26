use super::place::METRICS;
use super::*;
use crate::game_data::GameData;
use std::collections::HashMap;

fn block(building: &str, clocks: Vec<f64>) -> Block {
    let line = BeltLine { item: "x".into(), item_name: "x".into(), per_min: 1.0, fluid: false, over: false };
    let row = Row { inputs: vec![], machines: clocks, output: line, node: None };
    let (x, y, width, height, row_height) = (0.0, 0.0, 0.0, 0.0, 0.0);
    Block { item: building.into(), title: building.into(), building_name: building.into(), x, y, width, height, row_height, rows: vec![row], input_buses: vec![], output_buses: vec![] }
}

fn layout(blocks: Vec<Block>) -> Layout {
    Layout { width: 0.0, height: 0.0, blocks, connections: vec![], metrics: METRICS, counts: vec![], power_mw: 0.0 }
}

fn data() -> GameData {
    let mut data = GameData::default();
    data.machine_power.insert("Smelter".into(), 4.0);
    data.machine_power.insert("Miner Mk1".into(), 5.0);
    data.recipe_power.insert("Recipe_Diamond_C".into(), 500.0);
    data.recipe_power.insert("Recipe_IronIngot_C".into(), 0.5);
    data
}

fn recipes() -> HashMap<String, String> {
    HashMap::from([("Particle Accelerator".into(), "Recipe_Diamond_C".into()), ("Smelter".into(), "Recipe_IronIngot_C".into())])
}

#[test]
fn power_use_adds_every_machine_across_blocks() {
    let plan = layout(vec![block("Smelter", vec![100.0, 100.0]), block("Miner Mk1", vec![100.0])]);
    assert_eq!(power_use(&plan, &data(), &recipes()), 13.0);
}

#[test]
fn power_use_underclocked_machine_draws_by_clock_curve() {
    let plan = layout(vec![block("Smelter", vec![100.0, 50.0])]);
    assert!((power_use(&plan, &data(), &recipes()) - 5.6).abs() < 0.001);
}

#[test]
fn power_use_ignores_buildings_without_a_draw() {
    let plan = layout(vec![block("Depot", vec![100.0]), block("Coal-Powered Generator", vec![100.0])]);
    assert_eq!(power_use(&plan, &data(), &recipes()), 0.0);
}

#[test]
fn power_use_swinging_machine_draws_its_recipe_average() {
    let plan = layout(vec![block("Particle Accelerator", vec![100.0])]);
    assert_eq!(power_use(&plan, &data(), &recipes()), 500.0);
}

#[test]
fn power_use_steady_machine_ignores_its_recipe_range() {
    let plan = layout(vec![block("Smelter", vec![100.0])]);
    assert_eq!(power_use(&plan, &data(), &recipes()), 4.0);
}
