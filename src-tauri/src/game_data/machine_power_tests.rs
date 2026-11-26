use super::machine_power::*;
use serde_json::json;

#[test]
fn machine_power_reads_steady_draw_by_name_without_dots() {
    let miner = json!({ "mDisplayName": "Miner Mk.1", "mPowerConsumption": "5.000000" });
    assert_eq!(machine_power(&[&miner]).get("Miner Mk1"), Some(&5.0));
}

#[test]
fn machine_power_takes_middle_of_range_for_swinging_draw() {
    let accelerator = json!({
        "mDisplayName": "Particle Accelerator", "mPowerConsumption": "0.000000",
        "mEstimatedMininumPowerConsumption": "250.000000", "mEstimatedMaximumPowerConsumption": "1500.000000"
    });
    assert_eq!(machine_power(&[&accelerator]).get("Particle Accelerator"), Some(&875.0));
}

#[test]
fn machine_power_skips_building_that_draws_nothing() {
    let shop = json!({ "mDisplayName": "AWESOME Shop", "mPowerConsumption": "0.000000" });
    assert!(machine_power(&[&shop]).is_empty());
}

#[test]
fn clocked_mw_full_clock_draws_full_power() {
    assert_eq!(clocked_mw(4.0, 100.0), 4.0);
}

#[test]
fn clocked_mw_half_clock_draws_less_than_half() {
    assert!((clocked_mw(4.0, 50.0) - 1.6).abs() < 0.001);
}
