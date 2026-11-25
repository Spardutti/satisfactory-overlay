<script lang="ts">
  import { fmt, type Block, type Metrics } from "$lib/api";
  import Belt from "./Belt.svelte";
  import { itemColor } from "./colors";
  import { inputLane, outputLane, rowGeometry } from "./geometry";

  let { block, metrics: m }: { block: Block; metrics: Metrics } = $props();

  const tag = $derived(block.buildingName.slice(0, 4).toUpperCase());

  function line(x1: number, y1: number, x2: number, y2: number): string {
    return `M ${x1} ${y1} L ${x2} ${y2}`;
  }
</script>

<g transform="translate({block.x} {block.y})" data-block={block.item}>
  <rect class="frame" width={block.width} height={block.height} rx="6" />
  <text class="title" x="12" y="18">{block.title}</text>
  <text class="building" x="12" y="32">{block.buildingName}</text>

  {#each block.inputBuses as bus, j (j)}
    {@const lane = inputLane(block, m, j)}
    {#each bus.taps as tap (tap.row + "-" + tap.input)}
      {@const g = rowGeometry(block, m, tap.row)}
      <Belt d={line(lane.x, g.inputY(tap.input), g.machineLeft, g.inputY(tap.input))} color={itemColor(bus.item)} fluid={bus.fluid} />
    {/each}
    {#if bus.taps.length > 1}
      <Belt d={line(lane.x, lane.top, lane.x, lane.bottom)} color={itemColor(bus.item)} fluid={bus.fluid} />
      {#each lane.ys as y, t (t)}
        <rect class="splitter" x={lane.x - 7} y={y - 7} width="14" height="14" rx="2" />
      {/each}
    {/if}
  {/each}

  {#each block.outputBuses as bus, j (j)}
    {@const lane = outputLane(block, m, j)}
    {#each bus.taps as tap (tap.row)}
      {@const g = rowGeometry(block, m, tap.row)}
      <Belt d={line(g.machineRight, g.outputY, lane.x, g.outputY)} color={itemColor(bus.item)} fluid={bus.fluid} />
    {/each}
    {#if bus.taps.length > 1}
      <Belt d={line(lane.x, lane.top, lane.x, lane.bottom)} color={itemColor(bus.item)} fluid={bus.fluid} />
      {#each lane.ys as y, t (t)}
        <rect class="merger" x={lane.x - 7} y={y - 7} width="14" height="14" rx="2" />
      {/each}
    {/if}
  {/each}

  {#each block.rows as row, r (r)}
    {@const g = rowGeometry(block, m, r)}
    {#each row.machines as clock, i (i)}
      <rect class="machine" x={g.machineLeft} y={g.machineTop} width={m.machineWidth} height={g.machineHeight} rx="3" />
      <text class="tag" x={g.machineLeft + m.machineWidth / 2} y={g.machineTop + g.machineHeight / 2}>{tag}</text>
      {#if clock < 99.95}
        <text class="clock" x={g.machineLeft + m.machineWidth / 2} y={g.machineTop + g.machineHeight / 2 + 12}>{fmt(clock)}%</text>
      {/if}
    {/each}
    {#each row.inputs as input, k (k)}
      <text class="rate" x={g.machineLeft - 6} y={g.inputY(k) - 6} style:fill={itemColor(input.item)}>{fmt(input.perMin)}</text>
    {/each}
    <text class="rate out" x={g.machineRight + 6} y={g.outputY - 6} style:fill={itemColor(row.output.item)}>{fmt(row.output.perMin)}</text>
  {/each}
</g>

<style>
  .frame {
    fill: rgba(42, 45, 49, 0.92);
    stroke: var(--line);
    cursor: move;
  }
  .title {
    fill: var(--orange);
    font-weight: 600;
    font-size: 14px;
  }
  .building,
  .clock {
    fill: var(--muted);
    font-size: 10px;
  }
  .rate {
    font-size: 10px;
    font-weight: 600;
    text-anchor: end;
  }
  .rate.out {
    text-anchor: start;
  }
  .splitter,
  .merger {
    fill: #3a3e44;
    stroke: #c9ccd1;
    stroke-width: 2;
  }
  .machine {
    fill: var(--panel-2);
    stroke: var(--orange-dim);
    stroke-width: 2;
  }
  .tag,
  .clock {
    text-anchor: middle;
  }
  .tag {
    fill: var(--text);
    font-size: 11px;
    font-weight: 600;
    dominant-baseline: middle;
  }
</style>
