import type { Connection } from "$lib/api";
import type { Lane } from "./geometry";

export type Point = { x: number; y: number };
/** `approachY` differs from the end's y only when the belt has to come in through the lane's end. */
export type Link = {
  c: Connection;
  fluid: boolean;
  out: Lane;
  into: Lane;
  start: Point[];
  end: Point[];
  jogX?: number;
  approachY?: number;
};
/** A straight run of belt: `at` is its x when vertical, its y when horizontal. */
type Run = { at: number; from: number; to: number };
type Taken = { v: Run[]; h: Run[] };

export const SPACING = 18;
const TRIES = 60;

export const first = (link: Link) => link.end[0];
export const last = (link: Link) => link.start[link.start.length - 1];
const turns = (l: Link) => Math.abs(last(l).y - first(l).y) > 0.5;
const reach = (group: Link[]) => first(group[0]).x - last(group[0]).x;
const run = (at: number, a: number, b: number): Run => ({ at, from: Math.min(a, b), to: Math.max(a, b) });
const offset = (step: number) => (step % 2 ? -1 : 1) * Math.ceil(step / 2) * SPACING;

export function groupBy<T>(items: T[], key: (item: T) => string): T[][] {
  const groups = new Map<string, T[]>();
  for (const item of items) groups.set(key(item), [...(groups.get(key(item)) ?? []), item]);
  return [...groups.values()];
}

/** Picks where every belt turns so no two belts ever lie on top of each other. */
export function placeTurns(links: Link[]) {
  const taken = fixedRuns(links);
  const pairs = groupBy(links.filter(turns), (l) => `${l.c.item}|${l.c.toItem}`);
  pairs.sort((a, b) => reach(a) - reach(b));
  for (const group of pairs) {
    orderColumns(group);
    if (shiftTogether(group, taken)) group.forEach((l) => add(taken, runsOf(l, l.jogX!, first(l).y)));
    else group.forEach((l) => placeAlone(l, taken));
  }
}

function fixedRuns(links: Link[]): Taken {
  const lanes = new Map(links.flatMap((l) => [l.out, l.into]).map((lane) => [`${lane.x}|${lane.top}`, lane]));
  const multi = [...lanes.values()].filter((lane) => lane.ys.length > 1);
  const forks = links.flatMap((l) => [l.start, l.end]).filter((p) => p.length > 1);
  return {
    v: [...multi.map((lane) => run(lane.x, lane.top, lane.bottom)), ...forks.map(([p, q]) => run(p.x, p.y, q.y))],
    h: links.filter((l) => !turns(l)).map((l) => run(last(l).y, last(l).x, first(l).x)),
  };
}

/** Belts going down turn lowest-first and belts going up turn highest-first, so they never cross. */
function orderColumns(group: Link[]) {
  const center = (Math.max(...group.map((l) => last(l).x)) + Math.min(...group.map((l) => first(l).x))) / 2;
  const down = group.filter((l) => first(l).y > last(l).y).sort((a, b) => last(b).y - last(a).y);
  const up = group.filter((l) => first(l).y < last(l).y).sort((a, b) => last(a).y - last(b).y);
  for (const column of [down, up]) {
    column.forEach((link, k) => (link.jogX = center + (k - (column.length - 1) / 2) * SPACING));
  }
}

function shiftTogether(group: Link[], taken: Taken): boolean {
  for (let step = 0; step < TRIES; step++) {
    const dx = offset(step);
    if (!group.every((l) => fits(l, { x: l.jogX! + dx, approachY: first(l).y }, taken))) continue;
    for (const l of group) l.jogX! += dx;
    return true;
  }
  return false;
}

/** Heights past the lane's ends a belt can come in at, nearer end first. */
function approaches(l: Link): number[] {
  if (l.end.length > 1) return [];
  const steps = [...Array(10).keys()].map((k) => k * SPACING);
  const bottom = steps.map((d) => l.into.bottom + d);
  const top = steps.map((d) => l.into.top - d);
  return last(l).y > first(l).y ? [...bottom, ...top] : [...top, ...bottom];
}

/** One turn if there is room; otherwise a second turn that enters the lane through one of its ends. */
function placeAlone(l: Link, taken: Taken) {
  for (const approachY of [first(l).y, ...approaches(l)]) {
    for (let step = 0; step < TRIES; step++) {
      const x = l.jogX! + offset(step);
      if (!fits(l, { x, approachY }, taken)) continue;
      [l.jogX, l.approachY] = [x, approachY];
      break;
    }
    if (l.approachY !== undefined) break;
  }
  add(taken, runsOf(l, l.jogX!, l.approachY ?? first(l).y));
}

function runsOf(l: Link, x: number, approachY: number): Taken {
  const [a, b] = [last(l), first(l)];
  const edge = approachY > b.y ? l.into.bottom : l.into.top;
  const v = [run(x, a.y, approachY), ...(approachY === b.y ? [] : [run(b.x, approachY, edge)])];
  return { v, h: [run(a.y, a.x, x), run(approachY, x, b.x)] };
}

const clash = (runs: Run[], taken: Run[]) =>
  runs.some((r) => taken.some((t) => Math.abs(t.at - r.at) < SPACING && t.from < r.to && r.from < t.to));

type Turn = { x: number; approachY: number };

function fits(l: Link, { x, approachY }: Turn, taken: Taken): boolean {
  const runs = runsOf(l, x, approachY);
  const between = x > last(l).x + 4 && x < first(l).x - 4;
  return between && !clash(runs.v, taken.v) && !clash(runs.h, taken.h);
}

function add(taken: Taken, runs: Taken) {
  taken.v.push(...runs.v);
  taken.h.push(...runs.h);
}
