<script lang="ts">
  import { POWER, type PowerOption, type ExtractorOptions, type Named, type Target } from "$lib/api";
  import DepotRows from "./DepotRows.svelte";
  import PowerCount from "./PowerCount.svelte";
  import MaterialPicker from "./MaterialPicker.svelte";
  import PowerPicker from "./PowerPicker.svelte";
  import type { FormState } from "./form";

  let {
    form = $bindable(),
    targets,
    raws,
    extractors,
    powerOptions,
    onCalculate,
  }: {
    form: FormState;
    targets: Target[];
    raws: Named[];
    extractors: ExtractorOptions;
    powerOptions: PowerOption[];
    onCalculate: () => void;
  } = $props();

  const target = $derived(targets.find((t) => t.name.toLowerCase() === form.targetName.trim().toLowerCase()));
  const standard = $derived(target?.standard ?? null);
  const isPower = $derived(target?.id === POWER);
  const materials = $derived(targets.filter((t) => t.id !== POWER));
  const generator = $derived(powerOptions.find((o) => o.recipeId === form.choices[POWER]) ?? powerOptions[0]);

  /** Power plans aim for whole generators: one to start. */
  function pickPower() {
    const first = powerOptions[0];
    form.targetName = targets.find((t) => t.id === POWER)?.name ?? "";
    form.mode = "rate";
    form.choices[POWER] ??= first?.recipeId;
    form.rate = generator?.megawatts ?? first?.megawatts ?? 0;
  }

  function pickMaterial() {
    form.targetName = "";
    delete form.choices[POWER];
  }

  function addResource() {
    const unused = raws.find((r) => !form.resources.some((row) => row.item === r.id));
    if (unused) form.resources.push({ item: unused.id, amount: 60 });
  }
</script>

<form class="plan-form" onsubmit={(e) => (e.preventDefault(), onCalculate())}>
  <section>
    <h3><span>1</span> What do you want to build?</h3>
    <div class="modes">
      <button type="button" class:on={!isPower} onclick={pickMaterial}>Material</button>
      <button type="button" class:on={isPower} onclick={pickPower}>Power</button>
    </div>
    {#if isPower}
      <PowerPicker bind:choice={form.choices[POWER]} options={powerOptions} />
    {:else}
      <MaterialPicker bind:targetName={form.targetName} {materials} {standard} rate={form.mode === "rate" ? form.rate : 0} />
    {/if}
  </section>

  <section>
    <h3><span>2</span> How much?</h3>
    {#if isPower && generator}
      <PowerCount bind:rate={form.rate} perGenerator={generator.megawatts} generator={generator.generator} />
    {:else}
      <div class="modes">
        <button type="button" class:on={form.mode === "rate"} onclick={() => (form.mode = "rate")}>I want X / min</button>
        <button type="button" class:on={form.mode === "resources"} onclick={() => (form.mode = "resources")}>I have resources</button>
      </div>
      {#if form.mode === "rate"}
        <label class="row">
          <input type="number" min="0" step="any" bind:value={form.rate} aria-label="Per minute" />
          <span class="muted">/min</span>
        </label>
      {:else}
        {#each form.resources as row, i (i)}
          <div class="row">
            <select bind:value={row.item} aria-label="Resource">
              {#each raws as r (r.id)}<option value={r.id}>{r.name}</option>{/each}
            </select>
            <input type="number" min="0" step="any" bind:value={row.amount} aria-label="Amount per minute" />
            <span class="muted">/min</span>
            <button type="button" onclick={() => form.resources.splice(i, 1)} aria-label="Remove resource">✕</button>
          </div>
        {/each}
        <button type="button" onclick={addResource}>+ Resource</button>
      {/if}
    {/if}
  </section>

  <section>
    <h3><span>3</span> Your tiers</h3>
    <div class="tiers">
      <label>Miner <select bind:value={form.minerTier}>{#each extractors.tiers as t (t)}<option>{t}</option>{/each}</select></label>
      <label>Belt <select bind:value={form.beltTier}>{#each extractors.belts as b (b)}<option>{b}</option>{/each}</select></label>
      <label>Pipe <select bind:value={form.pipeTier}>{#each extractors.pipes as p (p)}<option>{p}</option>{/each}</select></label>
    </div>
  </section>

  <section>
    <h3><span>4</span> From a depot <small class="muted">optional</small></h3>
    <DepotRows bind:rows={form.depot} {targets} />
  </section>

  <button class="primary calculate" type="submit">Calculate</button>
</form>

<style>
  .plan-form {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  section {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
  h3 {
    margin: 0;
    font-size: 14px;
    display: flex;
    gap: 8px;
    align-items: center;
  }
  h3 span {
    display: inline-grid;
    place-items: center;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--orange);
    color: var(--bg);
    font-size: 12px;
  }
  .modes {
    display: flex;
  }
  .modes button {
    flex: 1;
    border-radius: 0;
  }
  .modes button:first-child {
    border-radius: 4px 0 0 4px;
  }
  .modes button:last-child {
    border-radius: 0 4px 4px 0;
  }
  .modes .on {
    background: var(--orange);
    color: var(--bg);
    border-color: var(--orange);
  }
  .row {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .row input[type="number"] {
    width: 90px;
  }
  .tiers {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .tiers label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    color: var(--muted);
    font-size: 12px;
  }
  .calculate {
    padding: 10px;
    font-size: 15px;
  }
</style>
