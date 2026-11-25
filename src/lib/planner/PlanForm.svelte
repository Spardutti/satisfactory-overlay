<script lang="ts">
  import { fmt, type ExtractorOptions, type Named, type Target } from "$lib/api";
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
  <label>
    Target
    <input list="targets" bind:value={form.targetName} placeholder="e.g. Screws" />
    <datalist id="targets">
      {#each targets as t (t.id)}<option value={t.name}></option>{/each}
    </datalist>
  </label>
  {#if standard}
    <p class="hint">
      Standard: <strong>{fmt(standard.perMachine)} /min per {standard.buildingName}</strong>
      {#if form.mode === "rate" && form.rate > 0}
        · {fmt(form.rate)} /min = {fmt(form.rate / standard.perMachine)} {standard.buildingName}s
      {/if}
    </p>
  {/if}

  <div class="modes">
    <label><input type="radio" bind:group={form.mode} value="rate" /> I want X / min</label>
    <label><input type="radio" bind:group={form.mode} value="resources" /> I have these resources</label>
  </div>

  {#if form.mode === "rate"}
    <label>Per minute <input type="number" min="0" step="any" bind:value={form.rate} /></label>
  {:else}
    {#each form.resources as row, i (i)}
      <div class="row">
        <select bind:value={row.item}>
          {#each raws as r (r.id)}<option value={r.id}>{r.name}</option>{/each}
        </select>
        <input type="number" min="0" step="any" bind:value={row.amount} />
        <span class="muted">/min</span>
        <button type="button" onclick={() => form.resources.splice(i, 1)}>✕</button>
      </div>
    {/each}
    <button type="button" onclick={addResource}>+ Resource</button>
  {/if}

  <div class="row">
    <label>
      Miner
      <select bind:value={form.minerTier}>
        {#each extractors.tiers as t (t)}<option>{t}</option>{/each}
      </select>
    </label>
    <label>
      Node
      <select bind:value={form.purity}>
        {#each extractors.purities as p (p)}<option>{p}</option>{/each}
      </select>
    </label>
  </div>

  <div class="row">
    <label>
      Belt
      <select bind:value={form.beltTier}>
        {#each extractors.belts as b (b)}<option>{b}</option>{/each}
      </select>
    </label>
    <label>
      Pipe
      <select bind:value={form.pipeTier}>
        {#each extractors.pipes as p (p)}<option>{p}</option>{/each}
      </select>
    </label>
  </div>

  <button class="primary" type="submit">Calculate</button>
</form>

<style>
  .plan-form {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    color: var(--muted);
  }
  .modes label,
  .row label {
    flex-direction: row;
    align-items: center;
    gap: 6px;
  }
  .row {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .hint {
    margin: -4px 0 0;
    font-size: 12px;
    color: var(--muted);
  }
  .hint strong {
    color: var(--orange);
  }
  .row input[type="number"] {
    width: 90px;
  }
</style>
