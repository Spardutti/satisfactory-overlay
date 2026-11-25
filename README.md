# Satisfactory Planner Overlay

A desktop app that plans which machines you need and shows the build list as a checklist on top of the game.

It is not a mod. It reads the recipe file the game ships with and draws a see-through window. Game updates show up on the next start.

## Important: game window mode

The overlay cannot draw over **exclusive fullscreen**. In Satisfactory, set **Window Mode → Borderless Windowed**.

## Using it

1. Start the app. It looks for `CommunityResources\Docs\en-US.json` in the usual Steam and Epic folders.
2. Not found? Press **Pick file** and choose that file in your game folder. The path is remembered.
3. **My alternates**: tick the alternate recipes you have unlocked.
4. **Plan**: pick a target (its standard rate per machine shows below it), then either "I want X / min" or "I have these resources". Pick your miner, belt and pipe tiers. Press **Calculate**.
5. Change the recipe of any step from its dropdown. Press **Save as plan**.
6. **Saved plans**: pick the active plan. The overlay shows it.

## Hotkeys

| Key | Does |
|---|---|
| `F8` | Overlay click-through on/off. Orange border = clickable. |
| `F9` | Show/hide the overlay. |
| `Ctrl+F8` | Mark the next unbuilt machine of the active plan. |

Drag the overlay by its title row while it is clickable. Size and position are remembered.

## Settings files

All in `%APPDATA%\com.spardutti.satisfactory-overlay\`:

- `config.json`: recipe file path, owned alternates, hotkeys. Edit hotkeys here, then restart.
- `raw_resources.json`: items treated as raw (mined or gathered). Created on first start.
- `extractors.json`: miner rates per tier and node purity factors.
- `logistics.json`: belt and pipe speeds per tier, used to split machines across belts.
- `plans.json`: saved plans and progress.

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
