# Soviet Simulator

**Kind:** repository entrypoint
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-05

A large-scale socialist planned-economy city, infrastructure, logistics and
society simulator. The player fantasy is **THE PLANNER**: quotas from above,
scarce means below. Nothing teleports — every effect has a physical cause.
There is no game over; failure is leaner tranches, colder homes, longer queues.

## The five binding pillars

From [`docs/plan/charter-1.0.md`](docs/plan/charter-1.0.md) — these bind every change:

| # | Pillar | One line |
|---|---|---|
| 1 | **Nothing teleports** | Goods move physically or they do not move; matching, payment, or allocation never moves stock |
| 2 | **Never game over** | Failure degrades into queues, shortages, colder homes, going without |
| 3 | **No domestic price** | Clearing is by queue, allocation, substitution, going without — never price |
| 4 | **Border money only** | The rouble is a single foreign currency, used only at physical border clearance |
| 5 | **Persistent identities** | Observable state and stable citizen IDs let the Planner catch the dishonest enterprise |

The core loop is the **dishonest enterprise**: an enterprise requests more input than its recipe
consumes, hoards the surplus, and the Planner catches it from observable state.

## Status — 2026-09-05

**Built and guarded** (105 simulation tests green, `cargo test -p simulation`):

- **Physical economy end to end.** Requests become truck dispatches
  (`ToSource → Loading → ToDestination → Unloading`); stock debits and credits only at physical
  arrivals; every deletion site records a named `Lost` sink. See
  [`docs/reference/mechanics-index.md`](docs/reference/mechanics-index.md).
- **Border settlement (ADR-0003).** Imports draw on a bounded, observable border ledger
  (`MAX_BORDER_STOCK`); export money settles at the border door, never at match time; domestic
  legs carry zero money.
- **The dishonest-enterprise loop.** `request_multiplier` over-request exists in production code,
  is visible in the HUD (trade status, hoard panel, company inspector), and is caught by
  conservation sweeps (`sentinel_journey` + `PillarLedger`).
- **Substrate.** Egregoria fork: road/lane/intersection map, traffic and pathfinding, procedural
  buildings, terrain, wgpu renderer, save/load, Lua prototype layer. Station-owned trucks with
  ranked dispatch; strategic route cache behind A*.
- **Verification culture.** Every guard is seen failing before it counts; mutation policy in
  [`docs/process/mutation-policy.md`](docs/process/mutation-policy.md); frozen replay fixtures;
  per-tick determinism hashes in every test tick.

**Not built** (charter rows awaiting ratified specs and target evidence — the re-derived corpus
tracks 21 requirements and 107 target scenarios, 0 implemented; see
[`docs/generated/roadmap.md`](docs/generated/roadmap.md)):

- Water/electricity/heating/waste as finite utility networks (water is never cargo).
- Field-cycle farming, livestock, demographics with death, education tiers, healthcare,
  landfill and incinerator.
- Planner placement tools (snapping, ghost, refusal), plans and onboarding, shell and comfort,
  presentation polish.

**Assets** (zero-spend policy per [`docs/reference/art-direction.md`](docs/reference/art-direction.md)):

| Asset | Source | Authority |
|---|---|---|
| Palette (16 fields, map geometry + UI accents) | `base_mod/colors.lua` → `simulation::colors()` | live code, parsed at startup |
| Building/zone/sprite tints | hardcoded `LinearColor::WHITE` | placeholder — no wider authority yet |
| Buildings, goods, recipes | `base_mod/` Lua declarations | map across Rust and Lua before changing |
| Procedural geometry | Egregoria procgen (`simulation/src/map/procgen/`) | fork-inherited |

## The economy model

Not a market sim with a red coat of paint. The design target is Kornai's
shortage economy as it actually existed:

- **No domestic money.** Requests clear through planned allocation, queues,
  substitution, and going without. The rouble exists only at physical border
  clearance.
- **Clearing by queue, not by price.** Domestic requests are allocated, not priced
  or traded. Excess demand becomes queue length, waiting time and empty shelves.
  Only border imports and exports use fixed per-kind prices.
- **Enterprises are not honest.** Soft budget constraints, input hoarding
  against uncertain supply, and plan bargaining. Factories inflate their input
  requests and argue their quota down; a large part of the game is seeing
  through the numbers your own enterprises report.
- **Money at the border is real.** The foreign rouble is the one hard
  constraint; internally the economy stays planned.

## Documentation map

| Start here | For |
|---|---|
| [`docs/index.md`](docs/index.md) | The front door |
| [`docs/plan/charter-1.0.md`](docs/plan/charter-1.0.md) | What 1.0 requires (binding on scope) |
| [`docs/reference/architecture/substrate.md`](docs/reference/architecture/substrate.md) | What the code actually implements (cited) |
| [`docs/process/development-cycle.md`](docs/process/development-cycle.md) | How work gets done: phases, roster, gates |
| [`docs/engineering/testing.md`](docs/engineering/testing.md) | Testing standard and guard rules |
| [`docs/generated/roadmap.md`](docs/generated/roadmap.md) | Generated status: requirements → target evidence |
| [`docs/meta/document-authority.md`](docs/meta/document-authority.md) | Which documents bind, in what order |

## Building

Requires the Rust toolchain and Git LFS for assets.

```bash
cargo run --release
```

On Ubuntu/Debian: `sudo apt-get install libasound2-dev libudev-dev pkg-config libx11-dev`.
The `--release` flag is not optional — a debug build is unplayably slow.

Simulation tests: `cargo test -p simulation` (parallel-safe, ~3 min).

## Layout

| Path | What |
|---|---|
| `simulation/` | The sim: map, pathfinding, transportation, souls, economy |
| `engine/` | wgpu renderer, terrain, PBR, LOD |
| `native_app/` | The game binary, UI (yakui + egui) |
| `prototypes/` | Lua-driven data definitions for buildings, goods, recipes |
| `geom/`, `common/` | Shared maths and utilities |
| `headless/`, `networking/` | Headless runner and multiplayer |
| `docs/` | Canonical charter, glossary, specifications, architecture, process, plans, generated status, and archive |

## Licence

**GPL-3.0**, inherited from Egregoria. See [`LICENSE`](LICENSE) and
[`NOTICE.md`](NOTICE.md).

The Bevy track (M1 through B8, ~23k LOC) is preserved on the
`bevy-track-archive` branch and tag `bevy-track-final`. Its status document is
[`docs/archive/bevy-track/README.md`](docs/archive/bevy-track/README.md).
As of 2026-08-22 the project was rebased onto a hard fork of
[Egregoria](https://github.com/Uriopass/Egregoria); see [`NOTICE.md`](NOTICE.md)
for provenance and licensing.
