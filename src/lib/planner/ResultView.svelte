<script lang="ts">
  import { fmt, type Calculation } from "$lib/api";

  let {
    calc,
    choices,
    onChoose,
  }: {
    calc: Calculation;
    choices: Record<string, string>;
    onChoose: (item: string, recipeId: string) => void;
  } = $props();

  const levels = $derived([...new Set(calc.result.steps.map((s) => s.level))]);
  const top = $derived(Math.max(...levels));
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

  <h3>1 · Extraction</h3>
  <ul>
    {#each calc.result.raw as r (r.item)}
      <li>
        {r.itemName}: {fmt(r.perMin)} {r.fluid ? "m³" : ""}/min
        {#if r.miners !== null}<span class="muted">· {r.miners} miner{r.miners === 1 ? "" : "s"}</span>{/if}
      </li>
    {/each}
  </ul>

  {#each levels as level (level)}
    <h3>{level + 1} · {level === top ? "Final product" : `Tier ${level}`}</h3>
    {#each calc.result.steps.filter((s) => s.level === level) as step (step.item)}
      {@const options = calc.options[step.item] ?? []}
      <div class="step">
        <span class="count">{step.machines}×</span>
        <span>{step.buildingName}</span>
        {#if options.length > 1}
          <select value={choices[step.item] ?? step.recipeId} onchange={(e) => onChoose(step.item, e.currentTarget.value)}>
            {#each options as o (o.id)}<option value={o.id}>{o.name}</option>{/each}
          </select>
        {:else}
          <span class="recipe">{step.recipeName}</span>
        {/if}
        <span class="muted">
          {fmt(step.outputPerMin)}/min · {fmt(step.machinesExact)} exact
          {#if step.lastMachinePct < 99.95}· last at {fmt(step.lastMachinePct)}%{/if}
        </span>
      </div>
    {/each}
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
  .step {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 4px 0;
    border-bottom: 1px solid var(--line);
  }
  .count {
    min-width: 32px;
    color: var(--orange);
    font-weight: 600;
  }
  .recipe {
    padding: 4px 8px;
  }
</style>
