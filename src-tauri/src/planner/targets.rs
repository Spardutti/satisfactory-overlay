use super::book::standard_recipe;
use crate::game_data::GameData;
use serde::Serialize;
use std::collections::BTreeSet;

/// An item you can plan for, with what one machine makes on its standard recipe.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub id: String,
    pub name: String,
    pub standard: Option<StandardRate>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardRate {
    pub building_name: String,
    pub per_machine: f64,
}

/// Every item some factory recipe makes, sorted by name.
pub fn targets(data: &GameData) -> Vec<Target> {
    let ids: BTreeSet<&str> =
        data.recipes.iter().flat_map(|r| r.products.iter().map(|p| p.item.as_str())).collect();
    let mut list: Vec<Target> = ids.into_iter().map(|id| target(data, id)).collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    list
}

fn target(data: &GameData, item: &str) -> Target {
    let standard = standard_recipe(data, item).map(|recipe| {
        let made = recipe.products.iter().find(|p| p.item == item).map_or(0.0, |p| p.amount);
        StandardRate { building_name: data.building_name(&recipe.building), per_machine: recipe.rate_per_min(made) }
    });
    Target { id: item.to_string(), name: data.item_name(item), standard }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data::fixture;

    fn find(id: &str) -> Target {
        targets(&fixture()).into_iter().find(|t| t.id == id).unwrap()
    }

    #[test]
    fn targets_screw_standard_rate_is_40_per_constructor() {
        let standard = StandardRate { building_name: "Constructor".into(), per_machine: 40.0 };
        assert_eq!(find("Desc_IronScrew_C").standard, Some(standard));
    }

    #[test]
    fn targets_item_made_only_by_an_alternate_has_no_standard_rate() {
        assert_eq!(find("Desc_Cement_C").standard, None);
    }

    #[test]
    fn targets_are_sorted_by_name() {
        let names: Vec<String> = targets(&fixture()).into_iter().map(|t| t.name).collect();
        assert_eq!(names, vec!["Biomass", "Concrete", "Iron Ingot", "Iron Rod", "Screw"]);
    }
}
