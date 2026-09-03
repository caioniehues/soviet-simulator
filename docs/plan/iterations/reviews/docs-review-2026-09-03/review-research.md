# Review: research

Verdict: incorrect (confidence 0.99)

The documentation rewrite contains a high-impact economy-status error: external import goods now move physically, but import roubles still settle before dispatch, contrary to the border-only rule. It also leaves contradictory fact-sheet classifications, incomplete research provenance, and generated/archive authority drift that can misdirect implementation and future audits.

## Findings

### [high] Keep import roubles unsettled until border clearance
`docs/reference/architecture/substrate.md:63-65`

The new substrate row says the import half is fixed and narrows the violation to exports, but external imports still carry a nonzero `money_delta` (`simulation/src/economy/market.rs:640-653`) and `market_update` applies it immediately (`simulation/src/economy/mod.rs:103-104`) before `advance_dispatches` (`simulation/src/economy/mod.rs:124-133`). Only buyer stock waits for the physical destination (`simulation/src/economy/market.rs:1059-1063`), so the import still violates the glossary’s border-only rouble rule (`docs/reference/glossary.md:31-35,86-89`). Split stock movement from payment and retain imports as conflicting until payment moves to the border transition.

### [medium] Rewrite superseded economy paragraphs, not just append drift notes
`docs/research/fact-sheets/wave1-economy.md:41-46`

ECO-SUB-002 still states that imports credit buyer capital immediately and classifies the whole surface as a stock teleport (`docs/research/fact-sheets/wave1-economy.md:35-40`), while this added note says imports are physical (`docs/research/fact-sheets/wave1-economy.md:41-46`). ECO-SUB-005 likewise retains its test-only heading/body and unreachable classification (`docs/research/fact-sheets/wave1-economy.md:64-68`) beside a note saying production is reachable (`:69-77`), which the current caller confirms (`simulation/src/souls/goods_company.rs:22-25`). A reader following the table can therefore select already-fixed work. Replace the old paragraphs, headings, and classifications with one current statement and move the 186e snapshot into a labelled historical block.

### [medium] Refresh the needs and queue claims after the retail fixes
`docs/research/fact-sheets/wave1-economy.md:41-46`

The fact-sheet still says bread consumption only updates `last_ate` without consuming inventory (`docs/research/fact-sheets/wave1-economy.md:16-20`), but `settle_retail` now debits seller stock (`simulation/src/economy/market.rs:479-488`) and the human eat arm invokes it before updating `last_ate` (`simulation/src/souls/desire/buyfood.rs:156-164`). ECO-SUB-001 also says waiting citizens do not repost (`docs/research/fact-sheets/wave1-economy.md:25-33`), while the current state machine returns to `Empty` after an order and claim disappear (`simulation/src/souls/desire/buyfood.rs:100-115`); only unmatched non-human orders still risk being dropped (`simulation/src/economy/market.rs:626-653`). Update the current rows and split human going-without from enterprise demand loss.

### [medium] Add dated source metadata to every mining lane
`docs/research/conversation-mining-2026-08-28/A-economy-control-loop.md:1-5`

The research contract requires dated current-substrate evidence and separates evidence, confidence, scope, interpretation, possible mechanic, and status (`docs/research/methodology.md:16-18,59-61`; `docs/research/index.md:21-25`). C1 supplies source/date fields (`docs/research/conversation-mining-2026-08-28/C1-rust-crates.md:3-6`) and SYNTHESIS supplies a commit/date and source inventory (`SYNTHESIS.md:1-17`), but A, B1, B2, C2, D, E, F, G, and H start with titles/summaries and no page-level source class, code commit, access date, or research date (`A-economy-control-loop.md:1-5`; `B1-society-households.md:1-6`; `B2-cia-sources.md:1-8`; `C2-architecture-vs-code.md:1-6`; `D-vehicles-traffic-utilities.md:1-7`; `E-code-gap-matrix.md:1-8`; `F-doc-overlap-and-consolidation.md:1-5`; `G-adversarial-review.md:1-8`; `H-game-modes-and-progression.md:1-8`). Their body citations cannot establish when code was read or which evidence class a CONFIRMED label represents.

### [medium] Record the September refresh and remove archive-agent dependencies
`docs/research/conversation-mining-2026-08-28/B1-society-households.md:515-516`

The synthesis still presents the raw files and ten lane reports as a complete unchanged reconciliation (`docs/research/conversation-mining-2026-08-28/SYNTHESIS.md:13-17,378-382`), yet the working tree has a 2026-09-02 refresh of B1 and `_brief-common.md` with no update/changelog or re-verification record. The refresh points an active research source list at `docs/archive/agents-2026-09-02/settlement-modeller.md` (`docs/research/conversation-mining-2026-08-28/B1-society-households.md:500-516`) and directs future miners to an archived cartographer definition (`_brief-common.md:21-22`). Archive policy makes those files historical, not current authority (`docs/archive/README.md:8-25`), so the change is both untraceable and coupled to a retired agent roster. Record exactly what changed and whether claims were revalidated; move reusable reference-game location guidance into active methodology and remove the archived agent from B1’s evidence list.

### [low] Keep future requirements out of research pages
`docs/research/engineering/rust-architecture-crates.md:32-37`

The research index explicitly forbids `MUST/SHALL` on research pages (`docs/research/index.md:21-25`), and methodology says research may suggest a mechanic but never state it as a rule (`docs/research/methodology.md:59-61`). Nevertheless C2 labels keyed randomness `MUST-DO-FIRST` (`docs/research/conversation-mining-2026-08-28/C2-architecture-vs-code.md:235-240`), the engineering research page repeats that label (`docs/research/engineering/rust-architecture-crates.md:32-37`), and SYNTHESIS issues “Phase 0 must add”/“Phase 1 must start” directives (`SYNTHESIS.md:276-279`). These directives create a competing process/design authority. Lowercase them as recommendations or move them to `docs/process/`/a ratified specification.

### [low] Describe specifications as draft until they are ratified
`docs/archive/raw-sessions/INDEX.md:20-22`

The updated raw-session index says the charter, glossary, “ratified specifications,” and fact-sheets establish scope/mechanism (`docs/archive/raw-sessions/INDEX.md:20-22`). The authority model says every specification is currently `Status: draft` and no accepted decision exists (`docs/meta/document-authority.md:15-23`; `docs/decisions/README.md:8-20`), and the specification register says a draft cannot establish completion or override the charter (`docs/reference/specifications/README.md:8-11`). This sentence tells readers that a binding class exists when none does. Say “draft specifications describe proposed mechanism; accepted decisions and active specifications bind once accepted.”

### [medium] Stop generated evidence from claiming a stale verification date
`docs/generated/evidence/coverage.md:7-8`

The generated evidence pages still report `Last verified: 2026-08-24` (`docs/generated/evidence/coverage.md:1-10`), and the writer emits that literal on every run (`docs/plan/iterations/evidence/build_evidence.py:250-253,290-294`). The same fixed date is emitted by the roadmap writer (`docs/plan/iterations/build_roadmap.py:64-67`) and by the traceability writer (`docs/plan/iterations/requirements/build_requirements.py:469-478`), so regeneration after a source update silently produces artifacts whose freshness metadata predates their inputs. Derive the date/commit from the validated inputs or require an explicit verified-at value, and apply the same policy to all generated outputs.

### [medium] Identify the generated migration ledger and place it with generated outputs
`docs/plan/traceability/story-migration.md:1-7`

The requirements index calls `story-migration.md` a deterministic generated artifact and says `build_requirements.py --check` byte-compares it (`docs/plan/iterations/requirements/README.md:9-12,26-30`), while the writer emits it at `docs/plan/traceability/story-migration.md` (`docs/plan/iterations/requirements/build_requirements.py:465-506`). Its own header has no generator, input, or regeneration command (`docs/plan/traceability/story-migration.md:1-7`), and it sits outside the project’s stated generated tree (`docs/explanation/research/documentation-architecture.md:88-92`). Readers can edit or treat this derived ledger as an authoritative hand-written plan. Move it under `docs/generated/` or explicitly document the checked-in exception and add generator/input metadata.
