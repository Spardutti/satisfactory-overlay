use super::{Block, Connection, Layout};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildCount {
    pub name: String,
    pub count: u32,
}

/// Sources the player builds nothing for.
const NOT_BUILT: [&str; 2] = ["Bring in", "Depot"];

/// Every building, splitter, merger and pipe junction the blueprint draws, most first.
pub fn build_counts(layout: &Layout) -> Vec<BuildCount> {
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    let mut add = |name: String, n: u32| *counts.entry(name).or_default() += n;
    for block in &layout.blocks {
        machines(block).into_iter().for_each(|(name, n)| add(name, n));
        bus_splits(block).into_iter().for_each(|(name, n)| add(name.into(), n));
    }
    link_splits(layout).into_iter().for_each(|(name, n)| add(name.into(), n));
    let mut list: Vec<BuildCount> = counts.into_iter().filter(|(_, n)| *n > 0).map(|(name, count)| BuildCount { name, count }).collect();
    list.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));
    list
}

fn machines(block: &Block) -> Option<(String, u32)> {
    if NOT_BUILT.contains(&block.building_name.as_str()) {
        return None;
    }
    Some((block.building_name.clone(), block.rows.iter().map(|r| r.machines.len() as u32).sum()))
}

/// A bus that serves several rows has a splitter or merger at each one.
fn bus_splits(block: &Block) -> Vec<(&'static str, u32)> {
    let inputs = block.input_buses.iter().map(|b| (b, "Splitter"));
    let outputs = block.output_buses.iter().map(|b| (b, "Merger"));
    inputs
        .chain(outputs)
        .filter(|(bus, _)| bus.taps.len() > 1)
        .map(|(bus, belt_part)| (if bus.fluid { "Pipeline Junction" } else { belt_part }, bus.taps.len() as u32))
        .collect()
}

/// One splitter where a bus feeds several blocks, one merger where several feed one bus.
fn link_splits(layout: &Layout) -> Vec<(&'static str, u32)> {
    let fluid = |c: &Connection| {
        let from = layout.blocks.iter().find(|b| b.item == c.item);
        from.and_then(|b| b.output_buses.get(c.from_bus)).is_some_and(|bus| bus.fluid)
    };
    let mut froms: BTreeMap<(&str, usize), (u32, bool)> = BTreeMap::new();
    let mut intos: BTreeMap<(&str, usize), (u32, bool)> = BTreeMap::new();
    for c in &layout.connections {
        froms.entry((&c.item, c.from_bus)).or_insert((0, fluid(c))).0 += 1;
        intos.entry((&c.to_item, c.to_bus)).or_insert((0, fluid(c))).0 += 1;
    }
    shared(froms.into_values(), "Splitter").chain(shared(intos.into_values(), "Merger")).collect()
}

/// Groups of (links, fluid); each group with 2+ links needs one part.
fn shared(groups: impl Iterator<Item = (u32, bool)>, belt_part: &'static str) -> impl Iterator<Item = (&'static str, u32)> {
    groups.filter(|(n, _)| *n > 1).map(move |(_, fluid)| (if fluid { "Pipeline Junction" } else { belt_part }, 1))
}
