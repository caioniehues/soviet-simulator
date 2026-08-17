# Wayfinder Brief — The Road to 1.0

**Purpose:** the complete input package for a `/wayfinder` session charting the
full end-to-end, finished, polished game. Wayfinder produces *decisions, not
deliverables*; this brief exists so every decision ticket starts from evidence
instead of vibes. Written 2026-08-17, immediately after G1 code-complete.

**How to use:** open a fresh session, run `/wayfinder`, and point it here.
Section 7 lists the candidate decision tickets; sections 1–6 are the evidence
they draw on; section 8 is the reference index. Bring the G1 playtest notes
(see §6) — they outrank everything else in this file.

---

## 1. Identity (settled — do not re-litigate)

From `docs/vision/session-2026-08-16.md` §0 and the 2026-08-17 grilling:

> A large-scale **socialist planned-economy** city, infrastructure, logistics
> and society simulator: CS2's authoring/legibility/scale + W&R's physical
> causality (real resources, real construction, real labour, real logistics)
> + Dwarf Fortress's persistent individual identities.

- **Player fantasy: THE PLANNER.** Quotas from above, scarce means below.
  Queue-as-demand, allocation-not-markets, brownout-before-blackout as policy.
- **The one rule:** nothing teleports; every effect has a physical cause.
  Fiat only as an explicitly marked bootstrap (`PlacePrebuilt`, infinite
  treasury in legacy bins).
- **Never game over:** failure = leaner tranches, colder homes, longer
  queues — pressure, not a rage-quit wall.
- Money is the *foreign* rouble at the border (customs); internally the
  economy stays planned. Decided twice (grilling + "option 3" revision).

## 2. What is actually built (as of 59b5f86, 115 tests, 7 bench gates)

Milestones M1–M3, B4–B8, P1, G1 (playtest pending). Concretely:

| Domain | Shipped mechanics |
|---|---|
| Roads/traffic | Spline roads → unified lane graph, incremental recompile; async A* PathService; congestion pricing; car-following + junction entry; stall→reroute→StallBoard; bezier curves. 10k vehicles @ 1.1 ms |
| Logistics | Band-driven dispatcher (buckets not offers), distance-weighted matching, round-robin amortisation, finite depot fleets, dock rates, shuttle sugar |
| Citizens/labour | 50k real citizens @ 0.57 ms; households, housing queue + weighted allocation + doubling; fission/couples; commutes, attendance, needs (food/rest/wellbeing) with rest-cap stack (overcrowded 0.6 / cold 0.65 / dark 0.75) |
| Construction | Phased sites (earthworks→structure→finishing), real material bills, machine fleets w/ skill throughput, named stalls, demolition+eviction |
| Transit | Bus lines/stops/depot buses, walk→queue→ride trips, transit-extended labour catchment |
| Utilities | One shared pool solver (union-find + priority classes): power (Housing>Industry brownout), water⇄sewage cycle, district heating on a 12-day climate sinusoid; Liebig gates on production |
| The Plan (G1) | Quota periods, fulfillment-scaled rouble tranches, Treasury gating all purchases + recruitment, customs border (exports sold, vehicles drive in), authored 5-period First Plan, mouse toolbar, fullscreen ledger, save v5 |
| Zoning | Districts as siting constraint + PLAN dashboard (demand surfaced as information, wired to no spawner) |
| Infra | SimTick substep clock (speed-invariant), stable u64 ids, save/load (custom serde columns, v5), 7 bench gates in CI discipline, per-milestone verified capture videos |

**Resources: 3** (Coal, Gravel, Goods). **Building kinds: 14.**
**Art:** P1 only (procedural meshes + CC0 textures; zero spend). **Audio: none.**
**Terrain: a flat plane.** **Game shell: none** (no menu, no settings, no
new-game flow; the binary boots straight into the sandbox).

## 3. Spec coverage (the plan of record vs. reality)

All 23 specs in `spec/` are "draft model (grounded in research), Phase 1" —
evidence-graded (CONFIRMED/OBSERVED/INFERRED/SPECULATIVE/OURS) against W&R
ini data and CS1 decompiled code. Consumed so far: roads, traffic,
pathfinding, logistics, vehicles (stage 1), construction, households,
citizens (stage 1), needs (stage 1), zoning, electricity (stage 1), water,
sewage, heating (stages 1–2), production (stages 1–2), resources (stages
1–2), trade (small slice via G1 customs).

**Specced, unbuilt:** education, healthcare, waste, crime (B9/B11); trade
stages 2–3, vehicles stage 3 / fuel, resources stage 3 (B10); needs stage 2
(wants), citizens stage 2 / demographics, households stage 3.

## 4. Gap analysis — three layers

### 4a. Specced but unbuilt (already on the ladder)
B9 services (education enrolment→qualification, healthcare loop, waste loop,
demographics: birth/ageing/death — issues #69–#74 filed, paused), B10 border
(dual currency, era calendar from 1917, vehicle manufacture, fuel as
commodity, voltage tiers, resource imports), B11 crime (militia, courts,
prison — spec exists), plus deferred fragments logged in issues: electric
heating fallback, water quality grades, CHP.

### 4b. In W&R 1.0 but neither specced nor built
Measured from the actual install (`media_soviet/buildings_types`: ~80
building types, 45 resources — full lists in §8):

- **Transport modes:** rail (engines, depots, cargo/passenger stations, rail
  construction offices), ships/docks, airplanes/airports, helicopters,
  forklifts + container facilities, pipelines, cableways. We have road only.
- **Resource tree:** 45 vs our 3 — steel/iron/oil/fuel chains, agriculture
  (farms, fields, livestock, food), construction materials (concrete,
  panels, bricks, boards), consumer goods (clothes, electronics, alcohol,
  meat), nuclear. Shops/pubs currently abstracted to one pantry `Goods`.
- **Services & culture:** kindergarten, school (we have none built),
  university, hospital (specced), orphanage, church, pub, kino, sport,
  hotel/attractions/tourism, broadcast (radio/TV → loyalty), shops +
  distribution offices, city hall.
- **State apparatus:** fire stations (fires!), police/courts/prison (B11),
  secret police, monuments/loyalty.
- **Vehicle lifecycle:** gas stations, repair offices/depots, scrapyards,
  vehicle wear + replacement.
- **Grid depth:** transformers/substations/switches, heating endstations,
  water/sewage treatment quality tiers, electric import/export at border.
- **World:** terrain with height, water bodies, map variety/biomes, seasons
  as *weather* (we have the temperature sinusoid, no visuals), pollution
  (meter exists in W&R), realistic-vs-fiat construction modes, campaign
  scenarios, era-gated tech/vehicle catalogue (1917→2000s).

**Wayfinder's core scoping question: which of these are 1.0, post-1.0, or
never.** The identity ("planner under scarcity") is the razor — e.g. fires
and tourism are omit-able; rail and agriculture are probably not, W&R's
whole logistics fantasy leans on rail; broadcast/loyalty feeds the fog-rung
"political legitimacy" which is the most *ours* of all candidates.

### 4c. Game shell (in no spec, no ladder — pure fog)
Main menu / new game / load; settings (video/audio/keys); saves UX beyond
F5/F9; onboarding & objectives surfacing in-HUD; pause/date/speed UI;
notifications/event log; camera polish; performance scaling options;
localisation (at minimum EN); accessibility basics; Linux/Windows builds,
itch/Steam packaging; crash/telemetry posture; trailer. P4 "Ship Shape" is
one line for all of this — wayfinder must explode it into decisions.

## 5. Lessons from the references (grounded)

- **W&R's #1 criticism is onboarding**: "it doesn't teach you anything" —
   12 hours of YouTube to learn basics; clunky placement controls are #2.
  Our G1 authored First Plan is already the right seed: the genre's best
  onboarding is a campaign that reveals mechanics progressively, not a
  tutorial dump. Decide early that *the First Plan IS the tutorial* and
  budget UX (placement snapping, feedback, undo) as first-class scope.
- **W&R shipped 1.0 after ~5 years EA** with: realistic construction mode,
  seasons, waste, tourism, airports, traffic depth — i.e. its cut line
  still excluded plenty (no interiors, no diplomacy). Precedent: a 1.0 cut
  line is normal and survivable.
- **Genre polish bar** (review corpus): stability at scale, audio that
  disappears into the game, progressive complexity, strong visual feedback.
  Our bench-gate discipline already covers the first; audio is a zero.
- **Our own B8 lesson (now doctrine):** system rungs without felt-play rungs
  make a deep, unfelt game. The braid (B→G→P interleave) is a standing
  decision to make, not re-derive per milestone.

## 6. The playtest (do this before wayfinder)

G1's acceptance — an unscripted ~30-min session of `cargo run` — is still
open (#79). Its notes are the highest-value wayfinder input because every
polish/priority decision below is a guess until someone has played. Capture:
what confused you in minute 1, what you wanted to click and couldn't, when
you first felt pressure, when you got bored, what you wished existed.

## 7. Candidate decision tickets (the map's starting nodes)

Meta (settle first — everything hangs off these):
1. **The 1.0 cut line** — which systems ship, which are post-1.0, which are
   never. Output: a finite release charter replacing open-ended ladders.
2. **The braid** — how B (systems) / G (game-feel) / P (art+audio) rungs
   interleave; proposal: no two consecutive B-rungs without a G or P rung.
3. **"Polished" defined** — explode P4 into concrete shell scope (§4c) with
   an acceptance bar per item.
4. **Who is this for** — self/friends itch build vs. public Steam EA;
   changes localisation, settings depth, telemetry, marketing scope.

System scope calls (each: in-1.0 at what stage / post / never — evidence in
the named spec + §4b):
5. Rail (the biggest single scope item; W&R identity-adjacent).
6. Resource tree depth — 3 → how many? (production.md stages; food chain
   implies agriculture; construction materials imply steel/concrete).
7. Agriculture + food (couples to seasons already built).
8. B9 services scope (already ticketed #69–#74 — confirm/trim stages).
9. B10 border scope — era calendar 1917 start (vision doc §2) vs. simpler
   fixed-era 1.0; vehicle manufacture; fuel.
10. Terrain — flat plane vs. heightmap + water (touches roads, pathfinding,
    construction; expensive to retrofit later — decide early).
11. Pollution + environment (meter, sources, health coupling).
12. Fire / disasters — in identity? (W&R has fires; CS leans disasters).
13. Loyalty/legitimacy + broadcast/monuments (the fog rung most *ours* —
    1.0 differentiator or post-1.0 crown jewel?).
14. Tourism/hotels/attractions — likely "never" candidate; decide and close.
15. Vehicle lifecycle (fuel, wear, repair, scrapyard) — W&R texture vs.
    complexity budget.

Experience calls:
16. The campaign — how many authored plans, what each teaches, procedural
    endless mode after; First-Plan-as-tutorial doctrine.
17. UX debt ledger — placement snapping, undo, tooltips, notifications,
    inspect depth; driven by playtest notes.
18. Audio direction (P3) — scope + zero-spend vs. paid/licensed.
19. Art completion (P2/P3 catch-up) — citizens visible? day/night? seasons
    visible? asset-gen spend decision.
20. Save compatibility policy from here to 1.0 (currently: version-gated
    hard breaks each milestone).

Engineering posture (cheap to settle, expensive to drift):
21. Performance targets for 1.0 (vision doc §31 names citizen-count tiers;
    pin the shipping target and the bench-gate ladder to it).
22. Machine-kind save bug (restored excavators/cranes load as trucks) +
    a general save-soak test — ticket now regardless of map.

## 8. Reference index

**Repo:**
- `ROADMAP.md` — current ladders (B4–B11 `[x]` through B8, G1, P1–P4, fog
  rungs: rail, seasons/agriculture, legitimacy).
- `spec/*.md` — 23 evidence-graded system specs (each ends with an evidence
  log + stage ladder; trust CONFIRMED lines, challenge OURS lines).
- `docs/vision/session-2026-08-16.md` — founding vision: identity, sim
  frequencies, road two-layer law, 1917 start, performance targets (§31),
  "what 1:1 means" (§26).
- `docs/adr/0001–0006` — clock, band registry, presentation split, stable
  ids, graph authority, determinism/warmup.
- `research/bevy-ecosystem.md` — crate survey (0.19-compatible list).
- `findings.md` — repo-survey notes incl. simutrans routing constants.
- `progress.md` / `task_plan.md` — milestone history + current state.
- GitHub issues: #69–#74 (B9, paused), #75/#79 (G1 acceptance open), #80
  (customs, closed).

**Game installs (primary sources, already mined by specs — mine further per
decision):**
- W&R: `~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/`
  — `buildings_types/*.ini` (1472 files; `$TYPE_*` taxonomy, `$PRODUCTION`/
  `$CONSUMPTION` chains, `$CONNECTION_*` network grammar), plus dlc1–4
  folders (biomes etc.), campaign1/2 (scenario grammar).
- CS1: `~/.local/share/Steam/steamapps/common/Cities_Skylines/` (compiled;
  CS1 mechanics already extracted into specs/research — decompile only on
  a specific question).

**Web (grounding for §5):**
- [W&R 1.0 patch notes](https://steamdb.info/patchnotes/14772639/) ·
  [Fandom: Patch 1.0.0](https://workers-resources.fandom.com/wiki/Patch_1.0.0) ·
  [Wikipedia: W&R](https://en.wikipedia.org/wiki/Workers_%26_Resources:_Soviet_Republic)
- Criticism corpus: [Gazettely review](https://gazettely.com/2024/07/games/workers-resources-soviet-republic-review/) ·
  [Metacritic](https://www.metacritic.com/game/workers-and-resources-soviet-republic/) ·
  [Steam "How hard is it?"](https://steamcommunity.com/app/784150/discussions/0/4352247707590451997/)
- Genre polish bar: [Urbek review (KeenGamer)](https://www.keengamer.com/articles/reviews/pc-reviews/urbek-city-builder-review-a-beginners-guide-to-city-planning/) ·
  [Town to City review](https://www.neonlightsmedia.com/blog/town-to-city-review-cozy-addictive)

**Constraints wayfinder must respect:** solo dev + agent, inline work (no
subagent delegation — cost), zero-spend art unless explicitly approved,
Bevy 0.19 native, bench-gate discipline per rung, verified-capture (or
played-session) acceptance per rung.

---

## Suggested wayfinder opening prompt

> /wayfinder — chart the road from today's state (G1 code-complete) to a
> finished, polished 1.0 of soviet-simulator. Read docs/wayfinder-brief.md
> first; it inventories what's built, what's specced, what W&R ships that
> we don't, and the game-shell fog. Start from its §7 candidate decision
> list: settle the four meta-decisions (cut line, braid, polish definition,
> audience) before any system scope call. My playtest notes: [paste].
