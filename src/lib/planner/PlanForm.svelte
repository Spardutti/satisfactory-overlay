<script lang="ts">
  import { fmt, type ExtractorOptions, type Named, type Target } from "$lib/api";
  import DepotRows from "./DepotRows.svelte";
  import type { FormState } from "./form";

  let {
    form = $bindable(),
    targets,
    raws,
    extractors,
    onCalculate,
  }: {
    form: FormState;
    targets: Target[];
    raws: Named[];
    extractors: ExtractorOptions;
    onCalculate: () => void;
  } = $props();

  const standard = $derived(
    targets.find((t) => t.name.toLowerCase() === form.targetName.trim().toLowerCase())?.standard ?? null,
  );

  function addResource() {
    const unused = raws.find((r) => !form.resources.some((row) => row.item === r.id));
    if (unused) form.resources.push({ item: unused.id, amount: 60 });
  }
</script>

<form class="plan-form" onsubmit={(e) => (e.preventDefault(), onCalculate())}>
  <section>
    <h3><span>1</span> What do you want to build?</h3>
    <input list="targets" bind:value={form.targetName} placeholder="e.g. AI Expansion Server" aria-label="Target item" />
    <datalist id="targets">
      {#each targets as t (t.id)}<option value={t.name}></option>{/each}
    </datalist>
    {#if standard}
      <p class="hint">
        <strong>{fmt(standard.perMachine)} /min</strong> per {standard.buildingName}
        {#if form.mode === "rate" && form.rate > 0}
          {@const count = form.rate / standard.perMachine}
          · {fmt(form.rate)} /min needs {fmt(count)} {standard.buildingName}{count === 1 ? "" : "s"}
        {/if}
      </p>
    {/if}
  </section>

  <section>
    <h3><span>2</span> How much?</h3>
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
  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--muted);
  }
  .hint strong {
    color: var(--orange);
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
