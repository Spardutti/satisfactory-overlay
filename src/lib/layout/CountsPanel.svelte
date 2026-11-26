<script lang="ts">
  import type { Block, BuildCount } from "$lib/api";

  let { counts, blocks, built }: { counts: BuildCount[]; blocks: Block[]; built: Record<string, number[]> } = $props();

  /** Ticks are per machine row, so a building's built count is the ticked rows of its blocks. */
  function builtOf(name: string): number {
    return blocks.filter((b) => b.buildingName === name).reduce((sum, b) => sum + (built[b.item]?.length ?? 0), 0);
  }
</script>

<div class="counts">
  <h3>Buildings to bring</h3>
  {#if counts.length}
    <ul>
      {#each counts as c (c.name)}
        {@const done = builtOf(c.name)}
        <li class:all={done > 0 && done >= c.count}>
          {#if done >= c.count && done > 0}
            ✓ {c.name} <span class="muted">· all {c.count} built</span>
          {:else}
            <b>{c.count - done}</b> × {c.name}{#if done}<span class="muted"> · {done} built</span>{/if}
          {/if}
        </li>
      {/each}
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
  .all {
    color: var(--ok);
  }
</style>
