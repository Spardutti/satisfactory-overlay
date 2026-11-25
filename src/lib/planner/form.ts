import type { Named, NodeCounts, Plan, PlanRequest } from "$lib/api";

export type ResourceRow = { item: string; amount: number };

export type FormState = {
  targetName: string;
  mode: "rate" | "resources";
  rate: number;
  resources: ResourceRow[];
  minerTier: string;
  beltTier: string;
  pipeTier: string;
  choices: Record<string, string>;
  nodes: Record<string, NodeCounts>;
};

export function emptyForm(): FormState {
  return {
    targetName: "",
    mode: "rate",
    rate: 60,
    resources: [{ item: "Desc_OreIron_C", amount: 60 }],
    minerTier: "Mk1",
    beltTier: "Mk1",
    pipeTier: "Mk1",
    choices: {},
    nodes: {},
  };
}

/** What a new plan is called unless the player names it, e.g. "Nuclear Pasta · Miner Mk2 · Belt Mk5 · Pipe Mk2". */
export function defaultName(form: FormState): string {
  const target = form.targetName.trim();
  if (!target) return "Plan name";
  return `${target} · Miner ${form.minerTier} · Belt ${form.beltTier} · Pipe ${form.pipeTier}`;
}

/** The request for the form, or an error message when the target is not a known item. */
export function toRequest(form: FormState, targets: Named[]): PlanRequest | string {
  const target = targets.find((t) => t.name.toLowerCase() === form.targetName.trim().toLowerCase());
  if (!target) return "Pick a target item from the list.";
  if (form.mode === "rate" && !isAmount(form.rate)) return "Enter a rate of 0 or more.";
  if (form.mode === "resources" && !form.resources.every((r) => isAmount(r.amount)))
    return "Enter an amount of 0 or more for every resource.";
  const mode =
    form.mode === "rate"
      ? { kind: "rate" as const, rate: form.rate }
      : { kind: "resources" as const, available: totalPerItem(form.resources) };
  const { minerTier, beltTier, pipeTier, choices, nodes } = form;
  return { target: target.id, mode, minerTier, beltTier, pipeTier, choices, nodes };
}

function isAmount(value: number | null): boolean {
  return typeof value === "number" && Number.isFinite(value) && value >= 0;
}

/** Two rows of the same resource (two nodes) add up. */
function totalPerItem(rows: ResourceRow[]): Record<string, number> {
  const totals: Record<string, number> = {};
  for (const row of rows) totals[row.item] = (totals[row.item] ?? 0) + row.amount;
  return totals;
}

export function fromPlan(plan: Plan, targets: Named[]): FormState {
  const { request } = plan;
  const targetName = targets.find((t) => t.id === request.target)?.name ?? request.target;
  const { minerTier, beltTier, pipeTier } = request;
  const choices = { ...request.choices };
  const base = { ...emptyForm(), targetName, minerTier, beltTier, pipeTier, choices, nodes: { ...request.nodes } };
  if (request.mode.kind === "rate") return { ...base, mode: "rate", rate: request.mode.rate };
  const resources = Object.entries(request.mode.available).map(([item, amount]) => ({ item, amount }));
  return { ...base, mode: "resources", resources };
}
