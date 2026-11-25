use super::*;
use crate::game_data::{fixture, GameData, ItemAmount, Recipe};
use crate::planner::{Planner, Transport};
use std::collections::{HashMap, HashSet};

const MK1: Transport = Transport { belt: 60.0, pipe: 300.0 };

fn plan(data: &GameData, raw: &[&str], target: &str, rate: f64) -> PlanResult {
    let raw: HashSet<String> = raw.iter().map(|r| r.to_string()).collect();
    let (choices, alternates) = (HashMap::new(), HashSet::new());
    let planner = Planner {
        data,
        raw: &raw,
        choices: &choices,
        enabled_alternates: &alternates,
        miner_rate: 60.0,
        transport: MK1,
    };
    planner.plan_rate(target, rate).unwrap()
}

fn screw_cards(rate: f64) -> Vec<ChecklistStep> {
    checklist(&plan(&fixture(), &["Desc_OreIron_C"], "Desc_IronScrew_C", rate), "Mk1")
}

fn card<'a>(cards: &'a [ChecklistStep], label: &str) -> &'a ChecklistStep {
    cards.iter().find(|c| c.label == label).unwrap()
}

#[test]
fn checklist_120_screws_lists_miner_then_machines() {
    let cards = screw_cards(120.0);
    let summary: Vec<(&str, u32)> = cards.iter().map(|c| (c.label.as_str(), c.total)).collect();
    assert_eq!(
        summary,
        vec![
            ("Miner Mk1 — Iron Ore", 1),
            ("Smelter — Iron Ingot", 1),
            ("Constructor — Iron Rod", 2),
            ("Constructor — Screw", 3),
        ]
    );
}

#[test]
fn checklist_screw_card_on_mk1_belt_needs_one_belt_per_machine() {
    let cards = screw_cards(120.0);
    let screw = card(&cards, "Constructor — Screw");
    assert_eq!(
        (screw.detail.as_str(), screw.output.as_str(), screw.goes_to.as_str()),
        ("3 × Constructor · 40 /min each · 120 /min total", "3 belts · 1 each (40)", "final product")
    );
}

#[test]
fn checklist_screw_card_inputs_share_one_rod_belt() {
    let cards = screw_cards(120.0);
    assert_eq!(card(&cards, "Constructor — Screw").inputs, vec!["Iron Rod: 1 belt · 3 machines (30)"]);
}

#[test]
fn checklist_miner_card_says_where_the_ore_goes() {
    let cards = screw_cards(120.0);
    let miner = card(&cards, "Miner Mk1 — Iron Ore");
    assert_eq!((miner.output.as_str(), miner.goes_to.as_str()), ("1 belt (30)", "30 to Iron Ingot"));
}

#[test]
fn checklist_partial_machine_shows_last_clock() {
    let cards = screw_cards(100.0);
    assert_eq!(cards.last().unwrap().detail, "3 × Constructor · 40 /min each · 100 /min total · last at 50%");
}

#[test]
fn checklist_partial_machine_shares_a_belt_with_a_full_one() {
    let cards = screw_cards(100.0);
    assert_eq!(cards.last().unwrap().output, "2 belts · 2 (60) + 1 (40)");
}

fn recipe(id: &str, inputs: &[(&str, f64)], outputs: &[(&str, f64)]) -> Recipe {
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
        building: "Refinery".into(),
    }
}

#[test]
fn checklist_two_items_from_one_recipe_get_distinct_labels() {
    let mut data = GameData::default();
    data.recipes.push(recipe("Plastic", &[("Oil", 1.0)], &[("Plastic", 1.0), ("Residue", 1.0)]));
    data.recipes.push(recipe("Top", &[("Plastic", 1.0), ("Residue", 1.0)], &[("Top", 1.0)]));
    let cards = checklist(&plan(&data, &["Oil"], "Top", 1.0), "Mk1");
    let labels: HashSet<&str> = cards.iter().map(|s| s.label.as_str()).collect();
    assert_eq!(labels.len(), cards.len(), "{labels:?}");
}

#[test]
fn plan_saved_before_belts_loads_with_mk1_tiers() {
    let old = r#"{"target":"Desc_IronScrew_C","mode":{"kind":"rate","rate":60},"minerTier":"Mk2","purity":"pure"}"#;
    let request: crate::plans::PlanRequest = serde_json::from_str(old).unwrap();
    assert_eq!((request.belt_tier.as_str(), request.pipe_tier.as_str()), ("Mk1", "Mk1"));
}

#[test]
fn checklist_miner_card_names_the_listed_nodes_and_the_shortfall() {
    let mut result = plan(&fixture(), &["Desc_OreIron_C"], "Desc_IronScrew_C", 480.0);
    let rates = crate::planner::NodeRates { pure: 120.0, normal: 60.0, impure: 30.0, belt: 60.0 };
    let listed = [("Desc_OreIron_C".to_string(), crate::planner::NodeCounts { pure: 1, normal: 0, impure: 1 })];
    crate::planner::apply_nodes(&mut result, &listed.into(), rates);
    let cards = checklist(&result, "Mk1");
    assert_eq!(
        card(&cards, "Miner Mk1 — Iron Ore").detail,
        "2 × Miner Mk1 on 1 pure + 1 impure nodes · 90 /min · 30 short · the belt holds each miner back, a faster belt needs fewer nodes"
    );
}

#[test]
fn checklist_miner_card_without_nodes_assumes_normal_nodes() {
    let cards = screw_cards(480.0);
    assert_eq!(card(&cards, "Miner Mk1 — Iron Ore").detail, "2 × Miner Mk1 on 2 normal nodes · 120 /min");
}
