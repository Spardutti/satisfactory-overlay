use super::*;
use std::collections::BTreeMap;

fn need(item: &str, per_min: f64) -> RawNeed {
    RawNeed {
        item: item.into(),
        item_name: item.into(),
        per_min,
        fluid: false,
        miners: None,
        capped: false,
        belts: vec![],
        goes_to: vec![],
        nodes: None,
        from_depot: false,
        extractor: None,
    }
}

fn plan(rate: f64, raw: Vec<RawNeed>) -> PlanResult {
    PlanResult { target: "plating".into(), target_name: "Smart Plating".into(), rate, raw, steps: vec![], surplus: vec![] }
}

fn depot(items: &[(&str, f64)]) -> BTreeMap<String, f64> {
    items.iter().map(|(i, n)| (i.to_string(), *n)).collect()
}

#[test]
fn depot_cap_10_rotors_for_a_plan_needing_15_caps_rate_20_to_13_33() {
    let cap = depot_cap(&plan(20.0, vec![need("rotor", 15.0)]), &depot(&[("rotor", 10.0)]));
    assert_eq!(cap.map(|c| (c * 100.0).round() / 100.0), Some(13.33));
}

#[test]
fn depot_cap_enough_supply_is_none() {
    assert_eq!(depot_cap(&plan(20.0, vec![need("rotor", 15.0)]), &depot(&[("rotor", 15.0)])), None);
}

#[test]
fn depot_cap_tightest_of_two_items_wins() {
    let raw = vec![need("rotor", 20.0), need("plate", 4.0)];
    assert_eq!(depot_cap(&plan(10.0, raw), &depot(&[("rotor", 10.0), ("plate", 3.0)])), Some(5.0));
}

#[test]
fn depot_cap_ignores_ore_not_in_the_depot() {
    let raw = vec![need("iron", 30.0), need("plate", 4.0)];
    assert_eq!(depot_cap(&plan(4.0, raw), &depot(&[("plate", 2.0)])), Some(2.0));
}

#[test]
fn mark_depot_flags_only_depot_items() {
    let mut result = plan(4.0, vec![need("iron", 30.0), need("rotor", 4.0)]);
    mark_depot(&mut result, &depot(&[("rotor", 4.0)]));
    assert_eq!(result.raw.iter().map(|r| r.from_depot).collect::<Vec<_>>(), vec![false, true]);
}

#[test]
fn rods_from_a_depot_leave_only_the_screw_machines() {
    let data = crate::game_data::fixture();
    let raw = std::collections::HashSet::from(["Desc_OreIron_C".to_string(), "Desc_IronRod_C".to_string()]);
    let (choices, alternates) = (Default::default(), Default::default());
    let transport = Transport { belt: 60.0, pipe: 300.0 };
    let planner = Planner { data: &data, raw: &raw, choices: &choices, enabled_alternates: &alternates, extractors: &crate::planner::ExtractorBook::miner_only(60.0, transport.belt), transport };
    let result = planner.plan_rate("Desc_IronScrew_C", 120.0).unwrap();
    let steps: Vec<&str> = result.steps.iter().map(|s| s.item.as_str()).collect();
    let sources: Vec<(&str, f64)> = result.raw.iter().map(|r| (r.item.as_str(), r.per_min)).collect();
    assert_eq!((steps, sources), (vec!["Desc_IronScrew_C"], vec![("Desc_IronRod_C", 30.0)]));
}
