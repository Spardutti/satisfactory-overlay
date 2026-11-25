use super::{Block, Metrics, Row};

pub const METRICS: Metrics = Metrics {
    machine_width: 64.0,
    machine_gap: 24.0,
    row_height: 64.0,
    input_step: 14.0,
    header: 40.0,
    padding: 24.0,
    column_gap: 280.0,
    block_gap: 48.0,
    min_block_width: 200.0,
};

/// Rows grow by one belt's height for every input after the first.
pub fn row_height(rows: &[Row]) -> f64 {
    let inputs = rows.iter().map(|r| r.inputs.len()).max().unwrap_or(0) as f64;
    METRICS.row_height + (inputs - 1.0).max(0.0) * METRICS.input_step
}

/// Width and height of a block that holds these rows.
pub fn block_size(rows: &[Row]) -> (f64, f64) {
    let m = METRICS;
    let widest = rows.iter().map(|r| r.machines.len()).max().unwrap_or(0) as f64;
    let machines_width = widest * m.machine_width + (widest - 1.0).max(0.0) * m.machine_gap;
    let width = (2.0 * m.padding + machines_width).max(m.min_block_width);
    let height = m.header + rows.len() as f64 * row_height(rows) + m.padding;
    (width, height)
}

/// Puts blocks in columns by level, left to right, stacked top to bottom; returns the canvas size.
pub fn place_columns(blocks: &mut [(u32, Block)]) -> (f64, f64) {
    let m = METRICS;
    let mut levels: Vec<u32> = blocks.iter().map(|(level, _)| *level).collect();
    levels.sort_unstable();
    levels.dedup();
    let (mut x, mut height) = (0.0_f64, 0.0_f64);
    for level in levels {
        let (mut y, mut column_width) = (0.0_f64, 0.0_f64);
        for (_, block) in blocks.iter_mut().filter(|(l, _)| *l == level) {
            (block.x, block.y) = (x, y);
            y += block.height + m.block_gap;
            column_width = column_width.max(block.width);
        }
        height = height.max(y - m.block_gap);
        x += column_width + m.column_gap;
    }
    ((x - m.column_gap).max(0.0), height.max(0.0))
}
