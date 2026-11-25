import { invoke } from "@tauri-apps/api/core";

export type Named = { id: string; name: string };
export type Target = Named & { standard: { buildingName: string; perMachine: number } | null };
export type DataSummary = { path: string; items: number; recipes: number; warnings: string[] };
export type Alternate = { id: string; name: string; makes: string; enabled: boolean };
export type ExtractorOptions = { tiers: string[]; belts: string[]; pipes: string[] };
export type BeltGroup = { machines: number; perMin: number; over: boolean };
export type Flow = { item: string; itemName: string; perMin: number; fluid: boolean; belts: BeltGroup[] };
export type Usage = { itemName: string; perMin: number };

export type Mode = { kind: "rate"; rate: number } | { kind: "resources"; available: Record<string, number> };

export type PlanRequest = {
  target: string;
  mode: Mode;
  minerTier: string;
  beltTier: string;
  pipeTier: string;
  choices: Record<string, string>;
  nodes: Record<string, NodeCounts>;
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
  nodes: NodeSupply | null;
};
export type NodeCounts = { pure: number; normal: number; impure: number };
export type NodeDraw = { purity: string; perMin: number; clock: number };
export type NodeSupply = { draws: NodeDraw[]; spare: NodeCounts; short: number };
export type Surplus = { item: string; itemName: string; perMin: number };
export type RawLeft = { item: string; itemName: string; available: number; used: number; left: number };
export type Limit = { maxRate: number; limiting: string; limitingName: string; resources: RawLeft[] };
export type RecipeOption = { id: string; name: string; alternate: boolean };

export type Calculation = {
  result: { target: string; targetName: string; rate: number; raw: RawNeed[]; steps: Step[]; surplus: Surplus[] };
  limit: Limit | null;
  /** the most target per minute the listed nodes allow, when an ore runs short */
  nodeLimit: number | null;
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
export type Plan = {
  id: string;
  name: string;
  request: PlanRequest;
  steps: ChecklistStep[];
  /** machines ticked as built: item → machine rows */
  built: Record<string, number[]>;
};
export type PlanStore = { plans: Plan[] };

export type BeltLine = { item: string; itemName: string; perMin: number; fluid: boolean; over: boolean };
export type LayoutRow = { inputs: BeltLine[]; machines: number[]; output: BeltLine; node: string | null };
export type Block = {
  item: string;
  title: string;
  buildingName: string;
  x: number;
  y: number;
  width: number;
  height: number;
  rowHeight: number;
  rows: LayoutRow[];
  inputBuses: Bus[];
  outputBuses: Bus[];
};
export type Tap = { row: number; input: number; perMin: number };
export type Bus = { item: string; itemName: string; fluid: boolean; perMin: number; taps: Tap[] };
export type Connection = { item: string; fromBus: number; toItem: string; toBus: number; perMin: number };
export type Metrics = {
  machineWidth: number;
  machineGap: number;
  rowHeight: number;
  inputStep: number;
  header: number;
  padding: number;
};
export type Layout = { width: number; height: number; blocks: Block[]; connections: Connection[]; metrics: Metrics };

export const PLANS_CHANGED = "plans-changed";

export const api = {
  loadGameData: () => invoke<DataSummary | null>("load_game_data"),
  setDocsPath: (path: string) => invoke<DataSummary>("set_docs_path", { path }),
  listTargets: () => invoke<Target[]>("list_targets"),
  listRawResources: () => invoke<Named[]>("list_raw_resources"),
  listAlternates: () => invoke<Alternate[]>("list_alternates"),
  setAlternate: (id: string, enabled: boolean) => invoke<void>("set_alternate", { id, enabled }),
  extractorOptions: () => invoke<ExtractorOptions>("extractor_options"),
  calculate: (request: PlanRequest) => invoke<Calculation>("calculate", { request }),
  planLayout: (planId: string) => invoke<Layout>("plan_layout", { planId }),
  moveBlock: (change: { planId: string; item: string; x: number; y: number }) => invoke<void>("move_block", { change }),
  resetLayout: (planId: string) => invoke<void>("reset_layout", { planId }),
  setNodes: (change: { planId: string; item: string; nodes: NodeCounts }) => invoke<void>("set_nodes", { change }),
  listPlans: () => invoke<PlanStore>("list_plans"),
  savePlan: (draft: { id: string | null; name: string; request: PlanRequest }) =>
    invoke<Plan>("save_plan", { draft }),
  deletePlan: (id: string) => invoke<void>("delete_plan", { id }),
  toggleBuilt: (change: { planId: string; item: string; row: number }) => invoke<void>("toggle_built", { change }),
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
