<script lang="ts">
  import { onMount } from "svelte";
  import { api, type HotkeyStatus, type Plan } from "$lib/api";
  import Alternates from "$lib/planner/Alternates.svelte";
  import DataStatus from "$lib/planner/DataStatus.svelte";
  import PlansList from "$lib/planner/PlansList.svelte";
  import PlanTab from "$lib/planner/PlanTab.svelte";
  import BlueprintTab from "$lib/layout/BlueprintTab.svelte";

  type Tab = "plan" | "alternates" | "plans" | "blueprint";
  const tabs: [Tab, string][] = [
    ["plan", "Plan"],
    ["alternates", "My alternates"],
    ["plans", "Saved plans"],
    ["blueprint", "Blueprint"],
  ];

  let loaded = $state(false);
  let tab = $state<Tab>("plan");
  let editing = $state<Plan | null>(null);
  let hotkeys = $state<HotkeyStatus | null>(null);
  let blueprintPlan = $state<string | null>(null);

  onMount(async () => {
    hotkeys = await api.hotkeyStatus();
  });

  function edit(plan: Plan) {
    editing = plan;
    tab = "plan";
  }

  function openBlueprint(plan: Plan) {
    blueprintPlan = plan.id;
    tab = "blueprint";
  }

  function newPlan() {
    editing = null;
    tab = "plan";
  }
</script>

<div class="app">
  <DataStatus onLoaded={() => (loaded = true)} />
  {#if loaded}
    <nav>
      {#each tabs as [id, label] (id)}
        <button class:on={tab === id} onclick={() => (id === "plan" ? newPlan() : (tab = id))}>{label}</button>
      {/each}
      {#if editing && tab === "plan"}<span class="muted">Editing “{editing.name}”</span>{/if}
    </nav>
    <main>
      {#if tab === "plan"}
        {#key editing?.id}
          <PlanTab {editing} />
        {/key}
      {:else if tab === "alternates"}
        <Alternates />
      {:else if tab === "blueprint"}
        <BlueprintTab bind:planId={blueprintPlan} />
      {:else}
        <PlansList onEdit={edit} onBlueprint={openBlueprint} />
      {/if}
    </main>
  {/if}
  {#if hotkeys}
    <footer class="muted">
      Overlay: {hotkeys.keys.toggleMode} click-through on/off · {hotkeys.keys.toggleVisible} show/hide ·
      {hotkeys.keys.markNext} mark next machine
      {#each hotkeys.errors as e (e)}<span class="error"> · {e}</span>{/each}
    </footer>
  {/if}
</div>

<style>
  :global(body) {
    background: var(--bg);
  }
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  nav {
    display: flex;
    gap: 4px;
    align-items: center;
    padding: 8px 12px 0;
    border-bottom: 1px solid var(--line);
  }
  nav button {
    border-bottom: none;
    border-radius: 3px 3px 0 0;
  }
  nav button.on {
    color: var(--orange);
    border-color: var(--orange);
  }
  main {
    flex: 1;
    min-height: 0;
    overflow: auto;
  }
  footer {
    padding: 6px 12px;
    border-top: 1px solid var(--line);
    font-size: 12px;
  }
</style>
