use super::*;

pub(super) const MK1: Transport = Transport { belt: 60.0, pipe: 300.0 };
use crate::game_data::{fixture, ItemAmount, Recipe};

pub(super) struct Setup {
    data: GameData,
    raw: HashSet<String>,
    pub(super) choices: HashMap<String, String>,
    pub(super) alternates: HashSet<String>,
    miner_rate: f64,
    pub(super) transport: Transport,
}

pub(super) fn setup(data: GameData) -> Setup {
    let raw = ["Desc_OreIron_C", "Desc_Stone_C", "Desc_Water_C", "Desc_Leaves_C"].map(String::from).into();
    Setup { data, raw, choices: HashMap::new(), alternates: HashSet::new(), miner_rate: 60.0, transport: MK1 }
}

impl Setup {
    pub(super) fn planner(&self) -> Planner<'_> {
        Planner {
            data: &self.data,
            raw: &self.raw,
            choices: &self.choices,
            enabled_alternates: &self.alternates,
            miner_rate: self.miner_rate,
            transport: self.transport,
        }
    }
}

fn machine_list(result: &PlanResult) -> Vec<(String, String, u32)> {
    result.steps.iter().map(|s| (s.building_name.clone(), s.item_name.clone(), s.machines)).collect()
}

fn screw_machines() -> Vec<(String, String, u32)> {
    [("Smelter", "Iron Ingot", 1), ("Constructor", "Iron Rod", 2), ("Constructor", "Screw", 3)]
        .map(|(b, i, n)| (b.to_string(), i.to_string(), n))
        .to_vec()
}

pub(super) fn recipe(id: &str, inputs: &[(&str, f64)], outputs: &[(&str, f64)]) -> Recipe {
    let amounts = |list: &[(&str, f64)]| {
        list.iter().map(|(item, amount)| ItemAmount { item: item.to_string(), amount: *amount }).collect()
    };
    Recipe {
        id: id.into(),
        name: id.into(),
        alternate: false,
        ingredients: amounts(inputs),
        products: amounts(outputs),
        duration_secs: 60.0,
        building: "Build_ConstructorMk1_C".into(),
    }
}

#[test]
fn max_rate_30_iron_ore_gives_120_screws_limited_by_ore() {
    let s = setup(fixture());
    let available = HashMap::from([("Desc_OreIron_C".to_string(), 30.0)]);
    let limit = s.planner().max_rate("Desc_IronScrew_C", &available).unwrap();
    assert_eq!((limit.max_rate, limit.limiting_name.as_str()), (120.0, "Iron Ore"));
    assert_eq!((limit.resources[0].used, limit.resources[0].left), (30.0, 0.0));
}

#[test]
fn plan_rate_at_mode_b_max_rate_gives_screw_machines() {
    let s = setup(fixture());
    let available = HashMap::from([("Desc_OreIron_C".to_string(), 30.0)]);
    let max = s.planner().max_rate("Desc_IronScrew_C", &available).unwrap().max_rate;
    assert_eq!(machine_list(&s.planner().plan_rate("Desc_IronScrew_C", max).unwrap()), screw_machines());
}

#[test]
fn plan_rate_120_screws_lists_machines_in_build_order() {
    let result = setup(fixture()).planner().plan_rate("Desc_IronScrew_C", 120.0).unwrap();
    assert_eq!(machine_list(&result), screw_machines());
}

#[test]
fn plan_rate_120_screws_needs_30_iron_ore_and_one_mk1_miner() {
    let result = setup(fixture()).planner().plan_rate("Desc_IronScrew_C", 120.0).unwrap();
    let ore = &result.raw[0];
    assert_eq!((ore.item_name.as_str(), ore.per_min, ore.miners), ("Iron Ore", 30.0, Some(1)));
}

#[test]
fn plan_rate_one_ore_past_miner_rate_needs_a_second_miner() {
    let mut s = setup(fixture());
    s.miner_rate = 29.0;
    let result = s.planner().plan_rate("Desc_IronScrew_C", 120.0).unwrap();
    assert_eq!(result.raw[0].miners, Some(2));
}

#[test]
fn plan_rate_100_screws_runs_last_constructor_at_half() {
    let result = setup(fixture()).planner().plan_rate("Desc_IronScrew_C", 100.0).unwrap();
    let screw = result.steps.last().unwrap();
    assert_eq!((screw.machines_exact, screw.machines, screw.last_machine_pct), (2.5, 3, 50.0));
}

#[test]
fn plan_rate_uses_chosen_alternate_when_owned() {
    let mut s = setup(fixture());
    s.alternates.insert("Recipe_Alternate_Screw_C".into());
    s.choices.insert("Desc_IronScrew_C".into(), "Recipe_Alternate_Screw_C".into());
    let result = s.planner().plan_rate("Desc_IronScrew_C", 50.0).unwrap();
    let steps: Vec<(&str, f64)> = result.steps.iter().map(|s| (s.recipe_name.as_str(), s.machines_exact)).collect();
    assert_eq!(steps, vec![("Iron Ingot", 12.5 / 30.0), ("Alternate: Cast Screw", 1.0)]);
}

#[test]
fn plan_rate_ignores_choice_of_alternate_not_owned() {
    let mut s = setup(fixture());
    s.choices.insert("Desc_IronScrew_C".into(), "Recipe_Alternate_Screw_C".into());
    let result = s.planner().plan_rate("Desc_IronScrew_C", 40.0).unwrap();
    assert_eq!(result.steps.last().unwrap().recipe_id, "Recipe_Screw_C");
}

#[test]
fn plan_rate_fluid_raw_has_no_miners() {
    let mut s = setup(fixture());
    s.alternates.insert("Recipe_Alternate_WetConcrete_C".into());
    let result = s.planner().plan_rate("Desc_Cement_C", 80.0).unwrap();
    let raw: Vec<(&str, f64, Option<u32>)> =
        result.raw.iter().map(|r| (r.item_name.as_str(), r.per_min, r.miners)).collect();
    assert_eq!(raw, vec![("Limestone", 120.0, Some(2)), ("Water", 100.0, None)]);
}

#[test]
fn plan_rate_item_without_recipe_is_an_error() {
    let err = setup(fixture()).planner().plan_rate("Desc_Cement_C", 10.0).unwrap_err();
    assert_eq!(err, "No recipe makes Concrete");
}

#[test]
fn plan_rate_lists_byproducts_as_surplus() {
    let mut data = GameData::default();
    data.recipes.push(recipe("Split", &[("Desc_OreIron_C", 2.0)], &[("A", 1.0), ("B", 3.0)]));
    let result = setup(data).planner().plan_rate("A", 2.0).unwrap();
    let surplus: Vec<(&str, f64)> = result.surplus.iter().map(|s| (s.item.as_str(), s.per_min)).collect();
    assert_eq!(surplus, vec![("B", 6.0)]);
}

#[test]
fn plan_rate_shared_ingredient_sums_demand_into_one_step() {
    let mut data = GameData::default();
    data.recipes.push(recipe("Base", &[("Desc_OreIron_C", 1.0)], &[("Base", 1.0)]));
    data.recipes.push(recipe("Mid", &[("Base", 1.0)], &[("Mid", 1.0)]));
    data.recipes.push(recipe("Top", &[("Base", 2.0), ("Mid", 1.0)], &[("Top", 1.0)]));
    let result = setup(data).planner().plan_rate("Top", 1.0).unwrap();
    let base: Vec<f64> = result.steps.iter().filter(|s| s.item == "Base").map(|s| s.output_per_min).collect();
    assert_eq!(base, vec![3.0]);
}

#[test]
fn max_rate_resource_not_available_limits_to_zero() {
    let available = HashMap::from([("Desc_Stone_C".to_string(), 100.0)]);
    let limit = setup(fixture()).planner().max_rate("Desc_IronScrew_C", &available).unwrap();
    assert_eq!((limit.max_rate, limit.limiting.as_str()), (0.0, "Desc_OreIron_C"));
}

#[test]
fn plan_rate_gathered_raw_item_has_no_miners() {
    let result = setup(fixture()).planner().plan_rate("Desc_GenericBiomass_C", 60.0).unwrap();
    let raw: Vec<(&str, f64, Option<u32>)> =
        result.raw.iter().map(|r| (r.item_name.as_str(), r.per_min, r.miners)).collect();
    assert_eq!(raw, vec![("Leaves", 120.0, None)]);
}
