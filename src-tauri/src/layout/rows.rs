use super::{BeltLine, Row};
use crate::planner::{BeltGroup, Flow, RawNeed, Step, Transport};

pub fn step_rows(step: &Step, transport: Transport) -> Vec<Row> {
    machine_clocks(step.machines, step.last_machine_pct)
        .into_iter()
        .map(|clock| {
            let share = clock / 100.0 / step.machines_exact;
            Row {
                inputs: step.inputs.iter().map(|f| belt(f, f.per_min * share, transport)).collect(),
                machines: vec![clock],
                output: belt(&step.output, step.output.per_min * share, transport),
            }
        })
        .collect()
}

/// One row per miner, or per line for fluids and gathered items.
pub fn source_rows(raw: &RawNeed, per_miner: f64) -> Vec<Row> {
    raw.belts.iter().flat_map(|group| group_rows(raw, group, per_miner)).collect()
}

/// Miners that share a belt each get a row; all run at 100% but the last.
fn group_rows(raw: &RawNeed, group: &BeltGroup, per_miner: f64) -> Vec<Row> {
    let line = |per_min: f64, machines: Vec<f64>| Row {
        inputs: vec![],
        machines,
        output: BeltLine {
            item: raw.item.clone(),
            item_name: raw.item_name.clone(),
            per_min,
            fluid: raw.fluid,
            over: group.over,
        },
    };
    if group.machines == 0 {
        return vec![line(group.per_min, vec![])];
    }
    let exact = group.per_min / per_miner;
    let last_pct = (exact - f64::from(group.machines - 1)) * 100.0;
    machine_clocks(group.machines, last_pct)
        .into_iter()
        .map(|clock| line(per_miner * clock / 100.0, vec![clock]))
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
