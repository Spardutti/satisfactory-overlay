use super::{Block, Layout};
use crate::game_data::{clocked_mw, GameData};

/// MW every machine in the blueprint draws, each at its own clock.
pub fn power_use(layout: &Layout, data: &GameData) -> f64 {
    layout.blocks.iter().map(|block| block_mw(block, data)).sum()
}

fn block_mw(block: &Block, data: &GameData) -> f64 {
    let full = data.full_clock_mw(&block.building_name);
    block.rows.iter().flat_map(|row| &row.machines).map(|clock| clocked_mw(full, *clock)).sum()
}
