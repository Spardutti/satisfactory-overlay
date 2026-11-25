<script lang="ts">
  import { onMount } from "svelte";
  import { fmt, type Block, type Layout } from "$lib/api";
  import Belt from "./Belt.svelte";
  import BlockView from "./BlockView.svelte";
  import { itemColor } from "./colors";
  import { buildRoutes } from "./belts";
  import { bounds, fitView, panBy, zoomAt, type View } from "./view";

  let {
    layout = $bindable(),
    built,
    onMoved,
    onToggle,
  }: {
    layout: Layout;
    built: Record<string, number[]>;
    onMoved: (item: string, x: number, y: number) => void;
    onToggle: (item: string, row: number) => void;
  } = $props();

  let screen = $state<HTMLDivElement>();
  let view = $state<View>({ scale: 1, x: 0, y: 0 });
  let drag: { x: number; y: number; block: Block | null; row: number | null; moved: boolean } | null = null;

  function fit() {
    if (!screen) return;
    view = fitView(bounds(layout.blocks), { width: screen.clientWidth, height: screen.clientHeight });
  }

  onMount(fit);

  function onWheel(e: WheelEvent) {
    e.preventDefault();
    const box = screen!.getBoundingClientRect();
    view = zoomAt(view, Math.exp(-e.deltaY * 0.0015), { x: e.clientX - box.left, y: e.clientY - box.top });
  }

  /** Dragging a block moves it; dragging anywhere else moves the view. */
  function onDown(e: PointerEvent) {
    if (e.button !== 0) return;
    const target = e.target as Element;
    if (target.closest("button")) return;
    const item = target.closest("[data-block]")?.getAttribute("data-block");
    const block = layout.blocks.find((b) => b.item === item) ?? null;
    const row = target.closest("[data-row]")?.getAttribute("data-row");
    drag = { x: e.clientX, y: e.clientY, block, row: row == null ? null : Number(row), moved: false };
    (e.currentTarget as Element).setPointerCapture(e.pointerId);
  }

  function onMove(e: PointerEvent) {
    if (!drag) return;
    const [dx, dy] = [e.clientX - drag.x, e.clientY - drag.y];
    if (drag.block) {
      drag.block.x += dx / view.scale;
      drag.block.y += dy / view.scale;
    } else {
      view = panBy(view, dx, dy);
    }
    drag = { ...drag, x: e.clientX, y: e.clientY, moved: drag.moved || dx !== 0 || dy !== 0 };
  }

  /** Also runs when the pointer is lost mid-drag, e.g. on alt-tab, so a drag never sticks. */
  function onUp(e: PointerEvent) {
    const done = drag;
    drag = null;
    if (!done?.block) return;
    if (done.moved) onMoved(done.block.item, Math.round(done.block.x), Math.round(done.block.y));
    else if (done.row !== null && e.type === "pointerup") onToggle(done.block.item, done.row);
  }

  const routes = $derived(buildRoutes(layout));
</script>

<div class="screen" bind:this={screen} onwheel={onWheel} onpointerdown={onDown} onpointermove={onMove} onpointerup={onUp} onpointercancel={onUp} onlostpointercapture={onUp} role="application" aria-label="Plan blueprint">
  <svg width="100%" height="100%">
    <g transform="translate({view.x} {view.y}) scale({view.scale})">
      {#each routes.trunks as trunk, i (i)}<Belt d={trunk.d} color={itemColor(trunk.item)} fluid={trunk.fluid} />{/each}
      {#each routes.branches as branch, i (i)}<Belt d={branch.d} color={itemColor(branch.item)} fluid={branch.fluid} />{/each}
      {#each routes.nodes as node, i (i)}
        <rect class="node" x={node.x - 9} y={node.y - 9} width="18" height="18" rx="2" />
        <text class="node-tag" x={node.x} y={node.y + 4}>{node.kind === "split" ? "S" : "M"}</text>
      {/each}
      {#each routes.branches as branch, i (i)}
        <text class="link-label" x={branch.label.x} y={branch.label.y} style:fill={itemColor(branch.item)}>{fmt(branch.perMin)}</text>
      {/each}
      {#each layout.blocks as block (block.item)}
        <BlockView {block} metrics={layout.metrics} built={built[block.item] ?? []} />
      {/each}
    </g>
  </svg>
  <div class="tools">
    <span class="muted">{Math.round(view.scale * 100)}%</span>
    <button onclick={fit}>Fit</button>
  </div>
</div>

<style>
  .screen {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
    cursor: grab;
    background-color: #17191b;
    background-image: radial-gradient(#2c3035 1px, transparent 1px);
    background-size: 24px 24px;
    touch-action: none;
  }
  .screen:active {
    cursor: grabbing;
  }
  .link-label {
    fill: var(--text);
    font-size: 11px;
    text-anchor: middle;
    paint-order: stroke;
    stroke: #17191b;
    stroke-width: 3px;
  }
  .node {
    fill: #3a3e44;
    stroke: #e0b04a;
    stroke-width: 2;
  }
  .node-tag {
    fill: #e0b04a;
    font-size: 10px;
    font-weight: 700;
    text-anchor: middle;
  }
  .tools {
    position: absolute;
    right: 12px;
    bottom: 12px;
    display: flex;
    gap: 8px;
    align-items: center;
  }
</style>
