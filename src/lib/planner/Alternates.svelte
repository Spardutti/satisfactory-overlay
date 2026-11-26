<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Alternate } from "$lib/api";

  let alternates = $state<Alternate[]>([]);
  let filter = $state("");
  let error = $state("");

  const shown = $derived(
    alternates.filter((a) => `${a.name} ${a.makes}`.toLowerCase().includes(filter.trim().toLowerCase())),
  );
  const owned = $derived(alternates.filter((a) => a.enabled).length);

  onMount(async () => {
    alternates = await api.listAlternates();
  });

  async function toggle(alt: Alternate) {
    try {
      await api.setAlternate(alt.id, !alt.enabled);
      alt.enabled = !alt.enabled;
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="alts">
  <div class="bar">
    <input bind:value={filter} placeholder="Filter by name or product" />
    <span class="muted">{owned} of {alternates.length} owned</span>
    {#if error}<span class="error">{error}</span>{/if}
  </div>
  <ul>
    {#each shown as alt (alt.id)}
      <li>
        <label>
          <input type="checkbox" checked={alt.enabled} onchange={() => toggle(alt)} />
          {alt.name} <span class="muted">→ {alt.makes}</span>
        </label>
      </li>
    {/each}
  </ul>
</div>

<style>
  .alts {
    padding: 0;
  }
  .bar {
    display: flex;
    gap: 12px;
    align-items: center;
    margin-bottom: 8px;
  }
  .bar input {
    width: 300px;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    columns: 2;
  }
  li {
    padding: 2px 0;
    break-inside: avoid;
  }
</style>
