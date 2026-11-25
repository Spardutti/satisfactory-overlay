use super::*;
use crate::game_data::{fixture, GameData};
use crate::planner::Planner;
use std::collections::{HashMap, HashSet};

const MK1: Transport = Transport { belt: 60.0, pipe: 300.0 };

fn screws(data: &GameData, rate: f64, transport: Transport) -> PlanResult {
    let raw = HashSet::from(["Desc_OreIron_C".to_string()]);
    let (choices, alternates) = (HashMap::new(), HashSet::new());
    let planner =
        Planner { data, raw: &raw, choices: &choices, enabled_alternates: &alternates, miner_rate: 60.0, transport };
    planner.plan_rate("Desc_IronScrew_C", rate).unwrap()
}

fn block<'a>(layout: &'a Layout, item: &str) -> &'a Block {
    layout.blocks.iter().find(|b| b.item == item).unwrap()
}

fn row_clocks(block: &Block) -> Vec<Vec<f64>> {
    block.rows.iter().map(|r| r.machines.clone()).collect()
}

#[test]
fn build_layout_120_screws_on_mk1_has_3_screw_rows_of_1() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    assert_eq!(row_clocks(block(&layout, "Desc_IronScrew_C")), vec![vec![100.0], vec![100.0], vec![100.0]]);
}

#[test]
fn build_layout_100_screws_last_machine_runs_at_50() {
    let layout = build_layout(&screws(&fixture(), 100.0, MK1), MK1, 60.0);
    assert_eq!(row_clocks(block(&layout, "Desc_IronScrew_C")).last().unwrap(), &vec![50.0]);
}

#[test]
fn build_layout_row_belts_carry_their_share() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    let row = &block(&layout, "Desc_IronScrew_C").rows[0];
    assert_eq!((row.inputs[0].per_min, row.output.per_min, row.output.over), (10.0, 40.0, false));
}

#[test]
fn build_layout_miner_block_has_one_row_per_miner() {
    let layout = build_layout(&screws(&fixture(), 480.0, MK1), MK1, 60.0);
    assert_eq!(row_clocks(block(&layout, "Desc_OreIron_C")), vec![vec![100.0], vec![100.0]]);
}

#[test]
fn build_layout_partly_used_miner_shows_its_share() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    assert_eq!(row_clocks(block(&layout, "Desc_OreIron_C")), vec![vec![50.0]]);
}

#[test]
fn build_layout_columns_go_miner_smelter_rod_screw_left_to_right() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    let xs: Vec<f64> = ["Desc_OreIron_C", "Desc_IronIngot_C", "Desc_IronRod_C", "Desc_IronScrew_C"]
        .iter()
        .map(|item| block(&layout, item).x)
        .collect();
    assert!(xs.windows(2).all(|pair| pair[0] < pair[1]), "{xs:?}");
}

fn connections_of<'a>(layout: &'a Layout, item: &str) -> Vec<(usize, &'a str, usize, f64)> {
    layout
        .connections
        .iter()
        .filter(|c| c.item == item)
        .map(|c| (c.from_bus, c.to_item.as_str(), c.to_bus, c.per_min))
        .collect()
}

fn bus_rows(buses: &[bus::Bus]) -> Vec<Vec<usize>> {
    buses.iter().map(|b| b.taps.iter().map(|t| t.row).collect()).collect()
}

#[test]
fn build_layout_rod_bus_feeds_all_3_screw_rows() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    assert_eq!(bus_rows(&block(&layout, "Desc_IronScrew_C").input_buses), vec![vec![0, 1, 2]]);
}

#[test]
fn build_layout_screw_output_needs_a_bus_per_row_on_mk1() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    assert_eq!(bus_rows(&block(&layout, "Desc_IronScrew_C").output_buses), vec![vec![0], vec![1], vec![2]]);
}

#[test]
fn build_layout_screws_on_mk3_share_one_output_bus() {
    let mk3 = Transport { belt: 270.0, pipe: 300.0 };
    let layout = build_layout(&screws(&fixture(), 120.0, mk3), mk3, 60.0);
    assert_eq!(bus_rows(&block(&layout, "Desc_IronScrew_C").output_buses), vec![vec![0, 1, 2]]);
}

#[test]
fn build_layout_8_smelters_on_mk1_take_ore_from_4_buses_of_2() {
    let layout = build_layout(&screws(&fixture(), 960.0, MK1), MK1, 60.0);
    let lanes = bus_rows(&block(&layout, "Desc_IronIngot_C").input_buses);
    assert_eq!(lanes, vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]]);
}

#[test]
fn build_layout_ingot_bus_runs_to_the_rod_bus() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    assert_eq!(connections_of(&layout, "Desc_IronIngot_C"), vec![(0, "Desc_IronRod_C", 0, 30.0)]);
}

#[test]
fn build_layout_each_of_4_miner_buses_runs_to_its_own_smelter_bus() {
    let layout = build_layout(&screws(&fixture(), 960.0, MK1), MK1, 60.0);
    let ingot = "Desc_IronIngot_C";
    assert_eq!(
        connections_of(&layout, "Desc_OreIron_C"),
        vec![(0, ingot, 0, 60.0), (1, ingot, 1, 60.0), (2, ingot, 2, 60.0), (3, ingot, 3, 60.0)]
    );
}

#[test]
fn build_layout_final_product_buses_connect_to_nothing() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    assert_eq!(connections_of(&layout, "Desc_IronScrew_C"), vec![]);
}

#[test]
fn apply_positions_moves_only_the_placed_block() {
    let mut layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    let rod_before = (block(&layout, "Desc_IronRod_C").x, block(&layout, "Desc_IronRod_C").y);
    apply_positions(&mut layout, &std::collections::BTreeMap::from([("Desc_IronScrew_C".to_string(), [5.0, -40.0])]));
    let screw = block(&layout, "Desc_IronScrew_C");
    assert_eq!((screw.x, screw.y), (5.0, -40.0));
    assert_eq!((block(&layout, "Desc_IronRod_C").x, block(&layout, "Desc_IronRod_C").y), rod_before);
}

#[test]
fn build_layout_rows_with_two_inputs_are_one_belt_taller() {
    let one = Row { inputs: vec![], machines: vec![], output: belt_line(), node: None };
    let two = Row { inputs: vec![belt_line(), belt_line()], ..one.clone() };
    assert_eq!(place::row_height(&[two]) - place::row_height(&[one]), METRICS.input_step);
}

fn belt_line() -> BeltLine {
    BeltLine { item: "X".into(), item_name: "X".into(), per_min: 1.0, fluid: false, over: false }
}

#[test]
fn build_layout_canvas_holds_every_block() {
    let layout = build_layout(&screws(&fixture(), 120.0, MK1), MK1, 60.0);
    let fits = layout.blocks.iter().all(|b| b.x + b.width <= layout.width && b.y + b.height <= layout.height);
    assert!(fits);
}

fn ore_need(belts: Vec<crate::planner::BeltGroup>) -> RawNeed {
    let per_min = belts.iter().map(|b| b.per_min).sum();
    RawNeed {
        item: "Desc_OreIron_C".into(),
        item_name: "Iron Ore".into(),
        per_min,
        fluid: false,
        miners: Some(belts.iter().map(|b| b.machines).sum()),
        capped: false,
        belts,
        goes_to: vec![],
        nodes: None,
    }
}

#[test]
fn source_rows_two_30_miners_sharing_a_60_belt_are_two_rows_at_100() {
    let group = crate::planner::BeltGroup { machines: 2, per_min: 60.0, over: false };
    let rows = source_rows(&ore_need(vec![group]), 30.0);
    assert_eq!(rows.iter().map(|r| r.machines.clone()).collect::<Vec<_>>(), vec![vec![100.0], vec![100.0]]);
}

#[test]
fn source_rows_listed_nodes_give_one_row_per_node_with_its_purity() {
    let mut ore = ore_need(vec![]);
    let rates = crate::planner::NodeRates { pure: 120.0, normal: 60.0, impure: 30.0 };
    ore.nodes = Some(crate::planner::draw_from_nodes(150.0, crate::planner::NodeCounts { pure: 1, normal: 0, impure: 2 }, rates));
    let rows: Vec<(Option<String>, f64)> = source_rows(&ore, 60.0).into_iter().map(|r| (r.node, r.output.per_min)).collect();
    assert_eq!(rows, vec![(Some("pure".into()), 120.0), (Some("impure".into()), 30.0)]);
}
