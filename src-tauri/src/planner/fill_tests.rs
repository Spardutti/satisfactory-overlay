use super::*;

const MK2_ON_MK2: NodeRates = NodeRates { pure: 240.0, normal: 120.0, impure: 60.0, belt: 120.0 };

fn ore(per_min: f64, miners: u32) -> RawNeed {
    RawNeed {
        item: "iron".into(),
        item_name: "Iron Ore".into(),
        per_min,
        fluid: false,
        miners: Some(miners),
        capped: false,
        belts: vec![],
        goes_to: vec![],
        nodes: None,
    }
}

#[test]
fn miner_capacity_two_mk1_miners_without_nodes_is_120() {
    let rates = NodeRates { pure: 120.0, normal: 60.0, impure: 30.0, belt: 60.0 };
    assert_eq!(miner_capacity(&ore(90.0, 2), rates), 120.0);
}

#[test]
fn miner_capacity_pure_and_normal_draws_is_belt_capped_sum() {
    let mut raw = ore(150.0, 2);
    raw.nodes = Some(draw_from_nodes(150.0, NodeCounts { pure: 1, normal: 1, impure: 0 }, MK2_ON_MK2));
    assert_eq!(miner_capacity(&raw, MK2_ON_MK2), 240.0);
}

#[test]
fn miner_capacity_fluid_without_miners_is_0() {
    let mut raw = ore(90.0, 0);
    raw.miners = None;
    assert_eq!(miner_capacity(&raw, MK2_ON_MK2), 0.0);
}
