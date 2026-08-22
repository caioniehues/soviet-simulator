## F1 — Court institution entirely absent from extraction
- classification: omission
- source: spec/crime.md:41
- verbatim: "We adopt the staffed-shell grammar: each is a real staffed, fuelled building; court capacity throttles sentencing throughput between arrest and prison."
- what should have covered it: no story / no scenario
- why it matters: the spec explicitly adopts Court as a staffed institution that throttles the arrest→prison pipeline (also spec/crime.md:53 `Court { staff; caseThroughput }`), but no story models a Court building, its staffing, or its case-throughput bottleneck between arrest and imprisonment — the extraction's arrest story (`Arrest a specific criminal...`) goes straight from arrest to Prison with no sentencing throughput gate.

## F2 — Note: court's role is flagged open ("does the court stage add anything mechanically") but the adopted grammar line (F1) is separate and not itself an open question
- classification: intentional-exclusion
- source: spec/crime.md:61
- verbatim: "Sentencing: does the court stage add anything mechanically in v1, or arrest→prison direct?"
- what should have covered it: n/a
- why it matters: this specific open question is legitimately excludable as undecided design — but it does not excuse omitting the already-adopted "court capacity throttles sentencing throughput" line from §Institutions (F1), which is stated as CONFIRMED-adopted, not open.

## F3 — PoliceStation fuel dependency never tested
- classification: missing-scenario
- source: spec/crime.md:52
- verbatim: "PoliceStation { staff; vehicles+fuel; cells }"
- what should have covered it: story "Arrest a specific criminal and hold them in a fed prison cell" / no scenario
- why it matters: the data model states police dispatch vehicles require fuel, exactly parallel to the hospital fuel dependency which DOES get a dedicated scenario ("Unfuelled hospital dispatches no ambulance..."). No equivalent scenario exists for an unfuelled PoliceStation failing to dispatch an officer, leaving a stated supply dependency (staff AND vehicles AND fuel AND cells) unenforced by any AC.

## F4 — Kindergarten's "no profesors" staff shape untested
- classification: missing-scenario
- source: spec/education.md:30
- verbatim: "kindergarten  wide         workers only (no profesors — kindergarten.ini:30)"
- what should have covered it: story "Progress a citizen through education tiers by seat-time" / no scenario
- why it matters: the spec gives a specific 3-tier staff-shape table (kindergarten: workers only; school: workers 10 + profesors 15; university: workers 100 + profesors 100) as the bootstrap texture for the education chain, but only the university tier's staffing bottleneck is covered by a scenario (university enrolment-cap scenario, and the university story's AC-3 bootstrap chicken-and-egg). No AC or scenario distinguishes kindergarten's simpler workers-only staffing requirement from school's two-tier requirement, so the tier-by-tier staffing shape the spec calls "good planned-economy texture" is only partially proven.

## F5 — School two-tier staff ratio (workers 10 + profesors 15) not encoded as a proof obligation
- classification: thin-AC
- source: spec/education.md:31
- verbatim: "school        wide         workers 10 + profesors 15 (school.ini:25-26)"
- what should have covered it: story "Enrol a child in a staffed school as a workplace-slot binding" / no scenario
- why it matters: AC-1 of the enrolment story tests only `studentCapacity`/`enrolled[]`, not that a school additionally requires its own two-tier staff (workers+profesors) to operate — an unstaffed school's seat-time consequence is tested (scenario "Seat-time only accrues while staffed and operating") but the requirement that operation itself depends on a specific staff composition (not just "nonzero staff") is never stated as an AC.

TOTAL FINDINGS: 4
