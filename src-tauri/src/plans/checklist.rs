use super::card_text::{belts_text, goes_to_text, round};
use super::model::ChecklistStep;
use crate::planner::{PlanResult, RawNeed, Step};

/// Turns a calculation into build-order cards: miners first, final product last.
pub fn checklist(result: &PlanResult, miner_tier: &str, purity: &str) -> Vec<ChecklistStep> {
    let miners = result.raw.iter().filter_map(|raw| miner_card(raw, miner_tier, purity));
    let machines = result.steps.iter().map(machine_card);
    miners.chain(machines).filter(|s| s.total > 0).collect()
}

fn miner_card(raw: &RawNeed, tier: &str, purity: &str) -> Option<ChecklistStep> {
    let miners = raw.miners?;
    Some(ChecklistStep {
        label: format!("Miner {tier} — {}", raw.item_name),
        detail: miner_detail(raw, tier, purity),
        total: miners,
        built: 0,
        inputs: vec![],
        output: belts_text(&raw.belts, raw.fluid),
        goes_to: goes_to_text(&raw.goes_to),
        item: raw.item.clone(),
    })
}

/// "2 × Miner Mk2 on 2 normal nodes · 120 /min", plus a warning when the belt holds miners back.
fn miner_detail(raw: &RawNeed, tier: &str, purity: &str) -> String {
    let miners = raw.miners.unwrap_or(0);
    let nodes = if miners == 1 { "node" } else { "nodes" };
    let base = format!("{miners} × Miner {tier} on {miners} {purity} {nodes} · {} /min", round(raw.per_min));
    match raw.capped {
        true => format!("{base} · the belt holds each miner back, a faster belt needs fewer nodes"),
        false => base,
    }
}

fn machine_card(step: &Step) -> ChecklistStep {
    ChecklistStep {
        label: machine_label(step),
        detail: machine_detail(step),
        total: step.machines,
        built: 0,
        inputs: step
            .inputs
            .iter()
            .map(|i| format!("{}: {}", i.item_name, belts_text(&i.belts, i.fluid)))
            .collect(),
        output: belts_text(&step.output.belts, step.output.fluid),
        goes_to: goes_to_text(&step.goes_to),
        item: step.item.clone(),
    }
}

/// Names the item too when its recipe is named after another product, so labels stay unique.
fn machine_label(step: &Step) -> String {
    match step.recipe_name == step.item_name {
        true => format!("{} — {}", step.building_name, step.recipe_name),
        false => format!("{} — {} → {}", step.building_name, step.recipe_name, step.item_name),
    }
}

/// "5 × Constructor · 40 /min each · 180 /min total · last at 50%"
fn machine_detail(step: &Step) -> String {
    let base = format!(
        "{} × {} · {} /min each · {} /min total",
        step.machines,
        step.building_name,
        round(step.per_machine),
        round(step.output_per_min)
    );
    match step.last_machine_pct < 99.95 {
        true => format!("{base} · last at {}%", round(step.last_machine_pct)),
        false => base,
    }
}

#[cfg(test)]
#[path = "checklist_tests.rs"]
mod tests;
