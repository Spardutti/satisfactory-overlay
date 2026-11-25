<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
  import { api, OVERLAY_MODE, PLANS_CHANGED, progress, type PlanStore } from "$lib/api";
  import StepCard from "$lib/StepCard.svelte";

  let store = $state<PlanStore>({ active: null, plans: [] });
  let interactive = $state(true);

  const plan = $derived(store.plans.find((p) => p.id === store.active) ?? null);
  const done = $derived(plan ? progress(plan) : { built: 0, total: 0 });
  const pct = $derived(done.total ? (done.built / done.total) * 100 : 0);
  const currentIndex = $derived(plan ? plan.steps.findIndex((s) => s.built < s.total) : -1);

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

  /** Clicking box n sets the count to n, or to n - 1 when box n was the last ticked one. */
  function tick(planId: string, step: number, box: number, built: number) {
    const target = built === box + 1 ? box : box + 1;
    api.adjustStep(planId, step, target - built);
  }
</script>

<div class="hud" class:interactive>
  <header data-tauri-drag-region>
    <span data-tauri-drag-region>{plan?.name ?? "No active plan"}</span>
    <span class="tag" data-tauri-drag-region>
      {plan ? `Belt ${plan.request.beltTier} · ` : ""}{interactive ? "INTERACTIVE" : "PASSIVE"}
    </span>
  </header>
  {#if plan}
    <div class="bar"><div class="fill" style:width="{pct}%"></div></div>
    <div class="muted total">{done.built} / {done.total} built</div>
    {#each plan.steps as step, i (i)}
      <StepCard card={step} number={i + 1} current={i === currentIndex}>
        {#snippet controls()}
          <span class="boxes">
            {#each { length: step.total }, box (box)}
              <button
                class="box"
                class:on={box < step.built}
                aria-label="Machine {box + 1}"
                onclick={() => tick(plan.id, i, box, step.built)}
              ></button>
            {/each}
          </span>
        {/snippet}
      </StepCard>
    {/each}
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
    opacity: 0.85;
  }
  .hud.interactive {
    background: rgba(29, 31, 34, 0.9);
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
  .total {
    font-size: 11px;
  }
  .boxes {
    display: flex;
    flex-wrap: wrap;
    gap: 3px;
  }
  .box {
    width: 14px;
    height: 14px;
    padding: 0;
    border: 1px solid var(--muted);
    border-radius: 2px;
    background: transparent;
  }
  .box.on {
    background: var(--orange);
    border-color: var(--orange);
  }
</style>
