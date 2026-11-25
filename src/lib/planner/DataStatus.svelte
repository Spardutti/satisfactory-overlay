<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import { api, type DataSummary } from "$lib/api";

  let { onLoaded }: { onLoaded: () => void } = $props();

  let summary = $state<DataSummary | null>(null);
  let error = $state("");
  let searching = $state(true);

  onMount(async () => {
    try {
      summary = await api.loadGameData();
      if (summary) onLoaded();
    } catch (e) {
      error = String(e);
    }
    searching = false;
  });

  async function pickFile() {
    const path = await open({ filters: [{ name: "Game docs", extensions: ["json"] }] });
    if (typeof path !== "string") return;
    try {
      summary = await api.setDocsPath(path);
      error = "";
      onLoaded();
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="status">
  {#if searching}
    <span class="muted">Looking for the game's recipe file…</span>
  {:else if summary}
    <span>{summary.recipes} recipes · {summary.items} items</span>
    <span class="muted path" title={summary.path}>{summary.path}</span>
    {#if summary.warnings.length}
      <span class="error" title={summary.warnings.join("\n")}>{summary.warnings.length} skipped</span>
    {/if}
  {:else}
    <span>Recipe file not found. Pick <code>CommunityResources/Docs/en-US.json</code> in the game folder.</span>
  {/if}
  {#if error}<span class="error">{error}</span>{/if}
  {#if !searching}<button onclick={pickFile}>{summary ? "Change file" : "Pick file"}</button>{/if}
</div>

<style>
  .status {
    display: flex;
    gap: 12px;
    align-items: center;
    padding: 8px 12px;
    background: var(--panel);
    border-bottom: 1px solid var(--line);
  }
  .path {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
