# Soviet city-builder — a hard fork of Egregoria

Rust, ECS. The fork happened 2026-08-22 and **the earlier Bevy track was discarded**. Bevy is not a
dependency; its documents live only under `docs/archive/bevy-track/`. Trust current code for
substrate behavior. The repo is GPL-3.0 by inheritance, permanently.

The core loop is the **dishonest enterprise**: an enterprise requests more input than its recipe
consumes, hoards the surplus, and the player — acting as THE PLANNER — catches it from observable
state. Two design pillars constrain every change:

- **Nothing teleports.** Goods move physically or they do not move. Stock must never change hands
  at trade-match time.
- **Never game over.** Failure degrades into queues, shortages and colder homes. It never terminates.

Clearing is by queue, substitution and going without — **never by price**. Money is not a gate.

- **How work gets done: `docs/dev-cycle.md`.** Eight phases, the 15-agent roster and what each is
  for. Every phase names the failure it exists to prevent. Read it before running a wave.
- Plan of record: `docs/plan/charter-1.0.md`. It **binds on scope**. Ratified files under
  `docs/reference/specifications/` bind mechanism inside that scope; legacy `spec/` files are
  rewrite inputs only. The charter's Post-1.0 and Never lists are absolute.
- Art direction: `docs/art-direction.md`. Palette, the look, and the asset provenance table.
- Live plan: `docs/superpowers/iterations/` — `roadmap.md`, `RESUME.md` (read this first),
  `requirements/EPIC-*.md`.
- Current substrate map: `docs/reference/architecture/substrate.md`; follow its fact-sheet citations.
- Keep durable project status in `README.md`: what is built, what is left, and an asset table.
- Generate visual assets with `/asset-gen`. Confirm the spend with the user before the first paid generation.

Run the sim's tests as `cargo test -p simulation -- --test-threads=1`. Parallel runs segfault
intermittently on a pre-existing unsynchronized `static mut` race in `init.rs` — see
`sov-test-race-initfuncs-qt6`. A green parallel run proves little.

## Task tracking — `br` is the shared surface, and the only one

**Every agent can reach `br`.** It is a CLI, you have Bash, so it works from a subagent, a pane
teammate, or the main session alike. The built-in Claude task tools (`TaskCreate`/`TaskUpdate`/
`TaskList`) are **available ONLY to the main session** — a subagent asking for them gets
`No matching deferred tools found`, regardless of `CLAUDE_CODE_ENABLE_TASKS`. Verified 2026-08-23.
So never coordinate workers through them; they are the lead's and the user's dashboard.

| Layer | Where | Who writes |
|---|---|---|
| **Macro** — the goal, the why, the traps | a `br` issue | lead creates, anyone updates status |
| **Micro** — progress, findings, blockers | `br comments add` | **the worker doing the work** |
| Live session view | Claude tasks | main session only, mirrors the macro layer |

### If you are a worker

Your brief names your `br` issue id. Then:

```bash
br show <id>                     # the goal, and the traps — read the DESCRIPTION, not just the title
br update <id> --status in_progress
br comments add <id> "<what you found / where you are>" --actor <your-name>
br close <id> --reason "commit <sha>: <the check that proves it>"
```

Log a comment when you learn something the next agent would otherwise rediscover — a wrong
premise in your brief, a blocked path, a file that is not what it claims. This is how three
agents avoided repeating each other's dead ends on STORY-0149.

Close with **evidence, not a claim**: the commit sha and the command output that proves it. A
closed issue must be auditable months later.

### Conventions

- Always pass `--slug`; ids are `sov-<slug>-<hash>` and an unslugged id is unreadable in a commit.
- **P1 is for gates** — checks that stop the line. Ordinary work is P2, cleanup P3. Do not inflate.
- Put the *traps* in the description. A future agent reads the description and nothing else; a
  title cannot warn it which mistake to avoid.
- Version exactly these four, never `git add .beads/` and never `git add -A`:
  `.beads/.gitignore .beads/config.yaml .beads/issues.jsonl .beads/metadata.json`

## Delivery

Judge progress from the running game, never from a clean build: verify the structural things yourself (it loads, no errors, assets present) and let what you see drive the next iteration.

Decide from how the task is framed how to work. A task that invites collaboration — open-ended, exploratory, phrased as a direction rather than a spec — gets the live game early: checkpoint at decisions of taste, scope, or cost, and build freely in between. A task handed over as a finished brief to execute gets reasonable calls and steady progress, no blocking. Either way the result is proven, not claimed — if the user hasn't seen it running, finish with a 15–20s video of the game in action, and watch it back before you call the work done.
