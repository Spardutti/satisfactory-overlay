use super::Planner;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawLeft {
    pub item: String,
    pub item_name: String,
    pub available: f64,
    pub used: f64,
    pub left: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Limit {
    pub max_rate: f64,
    pub limiting: String,
    pub limiting_name: String,
    pub resources: Vec<RawLeft>,
}

impl Planner<'_> {
    /// Mode B: the most `target` per minute the `available` raw resources allow.
    pub fn max_rate(&self, target: &str, available: &HashMap<String, f64>) -> Result<Limit, String> {
        let per_unit = self.plan_rate(target, 1.0)?.raw;
        let (limiting, max_rate) = per_unit
            .iter()
            .filter(|r| r.per_min > 0.0)
            .map(|r| (r.item.clone(), available.get(&r.item).copied().unwrap_or(0.0) / r.per_min))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .ok_or_else(|| format!("{} uses no raw resources", self.data.item_name(target)))?;
        let used = |item: &str| {
            per_unit.iter().find(|r| r.item == item).map_or(0.0, |r| r.per_min * max_rate)
        };
        let mut resources: Vec<RawLeft> = available
            .iter()
            .map(|(item, &avail)| RawLeft {
                item: item.clone(),
                item_name: self.data.item_name(item),
                available: avail,
                used: used(item),
                left: avail - used(item),
            })
            .collect();
        resources.sort_by(|a, b| a.item_name.cmp(&b.item_name));
        Ok(Limit { limiting_name: self.data.item_name(&limiting), limiting, max_rate, resources })
    }
}
