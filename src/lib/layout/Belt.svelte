<script lang="ts">
  let {
    d,
    color = "#a3a8ae",
    over = false,
    fluid = false,
  }: { d: string; color?: string; over?: boolean; fluid?: boolean } = $props();
</script>

<g class="belt" class:over class:fluid>
  <path class="rail" {d} />
  <path class="band" {d} />
  <path class="flow" {d} style:stroke={color} />
</g>

<style>
  path {
    fill: none;
    stroke-linecap: butt;
  }
  .rail {
    stroke: #6c7178;
    stroke-width: 12;
  }
  .over .rail {
    stroke: var(--bad);
  }
  .band {
    stroke: #26282c;
    stroke-width: 8;
  }
  .flow {
    stroke-width: 6;
    stroke-dasharray: 2 7;
    animation: run 0.9s linear infinite;
  }
  .fluid .rail {
    stroke: #4d7a99;
  }
  .fluid .band {
    stroke: #1d3a4d;
  }
  .fluid .flow {
    stroke-dasharray: 10 6;
  }
  @keyframes run {
    to {
      stroke-dashoffset: -9;
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .flow {
      animation: none;
    }
  }
</style>
