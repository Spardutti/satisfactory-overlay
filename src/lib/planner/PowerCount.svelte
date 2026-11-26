<script lang="ts">
  import { fmt } from "$lib/api";

  let { rate = $bindable(), perGenerator, generator }: { rate: number; perGenerator: number; generator: string } = $props();

  const count = $derived(Number(fmt(rate / perGenerator)));
</script>

<label class="row">
  <input
    type="number"
    min="0"
    step="1"
    value={count}
    oninput={(e) => (rate = Number(e.currentTarget.value) * perGenerator)}
    aria-label="Number of generators"
  />
  <span class="muted">× {generator}</span>
</label>
<label class="row">
  <input type="number" min="0" step="any" bind:value={rate} aria-label="Megawatts" />
  <span class="muted">MW</span>
</label>

<style>
  .row {
    display: flex;
    gap: 6px;
    align-items: center;
  }
  input {
    width: 80px;
  }
</style>
