mod bus;
mod connect;
mod model;
mod place;
mod rows;

pub use model::{BeltLine, Block, Connection, Layout, Metrics, Row};

use crate::planner::{PlanResult, RawNeed, Step, Transport};
use bus::{input_buses, output_buses};
use connect::connect;
use place::{block_size, place_columns, row_height, METRICS};
use rows::{source_rows, step_rows};
use std::collections::BTreeMap;

/// `per_miner` is what one miner delivers once its belt is taken into account.
pub fn build_layout(result: &PlanResult, transport: Transport, per_miner: f64) -> Layout {
    let raw = result.raw.iter().map(|raw| (0, source_block(raw, per_miner, transport)));
    let made = result.steps.iter().map(|step| (step.level, step_block(step, transport)));
    let mut blocks: Vec<(u32, Block)> = raw.chain(made).collect();
    let (width, height) = place_columns(&mut blocks);
    let blocks: Vec<Block> = blocks.into_iter().map(|(_, b)| b).collect();
    let connections = connect(&blocks);
    Layout { width, height, blocks, connections, metrics: METRICS }
}

/// Moves blocks the user placed by hand; the others keep their automatic place.
pub fn apply_positions(layout: &mut Layout, positions: &BTreeMap<String, [f64; 2]>) {
    for block in &mut layout.blocks {
        if let Some([x, y]) = positions.get(&block.item) {
            (block.x, block.y) = (*x, *y);
        }
    }
}

struct Head<'a> {
    item: &'a str,
    title: &'a str,
    building: &'a str,
}

fn source_block(raw: &RawNeed, per_miner: f64, transport: Transport) -> Block {
    let building = if raw.miners.is_some() { "Miner" } else { "Bring in" };
    let head = Head { item: &raw.item, title: &raw.item_name, building };
    block(head, source_rows(raw, per_miner), transport)
}

fn step_block(step: &Step, transport: Transport) -> Block {
    let head = Head { item: &step.item, title: &step.item_name, building: &step.building_name };
    block(head, step_rows(step, transport), transport)
}

fn block(head: Head, rows: Vec<Row>, transport: Transport) -> Block {
    let (width, height) = block_size(&rows);
    Block {
        item: head.item.to_string(),
        title: head.title.to_string(),
        building_name: head.building.to_string(),
        x: 0.0,
        y: 0.0,
        width,
        height,
        row_height: row_height(&rows),
        input_buses: input_buses(&rows, transport),
        output_buses: output_buses(&rows, transport),
        rows,
    }
}

#[cfg(test)]
mod tests;
