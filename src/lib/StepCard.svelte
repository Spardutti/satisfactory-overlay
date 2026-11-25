<script lang="ts">
  import type { Snippet } from "svelte";
  import type { ChecklistStep } from "$lib/api";

  let {
    card,
    number,
    current = false,
    controls,
  }: { card: ChecklistStep; number: number; current?: boolean; controls?: Snippet } = $props();

  const complete = $derived(card.total > 0 && card.built >= card.total);
</script>

<div class="card" class:complete class:current>
  <div class="head">
    <span class="num">{number}</span>
    <span class="title">{card.label}</span>
    {@render controls?.()}
  </div>
  <div class="detail">{card.detail}</div>
  {#each card.inputs as input (input)}
    <div class="line"><b>IN</b> {input}</div>
  {/each}
  {#if card.output}<div class="line" class:warn={card.output.includes("overfills")}><b>OUT</b> {card.output}</div>{/if}
  {#if card.goesTo}<div class="line"><b>→</b> {card.goesTo}</div>{/if}
</div>

<style>
  .card {
    padding: 6px 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }
  .card.complete {
    opacity: 0.4;
  }
  .card.current .title {
    color: var(--orange);
  }
  .head {
    display: flex;
    gap: 6px;
    align-items: center;
    flex-wrap: wrap;
  }
  .num {
    min-width: 18px;
    height: 18px;
    border-radius: 50%;
    background: var(--panel-2);
    font-size: 11px;
    text-align: center;
    line-height: 18px;
  }
  .title {
    flex: 1;
    font-weight: 600;
  }
  .detail,
  .line {
    font-size: 12px;
    color: var(--muted);
    padding-left: 24px;
  }
  .line b {
    display: inline-block;
    min-width: 30px;
    color: var(--text);
    font-weight: 600;
  }
  .warn {
    color: var(--orange);
  }
</style>
