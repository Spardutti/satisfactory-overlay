<script lang="ts">
  import type { BuildCount } from "$lib/api";

  let { counts }: { counts: BuildCount[] } = $props();
</script>

<div class="counts">
  <h3>Buildings to bring</h3>
  {#if counts.length}
    <ul>
      {#each counts as c (c.name)}<li><b>{c.count}</b> × {c.name}</li>{/each}
    </ul>
  {:else}
    <p class="muted">Nothing to build.</p>
  {/if}
</div>

<style>
  .counts {
    padding: 8px 12px;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
  h3 {
    margin: 0 0 6px;
    font-size: 14px;
  }
  ul {
    margin: 0;
    padding: 0;
    list-style: none;
    columns: 3 180px;
  }
  b {
    color: var(--orange);
  }
</style>
