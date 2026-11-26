<script lang="ts">
  import type { PowerOption } from "$lib/api";

  let { choice = $bindable(), options }: { choice: string | undefined; options: PowerOption[] } = $props();

  const chosen = $derived(options.find((o) => o.recipeId === choice) ?? options[0]);
  const generators = $derived([...new Set(options.map((o) => o.generator))]);
  const fuels = $derived(options.filter((o) => o.generator === chosen?.generator));

  function pickGenerator(name: string) {
    choice = options.find((o) => o.generator === name)?.recipeId;
  }
</script>

<label class="row">
  <span class="muted">Generator</span>
  <select value={chosen?.generator} onchange={(e) => pickGenerator(e.currentTarget.value)} aria-label="Generator">
    {#each generators as g (g)}<option value={g}>{g}</option>{/each}
  </select>
</label>
<label class="row">
  <span class="muted">Fuel</span>
  <select value={chosen?.recipeId} onchange={(e) => (choice = e.currentTarget.value)} aria-label="Fuel">
    {#each fuels as f (f.recipeId)}<option value={f.recipeId}>{f.fuel}</option>{/each}
  </select>
</label>
{#if chosen}<p class="hint"><strong>{chosen.megawatts} MW</strong> per {chosen.generator}</p>{/if}

<style>
  .row {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .row span {
    width: 70px;
  }
  .row select {
    flex: 1;
  }
  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--muted);
  }
  .hint strong {
    color: var(--orange);
  }
</style>
