use super::tests::{setup, MK1};
use super::*;
use crate::game_data::fixture;
use crate::settings::Extractors;

const MK1_PIPE_600: Transport = Transport { belt: 60.0, pipe: 600.0 };

fn built_in() -> Extractors {
    serde_json::from_str(include_str!("../../../config/extractors.json")).unwrap()
}

fn building(book: &ExtractorBook, item: &str) -> Option<String> {
    book.for_item(&fixture(), item).map(|e| e.building.clone())
}

#[test]
fn for_item_iron_ore_gets_the_miner() {
    let book = built_in().book("Mk1", MK1).unwrap();
    assert_eq!(building(&book, "Desc_OreIron_C"), Some("Miner Mk1".into()));
}

#[test]
fn for_item_water_gets_the_water_extractor_at_120_on_every_spot() {
    let book = built_in().book("Mk1", MK1).unwrap();
    let water = book.for_item(&fixture(), "Desc_Water_C").unwrap();
    assert_eq!((water.building.as_str(), water.rates.impure, water.rates.pure, water.purity), ("Water Extractor", 120.0, 120.0, false));
}

#[test]
fn for_item_leaves_get_no_extractor() {
    let book = built_in().book("Mk1", MK1).unwrap();
    assert_eq!(building(&book, "Desc_Leaves_C"), None);
}

#[test]
fn book_oil_extractor_makes_60_120_240_capped_by_the_pipe() {
    let book = built_in().book("Mk1", MK1_PIPE_600).unwrap();
    let rates = book.fluids["Desc_LiquidOil_C"].rates;
    assert_eq!(rates, NodeRates { pure: 240.0, normal: 120.0, impure: 60.0, belt: 600.0 });
}

#[test]
fn plan_300_water_on_mk1_pipe_needs_3_water_extractors() {
    let mut s = setup(fixture());
    s.alternates.insert("Recipe_Alternate_WetConcrete_C".into());
    s.extractors = built_in().book("Mk1", MK1).unwrap();
    let result = s.planner().plan_rate("Desc_Cement_C", 240.0).unwrap();
    let water = result.raw.iter().find(|r| r.item == "Desc_Water_C").unwrap();
    assert_eq!((water.per_min, water.miners), (300.0, Some(3)));
}

#[test]
fn oil_on_1_pure_node_draws_240_from_one_oil_extractor_at_100() {
    let rates = built_in().book("Mk1", MK1_PIPE_600).unwrap().fluids["Desc_LiquidOil_C"].rates;
    let supply = draw_from_nodes(240.0, NodeCounts { pure: 1, normal: 0, impure: 0 }, rates);
    let draws: Vec<(f64, f64)> = supply.draws.iter().map(|d| (d.per_min, d.clock)).collect();
    assert_eq!(draws, vec![(240.0, 100.0)]);
}

#[test]
fn book_old_extractors_file_without_fluids_still_gets_the_water_extractor() {
    let old: Extractors = serde_json::from_str(r#"{"miners":{"Mk1":60},"purity":{"impure":0.5,"normal":1.0,"pure":2.0}}"#).unwrap();
    assert_eq!(building(&old.book("Mk1", MK1).unwrap(), "Desc_Water_C"), Some("Water Extractor".into()));
}

#[test]
fn book_fluid_rate_of_0_is_an_error() {
    let zero: Extractors = serde_json::from_str(
        r#"{"miners":{"Mk1":60},"purity":{"impure":0.5,"normal":1.0,"pure":2.0},"fluids":{"Desc_Water_C":{"building":"Water Extractor","rate":0,"purity":false}}}"#,
    )
    .unwrap();
    assert_eq!(zero.book("Mk1", MK1).unwrap_err(), "Water Extractor needs a rate above 0 in extractors.json");
}
