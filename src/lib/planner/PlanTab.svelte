<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Calculation, type ExtractorOptions, type Named, type Plan } from "$lib/api";
  import { emptyForm, fromPlan, toRequest } from "./form";
  import PlanForm from "./PlanForm.svelte";
  import ResultView from "./ResultView.svelte";

  let { editing }: { editing: Plan | null } = $props();

  let targets = $state<Named[]>([]);
  let raws = $state<Named[]>([]);
  let extractors = $state<ExtractorOptions>({ tiers: [], purities: [] });
  let form = $state(emptyForm());
  let calc = $state<Calculation | null>(null);
  let error = $state("");
  let planName = $state("");
  let savedNote = $state("");
  let planId = $state<string | null>(null);

  onMount(async () => {
    [targets, raws, extractors] = await Promise.all([api.listTargets(), api.listRawResources(), api.extractorOptions()]);
    if (editing) {
      planId = editing.id;
      form = fromPlan(editing, targets);
      planName = editing.name;
      await calculate();
    }
  });

  async function calculate() {
    savedNote = "";
    const request = toRequest(form, targets);
    if (typeof request === "string") {
      error = request;
      return;
    }
    try {
      calc = await api.calculate(request);
      error = "";
    } catch (e) {
      calc = null;
      error = String(e);
    }
  }

  function choose(item: string, recipeId: string) {
    form.choices[item] = recipeId;
    calculate();
  }

  function resetChoices() {
    form.choices = {};
    calculate();
  }

  async function save() {
    const request = toRequest(form, targets);
    if (typeof request === "string") return (error = request);
    const name = planName.trim() || form.targetName;
    try {
      const plan = await api.savePlan({ id: planId, name, request });
      planId = plan.id;
      savedNote = `Saved “${plan.name}” with ${plan.steps.length} steps.`;
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="tab">
  <aside>
    <PlanForm bind:form {targets} {raws} {extractors} onCalculate={calculate} />
    {#if Object.keys(form.choices).length}
      <button onclick={resetChoices}>Reset recipe picks</button>
    {/if}
  </aside>
  <section>
    {#if error}<p class="error">{error}</p>{/if}
    {#if calc}
      <div class="save">
        <input bind:value={planName} placeholder={form.targetName || "Plan name"} />
        <button class="primary" onclick={save}>{planId ? "Update plan" : "Save as plan"}</button>
        {#if savedNote}<span class="muted">{savedNote}</span>{/if}
      </div>
      <ResultView {calc} choices={form.choices} onChoose={choose} />
    {:else if !error}
      <p class="muted">Pick a target and press Calculate.</p>
    {/if}
  </section>
</div>

<style>
  .tab {
    display: grid;
    grid-template-columns: 280px 1fr;
    gap: 16px;
    padding: 12px;
  }
  .save {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-bottom: 12px;
  }
</style>
