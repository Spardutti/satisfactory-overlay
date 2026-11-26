use super::tests::setup;
use crate::game_data::{fixture, Item, ItemAmount, Recipe, POWER};

/// A 75 MW generator burning 15 stone and 45 water a minute, standing in for coal.
fn stone_generator() -> Recipe {
    let amount = |item: &str, amount: f64| ItemAmount { item: item.into(), amount };
    Recipe {
        id: "Power_Build_GeneratorCoal_C_Desc_Stone_C".into(),
        name: "Coal-Powered Generator — Limestone".into(),
        alternate: false,
        ingredients: vec![amount("Desc_Stone_C", 15.0), amount("Desc_Water_C", 45.0)],
        products: vec![amount(POWER, 75.0)],
        duration_secs: 60.0,
        building: "Build_GeneratorCoal_C".into(),
    }
}

#[test]
fn plan_150_mw_needs_2_generators_30_fuel_and_90_water() {
    let mut data = fixture();
    data.items.insert(POWER.into(), Item { id: POWER.into(), name: "Power (MW)".into(), fluid: false, resource: false });
    data.recipes.push(stone_generator());
    let result = setup(data).planner().plan_rate(POWER, 150.0).unwrap();
    let raw: Vec<(&str, f64)> = result.raw.iter().map(|r| (r.item.as_str(), r.per_min)).collect();
    assert_eq!((result.steps[0].machines, raw), (2, vec![("Desc_Stone_C", 30.0), ("Desc_Water_C", 90.0)]));
}

#[test]
fn plan_300_mw_power_runs_on_one_line_never_over() {
    let mut data = fixture();
    data.items.insert(POWER.into(), Item { id: POWER.into(), name: "Power (MW)".into(), fluid: false, resource: false });
    data.recipes.push(stone_generator());
    let result = setup(data).planner().plan_rate(POWER, 300.0).unwrap();
    let lines: Vec<(u32, bool)> = result.steps[0].output.belts.iter().map(|b| (b.machines, b.over)).collect();
    assert_eq!(lines, vec![(4, false)]);
}
