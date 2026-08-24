# Soviet Simulator agent guide

## Start here

1. Read `CLAUDE.md` before any work. It contains the fork reality, domain pillars, task ledger, verification command, and delivery bar.
2. Read `docs/reference/glossary.md` before naming domain concepts or changing the simulation model.
3. Read `docs/dev-cycle.md` before planning or running a multi-agent wave.
4. Read `docs/superpowers/iterations/RESUME.md` before resuming an iteration.
5. Treat `docs/plan/charter-1.0.md` as scope authority, `docs/reference/specifications/` as mechanism authority after each specification is ratified, `br` as task-state authority, and current code as substrate authority. Legacy `spec/` files are rewrite inputs, not current authority.

`docs/archive/bevy-track/ROADMAP.md` preserves the discarded Bevy-era history. It is not the plan of record.

## Non-negotiable model

- This is a Rust/Egregoria hard fork. Bevy guidance and Bevy memories are stale for this tree.
- Goods move physically; matching, payment, or allocation never teleports stock.
- Failure degrades into queues, shortages, substitution, and going without. It never ends the game.
- Domestic clearing is never price-based. Roubles exist only at the border.
- The player is the Planner; presentation reads authoritative simulation state.

## Orchestration

- Delegate Phase 0 mapping to `substrate-cartographer` plus the relevant domain advisor before a brief asserts substrate behavior.
- Keep Phase 1 planning and Phase 5 finding disposition in the lead thread.
- In Phase 2, use `sim-implementer`, `ui-implementer`, and `data-implementer` only on disjoint ownership; serialize shared files and write contracts before parallel consumers.
- Run Phase 3 `evidence-auditor`, then Phase 4 in order: `wiring-auditor`, conditional `ledger-invariant-checker`, `reviewer`, relevant domain sign-off.
- Finish substantive waves with `doc-reality-auditor` and `scribe`; use release and performance roles only at their documented gates.

Use two or three subagents for normal waves and up to five for genuinely independent read-only work. Run at most two writing agents concurrently, with disjoint ownership. Every subagent receives a bounded brief, owned files, acceptance criteria, a `br` issue when applicable, and the exact verification command.

## Verification and delivery

- Run simulation tests as `cargo test -p simulation -- --test-threads=1`; parallel runs intermittently segfault on the known `init.rs` race.
- Name what each check proves and confirm test filters execute at least one test.
- Preserve unrelated changes and never stage with `git add -A` or `git add .`.
- Stage only the four documented `.beads` files when task-ledger state changes.
- Player-facing work finishes with an inspected screenshot or 15–20 second video when `CLAUDE.md` requires visual proof.

For generated visual assets, use Codex's `imagegen` skill and confirm paid generation with the user before the first spend.
