<script lang="ts">
  import { fmt, type Target } from "$lib/api";

  let {
    targetName = $bindable(),
    materials,
    standard,
    rate,
  }: { targetName: string; materials: Target[]; standard: Target["standard"]; rate: number } = $props();
</script>

<input list="targets" bind:value={targetName} placeholder="e.g. AI Expansion Server" aria-label="Target item" />
<datalist id="targets">
  {#each materials as t (t.id)}<option value={t.name}></option>{/each}
</datalist>
{#if standard}
  <p class="hint">
    <strong>{fmt(standard.perMachine)} /min</strong> per {standard.buildingName}
    {#if rate > 0}
      {@const count = rate / standard.perMachine}
      · {fmt(rate)} /min needs {fmt(count)} {standard.buildingName}{count === 1 ? "" : "s"}
    {/if}
  </p>
{/if}

<style>
  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--muted);
  }
  .hint strong {
    color: var(--orange);
  }
</style>
