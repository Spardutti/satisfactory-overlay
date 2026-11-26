use super::*;
use crate::planner::RawNeed;
use std::collections::HashMap;

fn request(mode: Mode) -> PlanRequest {
    PlanRequest {
        target: "Desc_IronPlate_C".into(),
        mode,
        miner_tier: "Mk1".into(),
        belt_tier: "Mk1".into(),
        pipe_tier: "Mk1".into(),
        choices: HashMap::new(),
        nodes: Default::default(),
    }
}

fn iron(per_min: f64) -> RawNeed {
    RawNeed {
        item: "iron".into(),
        item_name: "Iron Ore".into(),
        per_min,
        fluid: false,
        miners: Some(1),
        capped: false,
        belts: vec![],
        goes_to: vec![],
        nodes: None,
    }
}

fn rate_of(request: &PlanRequest) -> f64 {
    match request.mode {
        Mode::Rate { rate } => rate,
        Mode::Resources { .. } => panic!("expected rate mode"),
    }
}

#[test]
fn grow_to_fill_rate_mode_75_percent_miner_scales_rate_30_to_40() {
    let mut req = request(Mode::Rate { rate: 30.0 });
    let grew = req.grow_to_fill(&iron(45.0), 60.0);
    assert_eq!((grew, rate_of(&req)), (true, 40.0));
}

#[test]
fn grow_to_fill_resources_mode_sets_that_ore_to_capacity() {
    let available = HashMap::from([("iron".to_string(), 45.0), ("copper".to_string(), 30.0)]);
    let mut req = request(Mode::Resources { available });
    req.grow_to_fill(&iron(45.0), 60.0);
    let Mode::Resources { available } = &req.mode else { panic!("expected resources mode") };
    assert_eq!((available["iron"], available["copper"]), (60.0, 30.0));
}

#[test]
fn grow_to_fill_full_miners_change_nothing() {
    let mut req = request(Mode::Rate { rate: 60.0 });
    let grew = req.grow_to_fill(&iron(60.0), 60.0);
    assert_eq!((grew, rate_of(&req)), (false, 60.0));
}

#[test]
fn grow_to_fill_ore_not_used_changes_nothing() {
    let mut req = request(Mode::Rate { rate: 60.0 });
    let grew = req.grow_to_fill(&iron(0.0), 60.0);
    assert_eq!((grew, rate_of(&req)), (false, 60.0));
}
