<script lang="ts">
  import { fmt, type Calculation } from "$lib/api";
  import StepCard from "$lib/StepCard.svelte";

  let {
    calc,
    choices,
    onChoose,
  }: {
    calc: Calculation;
    choices: Record<string, string>;
    onChoose: (item: string, recipeId: string) => void;
  } = $props();

  const stepByItem = $derived(new Map(calc.result.steps.map((s) => [s.item, s])));
  const unmined = $derived(calc.result.raw.filter((r) => r.miners === null));
</script>

<div class="result">
  {#if calc.limit}
    <div class="box">
      Max <strong>{fmt(calc.limit.maxRate)} /min</strong> of {calc.result.targetName}. Limited by
      <strong>{calc.limit.limitingName}</strong>.
      <ul>
        {#each calc.limit.resources as r (r.item)}
          <li>{r.itemName}: uses {fmt(r.used)} of {fmt(r.available)} · <span class="muted">{fmt(r.left)} left</span></li>
        {/each}
      </ul>
    </div>
  {/if}

  {#if unmined.length}
    <h3>Bring in</h3>
    <ul>
      {#each unmined as r (r.item)}
        <li>{r.itemName}: {fmt(r.perMin)} {r.fluid ? "m³" : ""}/min</li>
      {/each}
    </ul>
  {/if}

  <h3>Build in this order</h3>
  {#each calc.cards as card, i (card.label)}
    {@const options = calc.options[card.item] ?? []}
    {@const step = stepByItem.get(card.item)}
    <StepCard {card} number={i + 1}>
      {#snippet controls()}
        {#if step && options.length > 1}
          <select value={choices[card.item] ?? step.recipeId} onchange={(e) => onChoose(card.item, e.currentTarget.value)}>
            {#each options as o (o.id)}<option value={o.id}>{o.name}</option>{/each}
          </select>
        {/if}
      {/snippet}
    </StepCard>
  {/each}

  {#if calc.result.surplus.length}
    <h3>Left over</h3>
    <ul>
      {#each calc.result.surplus as s (s.item)}<li>{s.itemName}: {fmt(s.perMin)}/min</li>{/each}
    </ul>
  {/if}
</div>

<style>
  .result h3 {
    margin-top: 14px;
  }
  ul {
    margin: 0;
    padding-left: 18px;
  }
  .box {
    padding: 8px 12px;
    background: var(--panel-2);
    border-left: 3px solid var(--orange);
  }
</style>
