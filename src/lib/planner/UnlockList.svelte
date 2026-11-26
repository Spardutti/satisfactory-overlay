<script lang="ts">
  import type { Unlock } from "$lib/api";

  let { unlocks, onToggle }: { unlocks: Unlock[]; onToggle: (unlock: Unlock) => void } = $props();

  const groupOf = (u: Unlock) => (u.kind === "milestone" ? `Tier ${u.tier}` : u.tree);
  const groups = $derived(
    [...new Set(unlocks.map(groupOf))].map((name) => ({ name, items: unlocks.filter((u) => groupOf(u) === name) })),
  );
</script>

{#each groups as group (group.name)}
  <h4>{group.name}</h4>
  <ul>
    {#each group.items as u (u.id)}
      <li>
        <label>
          <input type="checkbox" checked={u.ticked} onchange={() => onToggle(u)} />
          {u.name} <span class="muted">→ {u.makes.join(", ")}</span>
        </label>
      </li>
    {/each}
  </ul>
{/each}

<style>
  h4 {
    margin: 10px 0 4px;
    color: var(--orange);
    font-size: 13px;
  }
  ul {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  li {
    padding: 2px 0;
  }
</style>
