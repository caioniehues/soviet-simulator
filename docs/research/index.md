# Research

**Kind:** index
**Authority:** research — evidence and interpretation; nothing here is scope, mechanism or completion
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Belongs here

Historical evidence about how planned economies and socialist societies actually behaved;
technical evidence about engineering models and Rust crates; observational evidence about this
repository's code at a stated commit (fact-sheets); syntheses that reconcile such evidence.

## Does not belong here

Design proposals (they cite research; they live in the simulation tree or `docs/plan/proposals/`),
specifications, task state.

## How to read a research page

Every substantial page separates **evidence** (what the source says), **confidence** (the label),
**historical scope** (which system, which period), **interpretation**, **possible mechanic** and
**scope status**. Evidence classes and their weight are in [methodology](methodology.md). A
research page never uses MUST/SHALL.

## The corpus

### Repository evidence (current-state, cited to `path:line` at a commit)

- [Wave 1 fact-sheet — economy](fact-sheets/wave1-economy.md) · [logistics](fact-sheets/wave1-logistics.md) · [substrate](fact-sheets/wave1-substrate.md) · [Wave 2 — substrate](fact-sheets/wave2-substrate.md) · [Wave 3 — corpus](fact-sheets/wave3-corpus.md) (superseded)
- [Substrate architecture map](../reference/architecture/substrate.md) — the classification layer over the fact-sheets
- [Lane E — code-gap matrix](conversation-mining-2026-08-28/E-code-gap-matrix.md) — 136 design claims against the code (2026-08-28)
- [Lane C2 — architecture vs code](conversation-mining-2026-08-28/C2-architecture-vs-code.md)

### Historical and social evidence

- [Lane A — the planned economy as a control system](conversation-mining-2026-08-28/A-economy-control-loop.md) — Kornai, Berliner, Weitzman, Gregory & Harrison, Nove
- [Lane B1 — society, households, time, housing, informal networks](conversation-mining-2026-08-28/B1-society-households.md) — Ledeneva, Andrusz, Morton, Feshbach, Gordon & Klopov, Zaslavskaya
- [Lane B2 — CIA-source verification and calibration table](conversation-mining-2026-08-28/B2-cia-sources.md) — 48 declassified documents by ID; the only sanctioned numbers
- [Lane H — game modes, campaigns, progression](conversation-mining-2026-08-28/H-game-modes-and-progression.md) — Sovnarkhoz, Kosygin, Yugoslav self-management, danwei, BAM
- [Lane G — adversarial review of the design thread](conversation-mining-2026-08-28/G-adversarial-review.md)

### Engineering evidence

- [Lane D — vehicles, traffic, utilities](conversation-mining-2026-08-28/D-vehicles-traffic-utilities.md) — IDM/MOBIL, BPR/Gawron, CTM/LTM, EPANET, SWMM, W&R reference install
- [Rust crates and architecture](engineering/rust-architecture-crates.md) — verified crate findings and the open conflicts (2026-08-28)
- [Lane C1 — Rust crates](conversation-mining-2026-08-28/C1-rust-crates.md) — the underlying survey
- [Project-fit crate survey](awesome-rust-project-fit.md) (2026-08-27)
- [Technical stack, upstream](../explanation/research/technical-stack-upstream-2026-08-24.md) (2026-08-24)

### Syntheses

- [Conversation-mining synthesis](conversation-mining-2026-08-28/SYNTHESIS.md) — the reconciliation of the GPT design thread, its consolidated bible, ten validation lanes and the code; the provenance of the product and simulation pages
- [Lane F — overlap and consolidation audit](conversation-mining-2026-08-28/F-doc-overlap-and-consolidation.md)

### Process research

- [Process review vs SwarmForge](../process/review-2026-08-26-vs-swarmforge.md) · [documentation audit 2026-08-26](../process/doc-audit-2026-08-26.md) · [corpus survey 2026-08-26](../process/doc-survey-2026-08-26.md) · [agent frameworks](../explanation/research/agent-frameworks/study-brief.md)

### Raw provenance

- [Raw sessions archive](../archive/raw-sessions/INDEX.md) — the GPT export, the consolidated bible, the mining session log. Historical; not authority.

## Related

- [Methodology](methodology.md)
- [Document authority](../meta/document-authority.md)
- [Simulation knowledge tree](../simulation/index.md) — where research becomes design, labelled
