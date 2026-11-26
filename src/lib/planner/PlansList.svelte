<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { api, PLANS_CHANGED, progress, type Plan, type PlanStore } from "$lib/api";

  let { onNew, onEdit, onOpen }: { onNew: () => void; onEdit: (plan: Plan) => void; onOpen: (plan: Plan) => void } =
    $props();

  let store = $state<PlanStore>({ plans: [] });

  async function refresh() {
    store = await api.listPlans();
  }

  onMount(() => {
    refresh();
    const unlisten = listen(PLANS_CHANGED, refresh);
    return () => unlisten.then((stop) => stop());
  });

  function remove(plan: Plan) {
    if (confirm(`Delete “${plan.name}”?`)) api.deletePlan(plan.id);
  }
</script>

<div class="plans">
  <button class="primary" onclick={onNew}>New blueprint</button>
  {#if !store.plans.length}
    <p class="muted">No blueprints yet. Press “New blueprint” to make one.</p>
  {/if}
  {#each store.plans as plan (plan.id)}
    {@const p = progress(plan)}
    <div class="plan">
      <span>{plan.name}</span>
      <span class="muted">{p.built} / {p.total} machines built</span>
      <button onclick={() => onOpen(plan)}>Open</button>
      <button onclick={() => onEdit(plan)}>Edit numbers</button>
      <button onclick={() => remove(plan)}>Delete</button>
    </div>
  {/each}
</div>

<style>
  .plans {
    padding: 12px;
  }
  .plans > .primary {
    margin-bottom: 8px;
  }
  .plan {
    display: flex;
    gap: 12px;
    align-items: center;
    padding: 6px 8px;
    border-bottom: 1px solid var(--line);
  }
  .plan > span:first-child {
    flex: 1;
  }
</style>
