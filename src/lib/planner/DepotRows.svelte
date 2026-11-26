<script lang="ts">
  import type { Named } from "$lib/api";
  import type { DepotRow } from "./form";

  let { rows = $bindable(), targets }: { rows: DepotRow[]; targets: Named[] } = $props();
</script>

<p class="hint">Items another factory already sends you. Nothing gets built for them.</p>
{#each rows as row, i (i)}
  <div class="row">
    <input list="depot-items" bind:value={row.itemName} placeholder="e.g. Reinforced Iron Plate" aria-label="Depot item" />
    <input type="number" min="0" step="any" bind:value={row.amount} aria-label="Depot amount per minute" />
    <span class="muted">/min</span>
    <button type="button" onclick={() => rows.splice(i, 1)} aria-label="Remove depot item">✕</button>
  </div>
{/each}
<datalist id="depot-items">
  {#each targets as t (t.id)}<option value={t.name}></option>{/each}
</datalist>
<button type="button" onclick={() => rows.push({ itemName: "", amount: 10 })}>+ Depot item</button>

<style>
  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--muted);
  }
  .row {
    display: flex;
    gap: 6px;
    align-items: center;
  }
  .row input[list] {
    flex: 1;
    min-width: 0;
  }
  .row input[type="number"] {
    width: 70px;
  }
</style>
