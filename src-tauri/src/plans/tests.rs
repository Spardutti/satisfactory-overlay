use super::model::ChecklistStep;
use super::*;
use std::collections::HashMap;

fn step(label: &str, total: u32, built: u32) -> ChecklistStep {
    ChecklistStep { label: label.into(), total, built, ..ChecklistStep::default() }
}

fn plan(id: &str, steps: Vec<ChecklistStep>) -> Plan {
    let request = PlanRequest {
        target: "Desc_IronScrew_C".into(),
        mode: Mode::Rate { rate: 120.0 },
        miner_tier: "Mk1".into(),
        purity: "normal".into(),
        belt_tier: "Mk1".into(),
        pipe_tier: "Mk1".into(),
        choices: HashMap::new(),
    };
    Plan { id: id.into(), name: id.into(), request, steps, positions: Default::default() }
}

fn built(plan: &Plan) -> Vec<u32> {
    plan.steps.iter().map(|s| s.built).collect()
}

#[test]
fn mark_next_fills_first_unfinished_step() {
    let mut p = plan("a", vec![step("Smelter", 1, 1), step("Rod", 2, 0)]);
    assert!(p.mark_next());
    assert_eq!(built(&p), vec![1, 1]);
}

#[test]
fn mark_next_when_everything_is_built_changes_nothing() {
    let mut p = plan("a", vec![step("Smelter", 1, 1)]);
    assert!(!p.mark_next());
    assert_eq!(built(&p), vec![1]);
}

#[test]
fn adjust_stops_at_total() {
    let mut p = plan("a", vec![step("Rod", 2, 2)]);
    p.adjust(0, 1);
    assert_eq!(built(&p), vec![2]);
}

#[test]
fn adjust_stops_at_zero() {
    let mut p = plan("a", vec![step("Rod", 2, 0)]);
    p.adjust(0, -1);
    assert_eq!(built(&p), vec![0]);
}

#[test]
fn replace_steps_keeps_progress_of_matching_labels_within_new_total() {
    let mut p = plan("a", vec![step("Smelter", 2, 2), step("Rod", 3, 3)]);
    p.replace_steps(vec![step("Smelter", 1, 0), step("Rod", 4, 0), step("Screw", 2, 0)]);
    assert_eq!(built(&p), vec![1, 3, 0]);
}

#[test]
fn upsert_first_plan_becomes_active() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![]));
    store.upsert(plan("b", vec![]));
    assert_eq!(store.active.as_deref(), Some("a"));
}

#[test]
fn upsert_same_id_replaces_plan() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![]));
    store.upsert(plan("a", vec![step("Rod", 1, 0)]));
    assert_eq!((store.plans.len(), store.plans[0].steps.len()), (1, 1));
}

#[test]
fn remove_active_plan_activates_the_next_one() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![]));
    store.upsert(plan("b", vec![]));
    store.remove("a");
    assert_eq!(store.active.as_deref(), Some("b"));
}

#[test]
fn set_active_unknown_id_keeps_current() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![]));
    store.set_active("missing");
    assert_eq!(store.active.as_deref(), Some("a"));
}

#[test]
fn plan_store_round_trips_through_json() {
    let mut store = PlanStore::default();
    store.upsert(plan("a", vec![step("Rod", 2, 1)]));
    let back: PlanStore = serde_json::from_str(&serde_json::to_string(&store).unwrap()).unwrap();
    assert_eq!((back.active, back.plans[0].steps.clone()), (Some("a".into()), vec![step("Rod", 2, 1)]));
}
