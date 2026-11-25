use super::tests::setup;
use super::*;
use crate::game_data::fixture;

fn screw_step(rate: f64) -> Step {
    let result = setup(fixture()).planner().plan_rate("Desc_IronScrew_C", rate).unwrap();
    result.steps.into_iter().find(|s| s.item == "Desc_IronScrew_C").unwrap()
}

#[test]
fn plan_rate_screw_step_shows_40_per_machine() {
    assert_eq!(screw_step(120.0).per_machine, 40.0);
}

#[test]
fn plan_rate_120_screws_on_mk1_belt_needs_3_output_belts() {
    let machines_per_belt: Vec<u32> = screw_step(120.0).output.belts.iter().map(|b| b.machines).collect();
    assert_eq!(machines_per_belt, vec![1, 1, 1]);
}

#[test]
fn plan_rate_120_screws_screw_step_input_is_30_iron_rod() {
    let inputs: Vec<(String, f64)> = screw_step(120.0).inputs.into_iter().map(|i| (i.item_name, i.per_min)).collect();
    assert_eq!(inputs, vec![("Iron Rod".to_string(), 30.0)]);
}

#[test]
fn plan_rate_iron_ingot_goes_to_iron_rod() {
    let result = setup(fixture()).planner().plan_rate("Desc_IronScrew_C", 120.0).unwrap();
    let ingot = result.steps.iter().find(|s| s.item == "Desc_IronIngot_C").unwrap();
    assert_eq!(ingot.goes_to, vec![Usage { item_name: "Iron Rod".into(), per_min: 30.0 }]);
}

#[test]
fn plan_rate_100_water_on_mk1_pipe_needs_1_line() {
    let mut s = setup(fixture());
    s.alternates.insert("Recipe_Alternate_WetConcrete_C".into());
    let result = s.planner().plan_rate("Desc_Cement_C", 80.0).unwrap();
    let water = result.raw.iter().find(|r| r.item == "Desc_Water_C").unwrap();
    assert_eq!(water.belts.len(), 1);
}
