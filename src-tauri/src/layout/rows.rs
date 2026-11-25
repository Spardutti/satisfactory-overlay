use super::{BeltLine, Row};
use crate::planner::{Flow, RawNeed, Step, Transport};

/// One line per machine, input on the left and output on the right, like in the game.
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

/// One row per miner (each fills its own belt), or per line for fluids and gathered items.
pub fn source_rows(raw: &RawNeed, per_miner: f64) -> Vec<Row> {
    raw.belts
        .iter()
        .map(|group| Row {
            inputs: vec![],
            machines: match group.machines {
                0 => vec![],
                _ => vec![group.per_min / per_miner * 100.0],
            },
            output: BeltLine {
                item: raw.item.clone(),
                item_name: raw.item_name.clone(),
                per_min: group.per_min,
                fluid: raw.fluid,
                over: group.over,
            },
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

/// 100 for every machine, except the last one which runs at last_pct.
fn machine_clocks(machines: u32, last_pct: f64) -> Vec<f64> {
    (0..machines).map(|i| if i + 1 == machines { last_pct } else { 100.0 }).collect()
}
