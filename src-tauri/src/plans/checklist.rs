use super::model::ChecklistStep;
use crate::planner::PlanResult;

/// Turns a calculation into build-order steps: miners first, final product last.
pub fn checklist(result: &PlanResult, miner_tier: &str, purity: &str) -> Vec<ChecklistStep> {
    let miners = result.raw.iter().filter_map(|raw| {
        Some(ChecklistStep {
            label: format!("Miner {miner_tier} — {}", raw.item_name),
            detail: format!("{} /min on {purity} nodes", round(raw.per_min)),
            total: raw.miners?,
            built: 0,
        })
    });
    let machines = result.steps.iter().map(|step| ChecklistStep {
        label: format!("{} — {}", step.building_name, step.recipe_name),
        detail: machine_detail(step.output_per_min, step.last_machine_pct),
        total: step.machines,
        built: 0,
    });
    miners.chain(machines).filter(|s| s.total > 0).collect()
}

fn machine_detail(per_min: f64, last_pct: f64) -> String {
    let rate = format!("{} /min", round(per_min));
    match last_pct < 99.95 {
        true => format!("{rate} · last one at {}%", round(last_pct)),
        false => rate,
    }
}

fn round(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data::fixture;
    use crate::planner::Planner;
    use std::collections::{HashMap, HashSet};

    fn screw_result(rate: f64) -> PlanResult {
        let data = fixture();
        let raw = HashSet::from(["Desc_OreIron_C".to_string()]);
        let (choices, alternates) = (HashMap::new(), HashSet::new());
        let planner =
            Planner { data: &data, raw: &raw, choices: &choices, enabled_alternates: &alternates, miner_rate: 60.0 };
        planner.plan_rate("Desc_IronScrew_C", rate).unwrap()
    }

    fn summary(steps: &[ChecklistStep]) -> Vec<(&str, &str, u32)> {
        steps.iter().map(|s| (s.label.as_str(), s.detail.as_str(), s.total)).collect()
    }

    #[test]
    fn checklist_120_screws_lists_miner_then_machines() {
        let steps = checklist(&screw_result(120.0), "Mk1", "normal");
        assert_eq!(
            summary(&steps),
            vec![
                ("Miner Mk1 — Iron Ore", "30 /min on normal nodes", 1),
                ("Smelter — Iron Ingot", "30 /min", 1),
                ("Constructor — Iron Rod", "30 /min", 2),
                ("Constructor — Screw", "120 /min", 3),
            ]
        );
    }

    #[test]
    fn checklist_partial_machine_shows_last_clock() {
        let steps = checklist(&screw_result(100.0), "Mk1", "normal");
        assert_eq!(steps.last().unwrap().detail, "100 /min · last one at 50%");
    }
}
