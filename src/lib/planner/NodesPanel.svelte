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

  function change(item: string, purity: keyof NodeCounts, value: string) {
    const count = Math.max(0, Math.floor(Number(value) || 0));
    onChange(item, { ...(nodes[item] ?? NONE), [purity]: count });
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
    <h3>Your nodes <span class="muted">optional — leave empty to assume normal nodes</span></h3>
    {#each mined as raw (raw.item)}
      {@const counts = nodes[raw.item] ?? NONE}
      <div class="ore">
        <strong>{raw.itemName}</strong>
        {#each PURITIES as purity (purity)}
          <label>
            {purity}
            <input type="number" min="0" step="1" value={counts[purity]} onchange={(e) => change(raw.item, purity, e.currentTarget.value)} />
          </label>
        {/each}
        <span class:short={(raw.nodes?.short ?? 0) > 0} class="muted">{status(raw)}</span>
      </div>
    {/each}
    {#if calc.nodeLimit !== null}
      <p class="short">Your nodes allow {fmt(calc.nodeLimit)} /min of {calc.result.targetName}.</p>
    {/if}
  </div>
{/if}

<style>
  .nodes {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 12px;
  }
  .ore {
    display: flex;
    gap: 10px;
    align-items: center;
  }
  .ore strong {
    min-width: 110px;
  }
  label {
    display: flex;
    gap: 4px;
    align-items: center;
    color: var(--muted);
  }
  input {
    width: 52px;
  }
  .short {
    color: var(--orange);
  }
</style>
