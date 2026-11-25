<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { api, PLANS_CHANGED, type Calculation, type Layout, type NodeCounts, type PlanStore } from "$lib/api";
  import NodesPanel from "$lib/planner/NodesPanel.svelte";
  import Blueprint from "./Blueprint.svelte";

  let { planId = $bindable() }: { planId: string | null } = $props();

  let store = $state<PlanStore>({ active: null, plans: [] });
  let layout = $state<Layout | null>(null);
  let error = $state("");
  let showNodes = $state(false);
  let calc = $state<Calculation | null>(null);
  const plan = $derived(store.plans.find((p) => p.id === planId) ?? null);

  onMount(() => {
    refresh().then(load);
    const unlisten = listen(PLANS_CHANGED, onPlansChanged);
    return () => unlisten.then((stop) => stop());
  });

  async function refresh() {
    store = await api.listPlans();
    if (!store.plans.some((p) => p.id === planId)) planId = store.active ?? store.plans[0]?.id ?? null;
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
    if (!planId) return;
    try {
      layout = await api.planLayout(planId);
      if (plan) calc = await api.calculate(plan.request);
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  function saveMove(item: string, x: number, y: number) {
    if (planId) api.moveBlock({ planId, item, x, y }).catch((e) => (error = String(e)));
  }

  async function setNodes(item: string, nodes: NodeCounts) {
    if (!planId) return;
    try {
      await api.setNodes({ planId, item, nodes });
      await refresh();
      await load();
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
    <button onclick={resetLayout} disabled={!planId}>Reset layout</button>
    <button onclick={() => (showNodes = !showNodes)} disabled={!calc} class:primary={showNodes}>Nodes</button>
    <span class="muted">Scroll to zoom · drag the background to move · drag a block to place it</span>
    {#if error}<span class="error">{error}</span>{/if}
  </div>
  {#if showNodes && calc && plan}
    <div class="pad"><NodesPanel {calc} nodes={plan.request.nodes} onChange={setNodes} /></div>
  {/if}
  {#if layout}
    {#key layout}
      <Blueprint bind:layout onMoved={saveMove} />
    {/key}
  {:else if !store.plans.length}
    <p class="muted pad">Save a plan first, then open its blueprint here.</p>
  {/if}
</div>

<style>
  .tab {
    display: flex;
    flex-direction: column;
    height: 100%;
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
