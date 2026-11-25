use super::bus::Bus;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub width: f64,
    pub height: f64,
    pub blocks: Vec<Block>,
    pub connections: Vec<Connection>,
    pub metrics: Metrics,
}

/// Sizes the screen draws rows and machines with, so both sides agree.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub machine_width: f64,
    pub machine_gap: f64,
    pub row_height: f64,
    /// extra row height for every input belt after the first
    pub input_step: f64,
    pub header: f64,
    pub padding: f64,
    pub column_gap: f64,
    pub block_gap: f64,
    pub min_block_width: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub item: String,
    pub title: String,
    pub building_name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub row_height: f64,
    pub rows: Vec<Row>,
    pub input_buses: Vec<Bus>,
    pub output_buses: Vec<Bus>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub inputs: Vec<BeltLine>,
    /// clock of each machine in the row, 100 unless it is the last one
    pub machines: Vec<f64>,
    pub output: BeltLine,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BeltLine {
    pub item: String,
    pub item_name: String,
    pub per_min: f64,
    pub fluid: bool,
    pub over: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub item: String,
    pub from_bus: usize,
    pub to_item: String,
    pub to_bus: usize,
    pub per_min: f64,
}
