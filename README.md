# Satisfactory Planner Overlay

A desktop app that plans which machines you need and draws them as a blueprint you can build from.

It is not a mod. It reads the recipe file the game ships with. Game updates show up on the next start.

## Using it

1. Start the app. It looks for `CommunityResources\Docs\en-US.json` in the usual Steam and Epic folders.
2. Not found? Press **Pick file** and choose that file in your game folder. The path is remembered.
3. **My alternates**: tick the alternate recipes you have unlocked.
4. **Plan**: pick a target (its standard rate per machine shows below it), then either "I want X / min" or "I have these resources". Pick your miner, belt and pipe tiers. Press **Calculate**.
5. Optional: under **Your nodes**, enter the pure, normal and impure nodes you found per ore. Without them, normal nodes are assumed.
6. Change the recipe of any step from its dropdown. Press **Save as plan**.
7. **Blueprint**: every machine, with straight belts between them. Scroll to zoom, drag to move. Click a machine to tick it built.

## Settings files

All in `%APPDATA%\com.spardutti.satisfactory-overlay\`:

- `config.json`: recipe file path and owned alternates.
- `raw_resources.json`: items treated as raw (mined or gathered). Created on first start.
- `extractors.json`: miner rates per tier and node purity factors.
- `logistics.json`: belt and pipe speeds per tier, used to split machines across belts.
- `plans.json`: saved plans, block positions and ticked machines.

Miners are counted only for ore nodes. Fluids and gathered items show the amount per minute only.

## Development

Needs Rust, Node 20+, pnpm, and the [Tauri prerequisites](https://tauri.app/start/prerequisites/).

```sh
pnpm install
pnpm tauri dev                                   # run the app
cargo test --manifest-path src-tauri/Cargo.toml  # planner and parser tests
pnpm check                                       # frontend type check
```

Print recipes from a docs file:

```sh
cargo run --manifest-path src-tauri/Cargo.toml --example show_recipe -- <path to en-US.json> Screw
```

## Windows release build

Run on Windows: `pnpm tauri build`. The installers land in `src-tauri\target\release\bundle\`.
