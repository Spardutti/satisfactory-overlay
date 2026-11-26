<script lang="ts">
  import { untrack } from "svelte";

  let { maxTier, current, onPick }: { maxTier: number; current: number; onPick: (tier: number) => void } = $props();

  let tier = $state(untrack(() => current));
</script>

<form class="pick" onsubmit={(e) => (e.preventDefault(), onPick(tier))}>
  <span>I've finished up to</span>
  <select bind:value={tier} aria-label="Highest finished tier">
    {#each Array.from({ length: maxTier + 1 }, (_, i) => i) as t (t)}<option value={t}>Tier {t}</option>{/each}
  </select>
  <button class="primary" type="submit">Apply</button>
  <span class="muted">Ticks every milestone up to that tier, and unticks the ones above.</span>
</form>

<style>
  .pick {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
  }
</style>
