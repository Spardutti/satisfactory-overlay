<script lang="ts">
  import type { Plan } from "$lib/api";
  import Alternates from "$lib/planner/Alternates.svelte";
  import DataStatus from "$lib/planner/DataStatus.svelte";
  import PlansList from "$lib/planner/PlansList.svelte";
  import PlanTab from "$lib/planner/PlanTab.svelte";
  import BlueprintTab from "$lib/layout/BlueprintTab.svelte";

  type Tab = "blueprints" | "alternates";
  const tabs: [Tab, string][] = [
    ["blueprints", "Blueprints"],
    ["alternates", "My alternates"],
  ];
  type Screen = "list" | "form" | "blueprint";

  let loaded = $state(false);
  let tab = $state<Tab>("blueprints");
  let screen = $state<Screen>("list");
  let editing = $state<Plan | null>(null);
  let blueprintPlan = $state<string | null>(null);

  function pick(id: Tab) {
    tab = id;
    screen = "list";
  }

  function edit(plan: Plan) {
    editing = plan;
    screen = "form";
  }

  function newPlan() {
    editing = null;
    screen = "form";
  }

  function openBlueprint(plan: Plan) {
    blueprintPlan = plan.id;
    screen = "blueprint";
  }
</script>

<div class="app">
  <DataStatus onLoaded={() => (loaded = true)} />
  {#if loaded}
    <nav>
      {#each tabs as [id, label] (id)}
        <button class:on={tab === id} onclick={() => pick(id)}>{label}</button>
      {/each}
      {#if tab === "blueprints" && screen === "form"}
        <span class="muted">{editing ? `Editing “${editing.name}”` : "New blueprint"}</span>
      {/if}
    </nav>
    <main>
      {#if tab === "alternates"}
        <Alternates />
      {:else if screen === "form"}
        {#key editing?.id}
          <PlanTab {editing} onSaved={openBlueprint} />
        {/key}
      {:else if screen === "blueprint"}
        <BlueprintTab bind:planId={blueprintPlan} onEdit={edit} />
      {:else}
        <PlansList onNew={newPlan} onEdit={edit} onOpen={openBlueprint} />
      {/if}
    </main>
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
</style>
