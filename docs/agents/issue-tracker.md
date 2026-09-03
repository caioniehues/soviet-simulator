# Issue tracker: bd (beads)

Issues and specs for this repo live in `bd` — the beads tracker pinned at 1.2.2 with prefix `sov`.
This file maps skill operations onto `bd` commands. `CLAUDE.md` § "Task tracking" holds the
governing rules (layers, claims, evidence-on-close, versioned `.beads/` files); this file does not
override it.

## Conventions

- **Create an issue**: `bd create "<title>" -t <bug|feature|task|epic|chore|decision> -p <0-4> -d "<traps>" --acceptance "<criteria>"`.
  Multi-line bodies: `--body-file -` with a heredoc. Add `--silent` when scripting to get only the id.
- **Read an issue**: `bd show <id>` — read the description, not just the title. `bd comments list <id>` for the thread.
- **List issues**: `bd list --status open` with `-l <label>` / `--label-any` / `--exclude-label`, `-a <assignee>`, `--no-assignee`, `--parent <id>`. `--json` for machine output.
- **Ready work**: `bd ready` (open, no active blockers).
- **Comment**: `bd comments add <id> "<text>" --author <your-name>`.
- **Apply / remove labels**: `bd update <id> --add-label <l>` / `--remove-label <l>` (or `bd label add|remove`).
- **Claim**: `bd update <id> --claim` — atomic assignee + `in_progress`.
- **Close**: `bd close <id> --reason "commit <sha>: <the check that proves it>"` — evidence, not a claim.
- **After mutating tracker state**: `bd export -o .beads/issues.jsonl` before committing it.

Priorities: P1 is for gates that stop the line; ordinary work is P2; cleanup is P3.

## When a skill says "publish to the issue tracker"

Create a `bd` issue. Put traps in `-d`, acceptance criteria in `--acceptance`; a future agent reads
the description and nothing else.

## When a skill says "fetch the relevant ticket"

Run `bd show <id>` and `bd comments list <id>`.

## Wayfinding operations

Used by `/wayfinder`. The **map** is a single issue with **child** issues as tickets.

- **Map**: `bd create "<map title>" -t epic -l wayfinder:map -d "<Notes / Decisions-so-far / Fog>"`.
- **Child ticket**: `bd create "<title>" --parent <map-id> -l wayfinder:<research|prototype|grilling|task>`.
  Children inherit the parent's labels unless `--no-inherit-labels`. Once claimed, the ticket is
  assigned to the driving dev.
- **Blocking**: `bd dep <blocker-id> --blocks <blocked-id>` — note the direction. `bd dep list <id>`
  and `bd dep tree <id>` to inspect; `bd blocked` for everything currently gated.
- **Frontier query**: `bd list --parent <map-id> --ready --no-assignee` — open children with no
  active blocker and no assignee; first in map order wins.
- **Claim**: `bd update <id> --claim` — the session's first write.
- **Resolve**: `bd comments add <id> "<answer>"`, `bd close <id> --reason "<evidence>"`, then
  `bd update <map-id> --append-notes "<context pointer>"` on the map's Decisions-so-far.
