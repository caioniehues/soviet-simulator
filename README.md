# Soviet Simulator

**Kind:** repository entrypoint
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

A large-scale socialist planned-economy city, infrastructure, logistics and
society simulator. The player fantasy is **THE PLANNER**: quotas from above,
scarce means below. Nothing teleports — every effect has a physical cause.
There is no game over; failure is leaner tranches, colder homes, longer queues.

As of 2026-08-22 the project was rebased off its
own Bevy prototype onto a hard fork of [Egregoria](https://github.com/Uriopass/Egregoria),
which brings a mature road/lane/intersection map model, traffic and pathfinding,
procedural buildings, terrain, a wgpu renderer, save/load and a Lua prototype
layer. See [`NOTICE.md`](NOTICE.md) for provenance and licensing.

The Bevy track (M1 through B8, ~23k LOC) is preserved on the
`bevy-track-archive` branch and tag `bevy-track-final`. Its status document is
[`docs/archive/bevy-track/README.md`](docs/archive/bevy-track/README.md).

## Licence

**GPL-3.0**, inherited from Egregoria. See [`LICENSE`](LICENSE) and
[`NOTICE.md`](NOTICE.md).

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

## Building

Requires the Rust toolchain and Git LFS for assets.

```bash
cargo run --release
```

On Ubuntu/Debian: `sudo apt-get install libasound2-dev libudev-dev pkg-config libx11-dev`.
The `--release` flag is not optional — a debug build is unplayably slow.

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

Start with [`docs/index.md`](docs/index.md). See the
[documentation authority map](docs/meta/document-authority.md) before relying on a document.
