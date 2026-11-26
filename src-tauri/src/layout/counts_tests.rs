use super::bus::{Bus, Tap};
use super::place::METRICS;
use super::*;

fn line() -> BeltLine {
    BeltLine { item: "x".into(), item_name: "x".into(), per_min: 1.0, fluid: false, over: false }
}

fn block(item: &str, building: &str, machines: usize) -> Block {
    let row = Row { inputs: vec![], machines: vec![100.0; machines], output: line(), node: None };
    Block {
        item: item.into(),
        title: item.into(),
        building_name: building.into(),
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
        row_height: 0.0,
        rows: vec![row],
        input_buses: vec![],
        output_buses: vec![],
    }
}

fn bus(taps: usize, fluid: bool) -> Bus {
    let taps = (0..taps).map(|row| Tap { row, input: 0, per_min: 1.0 }).collect();
    Bus { item: "x".into(), item_name: "x".into(), fluid, per_min: 1.0, taps }
}

fn link(item: &str, to_item: &str) -> Connection {
    Connection { item: item.into(), from_bus: 0, to_item: to_item.into(), to_bus: 0, per_min: 1.0 }
}

fn layout(blocks: Vec<Block>, connections: Vec<Connection>) -> Layout {
    Layout { width: 0.0, height: 0.0, blocks, connections, metrics: METRICS, counts: vec![] }
}

fn counts(layout: &Layout) -> Vec<(String, u32)> {
    build_counts(layout).into_iter().map(|c| (c.name, c.count)).collect()
}

#[test]
fn build_counts_1_miner_4_smelters_lists_smelter_then_miner_mk1() {
    let l = layout(vec![block("ore", "Miner Mk1", 1), block("ingot", "Smelter", 4)], vec![]);
    assert_eq!(counts(&l), vec![("Smelter".into(), 4), ("Miner Mk1".into(), 1)]);
}

#[test]
fn build_counts_input_bus_feeding_4_machines_needs_4_splitters() {
    let mut b = block("plate", "Constructor", 0);
    b.input_buses = vec![bus(4, false)];
    assert_eq!(counts(&layout(vec![b], vec![])), vec![("Splitter".into(), 4)]);
}

#[test]
fn build_counts_output_bus_of_3_rows_needs_3_mergers() {
    let mut b = block("plate", "Constructor", 0);
    b.output_buses = vec![bus(3, false)];
    assert_eq!(counts(&layout(vec![b], vec![])), vec![("Merger".into(), 3)]);
}

#[test]
fn build_counts_bus_of_1_row_needs_no_splitter() {
    let mut b = block("plate", "Constructor", 0);
    b.input_buses = vec![bus(1, false)];
    assert_eq!(counts(&layout(vec![b], vec![])), vec![]);
}

#[test]
fn build_counts_output_shared_by_2_blocks_adds_1_splitter() {
    let mut ingot = block("ingot", "Smelter", 0);
    ingot.output_buses = vec![bus(1, false)];
    let l = layout(vec![ingot], vec![link("ingot", "plate"), link("ingot", "rod")]);
    assert_eq!(counts(&l), vec![("Splitter".into(), 1)]);
}

#[test]
fn build_counts_2_blocks_into_one_bus_adds_1_merger() {
    let l = layout(vec![], vec![link("ore", "ingot"), link("scrap", "ingot")]);
    assert_eq!(counts(&l), vec![("Merger".into(), 1)]);
}

#[test]
fn build_counts_fluid_bus_counts_pipe_junctions() {
    let mut b = block("plastic", "Refinery", 0);
    b.input_buses = vec![bus(3, true)];
    assert_eq!(counts(&layout(vec![b], vec![])), vec![("Pipeline Junction".into(), 3)]);
}

#[test]
fn build_counts_skips_depot_and_bring_in_blocks() {
    let l = layout(vec![block("rip", "Depot", 2), block("water", "Bring in", 1)], vec![]);
    assert_eq!(counts(&l), vec![]);
}
