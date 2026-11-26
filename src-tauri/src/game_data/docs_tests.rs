use super::*;
use crate::game_data::fixture;

fn recipe<'a>(data: &'a GameData, id: &str) -> Option<&'a Recipe> {
    data.recipes.iter().find(|r| r.id == id)
}

#[test]
fn parse_docs_reads_screw_recipe_with_building_and_duration() {
    let data = fixture();
    let screw = recipe(&data, "Recipe_Screw_C").unwrap();
    assert_eq!(
        (screw.name.as_str(), screw.building.as_str(), screw.duration_secs, screw.alternate),
        ("Screw", "Build_ConstructorMk1_C", 6.0, false)
    );
    assert_eq!(screw.products, vec![ItemAmount { item: "Desc_IronScrew_C".into(), amount: 4.0 }]);
}

#[test]
fn parse_docs_marks_alternate_recipes() {
    assert!(recipe(&fixture(), "Recipe_Alternate_Screw_C").unwrap().alternate);
}

#[test]
fn parse_docs_divides_fluid_amounts_by_1000() {
    let wet = recipe(&fixture(), "Recipe_Alternate_WetConcrete_C").unwrap().clone();
    assert_eq!(
        wet.ingredients,
        vec![
            ItemAmount { item: "Desc_Stone_C".into(), amount: 6.0 },
            ItemAmount { item: "Desc_Water_C".into(), amount: 5.0 },
        ]
    );
}

#[test]
fn parse_docs_drops_build_gun_recipes() {
    assert!(recipe(&fixture(), "Recipe_ConstructorMk1_C").is_none());
}

#[test]
fn parse_docs_skips_broken_entries_with_warnings() {
    let data = fixture();
    assert!(recipe(&data, "Recipe_NoDuration_C").is_none());
    assert_eq!(
        data.warnings,
        vec![
            "Skipped an item without ClassName or mDisplayName",
            "Skipped recipe Recipe_NoDuration_C: missing mManufactoringDuration",
        ]
    );
}

#[test]
fn parse_docs_reads_building_display_names() {
    assert_eq!(fixture().building_name("Build_ConstructorMk1_C"), "Constructor");
}

#[test]
fn parse_docs_reads_item_form() {
    let data = fixture();
    assert_eq!((data.is_fluid("Desc_Water_C"), data.is_fluid("Desc_OreIron_C")), (true, false));
}

#[test]
fn parse_docs_non_array_root_is_an_error() {
    assert!(parse_docs("{}").unwrap_err().starts_with("Docs file is not a JSON array"));
}

#[test]
fn parse_docs_reads_factory_power_draw() {
    let data = fixture();
    assert_eq!((data.full_clock_mw("Smelter"), data.full_clock_mw("Constructor")), (4.0, 0.0));
}
