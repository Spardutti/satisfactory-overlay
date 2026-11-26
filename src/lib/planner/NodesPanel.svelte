<script lang="ts">
  import { fmt, type Calculation, type NodeCounts, type RawNeed } from "$lib/api";

  /** One row per mined ore: pure, normal and impure counts, and how well they cover the need. */
  let {
    calc,
    nodes,
    onChange,
  }: { calc: Calculation; nodes: Record<string, NodeCounts>; onChange: (item: string, counts: NodeCounts) => void } =
    $props();

  const PURITIES = ["pure", "normal", "impure"] as const;
  const NONE: NodeCounts = { pure: 0, normal: 0, impure: 0 };
  const mined = $derived(calc.result.raw.filter((r) => r.miners !== null));

  function step(item: string, purity: keyof NodeCounts, by: number) {
    const counts = nodes[item] ?? NONE;
    onChange(item, { ...counts, [purity]: Math.max(0, counts[purity] + by) });
  }

  function spareText(spare: NodeCounts): string {
    const left = PURITIES.filter((p) => spare[p] > 0).map((p) => `${spare[p]} ${p}`);
    return left.length ? ` · spare: ${left.join(", ")}` : "";
  }

  function status(raw: RawNeed): string {
    if (!raw.nodes) return `${fmt(raw.perMin)} /min · assumes ${raw.miners} normal nodes`;
    const { short, spare } = raw.nodes;
    const shortText = short > 0 ? ` · ${fmt(short)} short` : "";
    return `${fmt(raw.perMin - short)} of ${fmt(raw.perMin)} /min${shortText}${spareText(spare)}`;
  }
</script>

{#if mined.length}
  <div class="nodes">
    <h3>Your nodes <span class="muted">optional — leave at 0 to assume normal nodes</span></h3>
    <table>
      <thead>
        <tr>
          <th>Ore</th>
          {#each PURITIES as purity (purity)}<th class="purity">{purity}</th>{/each}
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each mined as raw (raw.item)}
          {@const counts = nodes[raw.item] ?? NONE}
          <tr>
            <td class="ore">{raw.itemName}</td>
            {#each PURITIES as purity (purity)}
              <td class="stepper">
                <button onclick={() => step(raw.item, purity, -1)} disabled={counts[purity] === 0} aria-label="One less {purity} {raw.itemName} node">−</button>
                <span class:zero={counts[purity] === 0}>{counts[purity]}</span>
                <button onclick={() => step(raw.item, purity, 1)} aria-label="One more {purity} {raw.itemName} node">+</button>
              </td>
            {/each}
            <td class="muted" class:short={(raw.nodes?.short ?? 0) > 0}>{status(raw)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
    {#if calc.nodeLimit !== null}
      <p class="short">Your nodes allow {fmt(calc.nodeLimit)} /min of {calc.result.targetName}.</p>
    {/if}
  </div>
{/if}

<style>
  .nodes {
    margin-bottom: 16px;
  }
  table {
    border-collapse: collapse;
  }
  th {
    text-align: left;
    font-weight: 500;
    color: var(--muted);
    padding: 4px 10px;
  }
  .purity {
    text-transform: capitalize;
    text-align: center;
  }
  td {
    padding: 4px 10px;
    border-top: 1px solid var(--line);
  }
  .ore {
    font-weight: 600;
  }
  .stepper {
    white-space: nowrap;
    text-align: center;
  }
  .stepper span {
    display: inline-block;
    min-width: 22px;
    text-align: center;
  }
  .stepper button {
    width: 24px;
    height: 24px;
    padding: 0;
  }
  .zero {
    color: var(--muted);
  }
  .short {
    color: var(--orange);
  }
</style>
