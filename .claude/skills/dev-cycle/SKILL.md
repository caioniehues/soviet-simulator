---
name: dev-cycle
description: This project's iteration process — the 8 phases, the 16-agent roster, and the gates. Invoke bare as the reference front door before dispatching ANY implementation or gate agent; invoke with an argument (/dev-cycle <bd-id> or /dev-cycle new "title") to RUN the cycle for that story. Sovereign for process inside this repo.
---

Read `docs/process/development-cycle.md` fully — it is the single source of
truth for process here; this skill is only its front door and duplicates
nothing. Then follow the phase you are in. `bd` stays authoritative for task
state, the 1.0 charter for scope.

## With an argument: run the cycle

`/dev-cycle <bd-id>` — drive that story through the phases.
`/dev-cycle new "title"` — create the bd issue first (P2 unless told, traps
in `-d`, criteria in `--acceptance`), then drive it.

You are the lead. Workers execute; you orchestrate, gate, and synthesize.

1. **Locate the story in the cycle.** `bd show <id>` (description AND
   comments), then determine the last completed phase from evidence — a
   cartographer fact-sheet means Phase 0 ground exists, a diff means Phase 2
   happened, closed gate steps mean Phase 4 progress. Never restart a phase
   whose artifact already exists; resume after it.
2. **Run the remaining phases in order**, each per its section of the
   process doc, with the roster agent that owns the lane. Phase 1 (PLAN) and
   Phase 5 (DISPOSITION) are lead-only — never delegated. Phase 4 goes
   through the poured chain: `bd mol pour gate-chain --var story=<id>
   --var scope=<range>` (skip pouring if the molecule already exists).
3. **Every dispatch**: self-contained brief, the bd issue id, verification
   command, reply to `main` (never "team-lead"). Announce scale before any
   multi-agent phase. Attribution is `--author <agent-name>` on
   `bd comments add` — do NOT set `BEADS_ACTOR`; bd 1.2.2's
   `prepare-commit-msg` hook is inert and no commit has ever carried an
   `Executed-By:` trailer.
4. **Pause and ask the user at judgment points** — everywhere else, keep
   moving:
   - Phase 0: a domain ruling that sets a mechanic's design (accept/adjust);
   - Phase 5: any disposition where a gate finding is not an obvious fix
     (send-back vs accept vs file);
   - before any commit, always.
5. **End of run**: lead-written synthesis (outcome first, one fact once,
   every caveat kept), `bd close` with evidence (sha + the check output),
   stale/orphans sweep, `bd export -o .beads/issues.jsonl` before any
   commit the user approves.

A gate send-back loops the story to Phase 2 with the findings in the new
brief; it does not need the user unless it changes scope or cost.
