# Draft `bd` issues from the 2026-08-28 mining pass

**Kind:** plan
**Authority:** advisory — drafts only; nothing here is filed. `bd` is the only task-state authority. File with `bd create` after the Planner chooses; put the traps in `-d`, the criteria in `--acceptance`.
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-28

| # | Priority | Title | Description (traps) | Acceptance |
|---|---|---|---|---|
| 1 | P2 | Planner can see requested vs consumed on the building inspector | `Market::requested()` is public and unread by `native_app/`. The fact-sheet ECO-SUB-005 is stale on the caller half (fixed at `0caee71`) and current on the observability half. Do NOT expose physical `capital` as if it were a report — label provenance per line (proposal: causal-inspector). | Inspector shows requested / received / consumed / on-hand / oldest open request with provenance labels; scenario asserts the model; inspected frame. |
| 2 | P2 | Make export-side border trade physical | `make_trades` ext-trade block debits seller `capital` at match time and creates no `Dispatch`; import side is physical since `sov-abs`. Trap: the export trades are pushed after the dispatch-creation loop. | A truck carries exports to the freight station; `sov_e1q_*`-style test asserts no debit before pickup; ledger identity holds. |
| 3 | P2 | Retire the domestic money gate | `Government.money` is debited for buildings and roads (`world_command.rs`) and for workers per minute (`economy/mod.rs`); pillar violation ECO-SUB-004. Trap: `price` fields in `companies.lua` and the menu-bar money display depend on it. | No domestic debit path remains; border settlement only; `scenario_0097_production_never_checks_treasury` still green; UI updated. |
| 4 | P2 | Remove auto-generated roadside lots | `map/map.rs` generates lots on road construction (MAP-SUB-002); `SPEC-ZONING-003` forbids spawning from intent. Trap: `TestCtx::build_house_near` depends on lots — use `build_house_at`. | No lot is created by road construction; tests migrated; placement remains Planner-authored. |
| 5 | P2 | Pin `egui` and `yakui` git dependencies to revisions | `Cargo.toml` has `git = …` with no `rev`; lockfile pins `d4e8966a` / `6c6982ff`; `cargo update` advances silently. Trap: `deny.toml` `sources.allow-git` must keep matching. | `rev = "<sha>"` on all eight git entries; `cargo-deny check` green; policy doc re-recorded. |
| 6 | P2 | Add a repeat-run determinism test and a portable digest | `check_determinism` proves round-trip only; `hashes()` uses `FxHasher`. Trap: the replay baseline changes when RNG call order changes — regenerate deliberately. | Two fresh `Simulation`s with identical commands produce equal digests every 25 ticks; digest is XXH3 or BLAKE3 per the open decision. |
| 7 | P3 | Correct placeholder rolling-stock speeds | `base_mod/rollingstock.lua` locomotive `max_speed` 200 m/s (720 km/h), EMU 360 m/s. Trap: `calculate_locomotive` takes the min over the consist; freight behaviour changes. | Realistic values (~30 m/s freight, ~44 m/s passenger); train scenario still green. |
| 8 | P2 | Save envelope and `SaveMigration` seam | No migration path; version mismatch warns; decode failure defaults silently. Trap: every structural proposal waits on this. | Envelope with magic/version/checksum; one no-op migration proves the seam; major mismatch rejected. |
| 9 | P3 | Keyed randomness helper and first conversion | Global `RandProvider` stream; `common::rand` is the primitive. Trap: converting `spawn_human` changes every downstream draw once. | `keyed_rand` exists; `spawn_human` converted; round-trip test green; baseline regenerated with a note. |
| 10 | P3 | Phase labels in `SeqSchedule` without reorder | Trap: any reorder changes replay hashes; label only. | Twenty systems under eleven labels in current order; per-phase timing reported; hashes unchanged. |
| 11 | P3 | Bounded junction deadlock resolution | `road.rs` `Panicking` recovery is a random wait; nose-to-nose vehicles deadlock forever. | A scenario with two opposed vehicles resolves within a bound; no phase-through. |
| 12 | P3 | Write the missing 1.0 specifications | Agriculture, terrain/geology, weather, hydrology, pollution, Plan/Quota/Tranche, authored plans, notifications, shell/save/crash, presentation/audio (charter areas with no spec). | Each as a `draft` spec from the template, registered, indexed. |
| 13 | P3 | Glossary and doc drift sweep after this pass | `doc-reality-auditor` over the new knowledge base against the code. | Findings dispositioned. |

Not drafted as issues (research or decision items): the open architectural conflicts in
[migration sequence — open decisions](../../architecture/migration-sequence.md#open-decisions);
the enterprise intent model; the active-fraction target; alcohol, propiska, blat scope.
