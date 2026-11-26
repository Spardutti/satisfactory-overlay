# Changelog

## v0.2.0

### Breaking Changes
- The always-on-top overlay is gone. You now tick built machines right in the blueprint (#6).

### Added
- A full-screen blueprint of each saved plan, with blocks you can drag around (#3).
- Belt splits, build cards and the standard rate per machine in every plan (#2).
- List the nodes you found for each ore, and the plan uses them for its miners (#4).
- One Blueprints screen: make a new blueprint, save it, and land in its blueprint (#7).
- An ↑100% button on a slow miner grows the whole plan so that miner runs full (#7).
- Edit numbers and Untick all buttons in the blueprint (#7).
- Click a row in the Blueprints list to open it (#8).

### Changed
- The plan form is a set of numbered steps, and new plans get short names like "AI Expansion Server - 60" (#7).

### Fixed
- Miners use their real speed, and the view stays put while you edit nodes (#5).

## v0.1.0

### Added
- Reads recipes, items and buildings from the game's own recipe file, and finds it in the usual Steam and Epic folders.
- Plans the machines for a target amount per minute, in build order, with the clock of the last machine.
- Finds the most you can make from the raw resources you have, and names the resource that limits it.
- Counts the miners you need for your miner tier and node purity.
- Lets you tick the alternate recipes you own and pick a recipe for each step.
- Saves several plans as build checklists and remembers what you built.
- Shows the active plan in an always-on-top overlay with click-through, show/hide and mark-next hotkeys.
