<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Layout, type PlanStore } from "$lib/api";
  import Blueprint from "./Blueprint.svelte";

  let { planId = $bindable() }: { planId: string | null } = $props();

  let store = $state<PlanStore>({ active: null, plans: [] });
  let layout = $state<Layout | null>(null);
  let error = $state("");

  onMount(async () => {
    store = await api.listPlans();
    planId ??= store.active ?? store.plans[0]?.id ?? null;
    await load();
  });

  async function load() {
    layout = null;
    if (!planId) return;
    try {
      layout = await api.planLayout(planId);
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  function saveMove(item: string, x: number, y: number) {
    if (planId) api.moveBlock(planId, item, x, y).catch((e) => (error = String(e)));
  }

  async function resetLayout() {
    if (!planId) return;
    await api.resetLayout(planId);
    await load();
  }
</script>

<div class="tab">
  <div class="bar">
    <select bind:value={planId} onchange={load}>
      {#each store.plans as plan (plan.id)}<option value={plan.id}>{plan.name}</option>{/each}
    </select>
    <button onclick={resetLayout} disabled={!planId}>Reset layout</button>
    <span class="muted">Scroll to zoom · drag the background to move · drag a block to place it</span>
    {#if error}<span class="error">{error}</span>{/if}
  </div>
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
