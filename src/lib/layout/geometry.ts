import type { Block, Metrics } from "$lib/api";

const MACHINE_BASE_HEIGHT = 40;
const ROW_PAD = 12;
const LANE_GAP = 30;
const LANE_SPACING = 18;
const LANE_LEAD = 20;

/** `ys` holds the splitter or merger height of every machine the lane serves. */
export type Lane = { x: number; top: number; bottom: number; ys: number[] };

/** Where one machine line sits in its block: input feeds on the left, output on the right. */
export function rowGeometry(block: Block, m: Metrics, row: number) {
  const top = m.header + row * block.rowHeight;
  const inputs = block.rows[row].inputs.length;
  const machineTop = top + ROW_PAD;
  const machineHeight = MACHINE_BASE_HEIGHT + Math.max(inputs - 1, 0) * m.inputStep;
  const machineLeft = (block.width - m.machineWidth) / 2;
  return {
    machineTop,
    machineHeight,
    machineLeft,
    machineRight: machineLeft + m.machineWidth,
    inputY: (k: number) => machineTop + (machineHeight * (k + 1)) / (inputs + 1),
    outputY: machineTop + machineHeight / 2,
  };
}

/** X of the lanes for a row's input k, left of the block; one item's lanes never overlap, so they share it. */
export function inputLaneX(k: number): number {
  return -LANE_GAP - k * LANE_SPACING;
}

/** X of the output lanes, right of the block. */
export function outputLaneX(block: Block): number {
  return block.width + LANE_GAP;
}

function span(ys: number[]) {
  return { top: Math.min(...ys) - LANE_LEAD, bottom: Math.max(...ys) + LANE_LEAD, ys };
}

/** Input lane j relative to its block: runs past its first and last machine. */
export function inputLane(block: Block, m: Metrics, j: number): Lane {
  const ys = block.inputBuses[j].taps.map((t) => rowGeometry(block, m, t.row).inputY(t.input));
  return { x: inputLaneX(block.inputBuses[j].taps[0].input), ...span(ys) };
}

/** Output lane j relative to its block. */
export function outputLane(block: Block, m: Metrics, j: number): Lane {
  const ys = block.outputBuses[j].taps.map((t) => rowGeometry(block, m, t.row).outputY);
  return { x: outputLaneX(block), ...span(ys) };
}

/** A lane moved onto the canvas. */
export function onCanvas(block: Block, lane: Lane): Lane {
  return { x: block.x + lane.x, top: block.y + lane.top, bottom: block.y + lane.bottom, ys: lane.ys.map((y) => block.y + y) };
}
