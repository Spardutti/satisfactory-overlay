use super::{BeltLine, Row};
use crate::planner::Transport;
use serde::Serialize;

/// One belt running down a block's edge, with a splitter or merger at each row it serves.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bus {
    pub item: String,
    pub item_name: String,
    pub fluid: bool,
    pub per_min: f64,
    pub taps: Vec<Tap>,
}

/// Where a bus meets a row: which row, which of its input belts (0 for outputs), and how much.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tap {
    pub row: usize,
    pub input: usize,
    pub per_min: f64,
}

/// Groups neighbouring rows onto one belt until the next row would overfill it.
pub fn next_fit(amounts: &[f64], capacity: f64) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = vec![];
    let mut load = f64::INFINITY;
    for (row, amount) in amounts.iter().enumerate() {
        match groups.last_mut() {
            Some(group) if load + amount <= capacity + 1e-9 => {
                group.push(row);
                load += amount;
            }
            _ => {
                groups.push(vec![row]);
                load = *amount;
            }
        }
    }
    groups
}

/// One bus per input item and belt, each feeding a run of neighbouring rows.
pub fn input_buses(rows: &[Row], transport: Transport) -> Vec<Bus> {
    let Some(first) = rows.first() else { return vec![] };
    let mut buses = vec![];
    for (input, belt) in first.inputs.iter().enumerate() {
        let amounts: Vec<f64> = rows.iter().map(|r| r.inputs[input].per_min).collect();
        for group in next_fit(&amounts, transport.capacity(belt.fluid)) {
            let taps = group.iter().map(|&row| Tap { row, input, per_min: amounts[row] }).collect();
            buses.push(bus(belt, taps));
        }
    }
    buses
}

pub fn output_buses(rows: &[Row], transport: Transport) -> Vec<Bus> {
    let Some(first) = rows.first() else { return vec![] };
    let out = &first.output;
    let amounts: Vec<f64> = rows.iter().map(|r| r.output.per_min).collect();
    next_fit(&amounts, transport.capacity(out.fluid))
        .into_iter()
        .map(|group| {
            let taps = group.iter().map(|&row| Tap { row, input: 0, per_min: amounts[row] }).collect();
            bus(out, taps)
        })
        .collect()
}

fn bus(line: &BeltLine, taps: Vec<Tap>) -> Bus {
    let per_min = taps.iter().map(|t: &Tap| t.per_min).sum();
    Bus { item: line.item.clone(), item_name: line.item_name.clone(), fluid: line.fluid, per_min, taps }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_fit_keeps_rows_whole_and_in_order() {
        assert_eq!(next_fit(&[40.0, 40.0, 20.0], 60.0), vec![vec![0], vec![1, 2]]);
    }

    #[test]
    fn next_fit_fills_a_belt_exactly() {
        assert_eq!(next_fit(&[30.0, 30.0, 30.0], 60.0), vec![vec![0, 1], vec![2]]);
    }

    #[test]
    fn next_fit_no_rows_is_no_belts() {
        assert_eq!(next_fit(&[], 60.0), Vec::<Vec<usize>>::new());
    }
}
