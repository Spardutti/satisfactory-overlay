<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
  import { api, OVERLAY_MODE, PLANS_CHANGED, progress, type PlanStore } from "$lib/api";

  let store = $state<PlanStore>({ active: null, plans: [] });
  let interactive = $state(true);

  const plan = $derived(store.plans.find((p) => p.id === store.active) ?? null);
  const done = $derived(plan ? progress(plan) : { built: 0, total: 0 });
  const pct = $derived(done.total ? (done.built / done.total) * 100 : 0);

  async function refresh() {
    store = await api.listPlans();
  }

  onMount(() => {
    refresh();
    api.overlayInteractive().then((on) => (interactive = on));
    const window = getCurrentWebviewWindow();
    const stops = [
      listen(PLANS_CHANGED, refresh),
      window.listen<boolean>(OVERLAY_MODE, (e) => (interactive = e.payload)),
    ];
    return () => stops.forEach((s) => s.then((stop) => stop()));
  });
</script>

<div class="hud" class:interactive>
  <header data-tauri-drag-region>
    <span data-tauri-drag-region>{plan?.name ?? "No active plan"}</span>
    <span class="tag" data-tauri-drag-region>{interactive ? "INTERACTIVE" : "PASSIVE"}</span>
  </header>
  {#if plan}
    <div class="bar"><div class="fill" style:width="{pct}%"></div></div>
    <div class="muted total">{done.built} / {done.total} machines</div>
    <ol>
      {#each plan.steps as step, i (step.label)}
        <li class:complete={step.built >= step.total}>
          <div class="text">
            <div>{step.label}</div>
            <div class="muted small">{step.detail}</div>
          </div>
          <button onclick={() => api.adjustStep(plan.id, i, -1)}>−</button>
          <span class="count">{step.built}/{step.total}</span>
          <button onclick={() => api.adjustStep(plan.id, i, 1)}>+</button>
        </li>
      {/each}
    </ol>
  {:else}
    <p class="muted">Save a plan in the planner window to see it here.</p>
  {/if}
</div>

<style>
  :global(html),
  :global(body) {
    background: transparent;
    overflow: hidden;
  }
  .hud {
    height: 100vh;
    overflow: auto;
    padding: 8px;
    background: rgba(29, 31, 34, 0.72);
    border: 2px solid transparent;
    border-radius: 6px;
    opacity: 0.8;
  }
  .hud.interactive {
    background: rgba(29, 31, 34, 0.88);
    border-color: var(--orange);
    opacity: 1;
  }
  header {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    color: var(--orange);
    font-weight: 600;
    cursor: move;
  }
  .tag {
    font-size: 10px;
    color: var(--muted);
  }
  .bar {
    height: 6px;
    margin: 6px 0 2px;
    background: var(--panel-2);
  }
  .fill {
    height: 100%;
    background: var(--orange);
  }
  .total,
  .small {
    font-size: 11px;
  }
  ol {
    list-style: none;
    margin: 6px 0 0;
    padding: 0;
  }
  li {
    display: flex;
    gap: 6px;
    align-items: center;
    padding: 4px 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }
  li.complete {
    opacity: 0.45;
  }
  .text {
    flex: 1;
    min-width: 0;
  }
  .count {
    min-width: 34px;
    text-align: center;
  }
  li button {
    padding: 0 8px;
  }
</style>
