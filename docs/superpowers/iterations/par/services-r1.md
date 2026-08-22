## F1 — Court stage (sentencing throughput) has no story or AC at all
- classification: omission
- source: spec/crime.md:41
- verbatim: "courts (profesor-judges), prison (worker-guards only), secret police (profesors + fuel only) (§G1). We adopt the staffed-shell grammar: each is a real staffed, fuelled building; court capacity throttles sentencing throughput between arrest and prison."
- what should have covered it: no story ("Arrest a specific criminal and hold them in a fed prison cell" goes straight from arrest to prison, skipping the court/sentencing stage entirely) / no scenario
- why it matters: the spec explicitly models a staffed Court building that throttles the arrest→prison pipeline; the extraction's arrest story has the officer transport the citizen directly to Prison with no court/caseThroughput concept, silently dropping an entire confirmed institution shape (police.ini/court.ini/prison.ini staffing grammar).

## F2 — Court/caseThroughput data structure entirely unrepresented
- classification: omission
- source: spec/crime.md:53
- verbatim: "Court { staff; caseThroughput }   Prison { guards; cells; foodDemand; qualityOfLiving }"
- what should have covered it: no story / no scenario
- why it matters: same gap as F1 — the AC set never introduces a Court entity, so caseThroughput as a bottleneck (analogous to the university 3-per-cycle cap that DID get an AC/scenario) has zero proof obligation.

## F3 — Police vehicles+fuel not gated by any AC/scenario (unlike hospital fuel)
- classification: omission
- source: spec/crime.md:52
- verbatim: "PoliceStation { staff; vehicles+fuel; cells }"
- what should have covered it: no story ("Arrest a specific criminal and hold them in a fed prison cell" AC-1 only says "a staffed police station to send an officer... a vehicle trip", never mentions fuel) / no scenario
- why it matters: healthcare got an explicit falsifiable AC+scenario for "unfuelled hospital dispatches no ambulance" (spec/healthcare.md:27-29, extraction healthcare AC-4 + scenario "Unfuelled hospital dispatches no ambulance..."); crime.md states the identical W&R vehicle-base grammar applies to police stations, but no equivalent "unfuelled police station dispatches no officer" AC or scenario exists — an asymmetric gap in an otherwise parallel mechanic.

## F4 — Prison guard staffing (worker-guards only) has no AC
- classification: thin-AC
- source: spec/crime.md:41
- verbatim: "prison (worker-guards only)"
- what should have covered it: "Arrest a specific criminal and hold them in a fed prison cell" story / no scenario
- why it matters: the prison story's ACs cover cells and foodDemand but never test that a prison actually requires staffed guards to function (e.g., an unstaffed prison degrading admission/holding), unlike the hospital and school stories which do get staffing-gated ACs (cure rate requires staff present; seat-time requires staff present).

## F5 — Hospital bed capacity constant (100) not carried into any AC
- classification: thin-AC
- source: spec/healthcare.md:27
- verbatim: "2. Patient occupies one of `patientCapacity` **beds** (CS1: 100)."
- what should have covered it: "Treat a sick citizen via dispatch, bed occupancy, and staffed cure rate" AC-1 (states "beds: u32" generically, no numeric anchor or scenario asserting the CS1-derived value/shape)
- why it matters: numeric constants like the crime `unemploymentLength`→rate bands and `occupantCount*100` buffer cap DID make it into ACs and a scenario ("Crime buffer never exceeds the occupant-count cap"); the equivalent hospital bed-capacity number has no analogous falsifiable anchor.

## F6 — Hospital throughput-as-serial-pipeline (serve-rate 3/cycle) distinct from bed count is unrepresented
- classification: omission
- source: spec/healthcare.md:34
- verbatim: "W&R hospital serve-rate is tiny (`$CITIZEN_ABLE_SERVE 3` — same as universities): treatment is a slow serial pipeline (§F). Combined: beds + staff + fuel + cure-rate make healthcare a resource-constrained throughput facility (§J.4)."
- what should have covered it: no story / no scenario
- why it matters: education got a direct AC+scenario pair for the analogous university 3-per-cycle throughput cap ("University throughput is narrower than school throughput... enforced as a hard cap", scenario "University enrolment is capped below school-tier enrolment"), but the identical W&R serve-rate-3 constant applied to hospitals has no AC treating it as a distinct cap from bed occupancy.

## F7 — Two-tier hospital staff ratio (workers 50 / profesors 50) not anchored numerically
- classification: thin-AC
- source: spec/healthcare.md:42
- verbatim: "Hospitals need two-tier staff (W&R: workers 50 + profesors 50, `hospital.ini:48-50`); the profesor tier comes from medical universities (spec/education.md specialisation) — education → healthcare-labour is a closed loop in building types (§F)."
- what should have covered it: "Treat a sick citizen via dispatch, bed occupancy, and staffed cure rate" AC-3 (only says "workers + medically-specialised profesors present vs. required", no concrete ratio, and no scenario probes a partial-staffing degradation curve)
- why it matters: the cure-rate degradation curve is only tested at the binary zero-staff-vs-full-staff extremes (scenario "Sick citizen is cured only proportional to hospital staffing"); a partial/mismatched ratio (e.g., workers present but zero profesors) is never falsified even though the spec explicitly calls out a two-tier requirement.

## F8 — CS1 seat-capacity formula (StudentCount × 5/4) not carried into any AC
- classification: thin-AC
- source: spec/education.md:20
- verbatim: "A school offers `studentCapacity` seats (CS1: `StudentCount * 5/4` slots, serviceable count throttled by production rate — §B2)."
- what should have covered it: "Enrol a child in a staffed school as a workplace-slot binding" AC-1 (states studentCapacity is finite and gates enrolled.len(), but never anchors the CS1-derived 5/4 multiplier or the "throttled by production rate" clause)
- why it matters: "throttled by production rate" is a distinct, testable mechanism (capacity shrinks when the school underproduces/is understaffed) separate from the flat capacity cap already covered by the "Child cannot enrol when all schools are at capacity" scenario — that scenario only tests a static capacity, never a capacity that degrades with under-operation.

## F9 — Kindergarten's staffing shape (workers only, no profesors) is asserted but never falsified
- classification: missing-scenario
- source: spec/education.md:30
- verbatim: "kindergarten  wide         workers only (no profesors — kindergarten.ini:30)"
- what should have covered it: no story tests kindergarten specifically / no scenario (the specialisation story and its scenario test only the university tier's staff-matching rule)
- why it matters: the extraction proves the university profesor-matching rule (AC-2 of "Send a graduate to a university specialisation..." + its scenario) but never proves the opposite/boundary case named explicitly in the spec — that kindergartens function or are staffed without any profesor tier at all, leaving the low end of the "wide→narrow" pipeline shape unverified.

## F10 — School tier's own throughput narrowing (10→12) not distinguished from university's cap
- classification: thin-AC
- source: spec/education.md:26
- verbatim: "W&R's chain: kindergarten → school → university (+ orphanage as child welfare), with throughput narrowing up the ladder — `$CITIZEN_ABLE_SERVE` 10 → 12 → 3 (research/services.md §E)."
- what should have covered it: "Progress a citizen through education tiers by seat-time" AC-3 (only enforces the university-tier cap of 3; "school-tier 10-12 per cycle" is mentioned in the AC text but never given its own scenario the way the university cap gets "University enrolment is capped below school-tier enrolment")
- why it matters: the spec names three distinct throughput bands (10, 12, 3) across kindergarten/school/university, but only the narrowest (university=3) is scenario-tested; the kindergarten→school step of the "narrowing ladder" claim is asserted in prose but has no falsifiable proof obligation.

## F11 — "Known criminal ~4× harder" multiplier for crime AND the identical 4× recidivism concept is duplicated only in crime — no cross-check with arrest state
- classification: intentional-exclusion
- source: spec/crime.md (Generation section)
- verbatim: "known criminal: ~4× harder"
- what should have covered it: N/A
- why it matters: this IS covered (AC-1 of "Generate per-building crime pressure..." explicitly states the "~4x multiplier for citizens already flagged criminal"); listed only to confirm it was checked and correctly extracted, not a finding.

## F12 — Political dimension / secret police, prison labour, and adult re-education are open questions, correctly excluded
- classification: intentional-exclusion
- source: spec/crime.md:58-61, spec/education.md (Open questions section)
- verbatim: "Political dimension: is \"crime\" also dissent (W&R's secret police exists in data but its mechanic is native/undocumented — §G3 Gaps)? Keep separate from property crime; couples to loyalty (spec/needs.md)." / "Prison labour — period-authentic but sensitive; decision deferred" / "Adult re-education / night schools as a planner lever? (No substrate in either game — would be OURS.)"
- why it matters: these are explicitly unresolved design questions in the spec itself (not committed requirements), so their absence from the extraction is correct, not an omission.

## F13 — Orphanage (child welfare, fed via storage-demand like prison) is an open question, correctly excluded
- classification: intentional-exclusion
- source: spec/education.md (Open questions section)
- verbatim: "Orphanage (W&R has one, physically fed via `$STORAGE_DEMAND_PRISON`) — model with households or with services?"
- why it matters: explicitly unresolved scope question in the spec, not a committed requirement — correctly absent from the extraction.

TOTAL FINDINGS: 10
