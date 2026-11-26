use super::*;

fn extractors() -> Extractors {
    serde_json::from_str(DEFAULT_EXTRACTORS).unwrap()
}

#[test]
fn miner_rate_mk2_on_pure_node_is_240() {
    assert_eq!(extractors().miner_rate("Mk2", "pure"), Ok(240.0));
}

#[test]
fn node_rates_mk2_on_mk1_belt_keeps_full_miner_rates() {
    let rates = extractors().node_rates("Mk2", 60.0).unwrap();
    assert_eq!(rates, NodeRates { pure: 240.0, normal: 120.0, impure: 60.0, belt: 60.0 });
}

#[test]
fn miner_rate_mk1_on_impure_node_is_30() {
    assert_eq!(extractors().miner_rate("Mk1", "impure"), Ok(30.0));
}

#[test]
fn miner_rate_of_zero_is_an_error() {
    let mut zero = extractors();
    zero.miners.insert("Mk1".into(), 0.0);
    assert_eq!(
        zero.miner_rate("Mk1", "normal"),
        Err("Miner Mk1 on normal nodes needs a rate above 0 in extractors.json".into())
    );
}

#[test]
fn miner_rate_unknown_tier_is_an_error() {
    assert_eq!(extractors().miner_rate("Mk9", "pure"), Err("Unknown miner tier Mk9".into()));
}

#[test]
fn load_editable_first_run_writes_default_files() {
    let dir = std::env::temp_dir().join(format!("sf-settings-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let editable = load_editable(&dir, &mut vec![]);
    assert!(editable.raw.contains("Desc_OreIron_C"));
    assert!(dir.join("raw_resources.json").is_file());
    assert!(dir.join("extractors.json").is_file());
}

fn logistics() -> Logistics {
    serde_json::from_str(DEFAULT_LOGISTICS).unwrap()
}

#[test]
fn belt_speed_mk3_is_270() {
    assert_eq!(logistics().belt_speed("Mk3"), Ok(270.0));
}

#[test]
fn belt_speed_unknown_tier_is_an_error_naming_it() {
    assert_eq!(logistics().belt_speed("Mk9"), Err("Unknown belt tier Mk9".into()));
}

#[test]
fn belt_speed_of_zero_is_an_error() {
    let zero = Logistics { belts: BTreeMap::from([("Mk1".to_string(), 0.0)]), pipes: BTreeMap::new() };
    assert_eq!(zero.belt_speed("Mk1"), Err("The belt tier Mk1 needs a speed above 0 in logistics.json".into()));
}

#[test]
fn pipe_speed_mk2_is_600() {
    assert_eq!(logistics().pipe_speed("Mk2"), Ok(600.0));
}

#[test]
fn read_or_backup_missing_config_returns_defaults() {
    let config: AppConfig = read_or_backup(Path::new("/nonexistent/config.json"), &mut vec![]);
    assert_eq!((config.docs_path, config.enabled_alternates.len()), (None, 0));
}
