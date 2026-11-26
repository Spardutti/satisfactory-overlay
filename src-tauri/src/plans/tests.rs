use super::model::ChecklistStep;
use super::*;
use crate::planner::NodeCounts;
use std::collections::HashMap;

fn step(item: &str, total: u32) -> ChecklistStep {
    ChecklistStep { label: item.into(), item: item.into(), total, ..ChecklistStep::default() }
}

fn plan(id: &str, steps: Vec<ChecklistStep>) -> Plan {
    let request = PlanRequest {
        target: "Desc_IronScrew_C".into(),
        mode: Mode::Rate { rate: 120.0 },
        miner_tier: "Mk1".into(),
        belt_tier: "Mk1".into(),
        pipe_tier: "Mk1".into(),
        choices: HashMap::new(),
        nodes: Default::default(),
        depot: Default::default(),
    };
    Plan { id: id.into(), name: id.into(), request, steps, positions: Default::default(), built: Default::default() }
}

fn built(plan: &Plan) -> Vec<u32> {
    plan.steps.iter().map(|s| s.built).collect()
}

#[test]
fn toggle_built_ticks_one_machine() {
    let mut p = plan("a", vec![step("Smelter", 2), step("Rod", 3)]);
    p.toggle_built("Rod", 2);
    assert_eq!((built(&p), p.built["Rod"].iter().copied().collect::<Vec<_>>()), (vec![0, 1], vec![2]));
}

#[test]
fn toggle_built_twice_unticks_the_machine() {
    let mut p = plan("a", vec![step("Rod", 3)]);
    p.toggle_built("Rod", 1);
    p.toggle_built("Rod", 1);
    assert_eq!(built(&p), vec![0]);
}

#[test]
fn toggle_built_last_machine_is_ticked() {
    let mut p = plan("a", vec![step("Rod", 3)]);
    p.toggle_built("Rod", 2);
    assert_eq!(built(&p), vec![1]);
}

#[test]
fn toggle_built_row_past_the_machine_count_is_ignored() {
    let mut p = plan("a", vec![step("Rod", 3)]);
    p.toggle_built("Rod", 3);
    assert_eq!((built(&p), p.built.is_empty()), (vec![0], true));
}

#[test]
fn toggle_built_item_without_a_step_is_ignored() {
    let mut p = plan("a", vec![step("Rod", 3)]);
    p.toggle_built("Water", 0);
    assert!(p.built.is_empty());
}

#[test]
fn replace_steps_drops_ticks_on_machines_that_no_longer_exist() {
    let mut p = plan("a", vec![step("Rod", 3)]);
    p.toggle_built("Rod", 0);
    p.toggle_built("Rod", 2);
    p.replace_steps(vec![step("Rod", 2), step("Screw", 1)]);
    assert_eq!((built(&p), p.built["Rod"].iter().copied().collect::<Vec<_>>()), (vec![1, 0], vec![0]));
}

#[test]
fn upsert_same_id_replaces_plan() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![]));
    store.upsert(plan("a", vec![step("Rod", 1)]));
    assert_eq!((store.plans.len(), store.plans[0].steps.len()), (1, 1));
}

#[test]
fn remove_drops_only_that_plan() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![]));
    store.upsert(plan("b", vec![]));
    store.remove("a");
    assert_eq!(store.plans.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["b"]);
}

#[test]
fn plan_store_round_trips_ticks_through_json() {
    let mut store = PlanStore::default();
    let mut p = plan("a", vec![step("Rod", 2)]);
    p.toggle_built("Rod", 1);
    store.upsert(p);
    let back: PlanStore = serde_json::from_str(&serde_json::to_string(&store).unwrap()).unwrap();
    assert_eq!((built(&back.plans[0]), back.plans[0].built["Rod"].len()), (vec![1], 1));
}

#[test]
fn set_nodes_1_pure_on_iron_ore_stores_it() {
    let mut request = plan("p", vec![]).request;
    request.set_nodes("Desc_OreIron_C".into(), NodeCounts { pure: 1, normal: 0, impure: 0 });
    assert_eq!(request.nodes.get("Desc_OreIron_C"), Some(&NodeCounts { pure: 1, normal: 0, impure: 0 }));
}

#[test]
fn set_nodes_all_zero_removes_the_ore() {
    let mut request = plan("p", vec![]).request;
    request.set_nodes("Desc_OreIron_C".into(), NodeCounts { pure: 1, normal: 0, impure: 0 });
    request.set_nodes("Desc_OreIron_C".into(), NodeCounts::default());
    assert!(request.nodes.is_empty());
}

#[test]
fn replace_steps_drops_ticks_when_the_recipe_changes() {
    let mut p = plan("a", vec![step("Screw", 6)]);
    p.toggle_built("Screw", 0);
    let assembler = ChecklistStep { label: "Assembler — Cast Screw".into(), ..step("Screw", 6) };
    p.replace_steps(vec![assembler]);
    assert_eq!((built(&p), p.built.is_empty()), (vec![0], true));
}

#[test]
fn set_nodes_clears_the_ticks_of_that_ore() {
    let mut p = plan("a", vec![step("Desc_OreIron_C", 2), step("Rod", 2)]);
    p.toggle_built("Desc_OreIron_C", 0);
    p.toggle_built("Rod", 0);
    p.set_nodes("Desc_OreIron_C".into(), NodeCounts { pure: 1, normal: 0, impure: 0 });
    assert_eq!(p.built.keys().map(String::as_str).collect::<Vec<_>>(), vec!["Rod"]);
}

const RIP: &str = "Desc_IronPlateReinforced_C";

#[test]
fn set_depot_2_5_rip_stores_it() {
    let mut p = plan("a", vec![]);
    p.request.set_depot(RIP.into(), Some(2.5)).unwrap();
    assert_eq!(p.request.depot.get(RIP), Some(&2.5));
}

#[test]
fn set_depot_none_takes_it_away() {
    let mut p = plan("a", vec![]);
    p.request.set_depot(RIP.into(), Some(2.5)).unwrap();
    p.request.set_depot(RIP.into(), None).unwrap();
    assert!(p.request.depot.is_empty());
}

#[test]
fn set_depot_zero_takes_it_away() {
    let mut p = plan("a", vec![]);
    p.request.set_depot(RIP.into(), Some(2.5)).unwrap();
    p.request.set_depot(RIP.into(), Some(0.0)).unwrap();
    assert!(p.request.depot.is_empty());
}

#[test]
fn set_depot_negative_is_refused() {
    let mut p = plan("a", vec![]);
    p.request.set_depot(RIP.into(), Some(2.5)).unwrap();
    let refused = p.request.set_depot(RIP.into(), Some(-1.0));
    assert_eq!((refused, p.request.depot.get(RIP)), (Err("Enter an amount above 0".into()), Some(&2.5)));
}

#[test]
fn clear_built_unticks_every_machine() {
    let mut p = plan("a", vec![step("Smelter", 2), step("Rod", 3)]);
    p.toggle_built("Smelter", 0);
    p.toggle_built("Rod", 2);
    p.clear_built();
    assert_eq!((built(&p), p.built.is_empty()), (vec![0, 0], true));
}
