/** Where the blueprint canvas sits: scale and offset of its top-left corner. */
export type View = { scale: number; x: number; y: number };
type Point = { x: number; y: number };
type Size = { width: number; height: number };

export const MIN_SCALE = 0.2;
export const MAX_SCALE = 4;
const FIT_MARGIN = 40;

function clampScale(scale: number): number {
  return Math.min(MAX_SCALE, Math.max(MIN_SCALE, scale));
}

/** Zooms by factor, keeping the canvas point under the cursor where it is. */
export function zoomAt(view: View, factor: number, cursor: Point): View {
  const scale = clampScale(view.scale * factor);
  const ratio = scale / view.scale;
  return { scale, x: cursor.x - (cursor.x - view.x) * ratio, y: cursor.y - (cursor.y - view.y) * ratio };
}

export function panBy(view: View, dx: number, dy: number): View {
  return { ...view, x: view.x + dx, y: view.y + dy };
}

type Box = { x: number; y: number; width: number; height: number };
/** Room for the buses that stick out on both sides of a block. */
const BUS_ROOM = 120;

/** The box around every block, wherever the user moved them. */
export function bounds(blocks: Box[]): Box {
  if (!blocks.length) return { x: 0, y: 0, width: 1, height: 1 };
  const left = Math.min(...blocks.map((b) => b.x)) - BUS_ROOM;
  const top = Math.min(...blocks.map((b) => b.y)) - BUS_ROOM;
  const right = Math.max(...blocks.map((b) => b.x + b.width)) + BUS_ROOM;
  const bottom = Math.max(...blocks.map((b) => b.y + b.height)) + BUS_ROOM;
  return { x: left, y: top, width: right - left, height: bottom - top };
}

/** The view that fits the box inside the screen, centered. */
export function fitView(box: Box, screen: Size): View {
  const room = { width: screen.width - 2 * FIT_MARGIN, height: screen.height - 2 * FIT_MARGIN };
  const scale = clampScale(Math.min(room.width / box.width, room.height / box.height));
  return {
    scale,
    x: (screen.width - box.width * scale) / 2 - box.x * scale,
    y: (screen.height - box.height * scale) / 2 - box.y * scale,
  };
}
