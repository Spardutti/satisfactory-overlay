use crate::planner::{BeltGroup, Usage};

/// How machines share belt or pipe lines, e.g. "3 belts · 2 each (60)" or "2 belts · 3 (60) + 2 (30)".
pub fn belts_text(groups: &[BeltGroup], fluid: bool) -> String {
    let unit = if fluid { "pipe" } else { "belt" };
    let count = match groups.len() {
        1 => format!("1 {unit}"),
        n => format!("{n} {unit}s"),
    };
    let text = match groups {
        [] => "—".to_string(),
        [one] if one.machines <= 1 => format!("{count} ({})", round(one.per_min)),
        [one] => format!("{count} · {} machines ({})", one.machines, round(one.per_min)),
        [first, ..] if groups.iter().all(|g| g == first) && first.machines > 0 => {
            format!("{count} · {} each ({})", first.machines, round(first.per_min))
        }
        _ => format!("{count} · {}", group_list(groups)),
    };
    match groups.iter().any(|g| g.over) {
        true => format!("{text} · one machine overfills a {unit}, use a faster tier"),
        false => text,
    }
}

fn group_list(groups: &[BeltGroup]) -> String {
    let parts: Vec<String> = groups
        .iter()
        .map(|g| match g.machines {
            0 => round(g.per_min),
            m => format!("{m} ({})", round(g.per_min)),
        })
        .collect();
    parts.join(" + ")
}

/// "135 to Iron Plate · 45 to Iron Rod", or "final product" when nothing uses it.
pub fn goes_to_text(usages: &[Usage]) -> String {
    if usages.is_empty() {
        return "final product".into();
    }
    let parts: Vec<String> = usages.iter().map(|u| format!("{} to {}", round(u.per_min), u.item_name)).collect();
    parts.join(" · ")
}

pub fn round(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// "1 pure + 2 impure nodes", best purity first.
pub fn nodes_text(purities: &[&str]) -> String {
    let count = |p: &str| purities.iter().filter(|x| **x == p).count();
    let parts: Vec<String> = ["pure", "normal", "impure"].iter().filter(|p| count(p) > 0).map(|p| format!("{} {p}", count(p))).collect();
    let noun = if purities.len() == 1 { "node" } else { "nodes" };
    format!("{} {noun}", parts.join(" + "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(machines: u32, per_min: f64) -> BeltGroup {
        BeltGroup { machines, per_min, over: false }
    }

    #[test]
    fn belts_text_one_machine_on_one_belt() {
        assert_eq!(belts_text(&[group(1, 30.0)], false), "1 belt (30)");
    }

    #[test]
    fn belts_text_equal_groups_say_each() {
        assert_eq!(belts_text(&vec![group(2, 60.0); 3], false), "3 belts · 2 each (60)");
    }

    #[test]
    fn belts_text_unequal_groups_are_listed() {
        assert_eq!(belts_text(&[group(3, 60.0), group(2, 30.0)], false), "2 belts · 3 (60) + 2 (30)");
    }

    #[test]
    fn belts_text_fluid_lines_without_machines_list_amounts() {
        assert_eq!(belts_text(&[group(0, 300.0), group(0, 100.0)], true), "2 pipes · 300 + 100");
    }

    #[test]
    fn belts_text_overfilled_belt_warns() {
        let over = BeltGroup { machines: 1, per_min: 90.0, over: true };
        assert_eq!(belts_text(&[over], false), "1 belt (90) · one machine overfills a belt, use a faster tier");
    }

    #[test]
    fn goes_to_text_lists_each_user() {
        let usages = [
            Usage { item: "P".into(), item_name: "Iron Plate".into(), per_min: 135.0 },
            Usage { item: "R".into(), item_name: "Iron Rod".into(), per_min: 45.0 },
        ];
        assert_eq!(goes_to_text(&usages), "135 to Iron Plate · 45 to Iron Rod");
    }

    #[test]
    fn goes_to_text_nothing_is_final_product() {
        assert_eq!(goes_to_text(&[]), "final product");
    }
}
