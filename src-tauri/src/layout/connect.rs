use super::{Block, Connection};

const EPS: f64 = 1e-6;

/// Fills each demand in order from the supplies in order; a supply is split only when it must be.
/// Returns (supply index, demand index, amount).
pub fn match_belts(supply: &[f64], demand: &[f64]) -> Vec<(usize, usize, f64)> {
    let (mut left, mut need) = (supply.to_vec(), demand.to_vec());
    let (mut s, mut d) = (0, 0);
    let mut matches = vec![];
    while s < left.len() && d < need.len() {
        let amount = left[s].min(need[d]);
        if amount > EPS {
            matches.push((s, d, amount));
        }
        left[s] -= amount;
        need[d] -= amount;
        if left[s] <= EPS {
            s += 1;
        }
        if need[d] <= EPS {
            d += 1;
        }
    }
    matches
}

/// Every belt from a block's output buses to the input buses that use that item.
pub fn connect(blocks: &[Block]) -> Vec<Connection> {
    let mut connections = vec![];
    for source in blocks {
        let supply: Vec<f64> = source.output_buses.iter().map(|b| b.per_min).collect();
        let targets: Vec<(&str, usize, f64)> = blocks
            .iter()
            .flat_map(|block| {
                let used = block.input_buses.iter().enumerate().filter(|(_, bus)| bus.item == source.item);
                used.map(|(index, bus)| (block.item.as_str(), index, bus.per_min))
            })
            .collect();
        let demand: Vec<f64> = targets.iter().map(|t| t.2).collect();
        for (s, d, amount) in match_belts(&supply, &demand) {
            connections.push(Connection {
                item: source.item.clone(),
                from_bus: s,
                to_item: targets[d].0.to_string(),
                to_bus: targets[d].1,
                per_min: amount,
            });
        }
    }
    connections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_belts_one_to_one() {
        assert_eq!(match_belts(&[60.0, 60.0], &[60.0, 60.0]), vec![(0, 0, 60.0), (1, 1, 60.0)]);
    }

    #[test]
    fn match_belts_splits_a_belt_across_two_rows() {
        assert_eq!(match_belts(&[60.0, 60.0], &[30.0, 30.0, 60.0]), vec![(0, 0, 30.0), (0, 1, 30.0), (1, 2, 60.0)]);
    }

    #[test]
    fn match_belts_merges_two_belts_into_one_row() {
        assert_eq!(match_belts(&[60.0, 20.0], &[40.0, 40.0]), vec![(0, 0, 40.0), (0, 1, 20.0), (1, 1, 20.0)]);
    }

    #[test]
    fn match_belts_nothing_needed_connects_nothing() {
        assert_eq!(match_belts(&[60.0], &[]), vec![]);
    }
}
