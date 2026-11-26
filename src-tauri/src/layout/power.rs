use super::{Block, Layout};
use crate::game_data::{clocked_mw, GameData};
use std::collections::HashMap;

pub fn power_use(layout: &Layout, data: &GameData, recipe_of: &HashMap<String, String>) -> f64 {
    layout.blocks.iter().map(|block| block_mw(block, data, recipe_of.get(&block.item))).sum()
}

fn block_mw(block: &Block, data: &GameData, recipe_id: Option<&String>) -> f64 {
    let full = data.full_clock_mw(&block.building_name, recipe_id.map(String::as_str));
    block.rows.iter().flat_map(|row| &row.machines).map(|clock| clocked_mw(full, *clock)).sum()
}
