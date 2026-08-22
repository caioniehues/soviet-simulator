#!/usr/bin/env python3
"""Generate roadmap.md from the requirements index.

Assignment is epic -> iteration, with per-story overrides for the six stories the
adversarial scope review split. Impacted scenarios are DERIVED from the committed
stories' AC `scenario:` tags — never hand-written — which is the defect reviewer 2
found in the first draft (11 of 12 iterations listed only sentinels).
"""
import re, glob, collections

REQ = 'docs/superpowers/iterations/requirements'
SENTINELS = ['JOURNEY-0001', 'SCENARIO-0009', 'SCENARIO-0015',
             'SCENARIO-0090', 'SCENARIO-0115', 'SCENARIO-0118']

# epic theme -> iteration
EPIC_ITER = {
    'Recipe machinery (already provided)': 0,
    'Hoarding & quota deception (walking skeleton)': 0,
    'Liebig bottleneck (build)': 1,
    'Resource ontology (greenfield)': 2,
    'Byproducts and waste (greenfield)': 2,
    'Policy scheduler': 3,
    'Transport-class compatibility': 3,
    'Vehicle asset lifecycle': 3,
    'Construction Process': 4,
    'Construction Agents': 4,
    'Building Lifecycle': 4,
    'Zoning & Siting': 5,
    'Road network & routing substrate': 6,
    'Emergent congestion (greenfield)': 6,
    'Stall handling as a planning signal': 6,
    'Persistent Identity & Lifecycle': 7,
    'Labour Allocation & Workplace Binding': 7,
    'Needs & Consumption': 7,
    'Households & Housing': 7,
    'Scale & Performance': 7,
    'Electricity — wired network replaces coverage': 8,
    'Weather and climate (greenfield)': 9,          # own iteration, before heating
    'Water — quality-graded pipe network': 10,
    'Sewage — the return half of the water cycle': 10,
    'Heating — district network with temperature-driven demand': 10,
    'Waste — vehicle-hauled typed circular economy': 10,
    'Education — schooling pipeline': 11,
    'Healthcare — sickness and treatment': 11,
    'Crime — deviance and the justice chain': 11,
    'Machinery and skilled labour (greenfield)': 11,
    'Two-circuit money': 12,
    'Price administration': 12,
    'Physical foreign trade': 12,
    'Foreign currency': 12,
    'Foreign trade catalogue': 12,
    'Environmental production modifiers (greenfield)': 13,
}

# per-title overrides: the split halves land where their dependencies exist
OVERRIDE = {
    'Sequence every dispatch through travel, load, travel, unload': 0,
    'Bind a building to electricity only through a declared connection point': 8,
    'Exempt electric vehicles from the fuel halt via live grid draw': 8,
    'Bind a building to district heating only through a declared connection point': 10,
    'Derive heat demand from a live outdoor temperature signal': 10,
    'Bind a building to rail only through a declared siding connection point': 13,
    'Retire vehicles by lifespan and gate them by historical availability window': 13,
    'Enforce transport-class compatibility when loading a vehicle': 3,
    'Bind a vehicle trip to a specific citizen driver': 7,
    'Couple wellbeing to work attendance and crime propensity': 11,
    'Gate production below a required water purity threshold': 10,   # the original split
    # Depth stories that the epic-theme map would otherwise drag into ITER-0003.
    # Vehicle manufacture is the load-bearing one: its AC-2 debits Government.money
    # (EPIC-035.md:77), so it cannot precede the money iteration.
    'Manufacture vehicles as a real production chain': 13,
    'Split rail vehicles into locomotive and wagon consists': 13,
    'Scrap a vehicle at end of life into recoverable materials': 13,
    'Transport passengers under the same cargo-class model as freight': 13,
    'Route cargo through medium-transfer cargo stations': 13,
}

stories = {}
for f in sorted(glob.glob(f'{REQ}/EPIC-*.md')):
    txt = open(f).read()
    epic, theme = re.match(r'# (EPIC-\d+) — (.*)', txt).groups()
    for blk in re.split(r'\n(?=## STORY-)', txt)[1:]:
        sid = re.match(r'## (STORY-\d+)', blk).group(1)
        title = (re.search(r'\*\*Title:\*\*\s*(.+)', blk) or [None, ''])[1].strip()
        scen = sorted(set(re.findall(r'scenario:`((?:SCENARIO|JOURNEY)-\d+)`', blk)))
        nac = len(re.findall(r'^- AC-\d+:', blk, re.M))
        deferred = '**Deferred:** true' in blk or 'DEFERRED to Post-1.0' in blk
        it = OVERRIDE.get(title, EPIC_ITER.get(theme))
        assert it is not None, f'no iteration for theme {theme!r} / {title!r}'
        stories[sid] = dict(sid=sid, title=title, epic=epic, theme=theme,
                            scen=scen, nac=nac, it=it, deferred=deferred)

by_it = collections.defaultdict(list)
for s in stories.values():
    if s['deferred']:
        continue
    by_it[s['it']].append(s)
for v in by_it.values():
    v.sort(key=lambda s: s['sid'])

def scen_for(it):
    out = set()
    for s in by_it[it]:
        out |= set(s['scen'])
    return sorted(out)

def nocov(it):
    return [s['sid'] for s in by_it[it] if not s['scen']]

TITLES = {
 0:'Walking skeleton', 1:'Liebig bottleneck and its legibility',
 2:'Resource ontology, byproducts and waste emission', 3:'Logistics: the physical goods network',
 4:'Construction as a physical process', 5:'Zoning, siting and the end of autospawn',
 6:'Roads, routing and emergent congestion', 7:'Citizens, households and needs',
 8:'Electricity: wire replaces coverage', 9:'Weather and climate',
 10:'Water, sewage, heating and waste', 11:'Services: education, healthcare, crime',
 12:'Money, prices and foreign trade', 13:'Remaining depth',
}
RATIONALE = {
 1:("Turns the skeleton's single-input production into the real multiplicative factor model, where "
    "the scarcest factor governs and the reason is surfaced to the player. The bottleneck readout is "
    "held here rather than in the skeleton precisely because it needs several competing factors to be "
    "worth reading. The water-quality gate is NOT here — it is split forward to ITER-0010 where water exists.",
    "Establishes the factor-gate model construction reuses wholesale in ITER-0004. Blocks nothing upstream."),
 2:("Every downstream system needs typed resources — storage class, transport class, container class, tier, "
    "perishability. Byproducts land here because a recipe that emits waste is meaningless until waste is a "
    "typed, storable resource. Note the DECLARATION half only: enforcing transport-class compatibility when "
    "loading was split forward to ITER-0003, because enforcement needs a vehicle with a cargoClass to load onto.",
    "Hard prerequisite for ITER-0003 and ITER-0010 (waste hauling)."),
 3:("Promotes the skeleton's single hard-coded truck trip into a real scheduler: policy as target stock levels, "
    "deficit-ranked dispatch, dock throughput, rate limiting, and vehicles as owned physical assets with bounded "
    "depots. Receives the transport-class ENFORCEMENT ACs split forward from ITER-0002.",
    "Blocked by ITER-0002. Blocks ITER-0004 (materials must be haulable) and ITER-0010 (waste collection reuses this dispatcher)."),
 4:("Replaces Egregoria's instant, money-priced `Building::make` with a phased project consuming a bill of "
    "quantities delivered by the ITER-0003 dispatcher. Includes the road/infrastructure phase pipeline the spec "
    "calls \"the mechanic the project is named for\". The building-prototype story keeps only its pure schema ACs — "
    "its utility connection-point AC was split out to ITER-0008/0010/0013, since \"connected only by explicit "
    "declaration\" IS the electricity rewrite.",
    "Blocked by ITER-0001 and ITER-0003. Blocks ITER-0005 and every later iteration that places a building."),
 5:("The other end of the building's life — demolition, repair, renovation, decay — plus the zoning posture that "
    "makes this emphatically not Cities:Skylines: paint creates nothing, and auto-spawning lots die outright.",
    "Blocked by ITER-0004. HAZARD: killing auto-lots breaks `TestCtx::build_house_near`, which selects from "
    "`map().lots()` — populated today only because `Lot::generate_along_road` runs live from `Map::connect` "
    "(`simulation/src/map/map.rs:719`). Migrate the harness off it first — see `sov-harness-lots-k54`."),
 6:("Largely PROVIDED substrate pinned and then extended: surface classes with a real speed payoff, congestion as "
    "an EMA-driven route cost, and stalls escalated into planner-visible bottlenecks rather than despawned vehicles.",
    "Blocked by ITER-0004 for the road-upgrade story only; the rest could run earlier if the schedule demands it."),
 7:("The demand side. Households as shared-pantry units, needs that clear by waiting, substituting or going without "
    "(the Kornai rule made falsifiable), housing by explicit queue, and planner-directed labour replacing "
    "distance-only job matching. Receives the driver-binding ACs split out of the vehicle-asset story.",
    "Blocked by ITER-0003 (goods must reach shops). The wellbeing→crime coupling was split forward to ITER-0011, "
    "where crime exists — otherwise `wellbeing` would have two competing owners."),
 8:("The only domain where we replace a working system rather than build one. Egregoria's `ElectricityCache` is a "
    "union-find over road adjacency — any building touching any road touching a producer is powered. This iteration "
    "makes that fail: no wire, no power. Brownout-before-blackout is the never-game-over rule in electrical form. "
    "Receives the building electricity connection-point AC and the electric-vehicle grid-draw exemption, both of "
    "which are only falsifiable once laid-wire connectivity exists.",
    "Blocked by ITER-0004 (wires and substations are built things). Retires the CONFLICTS verdicts across EPIC-005."),
 9:("A subsystem that does not exist at all: `grep -rniE \"weather|climate|temperature|season\" simulation/src` "
    "returns ZERO hits. It has two dependents already (temperature-driven heat demand, and the electricity fallback "
    "for unmet heat), so it gets its own scheduled iteration rather than a footnote inside a 16-story one. Must be "
    "deterministic under the fixed-seed harness and survive save/load, or it poisons every sentinel run.",
    "Blocks ITER-0010's heat-demand story. Small, but genuinely blocking — do not fold it into ITER-0010."),
 10:("The three remaining pipe networks plus vehicle-hauled waste. Receives the water-quality production gate split "
     "forward from ITER-0001, the heating connection-point AC from ITER-0004, and the live-temperature heat demand "
     "story now that ITER-0009 provides T(t).",
     "Blocked by ITER-0003 (waste hauling), ITER-0008 (pumps and treatment draw power) and ITER-0009 (temperature)."),
 11:("Staffed, fuelled service buildings whose shortages degrade rather than end the run. Includes the justice chain "
     "end to end — arrest, the staffed court that throttles sentencing, prison — and the black market that leaks state "
     "inventory under shortage. Receives the wellbeing→crime-propensity ACs split out of job matching.",
     "Blocked by ITER-0007 (there must be citizens to educate, sicken and arrest)."),
 12:("Deliberately last of the major systems. Egregoria has no household money at all, so this is entirely net-new, and "
     "the physical economy must be proven working before money is layered over it — otherwise money becomes the thing "
     "that makes the economy appear to function. An adversarial sweep of all ACs found every money mention in an "
     "earlier iteration to be a NEGATIVE assertion (\"no Money is debited\", \"queue position, never money\") — with "
     "ONE exception, since corrected: vehicle manufacture's AC-2 debits `Government.money` (EPIC-035.md:77), so that "
     "story was moved out of the logistics iteration to ITER-0013, after money exists.",
     "Blocked by ITER-0003 and ITER-0007. Per-currency loans are EXCLUDED — deferred to Post-1.0."),
 13:("Depth no earlier iteration needs: renewable output modulation, resource depletion over calendar time, vehicle "
     "manufacture, locomotive/wagon consists, scrapyard recovery, passengers as a cargo class, multi-modal cargo "
     "stations, and the rail-siding connection point. The five vehicle/transport depth stories are here by explicit "
     "override rather than by epic theme — the theme map would place them in ITER-0003, which would schedule vehicle "
     "manufacture nine iterations before the money it debits.",
     "Blocked by ITER-0003, ITER-0010, and ITER-0012 — vehicle manufacture debits `Government.money` "
     "(EPIC-035.md:77), so the money system must precede it. Nothing depends on this iteration."),
}

L = []
A = L.append
total = sum(len(v) for v in by_it.values())
A('# Roadmap\n')
A(f'{total} scheduled stories across 14 iterations, plus 1 deferred. Generated by `build_roadmap.py` — '
  'each iteration\'s impacted-scenario set is DERIVED from its committed stories\' AC `scenario:` tags, '
  'not hand-written.\n')
A('Standing rules that shape the order:')
A('- **Nothing teleports.** Physical delivery exists before anything that consumes delivered goods.')
A('- **Clearing is by queue, substitution and going without, never by price.** Money is never a gate.')
A('- **Never game over.** Every failure mode degrades; none terminates.')
A('- PROVIDED stories are verification work, not build work — cheap, and they belong early, because they pin')
A('  the substrate before we start replacing parts of it.\n')
A('**Sentinels** (re-run every iteration, in addition to each iteration\'s impacted set): '
  + ', '.join(SENTINELS) + '\n')

ws = by_it[0]
A('## Walking skeleton (ITER-0000)\n')
A('**Intent:** One mill hoards coal delivered by one truck over one road, and the planner catches it from observable state.\n')
A("**Design rationale:** The thinnest slice that proves the product exists. It exercises the full vertical — a recipe "
  "consuming inputs, a physical vehicle moving them (nothing teleports), a production ledger separating what was "
  "*requested* from what was *consumed*, and a readout making the deception legible. The dishonest enterprise is the "
  "core loop of the whole game, so the skeleton is that loop at its smallest honest size.\n")
A("Deliberately excluded, with reasons: no money or wages (hoarding is physical); no Liebig multi-factor throttling "
  "(one input suffices to show request≠consumption); no construction (the mill is placed pre-built); no utilities, "
  "services, citizens or foreign trade. Each adds machinery without adding proof.\n")
A("**Two teleport paths this iteration must fence, found by adversarial review and confirmed in the working tree:**")
A("- `simulation/src/economy/market.rs:284-296` — the infinite external partner credits any unmet buy order to the "
  "buyer *unconditionally*, and credits capital BEFORE calling `find_external`, so the mill receives coal even with no "
  "partner at all. Fenced by a JOURNEY-0001 precondition: coal and steel authored with `optout_exttrade = true`.")
A("- `simulation/src/economy/market.rs:277-279` — `*cap_buyer += trade.qty;` moves stock at MATCH time. Without an "
  "explicit AC binding transfer to the load/unload transitions, the dispatch state machine passes every AC while the "
  "truck is decorative. That AC now exists.\n")
A('**Journey scenario:** JOURNEY-0001 — A mill hoards coal and the planner catches it from observable state alone\n')
A("**Harness-first:** the first task is the behavior harness, before any feature work. It extends `TestCtx` at "
  "`simulation/src/tests/mod.rs` (NOT `test_iso.rs`, which merely consumes it) into a scenario runner: fixed "
  "construction, scenario-ID-addressable fixtures, and a sentinel runner. `TestCtx::tick()` already asserts determinism "
  "per tick via a bincode round-trip and per-key hash compare — that nearly hands us the journey's determinism "
  "observable free. Two known gaps, tracked in `sov-harness-perf-km4`: there is no seed field on `SimulationOptions`, "
  "and the per-tick round-trip will dominate a long journey.\n")
A("**Proof-seam caveat on step 5:** `TestCtx` is `pub(crate)` under `#![cfg(test)]` and cannot drive the UI panel. "
  "Step 5 is therefore proven as *sim-observable plus eyeballed panel*: assert the accessor the panel binds to is "
  "public API on `simulation`, and record the panel render as a manual observation this iteration. Not full e2e — "
  "stated rather than assumed.\n")
A('**Stories committed:**')
for s in ws:
    A(f"- {s['sid']} ({s['epic']}) — {s['title']}")
A('')
A(f"**Impacted scenarios:** {', '.join(scen_for(0)) or '—'}")
nc = nocov(0)
A(f"**Stories without planned evidence:** {', '.join(nc) if nc else 'none'}")
A('\n**Status:** pending\n')
A('## Iteration list\n')
for it in range(1, 14):
    if not by_it[it]:
        continue
    rat, look = RATIONALE[it]
    A(f'### ITER-{it:04d} — {TITLES[it]}\n')
    A(f"**Stories:** {', '.join(s['sid'] for s in by_it[it])}")
    A(f'**Rationale:** {rat}')
    A('**Status:** pending')
    A(f"**Impacted scenarios:** {', '.join(scen_for(it)) or '— none; see below'}")
    nc = nocov(it)
    A(f"**Stories without planned evidence:** {', '.join(nc) if nc else 'none'}")
    A(f'**Look-ahead check:** {look}\n')

A('## Deferred from 1.0\n')
for s in stories.values():
    if s['deferred']:
        A(f"- {s['sid']} — {s['title']}")
A('\nDual currency is Post-1.0 per `docs/charter-1.0.md:108`; the requirement is captured, not scheduled.')
A('The broader 1.0 cut is a separate open decision — see `sov-scope-cut-1p6`.')
A('\nStories carrying no planned evidence are listed per-iteration above and tracked in '
  '`sov-scenario-coverage-bt0`. They are expected to gain scenarios from `running-an-iteration`\'s '
  'evidence tasks; if an iteration closes without doing so, that debt becomes permanent.')

open('docs/superpowers/iterations/roadmap.md', 'w').write('\n'.join(L) + '\n')
print(f'scheduled {total} + {sum(1 for s in stories.values() if s["deferred"])} deferred = {len(stories)}')
for it in sorted(by_it):
    print(f'  ITER-{it:04d}: {len(by_it[it]):>2} stories, {len(scen_for(it)):>2} scenarios, {len(nocov(it)):>2} uncovered')
