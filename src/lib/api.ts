import { invoke } from "@tauri-apps/api/core";

export type Named = { id: string; name: string };
export type Target = Named & { standard: { buildingName: string; perMachine: number } | null };
export type DataSummary = { path: string; items: number; recipes: number; warnings: string[] };
export type Alternate = { id: string; name: string; makes: string; enabled: boolean };
export type ExtractorOptions = { tiers: string[]; purities: string[]; belts: string[]; pipes: string[] };
export type BeltGroup = { machines: number; perMin: number; over: boolean };
export type Flow = { item: string; itemName: string; perMin: number; fluid: boolean; belts: BeltGroup[] };
export type Usage = { itemName: string; perMin: number };

export type Mode = { kind: "rate"; rate: number } | { kind: "resources"; available: Record<string, number> };

export type PlanRequest = {
  target: string;
  mode: Mode;
  minerTier: string;
  purity: string;
  beltTier: string;
  pipeTier: string;
  choices: Record<string, string>;
};

export type Step = {
  item: string;
  itemName: string;
  recipeId: string;
  recipeName: string;
  buildingName: string;
  machinesExact: number;
  machines: number;
  lastMachinePct: number;
  outputPerMin: number;
  level: number;
  perMachine: number;
  inputs: Flow[];
  output: Flow;
  goesTo: Usage[];
};

export type RawNeed = {
  item: string;
  itemName: string;
  perMin: number;
  fluid: boolean;
  miners: number | null;
  belts: BeltGroup[];
  goesTo: Usage[];
};
export type Surplus = { item: string; itemName: string; perMin: number };
export type RawLeft = { item: string; itemName: string; available: number; used: number; left: number };
export type Limit = { maxRate: number; limiting: string; limitingName: string; resources: RawLeft[] };
export type RecipeOption = { id: string; name: string; alternate: boolean };

export type Calculation = {
  result: { target: string; targetName: string; rate: number; raw: RawNeed[]; steps: Step[]; surplus: Surplus[] };
  limit: Limit | null;
  options: Record<string, RecipeOption[]>;
  cards: ChecklistStep[];
};

export type ChecklistStep = {
  label: string;
  detail: string;
  total: number;
  built: number;
  inputs: string[];
  output: string;
  goesTo: string;
  item: string;
};
export type Plan = { id: string; name: string; request: PlanRequest; steps: ChecklistStep[] };
export type PlanStore = { active: string | null; plans: Plan[] };
export type HotkeyStatus = {
  keys: { toggleMode: string; toggleVisible: string; markNext: string };
  errors: string[];
};

export const PLANS_CHANGED = "plans-changed";
export const OVERLAY_MODE = "overlay-mode";

export const api = {
  loadGameData: () => invoke<DataSummary | null>("load_game_data"),
  setDocsPath: (path: string) => invoke<DataSummary>("set_docs_path", { path }),
  listTargets: () => invoke<Target[]>("list_targets"),
  listRawResources: () => invoke<Named[]>("list_raw_resources"),
  listAlternates: () => invoke<Alternate[]>("list_alternates"),
  setAlternate: (id: string, enabled: boolean) => invoke<void>("set_alternate", { id, enabled }),
  extractorOptions: () => invoke<ExtractorOptions>("extractor_options"),
  calculate: (request: PlanRequest) => invoke<Calculation>("calculate", { request }),
  listPlans: () => invoke<PlanStore>("list_plans"),
  savePlan: (draft: { id: string | null; name: string; request: PlanRequest }) =>
    invoke<Plan>("save_plan", { draft }),
  deletePlan: (id: string) => invoke<void>("delete_plan", { id }),
  setActivePlan: (id: string) => invoke<void>("set_active_plan", { id }),
  adjustStep: (planId: string, step: number, delta: number) =>
    invoke<void>("adjust_step", { change: { planId, step, delta } }),
  markNextStep: () => invoke<void>("mark_next_step"),
  overlayInteractive: () => invoke<boolean>("overlay_interactive"),
  hotkeyStatus: () => invoke<HotkeyStatus>("hotkey_status"),
};

export function progress(plan: Plan): { built: number; total: number } {
  return plan.steps.reduce(
    (sum, s) => ({ built: sum.built + s.built, total: sum.total + s.total }),
    { built: 0, total: 0 },
  );
}

export function fmt(value: number): string {
  return Number(value.toFixed(2)).toString();
}
