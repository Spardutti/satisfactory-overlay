use super::*;
use crate::planner::Usage;

fn source(from_depot: bool, used_by: &[&str]) -> RawNeed {
    let goes_to = used_by.iter().map(|i| Usage { item: i.to_string(), item_name: i.to_string(), per_min: 1.0 }).collect();
    RawNeed {
        item: "rotor".into(),
        item_name: "Rotor".into(),
        per_min: 4.0,
        fluid: false,
        miners: None,
        capped: false,
        belts: vec![],
        goes_to,
        nodes: None,
        from_depot,
        extractor: None,
    }
}

fn levels() -> HashMap<&'static str, u32> {
    HashMap::from([("plating", 4), ("motor", 5)])
}

#[test]
fn source_level_ore_is_column_0() {
    assert_eq!(source_level(&source(false, &["plating"]), &levels()), 0);
}

#[test]
fn source_level_depot_rotor_for_level_4_plating_is_column_3() {
    assert_eq!(source_level(&source(true, &["plating"]), &levels()), 3);
}

#[test]
fn source_level_depot_used_by_two_sits_left_of_the_earliest() {
    assert_eq!(source_level(&source(true, &["motor", "plating"]), &levels()), 3);
}

#[test]
fn source_level_depot_for_a_level_1_block_stays_at_0() {
    let levels = HashMap::from([("screw", 1)]);
    assert_eq!(source_level(&source(true, &["screw"]), &levels), 0);
}
