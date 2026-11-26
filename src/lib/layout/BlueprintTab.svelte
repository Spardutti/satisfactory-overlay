<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { api, PLANS_CHANGED, type Calculation, type Layout, type NodeCounts, type Plan, type PlanStore } from "$lib/api";
  import NodesPanel from "$lib/planner/NodesPanel.svelte";
  import Blueprint from "./Blueprint.svelte";

  let { planId = $bindable(), onEdit }: { planId: string | null; onEdit: (plan: Plan) => void } = $props();

  let store = $state<PlanStore>({ plans: [] });
  let layout = $state<Layout | null>(null);
  let error = $state("");
  let showNodes = $state(false);
  let calc = $state<Calculation | null>(null);
  const plan = $derived(store.plans.find((p) => p.id === planId) ?? null);
  const sources = $derived(calc?.fillable ?? []);

  onMount(() => {
    refresh().then(load);
    const unlisten = listen(PLANS_CHANGED, onPlansChanged);
    return () => unlisten.then((stop) => stop());
  });

  async function refresh() {
    store = await api.listPlans();
    if (!store.plans.some((p) => p.id === planId)) planId = store.plans[0]?.id ?? null;
  }

  /** Reloads the blueprint only when its plan is gone; a drag here also fires this event. */
  async function onPlansChanged() {
    const before = planId;
    await refresh();
    if (planId !== before) await load();
  }

  async function load() {
    layout = null;
    calc = null;
    await fetchPlan();
  }

  /** Swaps in fresh data without clearing first, so the view and the open panel stay put. */
  async function fetchPlan() {
    if (!planId) return;
    try {
      layout = await api.planLayout(planId);
      if (plan) calc = await api.calculate(plan.request);
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  function toggleBuilt(item: string, row: number) {
    if (planId) api.toggleBuilt({ planId, item, row }).catch((e) => (error = String(e)));
  }

  function clearBuilt() {
    if (planId && confirm("Untick every machine in this plan?")) api.clearBuilt(planId).catch((e) => (error = String(e)));
  }

  function saveMove(item: string, x: number, y: number) {
    if (planId) api.moveBlock({ planId, item, x, y }).catch((e) => (error = String(e)));
  }

  async function setNodes(item: string, nodes: NodeCounts) {
    if (!planId) return;
    try {
      await api.setNodes({ planId, item, nodes });
      await refresh();
      await fetchPlan();
    } catch (e) {
      error = String(e);
    }
  }

  async function fill(item: string) {
    if (!planId) return;
    try {
      await api.fillMiners({ planId, item });
      await refresh();
      await fetchPlan();
    } catch (e) {
      error = String(e);
    }
  }

  async function resetLayout() {
    if (!planId) return;
    try {
      await api.resetLayout(planId);
      await load();
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="tab">
  <div class="bar">
    <select bind:value={planId} onchange={load}>
      {#each store.plans as plan (plan.id)}<option value={plan.id}>{plan.name}</option>{/each}
    </select>
    {#if plan}
      <span class="tiers">
        <b>Miner {plan.request.minerTier}</b> · <b>Belt {plan.request.beltTier}</b> · <b>Pipe {plan.request.pipeTier}</b>
      </span>
    {/if}
    <button onclick={() => plan && onEdit(plan)} disabled={!plan}>Edit numbers</button>
    <button onclick={resetLayout} disabled={!planId}>Reset layout</button>
    <button onclick={clearBuilt} disabled={!planId}>Untick all</button>
    <button onclick={() => (showNodes = !showNodes)} disabled={!calc} class:primary={showNodes}>Nodes</button>
    <span class="muted">Scroll to zoom · drag to move · click a machine to tick it built</span>
    {#if error}<span class="error">{error}</span>{/if}
  </div>
  {#if showNodes && calc && plan}
    <div class="pad"><NodesPanel {calc} nodes={plan.request.nodes} onChange={setNodes} /></div>
  {/if}
  {#if layout}
    {#key planId}
      <Blueprint bind:layout built={plan?.built ?? {}} {sources} onMoved={saveMove} onToggle={toggleBuilt} onFill={fill} />
    {/key}
  {:else if !store.plans.length}
    <p class="muted pad">No blueprints yet.</p>
  {/if}
</div>

<style>
  .tab {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .tiers b {
    color: var(--orange);
    font-weight: 600;
  }
  .bar {
    display: flex;
    gap: 12px;
    align-items: center;
    padding: 8px 12px;
  }
  .pad {
    padding: 12px;
  }
</style>
