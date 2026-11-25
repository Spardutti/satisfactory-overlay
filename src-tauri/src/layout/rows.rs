use super::{BeltLine, Row};
use crate::planner::{BeltGroup, Flow, NodeDraw, RawNeed, Step, Transport};

pub fn step_rows(step: &Step, transport: Transport) -> Vec<Row> {
    machine_clocks(step.machines, step.last_machine_pct)
        .into_iter()
        .map(|clock| {
            let share = clock / 100.0 / step.machines_exact;
            Row {
                inputs: step.inputs.iter().map(|f| belt(f, f.per_min * share, transport)).collect(),
                machines: vec![clock],
                output: belt(&step.output, step.output.per_min * share, transport),
                node: None,
            }
        })
        .collect()
}

/// A miner's own rate on a normal node, and the belt that may hold it back.
#[derive(Debug, Clone, Copy)]
pub struct MinerSpeed {
    pub rate: f64,
    pub belt: f64,
}

/// One row per miner, or per line for fluids and gathered items.
pub fn source_rows(raw: &RawNeed, miner: MinerSpeed) -> Vec<Row> {
    match &raw.nodes {
        Some(supply) => supply.draws.iter().map(|draw| node_row(raw, draw)).collect(),
        None => raw.belts.iter().flat_map(|group| group_rows(raw, group, miner)).collect(),
    }
}

fn node_row(raw: &RawNeed, draw: &NodeDraw) -> Row {
    Row {
        inputs: vec![],
        machines: vec![draw.clock],
        output: ore_line(raw, draw.per_min, false),
        node: Some(draw.purity.clone()),
    }
}

fn ore_line(raw: &RawNeed, per_min: f64, over: bool) -> BeltLine {
    BeltLine { item: raw.item.clone(), item_name: raw.item_name.clone(), per_min, fluid: raw.fluid, over }
}

/// Miners that share a belt each get a row; all fill their share but the last.
fn group_rows(raw: &RawNeed, group: &BeltGroup, miner: MinerSpeed) -> Vec<Row> {
    let per_miner = miner.rate.min(miner.belt);
    let line = |per_min: f64, machines: Vec<f64>| Row { inputs: vec![], machines, output: ore_line(raw, per_min, group.over), node: None };
    if group.machines == 0 {
        return vec![line(group.per_min, vec![])];
    }
    let exact = group.per_min / per_miner;
    let last_pct = (exact - f64::from(group.machines - 1)) * 100.0;
    machine_clocks(group.machines, last_pct)
        .into_iter()
        .map(|share| {
            let per_min = per_miner * share / 100.0;
            line(per_min, vec![per_min / miner.rate * 100.0])
        })
        .collect()
}

fn belt(flow: &Flow, per_min: f64, transport: Transport) -> BeltLine {
    BeltLine {
        item: flow.item.clone(),
        item_name: flow.item_name.clone(),
        per_min,
        fluid: flow.fluid,
        over: per_min > transport.capacity(flow.fluid) + 1e-9,
    }
}

fn machine_clocks(machines: u32, last_pct: f64) -> Vec<f64> {
    (0..machines).map(|i| if i + 1 == machines { last_pct } else { 100.0 }).collect()
}
