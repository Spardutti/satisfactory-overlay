import type { Block, Layout } from "$lib/api";
import { inputLane, onCanvas, outputLane, type Lane } from "./geometry";
import { first, groupBy, last, placeTurns, SPACING, type Link, type Point } from "./tracks";

export type Segment = { d: string; item: string; fluid: boolean };
export type Branch = Segment & { label: Point; perMin: number };
export type Node = Point & { kind: "split" | "merge" };
export type Routes = { trunks: Segment[]; branches: Branch[]; nodes: Node[] };

/** How far a splitter or merger sits from its lane. */
const LEAD = 22;

/** A belt joins a lane at the splitter of its middle machine, so it lines up with a machine input. */
const middle = (lane: Lane) => lane.ys[Math.floor((lane.ys.length - 1) / 2)];

/** Straight belts between lanes, with right-angle turns placed by `placeTurns`. */
export function buildRoutes(layout: Layout): Routes {
  const routes: Routes = { trunks: [], branches: [], nodes: [] };
  const links = toLinks(layout);
  for (const group of groupBy(links, (l) => `${l.c.item}|${l.c.fromBus}`)) splitFrom(group, routes);
  for (const group of groupBy(links, (l) => `${l.c.toItem}|${l.c.toBus}`)) mergeInto(group, routes);
  placeTurns(links);
  for (const link of links) routes.branches.push(branch(link));
  return routes;
}

function toLinks(layout: Layout): Link[] {
  const byItem = new Map<string, Block>(layout.blocks.map((b) => [b.item, b]));
  return layout.connections.flatMap((c) => {
    const from = byItem.get(c.item);
    const to = byItem.get(c.toItem);
    if (!from || !to) return [];
    const out = onCanvas(from, outputLane(from, layout.metrics, c.fromBus));
    const into = onCanvas(to, inputLane(to, layout.metrics, c.toBus));
    return [{ c, out, into, fluid: from.outputBuses[c.fromBus].fluid, start: [], end: [] }];
  });
}

/** Offsets from a splitter or merger: the topmost belt goes forward, the rest step down past it. */
function side(index: number, count: number): number {
  return (index - Math.floor((count - 1) / 2)) * SPACING;
}

function fork(at: Point, dy: number): Point[] {
  return dy ? [at, { x: at.x, y: at.y + dy }] : [at];
}

function splitFrom(group: Link[], routes: Routes) {
  const { out, c, fluid } = group[0];
  const lanePoint = { x: out.x, y: middle(out) };
  if (group.length === 1) {
    group[0].start = [lanePoint];
    return;
  }
  const at = { x: out.x + LEAD, y: lanePoint.y };
  addNode(routes, { d: polyline([lanePoint, at]), item: c.item, fluid }, { ...at, kind: "split" });
  group.sort((a, b) => middle(a.into) - middle(b.into));
  group.forEach((link, i) => (link.start = fork(at, side(i, group.length))));
}

function mergeInto(group: Link[], routes: Routes) {
  const { into, c, fluid } = group[0];
  const lanePoint = { x: into.x, y: middle(into) };
  if (group.length === 1) {
    group[0].end = [lanePoint];
    return;
  }
  const at = { x: into.x - LEAD, y: lanePoint.y };
  addNode(routes, { d: polyline([at, lanePoint]), item: c.item, fluid }, { ...at, kind: "merge" });
  group.sort((a, b) => middle(a.out) - middle(b.out));
  group.forEach((link, i) => (link.end = fork(at, side(i, group.length)).reverse()));
}

function branch(link: Link): Branch {
  const a = last(link);
  const b = first(link);
  const approachY = link.approachY ?? b.y;
  const bend = approachY === b.y ? [] : [{ x: b.x, y: approachY }];
  const turn = link.jogX === undefined ? [] : [{ x: link.jogX, y: a.y }, { x: link.jogX, y: approachY }, ...bend];
  const points = [...link.start, ...turn, ...link.end];
  const label = { x: (a.x + (link.jogX ?? b.x)) / 2, y: a.y - 10 };
  return { d: polyline(points), item: link.c.item, fluid: link.fluid, label, perMin: link.c.perMin };
}

function polyline(points: Point[]): string {
  return points.map((p, i) => `${i ? "L" : "M"} ${p.x} ${p.y}`).join(" ");
}

function addNode(routes: Routes, trunk: Segment, node: Node) {
  routes.trunks.push(trunk);
  routes.nodes.push(node);
}
