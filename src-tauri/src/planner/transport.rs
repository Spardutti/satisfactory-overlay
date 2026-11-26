use super::result::whole_machines;
use crate::game_data::POWER;
use serde::Serialize;

/// Speed of one belt line and one pipe line for the plan's chosen tiers.
#[derive(Debug, Clone, Copy)]
pub struct Transport {
    pub belt: f64,
    pub pipe: f64,
}

impl Transport {
    pub fn capacity(&self, fluid: bool) -> f64 {
        match fluid {
            true => self.pipe,
            false => self.belt,
        }
    }

    /// What one line of `item` carries; power runs on cables, which have no limit.
    pub fn line_capacity(&self, item: &str, fluid: bool) -> f64 {
        if item == POWER { f64::INFINITY } else { self.capacity(fluid) }
    }
}

/// Machines whose outputs share one belt or pipe line.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BeltGroup {
    pub machines: u32,
    pub per_min: f64,
    /// one machine alone makes more than the line carries
    pub over: bool,
}

/// Belt or pipe lines needed: 60 on a 60 belt is 1, 61 is 2.
pub fn lines_needed(per_min: f64, capacity: f64) -> u32 {
    whole_machines(per_min / capacity)
}

/// Packs whole machines onto output lines, biggest output first; a machine has one output.
pub fn belt_groups(per_machine: f64, machines_exact: f64, capacity: f64) -> Vec<BeltGroup> {
    let mut groups: Vec<BeltGroup> = vec![];
    for output in machine_outputs(per_machine, machines_exact) {
        match groups.iter_mut().find(|g| g.per_min + output <= capacity + 1e-9) {
            Some(group) => {
                group.machines += 1;
                group.per_min += output;
            }
            None => groups.push(BeltGroup { machines: 1, per_min: output, over: output > capacity + 1e-9 }),
        }
    }
    groups
}

/// Lines for a flow no machine sets the pace of: full lines, then the rest.
pub fn line_groups(per_min: f64, capacity: f64) -> Vec<BeltGroup> {
    let lines = lines_needed(per_min, capacity);
    (0..lines)
        .map(|i| BeltGroup {
            machines: 0,
            per_min: (per_min - f64::from(i) * capacity).min(capacity),
            over: false,
        })
        .collect()
}

/// Full machines first, then the partly used last one.
fn machine_outputs(per_machine: f64, machines_exact: f64) -> Vec<f64> {
    let machines = whole_machines(machines_exact);
    (0..machines)
        .map(|i| match i + 1 == machines {
            true => (machines_exact - f64::from(i)) * per_machine,
            false => per_machine,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sizes(groups: &[BeltGroup]) -> Vec<(u32, f64)> {
        groups.iter().map(|g| (g.machines, g.per_min)).collect()
    }

    #[test]
    fn lines_needed_exactly_at_capacity_is_one() {
        assert_eq!(lines_needed(60.0, 60.0), 1);
    }

    #[test]
    fn lines_needed_one_over_capacity_is_two() {
        assert_eq!(lines_needed(61.0, 60.0), 2);
    }

    #[test]
    fn belt_groups_three_40s_on_60_belt_are_three_belts() {
        assert_eq!(sizes(&belt_groups(40.0, 3.0, 60.0)), vec![(1, 40.0), (1, 40.0), (1, 40.0)]);
    }

    #[test]
    fn belt_groups_three_40s_on_120_belt_share_one_belt() {
        assert_eq!(sizes(&belt_groups(40.0, 3.0, 120.0)), vec![(3, 120.0)]);
    }

    #[test]
    fn belt_groups_half_machine_joins_a_full_one() {
        assert_eq!(sizes(&belt_groups(40.0, 2.5, 60.0)), vec![(2, 60.0), (1, 40.0)]);
    }

    #[test]
    fn belt_groups_machine_faster_than_belt_is_marked_over() {
        assert_eq!(belt_groups(90.0, 1.0, 60.0), vec![BeltGroup { machines: 1, per_min: 90.0, over: true }]);
    }

    #[test]
    fn line_groups_700_water_on_300_pipes_fills_two_and_a_rest() {
        assert_eq!(sizes(&line_groups(700.0, 300.0)), vec![(0, 300.0), (0, 300.0), (0, 100.0)]);
    }

    #[test]
    fn transport_capacity_uses_pipe_for_fluids() {
        let transport = Transport { belt: 60.0, pipe: 300.0 };
        assert_eq!((transport.capacity(true), transport.capacity(false)), (300.0, 60.0));
    }
}
