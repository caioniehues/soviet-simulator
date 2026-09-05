# Repository Guidelines

**Kind:** operational
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03
**Source:** `docs/meta/document-authority.md` operational-entry-point rule (2026-08-28); content verified 2026-09-03

## Project Overview

Soviet Simulator is a GPL-3.0 Rust city-builder and a hard fork of Egregoria. The player is **the Planner**. The simulation models physical goods, shortages, queues, and observable enterprise hoarding.

Keep these model rules intact:

- Goods move physically. Never transfer stock at matching or clearing time.
- Domestic clearing uses queues, substitution, and going without. It is never price-based.
- Failure degrades the settlement. It never ends the game.
- Roubles clear only at the border.
- Citizen, household, and enterprise identities persist for life; the Planner acts on observable
  state, never hidden ledgers.

The five binding pillars live in `docs/plan/charter-1.0.md`; this list summarizes them.

Read `CLAUDE.md` before work. Read `docs/reference/glossary.md` before naming a domain concept. For substantive work, follow `docs/process/development-cycle.md` and track it in `bd`.

## Architecture & Data Flow

The workspace has 13 crates. `native_app` is the default desktop application. `simulation` is the authoritative game state. `engine` owns the framework and rendering layer.

- Startup: `native_app/src/main.rs` initializes the engine and starts `game_loop::State`.
- Frame loop: `native_app/src/game_loop.rs` reads input and UI state, updates the map, audio, camera, and renderer, and reads simulation state through `Arc<RwLock<Simulation>>`.
- Simulation tick: `simulation/src/lib.rs` applies `WorldCommand`s, advances game time, runs the ordered schedule, then records replay state.
- System setup: `simulation/src/init.rs` registers systems and resources in their execution order.
- Headless mode: `headless/src/main.rs` initializes the same simulation behind the networking server loop.

Keep simulation mutation inside command application and scheduled systems. Preserve the separation between `World` and typed `Resources`. Do not add a second state path around this schedule.

## Key Directories

- `native_app/src/` — desktop game loop, panels, input, audio, map rendering, and network integration.
- `simulation/src/` — deterministic game state, economy, map, map dynamics, souls, transport, commands, and scheduling.
- `engine/src/` — framework, GPU rendering, assets, input, audio, and graphics infrastructure.
- `common/`, `geom/`, `prototypes/` — shared simulation support, geometry, and prototypes.
- `goryak/`, `assets_gui/`, `egui-inspect*/` — widget and UI support.
- `networking/` — network protocol and replication.
- `base_mod/` — Lua game declarations; map a feature across Rust and Lua before changing it.
- `docs/` — canonical Markdown documentation. `book/` is generated output.
- `scripts/` — repository maintenance and documentation checks.

## Development Commands

Run commands from the repository root.

```bash
# Run the playable desktop application in release mode.
cargo run --release

# Run the parallel-safe simulation suite.
cargo test -p simulation

# Run a named scenario filter and show its output (verified 2026-09-03: 1 test).
cargo test -p simulation scenario_0151 -- --nocapture

# Check documentation and build the mdBook view.
python3 scripts/check_docs.py && mdbook build

# Audit dependency policy.
cargo-deny check
```

Use `cargo test -p simulation scenario_ -- --nocapture` for the scenario set (verified 2026-09-05: 31 tests). The `sentinel` filter runs the two journey-sentinel tests added 2026-09-04 (`sentinel_journey`); confirm that every filtered command runs at least one test.

## Code Conventions & Common Patterns

- Use Rust 2021. Format with the checked-in `rustfmt.toml` settings.
- Follow existing module facades and selective re-exports. Keep crate boundaries clear.
- Prefer explicit domain names from `docs/reference/glossary.md`, such as `Request`, `Custody`, `Dispatcher`, and `Policy`.
- Put authoritative simulation state in `simulation`. Presentation reads it; it must not create a competing truth.
- Preserve deterministic order. For scheduling, randomization, or ordering changes, add repeat-run determinism coverage.
- Use synchronization only at the application boundary. Do not spread `Arc<RwLock<_>>` into simulation internals.
- Keep new dependencies compatible with `deny.toml`: crates.io or the two allowed Git sources only; no wildcard dependencies.
- Use an existing error and ownership pattern in the target crate. Do not introduce a new dependency-injection or async framework without a documented need.

## Important Files

- `CLAUDE.md` — operational rules, domain pillars, task tracking, and delivery requirements.
- `Cargo.toml` — workspace crates, default member, shared dependencies, and development profiles.
- `native_app/src/main.rs` — desktop entry point.
- `native_app/src/game_loop.rs` — interactive application state and frame integration.
- `simulation/src/lib.rs` — simulation ownership, tick semantics, commands, and schedule.
- `simulation/src/init.rs` — system and resource registration order.
- `docs/reference/architecture/substrate.md` — cited map of current implementation.
- `docs/meta/document-authority.md` — authority order for documentation.
- `docs/engineering/testing.md` — active testing standard.
- `docs/plan/iterations/RESUME.md` — iteration handoff; verify live work with `bd ready`.

## Runtime/Tooling Preferences

- Use Cargo. `native_app` is the workspace default member.
- Use release mode for playable runs.
- The workspace has no checked-in `rust-toolchain` or minimum `rust-version`; do not claim one.
- `native_app` supports optional `multiplayer` and `profile` features. `engine` keeps Yakui behind its `yakui` feature.
- Documentation uses mdBook with `mdbook-pagetoc` and `mdbook-mermaid`. Edit canonical files under `docs/`, never generated `book/` output.
- Do not commit, push, or run `bd dolt push` without direct user approval.

## Testing & QA

Simulation tests use Rust's built-in test harness under `simulation/src/tests/`. `TestCtx` is the standard fixture for scenario tests.

- Register every new scenario file in `simulation/src/tests/scenarios/mod.rs` with `mod <file>;`.
- Name evidence tests `evid_<subsystem>_<claim>`.
- Name story scenarios `scenario_<nnnn>_<behaviour>`.
- Name bead-fix tests `sov_<id>_<behaviour>`.
- Test observable behavior, not incidental arithmetic.
- Test conservation for ledger changes, idempotency for replayable transitions, and determinism for scheduling or random changes.
- Make a new guard fail before restoring the implementation. Record evidence from a non-zero test filter.
- For UI or renderer changes, inspect the running surface and provide visual evidence when `CLAUDE.md` requires it.

For documentation changes, run `python3 scripts/check_docs.py && mdbook build`. The check validates active-document links, `SUMMARY.md` targets, titles, and required metadata. Preserve archived documents; they are historical and non-authoritative.
