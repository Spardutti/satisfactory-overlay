<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Calculation, type ExtractorOptions, type Named, type NodeCounts, type Plan, type PowerOption, type Target } from "$lib/api";
  import { defaultName, emptyForm, fromPlan, toRequest } from "./form";
  import NodesPanel from "./NodesPanel.svelte";
  import PlanForm from "./PlanForm.svelte";
  import ResultView from "./ResultView.svelte";

  let { editing, onSaved }: { editing: Plan | null; onSaved: (plan: Plan) => void } = $props();

  let targets = $state<Target[]>([]);
  let raws = $state<Named[]>([]);
  let powerOptions = $state<PowerOption[]>([]);
  let extractors = $state<ExtractorOptions>({ tiers: [], belts: [], pipes: [] });
  let form = $state(emptyForm());
  let calc = $state<Calculation | null>(null);
  let error = $state("");
  let planName = $state("");
  let planId = $state<string | null>(null);

  onMount(async () => {
    [targets, raws, extractors, powerOptions] = await Promise.all([
      api.listTargets(),
      api.listRawResources(),
      api.extractorOptions(),
      api.listPowerOptions(),
    ]);
    if (editing) {
      planId = editing.id;
      form = fromPlan(editing, targets);
      planName = editing.name;
      await calculate();
    }
  });

  async function calculate() {
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

  function setNodes(item: string, counts: NodeCounts) {
    if (counts.pure + counts.normal + counts.impure === 0) delete form.nodes[item];
    else form.nodes[item] = counts;
    calculate();
  }

  function resetChoices() {
    form.choices = {};
    calculate();
  }

  async function save() {
    const request = toRequest(form, targets);
    if (typeof request === "string") return (error = request);
    const name = planName.trim() || defaultName(form);
    try {
      onSaved(await api.savePlan({ id: planId, name, request }));
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="tab">
  <aside>
    <PlanForm bind:form {targets} {raws} {extractors} {powerOptions} onCalculate={calculate} />
    {#if Object.keys(form.choices).length}
      <button onclick={resetChoices}>Reset recipe picks</button>
    {/if}
  </aside>
  <section>
    {#if error}<p class="error">{error}</p>{/if}
    {#if calc}
      <div class="save">
        <input bind:value={planName} placeholder={defaultName(form)} />
        <button class="primary" onclick={save}>{planId ? "Update blueprint" : "Save blueprint"}</button>
      </div>
      <NodesPanel {calc} nodes={form.nodes} onChange={setNodes} />
      <ResultView {calc} choices={form.choices} onChoose={choose} />
    {/if}
  </section>
</div>

<style>
  .tab {
    display: grid;
    grid-template-columns: 300px 1fr;
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
