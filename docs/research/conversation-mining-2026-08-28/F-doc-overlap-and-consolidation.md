# Lane F — Overlap, contradiction and consolidation audit

**Kind:** research
**Authority:** research
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03
**Source:** GPT conversation export `gpt-vision-export-2026-08-28.md` (2026-08-28), validated against code paths cited inline; synthesis 2026-08-28

## 0. Summary (top ten findings)

1. **F-CONTRADICTS-01**: The export's "250,000 persistent citizen identities at 60 fps" as a "binding 1.0 ambition" (line 34) vs the cancelled `sov-1ae` 250k benchmark lane (2026-08-27; `.beads/issues.jsonl`). The charter *does* state 250k at 60 fps (charter:56-57) but names no benchmarks; the benchmark lane that would have proven it was cancelled and its WIP branch never merged. The export treats 250k as an established contract; the repo treats it as a target with no bench gate.

2. **F-CONTRADICTS-02**: The export's "border roubles" and "dual-circuit money" language (lines 26-27: "Border roubles exist only for foreign trade/customs") vs the charter and glossary's "single rouble" model. The charter says "The rouble is a single foreign currency used only at the border" (charter:33). The glossary says "The single foreign currency used only at a border customs clearance. Domestic clearing is not a monetary market." The export's "border roubles" plural and its reference to dual-circuit money (nal/beznal, reconstructed context line 26) is inherited from the earlier vision session, which the charter and controlled rewrite explicitly retired (STORYs 0040-0044 all `retired`, story-migration.md).

3. **F-CONTRADICTS-03**: The export's "buildings do not auto-spawn from zones" (line 29) aligns with the zoning spec (SPEC-ZONING-003: "Creating, changing, or removing intent MUST NOT spawn, activate, demolish…") BUT conflicts with the live code. The architecture review confirmed auto-lots have "ZERO production consumers" and `build_house_near` is test-only (architecture-review:auto-lot-seam). The memory note says "auto-lot NOT disabled". The conversation's claim that buildings never auto-spawn is the *design target*, not the current reality; consolidation must preserve this distinction.

4. **F-CONTRADICTS-04**: The export's ten-phase order `COMMAND → TOPOLOGY → ALLOCATION → DECISION → ROUTING → MOVEMENT → ARRIVAL → PRODUCTION → UTILITIES → ACCOUNTING` (line 41) appears nowhere in the current documentation corpus. The development-cycle.md describes an 8-phase *process* cycle (0 GROUND → 1 PLAN → 2 BUILD → 3 PROVE → 4 GATE → 5 DISPOSITION → 6 WRAP → 7 SHIP). The GOSPLAN proposal describes a 9-stage cycle. The Bevy-track ADR-0002 mentions "phase buckets" but for Bevy schedules. This ten-phase sim-tick order is a *design proposal* with no existing spec or implementation.

5. **F-CONTRADICTS-05**: The export says "Do not store a hidden `dishonest` enterprise flag; let the Planner infer strategic behavior from observable discrepancies" (line 48). This is **already codified** in SPEC-PRODUCTION-009 (production.md:49-55): "no authoritative `dishonest` flag may replace those observations." Full alignment — this is `ALREADY-EXISTS`.

6. **F-OVERLAP-01**: The "causal inspector: STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN" (line 47) has no existing specification. The resources spec mentions observability (resources.md:74-76), and the production spec requires inspectable discrepancies, but no "causal inspector" spec exists. This is a new vision-level idea.

7. **F-OVERLAP-02**: The export's "three worlds" model (Physical World / Institutional World / Planner World, lines 535-543) overlaps with the export's later "four realities" model (actual physical / reported institutional / Planner knowledge / household lived experience, lines 1049-1058). These are the same idea at two granularities. Neither exists in the current corpus. The production spec's dishonest-enterprise rule (SPEC-PRODUCTION-009) is the closest existing mechanism.

8. **F-NEW-01**: The entire "social reproduction" thesis (lines 335-516, 900-1171) — household time budgets, childcare as labor supply, housing as labor routing, monotown gameplay, mikrorayon completeness, informal networks/blat, household plots, queue burden — is substantially new. The citizens spec covers persistent identity and demographics; the households spec covers shared pantries and housing queues; the needs spec covers dwelling needs. But the *social reproduction loop* connecting them is not in any existing document.

9. **F-NEW-02**: The "planned economy as endogenous gameplay" thesis (lines 135-318) — the control-loop diagram, the self-generated shortage spiral, ratchet effect, planning credibility, slack as resilience — has no existing document. This is vision-level material. The production spec's dishonest-enterprise mechanism is the closest existing codification.

10. **F-CHARTER-SCOPE**: Of ~90 ideas in the export, roughly 35 are within 1.0 charter scope, 25 are Post-1.0 direction (game modes, organizational modes, space programme, mobilization, science cities, etc.), 15 are architecture/Rust proposals, and 15 are vision-level theses. Per the brief, the charter does not filter — but the consolidation layout must mark these boundaries.

---

## 1. Idea × document matrix

The export contains approximately 90 distinct ideas. I group them by the export's own section structure and assess each against the existing corpus.

### Section 1 — Reconstructed context (lines 10-80)

| # | Idea | Already in (doc:section) | Partially in | Contradicts (doc:line, how) | New |
|---|---|---|---|---|---|
| F-01 | Player is THE PLANNER | glossary.md: "Planner" definition | — | — | — |
| F-02 | The Plan is a sequence of quota periods on one continuous save | glossary.md: "Plan" and "Quota" definitions | — | — | — |
| F-03 | Domestic allocation is non-price-based | charter:32-33 "Domestic clearing uses queue, allocation, substitution, and going without, never price"; glossary: "Rouble" | — | — | — |
| F-04 | Border roubles exist only for foreign trade/customs | charter:33 "The rouble is a single foreign currency used only at the border"; glossary: "Rouble" | — | Export says "Border roubles" (plural) suggesting a distinct currency type; charter says "single rouble" — a framing contradiction | — |
| F-05 | Goods move physically or do not move | charter:29 pillar 1 | — | — | — |
| F-06 | Failure as queues, shortages, substitution, cold homes | charter:30-31 pillar 2 | — | — | — |
| F-07 | Buildings do not auto-spawn from zones | SPEC-ZONING-003 (zoning.md:35-37) | architecture-review:auto-lot-seam confirms code still has auto-lots (zero production consumers, test-only) | Export treats this as established fact; code has not removed auto-lots yet | — |
| F-08 | Public transit dominant; private cars from citizen needs | — | — | — | New: no transit or private-car spec exists |
| F-09 | Industrial logistics as central pleasure loop | — | — | — | New: vision-level, no doc |
| F-10 | Core rule: automate execution, not decisions | — | — | — | New: stated nowhere in docs |
| F-11 | 250,000 persistent citizen identities at 60 fps | charter:56-57 "Performance targets 250,000 citizen identities at 60 fps on the development machine" | — | Export calls it "binding 1.0 ambition"; `sov-1ae` benchmark lane CANCELLED (2026-08-27, `.beads/issues.jsonl`: "cancelled") and WIP branch never merged; perf-engineer.md:21-25 attributed bench gates to charter that names none (agent-roster-review:B3) | — |
| F-12 | Split persistent CitizenRecord from active CitizenBody | citizens.md has persistent identity (SPEC-CITIZENS-001); Bevy ADR-0004 discusses typed IDs | — | — | The specific split architecture is new |
| F-13 | Cadence bands rather than universal tick | Bevy ADR-0002 (archived) | — | — | Not in any active doc |
| F-14 | Ten-phase deterministic order (COMMAND→TOPOLOGY→…→ACCOUNTING) | — | — | — | Entirely new; no spec, no implementation |
| F-15 | Parallelize inside phases with intent buffers and merge/commit | — | — | — | New architecture proposal |
| F-16 | Decompose Market into demand, allocation, inventory, logistics, retail, border | logistics.md, trade.md, resources.md, production.md, needs.md partially decompose this | architecture-review:haul-out-of-market confirms Market fuses ledger + haul state machine | — | The specific six-way decomposition is a design proposal, partially reflected in specs |
| F-17 | Hierarchical routing and topology/traffic caches | pathfinding.md, traffic.md mention routing and congestion | — | — | Hierarchical routing is new |
| F-18 | Share topology across utilities, distinct domain physics | electricity.md, water.md, sewage.md, heating.md each specify distinct domain solvers | — | — | Partial overlap |
| F-19 | First-class causal inspector: STATUS/CAUSE/TREND/POLICY/PHYSICAL CHAIN | — | — | — | Entirely new |
| F-20 | Preserve request/receipt/consumption/surplus distinctions | production.md:49-55 (SPEC-PRODUCTION-009); resources.md:74-76 observability | — | — | Already exists in spec form |
| F-21 | Do not store hidden dishonest flag | production.md:54 "no authoritative `dishonest` flag may replace those observations" | — | — | Already exists verbatim |
| F-22 | National Projects as temporary distortions | — | — | — | New; charter does not mention National Projects |
| F-23 | Space exploration as industrial/logistics national project | — | — | — | New; not in charter scope |
| F-24 | Mobilization/war as home-front economy | — | — | — | New; not in charter scope |

### Section 2a — Planned economy as endogenous gameplay (lines 85-318)

| # | Idea | Already in | Partially in | Contradicts | New |
|---|---|---|---|---|---|
| F-25 | "Causal distinctness" as the resource-splitting rule | — | resources.md has 15+1 resource catalogue but no splitting rule | — | New design principle |
| F-26 | Planning deforms physical systems | — | production.md SPEC-PRODUCTION-009 captures the enterprise side | — | The thesis statement is new |
| F-27 | The control loop: Plan→enterprises→logistics→results→reports→Plan | — | — | — | New; no diagram or cycle doc exists |
| F-28 | Reported need ≠ true need; enterprises request extra | production.md:49-55 (SPEC-PRODUCTION-009) | — | — | Mechanism exists; the exposition is new |
| F-29 | Self-generated shortage spiral | — | — | — | New |
| F-30 | Electronics/space cascade example | — | — | — | New; illustrative, not mechanism |
| F-31 | "Priority cannot solve scarcity, only decides where scarcity appears" | — | — | — | New design principle |
| F-32 | Freight-plan stability / temporal demand bunching | — | traffic.md covers congestion but not temporal bunching | — | New |
| F-33 | Storming / shturmovshchina recursive propagation | — | — | — | New |
| F-34 | Quality degradation under storming, rework | — | — | — | New; Post-1.0 (quality lots not in charter) |
| F-35 | Ratchet effect (overfulfillment → higher quota) | — | — | — | New |
| F-36 | Planning credibility / institutional reliability memory | — | — | — | New |
| F-37 | Slack as resilience | — | — | — | New design principle |
| F-38 | Multiple reserve purposes (operating, safety, enterprise, state, project) | — | — | — | New |
| F-39 | Specialization vs resilience (local bearing workshop) | — | — | — | New |
| F-40 | Indicator design ("what you measure becomes what enterprises optimize") | — | — | — | New |
| F-41 | Construction opportunity cost | — | construction.md covers physical construction but not opportunity cost | — | New |
| F-42 | Citizens deepen the loop: labor footprint > direct workforce | — | — | — | New |
| F-43 | Material-balance UI, every aggregate clickable | — | — | — | New UI proposal |
| F-44 | Golden rule: every macro number resolves into physical/institutional state | — | — | — | New design principle |
| F-45 | Late game as coordination quality optimization | — | — | — | New vision |

### Section 2b — Socialist society / citizen simulation (lines 320-516)

| # | Idea | Already in | Partially in | Contradicts | New |
|---|---|---|---|---|---|
| F-46 | Social reproduction loop: Plan→production→household→labour→Plan | — | citizens.md, households.md, needs.md cover fragments | — | The loop is new |
| F-47 | Enterprise as miniature welfare state | — | — | — | New |
| F-48 | Social cost of industrialization (8k workers → 20k settlement) | — | — | — | New |
| F-49 | Monotown gameplay | — | — | — | New; a game mode proposal |
| F-50 | Housing allocation as persistent non-price queue | households.md SPEC-HOUSEHOLDS-002 through SPEC-HOUSEHOLDS-008 | — | — | Partially exists |
| F-51 | Mikrorayon completeness | — | — | — | New |
| F-52 | Time as citizen resource (sleep, work, commute, shopping, queueing…) | — | needs.md covers dwelling needs but not time budgets | — | New |
| F-53 | Household labor and childcare | — | — | — | New |
| F-54 | Household plots / dachas as food buffer | — | — | — | New |
| F-55 | Citizens adapt (substitutes, alternate stores, routes) | needs.md SPEC-NEEDS-003 "substitution" and SPEC-NEEDS-004 "going without" | — | — | Partially exists |
| F-56 | Informal networks / blat as allocation topology | — | — | — | New |
| F-57 | Non-monetary inequality (workplace, housing, geography, contacts) | — | — | — | New |
| F-58 | Labor shortage as socialist-economy characteristic | — | — | — | New |
| F-59 | Qualifications and life course (birth → school → employment → death) | citizens.md SPEC-CITIZENS-003 through SPEC-CITIZENS-005 | education.md covers education pipeline | — | Partially exists |
| F-60 | Queues as first-class scarcity objects | — | — | — | New |
| F-61 | Queue burden (human time lost to scarcity) | — | — | — | New |
| F-62 | Science cities / closed cities / priority geography | — | — | — | New; Post-1.0 game mode |
| F-63 | Migration responding to housing/jobs/services | — | — | — | New |
| F-64 | Capital dilution (unfinished construction locks stock) | — | construction.md covers construction but not capital dilution | — | New |
| F-65 | Innovation conflict (better machine vs current quota risk) | — | — | — | New; Post-1.0 |
| F-66 | Organizational modes (ministries, sovnarkhoz, self-management…) | — | — | — | New; Post-1.0 game modes |
| F-67 | 15 non-obvious future game modes | — | — | — | New; Post-1.0 |
| F-68 | Enterprise Director mode (play from subordinate perspective) | — | — | — | New; Post-1.0 game mode |
| F-69 | Citizen-to-nation causal ladder | — | — | — | New |

### Section 2c — Rust architecture research (lines 518-695)

| # | Idea | Already in | Partially in | Contradicts | New |
|---|---|---|---|---|---|
| F-70 | Three worlds (physical/institutional/planner) with module privacy | — | production.md:54 captures the dishonest-enterprise observation gap | — | New architecture |
| F-71 | Type system as physics enforcement (typed IDs, unit newtypes) | Bevy ADR-0004 (archived) discusses typed IDs | — | — | Partially new |
| F-72 | SoA citizens (CitizenStore columnar storage) | — | — | — | New architecture |
| F-73 | Temporal LOD (stable things sleep, wake on change) | — | — | — | New; matches export line 1169 |
| F-74 | Deterministic event calendar (timing wheel) | — | — | — | New |
| F-75 | Semantic LOD (aggregate until causal distinction matters) | — | resources.md has 15+1 resources but no semantic LOD rule | — | New |
| F-76 | Fixed resource arrays (dense, not hash maps) | — | — | — | New |
| F-77 | Integer/fixed-point authoritative state | — | — | — | New |
| F-78 | Deterministic parallelism (compute→intents→merge→sort→commit) | — | — | — | New |
| F-79 | Typed system contexts (narrow capability instead of &mut Simulation) | — | architecture-review:uiworld-any-bag notes UiWorld's 37-registration bag | — | New |
| F-80 | Keyed randomness (stable keys from seed+domain+entity) | — | — | — | New |
| F-81 | Bitset society (cohort queries as bitset intersections) | — | — | — | New |
| F-82 | Incremental Observatory / Change Journal | — | — | — | New |
| F-83 | Salsa for derived world | — | awesome-rust-project-fit.md surveyed crates but not Salsa | — | New |
| F-84 | Shadow simulation / Gosplan Computer (branching headless forecasts) | — | mcp-test-harness.md proposes headless sim but for testing, not player-facing forecast | — | New |
| F-85 | LP/MILP feasibility analysis | — | — | — | New |

### Section 2d — Vehicles, traffic, utilities, physical inertia (lines 698-892)

| # | Idea | Already in | Partially in | Contradicts | New |
|---|---|---|---|---|---|
| F-86 | Different inertia per network (mechanical, queue, braking, pressure, gravity, thermal, electric, linepack) | electricity.md, water.md, sewage.md, heating.md each cover their domain | — | — | The unifying "inertia" framing is new |
| F-87 | Lane-constrained vehicle physics (not rigid-body) | vehicles.md SPEC-VEHICLES-001-006 | traffic.md covers IDM-style following | — | Partially exists |
| F-88 | Loaded vs empty trucks on slopes | — | — | — | New |
| F-89 | Collision as avoidance, not destruction | traffic.md SPEC-TRAFFIC-004 "collision avoidance via following distance" | — | — | Partially exists |
| F-90 | BPR/Gawron routing cost | traffic.md SPEC-TRAFFIC-007-008 (BPR volume-delay, Gawron blending) | — | — | Already exists |
| F-91 | Spillback (downstream blockage propagates upstream) | traffic.md SPEC-TRAFFIC-005 | — | — | Partially exists |
| F-92 | Hybrid micro/meso traffic | — | traffic.md covers micro only | — | New |
| F-93 | Shift changes create passenger waves | — | — | — | New |
| F-94 | Public transport boarding/dwell/crowding/bunching | — | — | — | New; Post-1.0 (passenger rail cut) |
| F-95 | Rail: consist mass, braking, grade, track occupancy, yards | vehicles.md covers minimal freight rail | — | Export proposes much richer rail than charter's "minimal freight rail" | New scope expansion |
| F-96 | Water: EPANET-like network (nodes, pipes, pumps, tanks, pressure) | water.md SPEC-WATER-001-006 covers network water | — | — | Partially exists |
| F-97 | Sewage: gravity, backpressure, treatment | sewage.md covers sewage network | — | — | Partially exists |
| F-98 | Heating: transport delay, thermal mass | heating.md SPEC-HEATING-001-007 | — | — | Partially exists |
| F-99 | Electricity: ramp rates, min/max output, priority load shedding | electricity.md SPEC-ELECTRICITY-005 (brownout before blackout) | — | Export proposes ramp rates and startup state beyond charter scope | Partially new |
| F-100 | Gas network with linepack | — | — | — | New; charter does not mention gas |
| F-101 | Reservoirs / hydro (basin/reach mass balance) | charter:47 "reservoir-graph water, hydro dams" | — | — | Partially exists |
| F-102 | Snow/ice affecting road capacity; snow clearing as vehicle logistics | — | — | — | New |
| F-103 | Universal network reserves concept | — | — | — | New design principle |
| F-104 | Physical momentum / phase lag | — | — | — | New design principle |

### Section 2e — CIA-only citizen/society research (lines 898-1171)

| # | Idea | Already in | Partially in | Contradicts | New |
|---|---|---|---|---|---|
| F-105 | Time poverty / search-time burden from retail failure | — | needs.md covers substitution and going-without but not time | — | New |
| F-106 | Household scheduling (work, shopping, childcare, domestic, health, leisure) | — | — | — | New |
| F-107 | Childcare as labor-supply transformer | — | — | — | New |
| F-108 | Housing as labor routing (enterprise competes via housing) | households.md covers housing queue | — | — | New framing |
| F-109 | Long-term fertility / labor consequences | — | charter says "demographics including death" but not fertility | — | New; STORY-0075 retired ("birth mechanics not yet a stable contract") |
| F-110 | Private plots as adaptive production | — | — | — | New |
| F-111 | Informal economy as alternate allocation topology | — | — | — | New |
| F-112 | Sparse social graph / reciprocity | — | — | — | New |
| F-113 | Access privilege / non-monetary inequality channels | — | — | — | New |
| F-114 | Labor adaptation costs (turnover → productivity loss) | — | — | — | New |
| F-115 | Labor hoarding (surplus workers as buffer) | — | — | — | New |
| F-116 | Plan pressure enters worker life (overtime → fatigue → turnover) | — | — | — | New |
| F-117 | Health as production of future capacity | healthcare.md covers sickness and treatment | — | — | New framing |
| F-118 | Formal vs lived economy (formal plan looks good, households queue) | — | — | — | New |
| F-119 | Four realities (physical/reported/planner/household) | — | — | — | New; overlaps F-70 |
| F-120 | Citizen knowledge (not omniscient about shop inventory) | — | — | — | New |
| F-121 | Cohort expectations (generational learning) | — | — | — | New |
| F-122 | Social Reproduction Balance (district-level time accounting) | — | — | — | New |
| F-123 | Dense CitizenCore + sparse side stores | citizens.md SPEC-CITIZENS-001 covers persistent identity | — | — | New architecture |
| F-124 | Generalized reliability → reserve mechanism | — | — | — | New design principle |
| F-125 | Mature system as physical calm | — | — | — | New vision |
| F-126 | "Citizens should be agents of adaptation" / "stable things sleep; pressures wake them" | — | — | — | New design principle |

### Section 3 — Closing thesis (lines 1175-1210)

| # | Idea | Already in | Partially in | Contradicts | New |
|---|---|---|---|---|---|
| F-127 | Core sim concept: cheapest deterministic representation preserving causal distinctions | — | — | — | New design principle |
| F-128 | Core player fantasy: fragile system → calm republic capable of national projects | — | — | — | New; closest is charter identity paragraph |

---

## 2. Document taxonomy

The repo uses a controlled metadata header block on all active documents. The rules come from three sources:

### 2.1 Header block (from `docs/README.md` and `docs/explanation/research/documentation-architecture.md`)

Every active document declares:

```
**Kind:** <one of: reference | specification | explanation | decision | process | plan | generated | historical>
**Authority:** <binding | operational | reference | explanatory | historical | derived>
**Status:** <draft | active | accepted | superseded | archived>
**Owner:** <code area, process, or role>
**Last verified:** YYYY-MM-DD
```

The GOSPLAN proposal (gosplan.md §5.1) extends this to ten kinds with expanded lifecycle states:
`charter | specification | decision | fact-sheet | brief | process | explanation | generated | gate-report | handoff`

### 2.2 Templates (from `docs/templates/`)

Five templates exist:
- `decision.md` — ADR format with Kind: decision, Authority: binding
- `generated.md` — Kind: generated, Authority: derived
- `process.md` — Kind: process, Authority: operational
- `research.md` — Kind: explanation, Authority: explanatory
- `specification.md` — Kind: specification, Authority: binding

### 2.3 Controlled rewrite rules (from `docs/plan/controlled-documentation-rewrite.md`)

The rewrite firewall states (controlled-documentation-rewrite.md:37-53):

> "No old behavior claim survives without one of these inputs:
> 1. binding scope from the rewritten 1.0 charter;
> 2. current Rust, Lua, prototype, save, UI, and test evidence in a Phase 0 fact-sheet;
> 3. an explicitly labelled external observation or research source;
> 4. a newly ratified project decision;
> 5. a clearly unresolved question that makes no implementation claim."

Authority flow (controlled-documentation-rewrite.md:42-52):

```
charter + glossary + current substrate fact-sheets
                ↓
         specifications
                ↓
          requirements
                ↓
   scenarios and evidence bindings
                ↓
      generated roadmap and status
```

> "No generated file, RESUME handoff, old ADR, or archived document can establish scope or mechanism upstream."

### 2.4 Where things live

| Type | Current path | Rule |
|---|---|---|
| Vision | `docs/vision/` | **Currently empty.** GOSPLAN proposes no vision directory |
| Plan of record | `docs/plan/charter-1.0.md` | Binding scope |
| Proposals | `docs/plan/proposals/` | Advisory until ratified |
| Specifications | `docs/reference/specifications/` | Binding mechanism after ratification; all currently draft |
| Architecture | `docs/reference/architecture/substrate.md` | Current code-cited substrate |
| Research | `docs/research/` | Explanatory, snapshot |
| Explanation | `docs/explanation/` | Explanatory |
| Decisions | `docs/decisions/` | Register; no accepted ADRs exist yet |
| Templates | `docs/templates/` | Five templates |
| Archive | `docs/archive/` | Historical only; includes `raw-sessions/` for conversation transcripts |
| Process | `docs/process/` | Operational |
| Generated | `docs/generated/` | Derived, regenerate-only |
| Fact-sheets | `docs/research/fact-sheets/` | GOSPLAN proposes moving to `docs/reference/fact-sheets/` |
| Iterations | `docs/plan/iterations/` | Requirements, evidence, RESUME |

---

## 3. Proposed consolidation layout

### 3.1 The raw export → archive

**Target:** `docs/archive/raw-sessions/gpt-vision-session-2026-08-28.md`

The raw-sessions directory already holds `vision-session-2026-08-16.md` (INDEX.md documents the convention). The export should be archived there with the prefix indicating its GPT origin and date. The archive README (archive/README.md:19) says "No successor; retained only for provenance."

### 3.2 Vision-level theses → `docs/vision/`

The `docs/vision/` directory is currently empty. The export contains several vision-level theses that do not belong in specifications or proposals:

**Target file:** `docs/vision/design-pillars.md`
- **Kind:** explanation, **Authority:** explanatory, **Status:** draft, **Owner:** project lead
- Content: F-10 (automate execution not decisions), F-25 (causal distinctness), F-26 (planning deforms physics), F-31 (priority cannot solve scarcity), F-37 (slack as resilience), F-44 (every macro number resolves physically), F-103 (network reserves), F-104 (physical momentum/phase lag), F-124 (reliability→reserve), F-126 (stable things sleep), F-127 (cheapest deterministic representation), F-128 (fragile→calm fantasy)
- Why here: These are design principles that constrain specifications but are not themselves mechanisms. They are upstream of specs in authority but not binding in the charter's sense.

**Target file:** `docs/vision/social-reproduction.md`
- **Kind:** explanation, **Authority:** explanatory, **Status:** draft, **Owner:** project lead
- Content: F-46 (the loop), F-47 (enterprise welfare), F-48 (social cost), F-52 (time as resource), F-57 (non-monetary inequality), F-69 (causal ladder), F-118 (formal vs lived), F-119 (four realities), F-122 (social reproduction balance), F-125 (mature system as physical calm)
- Why here: This is a thematic thesis that drives many individual mechanisms. It is not one spec; it is the *why* behind citizens + households + needs + education + healthcare working together.

**Target file:** `docs/vision/planned-economy-loop.md`
- **Kind:** explanation, **Authority:** explanatory, **Status:** draft, **Owner:** project lead
- Content: F-27 (control loop), F-28 (reported≠true need), F-29 (shortage spiral), F-35 (ratchet effect), F-36 (planning credibility), F-40 (indicator design), F-45 (late game as coordination quality)
- Why here: The "dishonest enterprise as core loop" is in the charter identity paragraph, but the *mechanism* of how planning pressure creates physical consequences is nowhere in the docs.

### 3.3 Mechanism proposals → `docs/plan/proposals/`

Ideas that describe concrete mechanisms needing specification work:

**Target file:** `docs/plan/proposals/causal-inspector.md`
- **Kind:** proposal, **Authority:** advisory, **Status:** draft, **Owner:** project lead
- Content: F-19 (STATUS/CAUSE/TREND/POLICY/PHYSICAL CHAIN inspector), F-43 (material-balance UI with drill-down)
- Why here: This is a concrete UI/system proposal, not a vision thesis. It needs a spec if adopted.

**Target file:** `docs/plan/proposals/sim-tick-phases.md`
- **Kind:** proposal, **Authority:** advisory, **Status:** draft, **Owner:** project lead
- Content: F-14 (ten-phase order), F-15 (parallel inside phases with intent buffers), F-78 (deterministic parallelism), F-79 (typed system contexts)
- Why here: This proposes a specific execution architecture. It is more concrete than vision but not yet a decision or spec. It should explicitly note that the current sim uses `scheduler.rs` with `SeqSchedule`.

**Target file:** `docs/plan/proposals/citizen-architecture.md`
- **Kind:** proposal, **Authority:** advisory, **Status:** draft, **Owner:** project lead
- Content: F-12 (CitizenRecord/CitizenBody split), F-72 (SoA storage), F-73 (temporal LOD / wake-on-change), F-74 (event calendar), F-80 (keyed randomness), F-81 (bitset society), F-123 (dense core + sparse side stores)
- Why here: Architecture proposals for citizen representation. The citizens spec (SPEC-CITIZENS-001-007) defines behavior; these proposals define *how* to represent it.

### 3.4 Architecture research → `docs/research/`

**Target file:** `docs/research/rust-architecture-proposals-2026-08-28.md`
- **Kind:** explanation, **Authority:** explanatory, **Status:** research snapshot, **Owner:** research
- Content: F-70 (three worlds), F-71 (typed IDs), F-75 (semantic LOD), F-76 (fixed arrays), F-77 (integer/fixed-point), F-82 (change journal), F-83 (Salsa), F-84 (shadow simulation), F-85 (LP/MILP)
- Why here: These are research-grade architecture ideas, not proposals with enough detail to become specs. The `research.md` template fits.

### 3.5 Glossary terms → `docs/reference/glossary.md`

Terms to add (currently absent from glossary):

| Term | Source | Definition sketch |
|---|---|---|
| Storming / shturmovshchina | F-33 | End-period production rush that degrades quality and creates upstream demand spikes |
| Ratchet effect | F-35 | Overfulfillment raising the next period's quota, training enterprises to conceal capacity |
| Mikrorayon | F-51 | A residential district designed as a complete unit (housing + services + transit) |
| Blat | F-56 | Informal reciprocal exchange network that redistributes scarce goods |
| Capital dilution | F-64 | Physical stock locked in unfinished construction |
| Monotown | F-49 | A settlement dominated by a single enterprise |

### 3.6 Things that should become `bd` issues

These are actionable ideas that need task tracking, not document homes:

| Title | Description draft | Source |
|---|---|---|
| Spec gap: causal inspector UI | The export proposes STATUS/CAUSE/TREND/POLICY/PHYSICAL CHAIN inspector with drill-down from macro to physical state. No spec exists. Prerequisite: ratify what observability each subsystem spec owes. | F-19, F-43 |
| Spec gap: sim-tick phase order | The export proposes a ten-phase deterministic order. Current sim uses SeqSchedule with no documented phase contract. Needs: document current order, decide target order, write proposal. | F-14 |
| Spec gap: time budget for citizens | The export proposes finite household time (work, commute, shopping, queueing, childcare, etc.) as a scarce resource. Citizens spec has no time model. Needs: decide if 1.0 or Post-1.0. | F-52, F-106 |
| Doc gap: vision directory empty | `docs/vision/` is empty. The export contains substantial vision-level theses (design pillars, social reproduction, planned economy loop) that have no home. | F-127, F-128 |
| Contradiction: export "border roubles" vs charter "single rouble" | The export uses plural "border roubles" language inherited from the retired dual-circuit model. Consolidation must use the charter term. | F-04 |

### 3.7 Post-1.0 game modes → `docs/vision/` or `docs/plan/proposals/`

The export lists 15+ game mode ideas (F-67, F-68). These are clearly Post-1.0 per the charter. They belong in a vision document, not proposals (they don't propose mechanisms):

**Target file:** `docs/vision/game-modes-post-1p0.md`
- **Kind:** explanation, **Authority:** explanatory, **Status:** draft
- Content: Housing Campaign, Monotown, Science City, Shortages Amid Plenty, The Taut Plan, The Reform, Sovnarkhoz, Everyday Socialism, National Project, Frontier Corridor, Closed City, Late-System Maintenance, Self-Management, International Plan, Enterprise Director
- Note: This document must state that these are Post-1.0 direction. The charter's Post-1.0 list does not mention them but does not contradict them either (they are "proposed feature outside this charter" per charter:84).

### 3.8 Subsystem-specific ideas → extend existing specs or proposals

Several export ideas map directly onto existing specification gaps:

| Idea | Target | Action |
|---|---|---|
| F-87-89: Vehicle physics (mass, slope, jerk) | vehicles.md "Open questions" or a proposal | Extend existing draft spec |
| F-92: Hybrid micro/meso traffic | traffic.md | Add as open question or Post-1.0 direction |
| F-93: Shift changes as passenger waves | New proposal or citizens.md | Depends on 1.0 scope decision |
| F-95: Rich rail (mass, braking, yards) | vehicles.md Post-1.0 section | Charter says "minimal freight rail" |
| F-100: Gas network | New proposal | Charter does not mention gas; clearly Post-1.0 |
| F-102: Snow/ice road capacity | New proposal | Charter mentions "visible seasons" but not weather effects on traffic |

---

## 4. What the existing corpus gets wrong that the conversation gets right, and vice versa

### 4.1 Conversation gets right, corpus gets wrong or missing

1. **The dishonest enterprise as the CORE LOOP, not a feature.** The charter's identity paragraph mentions "the dishonest-enterprise loop" but treats it as one of five pillars. The export makes it *the* central gameplay mechanic from which everything else flows. The production spec (SPEC-PRODUCTION-009) codifies the no-flag rule but doesn't convey the depth of the control-loop thesis. The docs have the mechanism but not the vision.

2. **Social reproduction is missing entirely.** No document in the corpus describes the loop from Plan→production→housing/services→household life→labor force→enterprise capacity→Plan. The citizens, households, needs, education, and healthcare specs each describe fragments. The connecting thesis is absent.

3. **"Stable things sleep; pressures wake them"** is a fundamental architecture principle for achieving 250k citizens at 60 fps. No active document states it. It directly answers *how* the charter's 250k target might be achievable.

4. **The "four realities" model** (physical truth / institutional reports / planner knowledge / household experience) explains why module privacy matters. The production spec has the enterprise observation gap, but the broader principle is unstated.

### 4.2 Conversation gets wrong, corpus gets right

1. **"Border roubles" / dual-circuit language.** The export uses "border roubles" (plural) and references nal/beznal circuits. The charter, glossary, controlled rewrite, and story migration all explicitly retired the dual-circuit model. STORYs 0040-0044 are `retired` with explicit rationale. The controlled rewrite ruling states: "Domestic clearing has no money. The single rouble exists only at the border" (controlled-documentation-rewrite.md:25).

2. **250k as a delivered commitment.** The export treats "250,000 persistent citizen identities at 60 fps" as an established deliverable. The charter states it as a performance target. The benchmark lane (`sov-1ae`) was cancelled 2026-08-27. The `sov-bo3` OOM bug blocks even constructing 250k buildings. The agent-roster-review (B3) found five agents falsely attributing bench gates to the charter.

3. **"Buildings do not auto-spawn from zones" as current fact.** It is the design target (SPEC-ZONING-003), not the current implementation. The architecture review confirmed auto-lots still exist with zero production consumers. The memory note says "auto-lot NOT disabled."

4. **The ten-phase sim-tick order** is presented as an "architecture conclusion from earlier discussion." No evidence of this order exists anywhere in the codebase or documentation. The current sim uses `SeqSchedule` with a different arrangement.

5. **Scope inflation.** The export proposes gas networks, linepack, rich rail (consist mass, braking, yards), space programme logistics, mobilization economy, science cities, and 15+ game modes. Many of these are explicitly Post-1.0 or Never in the charter. The export's phrasing often presents them as settled design decisions rather than aspirational ideas.

### 4.3 Stale pointers noticed

1. **`docs/SUMMARY.md` references `docs/research/fact-sheets/wave3-corpus.md` as "(superseded)"** — correct, but the file is listed without a superseding pointer.

2. **`docs/process/development-cycle.md:57` says "The five bench gates at 250k"** for `perf-engineer`. The agent-roster-review (B3) identified this as wrong: the charter names no benchmarks, and `sov-1ae` is cancelled. This was partially fixed per the review but the development-cycle.md still references it at line 57.

3. **GOSPLAN proposal references `.planning/process-overhaul-2026-08-28/` reports.** These 14 report files are under `.planning/`, not under `docs/research/`. The GOSPLAN proposal itself is at `docs/plan/proposals/gosplan.md`. If GOSPLAN is ratified, these research reports should move to `docs/research/` or remain as `.planning/` operational files.

4. **`docs/SUMMARY.md` lists research-synthesis and architecture-review and agent-roster-review under `docs/research/` paths** but they actually live at `.planning/architecture-review-2026-08-27.md` etc. These files are referenced in the brief's reading list at `docs/research/` paths that do not exist.

5. **Vision directory is empty.** `docs/vision/` exists but contains no files. The archive `raw-sessions/vision-session-2026-08-16.md` was moved there from `docs/vision/`. No curated vision document has been written to replace it.

---

## 5. Cross-lane hooks

| What | Lane(s) that must know |
|---|---|
| The export's "one rouble" vs "border roubles" contradiction | Lane A (economy) — verify which terminology the export uses for the Kornai model |
| The export's 250k target vs cancelled benchmark | Lane E (code audit) — confirm `sov-1ae` status and `sov-bo3` blocker |
| The ten-phase sim-tick order has no substrate | Lane C1 (crates) / Lane C2 (architecture) — this is a design proposal, not an existing architecture |
| Social reproduction loop connects citizens/households/needs/education/healthcare | Lane B1 (society) / Lane B2 (CIA) — these lanes may extract the same loop |
| Auto-lot status (design target vs implementation) | Lane E (code audit) — confirm auto-lot state |
| Gas network and linepack proposed in export | Lane D (physics) — confirm charter scope excludes gas |

---

## 6. Open questions for the user

1. **Should `docs/vision/` be populated?** The directory exists and is empty. The export contains substantial vision-level content that has no other home. The GOSPLAN proposal does not mention a vision directory.

2. **Where should the export's game-mode proposals go?** They are clearly Post-1.0. Options: (a) `docs/vision/game-modes-post-1p0.md`, (b) individual `docs/plan/proposals/` files, (c) `bd` issues with `deferred` status. (a) is cheapest and most appropriate — these are aspirations, not mechanism proposals.

3. **Should the "four realities" / "three worlds" model get its own document?** It is a cross-cutting architecture principle that affects how module privacy, the inspector, and the dishonest-enterprise loop interact. It could go in `docs/vision/design-pillars.md` or `docs/reference/architecture/information-model.md`.

4. **How much of the social-reproduction thesis is 1.0 scope?** The charter says "demographics including death, two education tiers, healthcare" — that is the *existing* commitment. The export's full social-reproduction loop (household time budgets, childcare as labor supply, informal networks, generational expectations) goes well beyond this. The consolidation should mark the boundary.

---

## 7. Sources

### Files read in full
- `/home/caio/Downloads/soviet_simulator_conversation_export.md` (1,210 lines)
- `docs/README.md`, `docs/SUMMARY.md`
- `docs/plan/charter-1.0.md`
- `docs/reference/glossary.md`
- `docs/plan/controlled-documentation-rewrite.md`
- `docs/plan/documentation-migration.md`
- `docs/plan/proposals/gosplan.md`
- `docs/plan/proposals/mcp-test-harness.md`
- `docs/plan/traceability/story-migration.md` (first 100 lines; 149 rows)
- `docs/plan/iterations/RESUME.md`
- `docs/reference/specifications/README.md`
- `docs/reference/specifications/trade.md` (80 lines)
- `docs/reference/specifications/zoning.md` (80 lines)
- `docs/reference/specifications/production.md` (lines 45-74)
- `docs/reference/specifications/resources.md` (lines 65-94)
- `docs/research/awesome-rust-project-fit.md` (60 lines)
- `docs/decisions/README.md`
- `docs/process/development-cycle.md` (100 lines)
- `docs/explanation/research/documentation-architecture.md` (40 lines)
- `docs/archive/README.md`
- `docs/templates/*.md` (all five headers)
- `.planning/architecture-review-2026-08-27.md` (full, 133 lines)
- `.planning/agent-roster-review-2026-08-27.md` (100 lines)
- `.planning/research-synthesis-2026-08-27.html` (100 lines of CSS/structure)

### Files consulted via grep/search
- `.beads/issues.jsonl` — searched for `250k`, `benchmark`, `250,000`, `250000`
- All spec headers via `head -6`
- `docs/` tree searched for `auto-lot`, `dishonest`, `causal inspector`, `ten-phase`, `COMMAND.*TOPOLOGY`

### Key issue evidence
- `sov-1ae` ("Build the fixed-seed 250k benchmark contract"): CANCELLED per agent-roster-review:B3 and `.beads/issues.jsonl`
- `sov-bo3` (LAV::iter_keys OOM bug): blocks 250k construction, measured at 17.6 GB RSS
- STORYs 0040-0044: all `retired` — dual-circuit money explicitly removed
