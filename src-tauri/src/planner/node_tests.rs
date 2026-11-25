use super::*;
use crate::planner::RawNeed;

const MK1: NodeRates = NodeRates { pure: 120.0, normal: 60.0, impure: 30.0 };
const MK2_ON_MK5: NodeRates = NodeRates { pure: 240.0, normal: 120.0, impure: 60.0 };

fn nodes(pure: u32, normal: u32, impure: u32) -> NodeCounts {
    NodeCounts { pure, normal, impure }
}

fn pulls(supply: &NodeSupply) -> Vec<(&str, f64)> {
    supply.draws.iter().map(|d| (d.purity.as_str(), d.per_min)).collect()
}

#[test]
fn draw_from_nodes_400_from_1_pure_2_impure_mk2_on_mk5_is_360_and_40_short() {
    let supply = draw_from_nodes(400.0, nodes(1, 0, 2), MK2_ON_MK5);
    assert_eq!((pulls(&supply), supply.short), (vec![("pure", 240.0), ("impure", 60.0), ("impure", 60.0)], 40.0));
}

#[test]
fn draw_from_nodes_need_exactly_met_is_not_short() {
    let supply = draw_from_nodes(120.0, nodes(1, 0, 0), MK1);
    assert_eq!((pulls(&supply), supply.short, supply.draws[0].clock), (vec![("pure", 120.0)], 0.0, 100.0));
}

#[test]
fn draw_from_nodes_100_from_2_normal_mk1_slows_the_second_to_67_percent() {
    let supply = draw_from_nodes(100.0, nodes(0, 2, 0), MK1);
    let clocks: Vec<f64> = supply.draws.iter().map(|d| (d.clock * 100.0).round() / 100.0).collect();
    assert_eq!(clocks, vec![100.0, 66.67]);
}

#[test]
fn draw_from_nodes_60_from_1_pure_1_impure_mk1_leaves_the_impure_spare() {
    let supply = draw_from_nodes(60.0, nodes(1, 0, 1), MK1);
    assert_eq!((pulls(&supply), supply.spare), (vec![("pure", 60.0)], nodes(0, 0, 1)));
}

fn ore(item: &str, per_min: f64) -> RawNeed {
    RawNeed {
        item: item.into(),
        item_name: item.into(),
        per_min,
        fluid: false,
        miners: Some(1),
        capped: false,
        belts: vec![],
        goes_to: vec![],
        nodes: None,
    }
}

fn computers(raw: Vec<RawNeed>) -> PlanResult {
    PlanResult { target: "Computer".into(), target_name: "Computer".into(), rate: 5.0, raw, steps: vec![], surplus: vec![] }
}

#[test]
fn apply_nodes_sets_one_miner_and_one_belt_per_used_node() {
    let mut result = computers(vec![ore("iron", 150.0)]);
    apply_nodes(&mut result, &BTreeMap::from([("iron".to_string(), nodes(0, 3, 0))]), MK1);
    let belts: Vec<f64> = result.raw[0].belts.iter().map(|b| b.per_min).collect();
    assert_eq!((result.raw[0].miners, belts), (Some(3), vec![60.0, 60.0, 30.0]));
}

#[test]
fn apply_nodes_all_zero_counts_keep_the_normal_node_guess() {
    let mut result = computers(vec![ore("iron", 150.0)]);
    apply_nodes(&mut result, &BTreeMap::from([("iron".to_string(), nodes(0, 0, 0))]), MK1);
    assert_eq!((result.raw[0].miners, result.raw[0].nodes.is_none()), (Some(1), true));
}

#[test]
fn node_limit_5_computers_with_iron_80_percent_covered_is_4() {
    let mut result = computers(vec![ore("iron", 300.0), ore("copper", 60.0)]);
    let listed = BTreeMap::from([("iron".to_string(), nodes(2, 0, 0)), ("copper".to_string(), nodes(0, 1, 0))]);
    apply_nodes(&mut result, &listed, MK1);
    assert_eq!(node_limit(&result), Some(4.0));
}

#[test]
fn node_limit_all_ores_covered_is_none() {
    let mut result = computers(vec![ore("iron", 240.0)]);
    apply_nodes(&mut result, &BTreeMap::from([("iron".to_string(), nodes(2, 0, 0))]), MK1);
    assert_eq!(node_limit(&result), None);
}
