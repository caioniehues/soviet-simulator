## F1 — Labour-pool office for surge/temporary work is unmodeled
- classification: omission
- source: spec/citizens.md:47 (also :21, :74, :95)
- verbatim: "**Labour-pool office** (from W&R `$RESOURCE_SOURCE_WORKERS`, CONFIRMED): dispatches unassigned/temporary workers to short-lived demands without touching fixed assignments."
- what should have covered it: no story / no scenario
- why it matters: this is a named, CONFIRMED-evidence mechanism (the one W&R pattern explicitly kept) that dispatches surge labour without breaking fixed-workplace binding; no AC or scenario exercises it, so a regression that routes labour-pool dispatch through the fixed-assignment path (or drops it entirely) would pass unnoticed.

## F2 — School-as-workplace-assignment and throughput-limited education pipeline
- classification: omission
- source: spec/citizens.md:51-52
- verbatim: "**School as workplace assignment** (CS1's `SetStudentplace` trick, CONFIRMED): a student seat is a planned allocation like a job — unifies commutes, capacity, and labour-vs-study trade-offs." / "W&R's `$CITIZEN_ABLE_SERVE` throughput (kindergarten 10, school 12, university 3 — CONFIRMED) is the data shape: higher education is deliberately a slow pipeline."
- what should have covered it: no story / no scenario
- why it matters: education is explicitly modeled as capacity-limited attendance (with concrete per-tier throughput numbers), not ambient coverage; nothing in the extraction tests that a student occupies a capacity-bound seat or that the pipeline throttles by tier throughput.

## F3 — Ambient-education-field rejection has no regression proof obligation
- classification: thin-AC
- source: spec/citizens.md:52
- verbatim: "**Reject CS1's ambient education field** (coverage-granted diplomas, CONFIRMED mechanic) — violates physicality. Education requires attended capacity"
- what should have covered it: no story / no scenario
- why it matters: this is the same "reject the exploit" pattern the extraction DID cover for the pantry-teleport bug (as its own story+scenario), but the parallel education-ambient-coverage exploit gets no equivalent guard, despite being called out with the same CONFIRMED evidence rigor.

## F4 — Commute time-of-day probability curve
- classification: missing-scenario
- source: spec/citizens.md:65
- verbatim: "**Commute:** time-of-day probability curve (CS1 shape, CONFIRMED) is acceptable for v1; shift schedules are an open question."
- what should have covered it: no story / no scenario
- why it matters: commute behavior is asserted as a modeled, time-varying probability curve; no AC or scenario establishes that commute trips follow a time-of-day distribution rather than being uniformly/randomly timed.

## F5 — Coarse lifecycle stage graduation (child → student → worker → pensioner)
- classification: omission
- source: spec/citizens.md:68-70
- verbatim: "Coarse age ticks with threshold graduation (CS1 shape): child → student → worker → pensioner; death by age window widened by poor health + small accident chance"
- what should have covered it: "Progress citizen age and introduce death without deleting the household" (people.json) covers age increment and death only
- why it matters: the extracted story's ACs cover age incrementing and death, but omit the stage-transition behavior itself (child becomes student becomes worker becomes pensioner at thresholds) — a citizen that ages forever as a "child" with a job would violate the spec but pass every existing AC.

## F6 — workEfficiency(health) / workProbability(wellbeing) numeric coupling tables
- classification: omission
- source: spec/citizens.md:61
- verbatim: "CS1 CONFIRMED tables kept, made sharper by fixed workplaces: `workEfficiency(health)` 10–100% and `workProbability(wellbeing)` feed the *specific slot* in the *specific plant* — neglecting people physically reduces output"
- what should have covered it: no story / no scenario (health AC in "Model health, sickness and hospital-capacity resolution" covers only sickness freeze, not the continuous 10-100% efficiency curve)
- why it matters: this is the concrete mechanism binding citizen neglect to production output (spec/production.md consumes it) — the extraction has no AC asserting that work efficiency scales continuously with health (10-100%) or that attendance probability scales with wellbeing, only the binary "sick citizen freezes" case.

## F7 — Birth mechanics (couple + free slot, per-step chance boosted by childcare)
- classification: omission
- source: spec/households.md:54
- verbatim: "**Birth:** couple present + free member slot + both adults → per-step chance, boosted by childcare access (CS1 CONFIRMED: 1/12 → 1/8) — a services-to-demographics hook worth keeping."
- what should have covered it: no story / no scenario
- why it matters: population growth (births) is entirely absent from the extraction — the "Progress citizen age and introduce death" story covers only death/aging, not the birth side of demographic lifecycle, and the specific services-to-demographics coupling (childcare access raising birth rate 1/12→1/8) is a named CONFIRMED mechanism with no proof obligation anywhere.

## F8 — Fission and immigration as housing-queue entry sources
- classification: missing-scenario
- source: spec/households.md:33-41 (list), verbatim below from :40-41
- verbatim: "2. Adult children leaving home (CS1: slots 2–4 continuously bid to move out)." / "3. Immigrants — **recruited by the plan** (replacing CS1's demand-gated fabrication at the map edge; W&R also makes immigration player-driven)."
- what should have covered it: "Allocate housing through an explicit, player-visible queue" story/scenarios — only source #1 (new couples) and #4 (displaced households) are exercised by scenarios; sources #2 (adult-child fission) and #3 (plan-recruited immigration) have no scenario
- why it matters: the spec names exactly four queue-entry sources as a confirmed, enumerated list; two of the four are unrepresented in behavior evidence, so fission-driven or immigration-driven queue entries could silently break without any scenario catching it.

## F9 — Dwelling quality range and default-on heat/electricity requirement
- classification: omission
- source: spec/households.md:48
- verbatim: "`qualityOfLiving` authored per prefab (W&R CONFIRMED: 0.60 khrushchyovka → 1.02 village house) + default-on heat/electricity requirements (W&R CONFIRMED-by-absence: dwellings never declare them; only non-residential opts out via `$HEATING_DISABLE`)."
- what should have covered it: no story / no scenario
- why it matters: dwelling quality as a scalar input to satisfaction, and the default-on (opt-out only) heat/electricity requirement for residential buildings, are both asserted mechanisms with no AC — nothing tests that a residential building implicitly requires heat/power unless explicitly exempted, or that quality varies by prefab and feeds housingQuality need.

## F10 — Max household size cap is an undecided open question, but the extraction hard-asserts "a capped number" without resolving or flagging it
- classification: thin-AC
- source: spec/households.md:65
- verbatim: "Max household size cap — CS1 proves a cap simplifies everything; 5? 6 (three-generation flat)?"
- what should have covered it: "Introduce households as shared-pantry family units" AC-1 ("up to a capped number of member citizens")
- why it matters: the source spec leaves the cap value as an open question; the extraction's AC asserts capping exists but supplies no proof obligation tied to a specific value or to the open-question status, so the unresolved design decision is silently absorbed rather than flagged for the roadmap.

## F11 — Wants layer (leisure/culture building types, spatial/quality-weighted satisfaction) has no scenario
- classification: missing-scenario
- source: spec/needs.md:30-31
- verbatim: "### 2. Wants — improve wellbeing, absence is tolerable / Map to W&R's leisure/culture building types; satisfaction is spatial and quality-weighted (pollution lowers it, nature/water raise it — W&R `$ATTRACTIVE_FACTOR_*`, **CONFIRMED**)."
- what should have covered it: "Model per-citizen need satisfaction beyond food" AC-3 names wants/aspirations but no scenario exercises wants at all (only needs-decay and household-pantry scenarios exist)
- why it matters: wants satisfaction is explicitly spatial and quality-weighted (proximity/pollution/nature affect it), a distinct mechanism from the needs-decay model; nothing proves this weighting is implemented.

## F12 — Aspiration pressure formula and threshold-crossing → economic demand conversion
- classification: omission
- source: spec/needs.md:39-40 and :33-36
- verbatim: "Desire_car = MobilityGap + LeisureAccess + WorkAccess + FamilyNeeds\n           + StatusAspiration + ComfortPreference\n           - TransitQuality - CarCost - Scarcity"
- what should have covered it: "Model per-citizen need satisfaction beyond food" AC-3 mentions aspirations abstractly but no scenario proves threshold-crossing converts pressure into demand
- why it matters: the aspiration mechanism (chronic unmet pressure → crosses threshold → becomes real economic demand, e.g. car-buying pressure) is the spec's stated "core loop" novelty (OURS) and has zero scenario coverage of the pressure-accrual or threshold-crossing behavior.

## F13 — Loyalty / political legitimacy meta-need is entirely unrepresented
- classification: omission
- source: spec/needs.md:67
- verbatim: "add **loyalty / political legitimacy** from W&R (`$MONUMENT_GOVERNMENT_LOYALTY`, broadcast/propaganda) as a satisfaction-driven meta-need."
- what should have covered it: no story / no scenario
- why it matters: this is a named additional meta-need derived from aggregate satisfaction with its own CONFIRMED W&R source; none of the extracted stories mention loyalty/legitimacy at all, so this whole sub-domain of consequence-coupling is missing from requirements.

## F14 — Crime consequence of low wellbeing
- classification: omission
- source: spec/needs.md:65
- verbatim: "low **wellbeing** → higher crime, lower work probability (`GetCrimeRate`, `GetWorkProbability`)"
- what should have covered it: no story / no scenario
- why it matters: crime as a wellbeing-driven consequence is asserted alongside work probability (which the extraction does address via AC-3 of the labour story's wellbeing-cost framing, only for overqualification); the crime half of this CONFIRMED coupling has no proof obligation anywhere.

## F15 — Household-level vs individual-level need resolution distinction
- classification: omission
- source: spec/needs.md:82
- verbatim: "Household vs individual: which needs resolve at household level (housing, appliances, car) vs individual (food, health, education)? Lean household for durables (matches CS1's household abstraction), individual for consumables."
- what should have covered it: "Model per-citizen need satisfaction beyond food" (frames all of CitizenNeeds as per-citizen) vs "Introduce households as shared-pantry family units" (only covers pantry/food at household level)
- why it matters: the spec draws an explicit household-vs-individual split across need categories (durables like housing/appliances/car at household level, consumables like food/health/education at individual level) but the extraction's CitizenNeeds AC-1 puts housingSpace and housingQuality on the per-citizen struct with no AC distinguishing which needs are household-aggregated vs individually tracked — the household/individual boundary the spec explicitly draws is collapsed.

## F16 — Needs/aspiration update-frequency bands (low vs very-low/months)
- classification: omission
- source: spec/needs.md:79
- verbatim: "Needs update at **low** frequency; aspirations at **very-low** (months). See `architecture/simulation-clock.md`."
- what should have covered it: no story / no scenario
- why it matters: the spec specifies distinct simulation-clock update bands for needs vs. aspirations (aspirations update on a months-long cadence, distinctly slower than needs); no AC or scenario proves aspirations use a slower cadence than needs, so an implementation that updates both at the same frequency would pass every existing check.

## F17 — Rest/sleep need definition ("schedule allows it, short commute helps")
- classification: missing-scenario
- source: spec/needs.md:28
- verbatim: "Rest / sleep | schedule allows it (short commute helps) | OURS (CS1 has no explicit sleep need)"
- what should have covered it: "Model per-citizen need satisfaction beyond food" AC-1 lists `rest` as a field name only
- why it matters: rest is included as a struct field but the spec's actual behavioral rule (rest satisfaction depends on schedule/commute length) has no scenario — a rest field that never varies with commute time would still pass AC-1/AC-2 as written.

TOTAL FINDINGS: 17
