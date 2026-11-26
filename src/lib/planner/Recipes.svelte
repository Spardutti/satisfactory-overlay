<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Unlock } from "$lib/api";
  import Alternates from "./Alternates.svelte";
  import TierPicker from "./TierPicker.svelte";
  import UnlockList from "./UnlockList.svelte";

  let unlocks = $state<Unlock[]>([]);
  let error = $state("");

  const milestones = $derived(unlocks.filter((u) => u.kind === "milestone"));
  const mam = $derived(unlocks.filter((u) => u.kind === "mam"));
  const maxTier = $derived(Math.max(0, ...milestones.map((u) => u.tier)));
  const currentTier = $derived(Math.max(0, ...milestones.filter((u) => u.ticked).map((u) => u.tier)));
  const picked = $derived(milestones.some((u) => u.ticked));

  onMount(load);

  async function load() {
    try {
      unlocks = await api.listUnlocks();
    } catch (e) {
      error = String(e);
    }
  }

  async function toggle(u: Unlock) {
    try {
      await api.setUnlock(u.id, !u.ticked);
      u.ticked = !u.ticked;
    } catch (e) {
      error = String(e);
    }
  }

  async function pickTier(tier: number) {
    try {
      await api.setTier(tier);
      await load();
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="recipes">
  {#if error}<p class="error">{error}</p>{/if}
  {#if unlocks.length}
    <section>
      <h3>Milestones</h3>
      {#if !picked}<p class="warn">Pick the highest tier you finished. Plans only use recipes you have unlocked.</p>{/if}
      {#key currentTier}<TierPicker {maxTier} current={currentTier} onPick={pickTier} />{/key}
      <UnlockList unlocks={milestones} onToggle={toggle} />
    </section>
    <section>
      <h3>MAM research</h3>
      <UnlockList unlocks={mam} onToggle={toggle} />
    </section>
  {/if}
  <section>
    <h3>Alternates</h3>
    <Alternates />
  </section>
</div>

<style>
  .recipes {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px;
  }
  section {
    padding: 12px;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
  h3 {
    margin: 0 0 8px;
    font-size: 15px;
  }
  .warn {
    color: var(--orange);
    margin: 0 0 8px;
  }
</style>
