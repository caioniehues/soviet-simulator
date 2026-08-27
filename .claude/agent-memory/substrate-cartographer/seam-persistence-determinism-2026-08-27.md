---
name: seam-persistence-determinism-2026-08-27
description: Save/load + determinism seam mapped at 8531d3c — substrate map's "determinism absent" row is letter-wrong (a two-run check EXISTS but cannot fail); multiplayer lockstep is in networking/, NOT simulation/src/multiplayer/
metadata:
  type: project
---

Verified 2026-08-27 against commit **8531d3c + dirty**. The working tree HAS uncommitted changes in
code crates (`common/src/scroll.rs`, `native_app/src/uiworld.rs`, `simulation/src/economy/market.rs`
+ `mod.rs`, `map_dynamic/mod.rs` + `router.rs`, `souls/goods_company.rs`, `tests/scenarios/*`) from
sibling lenses working concurrently. **None of the files this fact-sheet cites are among them** —
`simulation/src/lib.rs`, `world.rs`, `init.rs`, `rerun.rs`, `utils/replay.rs`, `tests/mod.rs`,
`tests/test_iso.rs`, `common/src/saveload.rs`, `networking/**`, `native_app/src/network.rs` are all
clean. The 7.65s test run happened against this dirty tree.
LSP tool was **disabled for the session**; read via Read/Grep. The `lsp-first-read-guard` relent
threshold is `n > RELENT_AFTER` with `RELENT_AFTER = 2` — the **third** attempt on a file passes,
not the second (see [[gotcha-lsp-guard-starves-workers]], which implies the second).

## Claim falsified: "no check proves repeat-run determinism"

Substrate map `docs/reference/architecture/substrate.md:37` says *"No cited check proves repeat-run
determinism; the current helper proves serialization round-trip stability only"* — status **Absent**.

**Letter wrong, spirit right.** `simulation/src/tests/test_iso.rs:241-306`
(`test_world_survives_serde`) builds **two** sims from one replay (`:256-257`) and compares them with
`is_equal` at 9 checkpoints across 10,000 ticks. Observed: `cargo test -p simulation
test_world_survives_serde` passes in **7.65s**, printing 9 `--- tick` lines. `world_replay.json` has
`last_tick_recorded: 10000`, 91 commands, max command tick 3048. So a repeat-run comparison *does*
exist and *does* run.

**But it cannot fail.** All three mismatch branches (`:276`, `:284`, `:292`) end in `continue 'main`
after `check_size = check_size / 2`; 1024 halves to 0 in ≤11 restarts, then `if check_size == 0 {
break }` at `:253` exits and the test returns `ok`. There is **no `assert!`/`panic!` in the loop
body** — the only panics are `.unwrap()` on encode/decode. A divergence yields printlns plus
`world`/`world2` dumps in cwd, and a **green test**.

In-repo counterexample proving the shape is not universal: `quickcheck_map_ser` fails properly via
`TestResult::error` at `:233`.

Correct verdict: **mechanism PRESENT, proof ABSENT.** Do not let a brief restate this as
"determinism is tested".

## Claim falsified: "the multiplayer/ determinism machinery"

`simulation/src/multiplayer/` is **53 lines of chat only** (`mod.rs` 9 lines = `MultiplayerState {
chat }`; `chat.rs` 44 lines). **No lockstep, no frames, no desync check.**

The real netcode is the workspace crate **`networking/`** (1137 lines: `authent.rs`, `catchup.rs`,
`worldsend.rs`, `ring.rs`, `connections.rs`, `packets.rs`, `connection_client.rs`, plus `client/`
and `server/`). `native_app/Cargo.toml:13` has it `optional=true`; `:36` gates it behind feature
`multiplayer`, which is **not in `default = []`** (`:34`). `native_app/src/network.rs` splits
`mod inner` at `:16` (`cfg(not(feature))`) vs `:101` (`cfg(feature)`).

**It is live, not rot:** `cargo check -p native_app --features multiplayer` finishes clean
(warnings only, incl. the known `static mut` at `native_app/src/init.rs:101`).

`networking::Client<Simulation, WorldCommands>` (`network.rs:118-119`) — the netcode is generic over
the **same `Serialize`/`Deserialize` impls the save file uses**. `worldsend.rs` ships a serialized
world; `catchup.rs` replays inputs. Save and netcode are the same two mechanisms implemented twice.

## Silent default on decode failure — quantified

`simulation/src/init.rs:233-240`: the `load` closure matches `E::decode::<T>(&data)`; on `Err` it
calls `log::error!` and **returns**, leaving the init-time default resource in place. `Deserialize
for Simulation` (`lib.rs:428-439`) then returns `Ok(sim)`. The caller cannot learn the load
half-failed.

**16 resources cross the seam** (`simoptions, electricity_flow, market, ecostats, multiplayer_state,
random_vehicles, map, train_reservations, government, pmanagement, binfos, game_time,
transport_grid, randprovider, dispatcher, replay`). **Exactly 1 (`map`) has a post-load guard**:
`lib.rs:289` returns `None` when `environment.size().0 == 0`. A further 6 are
`register_resource_noserialize` and never cross the seam at all.

## Version gate ignores the patch field

`lib.rs:404-415`, `VERSION = "0.6.1"` (`VERSION` file, via `include_str!` at `lib.rs:113`):

```rust
if cur[0] != deser[0] || (cur[0] == "0" && cur[1] != deser[1]) { log::warn!(...) }
```

Computed: save `0.6.0` → no warn. `0.6.9` → no warn. `0.5.1`, `0.7.0`, `1.6.1` → warn. It **only
warns, never refuses**. Charter promises released-save compatibility from the 1.0 RC — once major
hits 1, the `"0"` branch dies and only **major** bumps warn at all.

## PRESENT-BUT-DEAD

`simulation/src/rerun.rs` — 48 lines, **entirely inside one `/* */` block** (`:2`-`:48`), file also
`#![allow(unused)]`. Sole call site commented out at `init.rs:34` (`//crate::rerun::init_rerun();`).
Deletion test passes cleanly: nothing reappears anywhere.

## Traps

- `is_equal` (`lib.rs:224-227`) writes `{name}_a.json` / `{name}_b.json` into **cwd** on mismatch —
  a production API doing filesystem writes as a side effect of a comparison.
- `Encoder::filename` (`common/src/saveload.rs:49-51`) hardcodes cwd-relative `world/{name}.{ext}`;
  `save_silent` does `std::fs::create_dir("world")`. No save-directory seam exists.
- `test_iso.rs:301` `std::mem::swap(&mut deser, &mut sim2)` — later checkpoints compare
  *continued-from-deserialized* state against a fresh run. That is **load-then-continue
  determinism**, a stronger and more valuable property than round-trip, and it is what the test
  would prove if it could fail.
- Two real adapters at the save seam (`headless/src/main.rs:39,74` and `native_app`) make it a real
  seam, not hypothetical.

Related: [[false-claims-failure-inventory]], [[seam-simwide-structure-2026-08-27]].
