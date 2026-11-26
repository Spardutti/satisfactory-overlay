<script lang="ts">
  import { untrack } from "svelte";

  let {
    itemName,
    perMin,
    onSave,
    onClose,
  }: { itemName: string; perMin: number | null; onSave: (perMin: number | null) => void; onClose: () => void } = $props();

  let amount = $state(untrack(() => perMin) ?? 10);
</script>

<form class="depot" onsubmit={(e) => (e.preventDefault(), onSave(amount))}>
  <span><b>{itemName}</b> from a depot:</span>
  <input type="number" min="0" step="any" bind:value={amount} aria-label="Depot amount per minute" />
  <span class="muted">/min</span>
  <button class="primary" type="submit">Save</button>
  {#if perMin !== null}<button type="button" onclick={() => onSave(null)}>Build it here instead</button>{/if}
  <button type="button" onclick={onClose}>Close</button>
  <small class="muted">Nothing gets built for it, and the plan shrinks if this is not enough.</small>
</form>

<style>
  .depot {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
    padding: 8px 12px;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
  b {
    color: var(--orange);
  }
  input {
    width: 80px;
  }
</style>
