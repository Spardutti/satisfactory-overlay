import type { Named, Plan, PlanRequest } from "$lib/api";

export type ResourceRow = { item: string; amount: number };

export type FormState = {
  targetName: string;
  mode: "rate" | "resources";
  rate: number;
  resources: ResourceRow[];
  minerTier: string;
  purity: string;
  choices: Record<string, string>;
};

export function emptyForm(): FormState {
  return {
    targetName: "",
    mode: "rate",
    rate: 60,
    resources: [{ item: "Desc_OreIron_C", amount: 60 }],
    minerTier: "Mk1",
    purity: "normal",
    choices: {},
  };
}

/** The request for the form, or an error message when the target is not a known item. */
export function toRequest(form: FormState, targets: Named[]): PlanRequest | string {
  const target = targets.find((t) => t.name.toLowerCase() === form.targetName.trim().toLowerCase());
  if (!target) return "Pick a target item from the list.";
  const mode =
    form.mode === "rate"
      ? { kind: "rate" as const, rate: form.rate }
      : { kind: "resources" as const, available: Object.fromEntries(form.resources.map((r) => [r.item, r.amount])) };
  return { target: target.id, mode, minerTier: form.minerTier, purity: form.purity, choices: form.choices };
}

export function fromPlan(plan: Plan, targets: Named[]): FormState {
  const { request } = plan;
  const targetName = targets.find((t) => t.id === request.target)?.name ?? request.target;
  const base = { ...emptyForm(), targetName, minerTier: request.minerTier, purity: request.purity, choices: { ...request.choices } };
  if (request.mode.kind === "rate") return { ...base, mode: "rate", rate: request.mode.rate };
  const resources = Object.entries(request.mode.available).map(([item, amount]) => ({ item, amount }));
  return { ...base, mode: "resources", resources };
}
