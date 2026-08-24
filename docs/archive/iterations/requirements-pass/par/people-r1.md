## F1 — Birth mechanics entirely absent from extraction
- classification: omission
- source: spec/households.md:54
- verbatim: "**Birth:** couple present + free member slot + both adults → per-step chance, boosted by childcare access (CS1 CONFIRMED: 1/12 → 1/8) — a services-to-demographics hook worth keeping."
- what should have covered it: no story / no scenario
- why it matters: birth is a named demographic lifecycle event (population growth) with a concrete precondition set and a services-to-demographics coupling (childcare access changes the birth rate); no AC or scenario anywhere in people.json tests population growth or the childcare→birth-rate hook.

## F2 — Household fission (adult children leaving home) not covered
- classification: omission
- source: spec/households.md:40
- verbatim: "2. Adult children leaving home (CS1: slots 2–4 continuously bid to move out)."
- what should have covered it: "Allocate housing through an explicit, player-visible queue" story only exercises the "new couples" queue-entry source in its scenario; no AC or scenario names fission/adult-children-leaving-home as a queue-entry trigger
- why it matters: this is one of the four named queue-entry sources; a household-formation event that grows the queue is untested.

## F3 — Plan-recruited immigration not covered
- classification: omission
- source: spec/households.md:41
- verbatim: "3. Immigrants — **recruited by the plan** (replacing CS1's demand-gated fabrication at the map edge; W&R also makes immigration player-driven)."
- what should have covered it: no story / no scenario
- why it matters: this is a named, deliberate replacement for CS1's automatic edge-of-map population fabrication; without an AC/scenario there is nothing that would catch a regression back to demand-gated auto-spawning of households.

## F4 — School-as-workplace-assignment (student seat as planned allocation) not covered
- classification: omission
- source: spec/citizens.md:51
- verbatim: "**School as workplace assignment** (CS1's `SetStudentplace` trick, CONFIRMED): a student seat is a planned allocation like a job — unifies commutes, capacity, and labour-vs-study trade-offs."
- what should have covered it: no story / no scenario (the labour-allocation story only covers job vacancies, not student-seat allocation)
- why it matters: this is a distinct observable mechanism — students are allocated into capacity-limited seats the same way workers are allocated into vacancies — and it is never asserted anywhere in the extraction.

## F5 — Education throughput limits (attended-capacity pipeline) not covered
- classification: omission
- source: spec/citizens.md:52
- verbatim: "Education requires attended capacity; W&R's `$CITIZEN_ABLE_SERVE` throughput (kindergarten 10, school 12, university 3 — CONFIRMED) is the data shape: higher education is deliberately a slow pipeline."
- what should have covered it: no story / no scenario
- why it matters: this is a falsifiable numeric constraint (specific throughput ceilings per education tier) directly rejecting the CS1 "ambient education field" bug that the extraction elsewhere protects against for shopping (teleport-credit) — the equivalent protection for education throughput/capacity is missing entirely.

## F6 — Labour-pool office (surge/temporary worker dispatch) not covered
- classification: omission
- source: spec/citizens.md:47
- verbatim: "**Labour-pool office** (from W&R `$RESOURCE_SOURCE_WORKERS`, CONFIRMED): dispatches unassigned/temporary workers to short-lived demands without touching fixed assignments."
- what should have covered it: no story / no scenario
- why it matters: this is a named, distinct allocation mechanism (temporary dispatch that must NOT disturb fixed job bindings) sitting alongside the fixed-workplace-binding story; there is no AC asserting that labour-pool dispatch leaves fixed Work.workplace bindings untouched.

## F7 — workProbability(wellbeing) attendance/crime coupling not covered
- classification: omission
- source: spec/needs.md:65
- verbatim: "low **wellbeing** → higher crime, lower work probability (`GetCrimeRate`, `GetWorkProbability`)"
- what should have covered it: no story / no scenario (the labour-allocation and needs stories cover health→efficiency but not wellbeing→work-probability or wellbeing→crime)
- why it matters: this is a second, distinct consequence coupling (crime rate, attendance probability) named alongside the health→efficiency coupling that IS covered — only half the "consequence coupling" pair from spec/needs.md made it into the extraction.

## F8 — Loyalty / political legitimacy meta-need not covered
- classification: omission
- source: spec/needs.md:67
- verbatim: "add **loyalty / political legitimacy** from W&R (`$MONUMENT_GOVERNMENT_LOYALTY`, broadcast/propaganda) as a satisfaction-driven meta-need."
- what should have covered it: no story / no scenario ("Model per-citizen need satisfaction beyond food" lists food/warmth/water/health/housingSpace/housingQuality/rest and wants/aspirations as a layer, but never names loyalty)
- why it matters: this is a named fourth category of citizen state (a meta-need derived from satisfaction) distinct from needs/wants/aspirations; it is completely absent from the extracted taxonomy.

## F9 — Wants taxonomy (specific want categories/building mappings) not covered
- classification: omission
- source: spec/needs.md:36
- verbatim: "Consumer goods (clothes, appliances, electronics), alcohol/social (`$TYPE_PUB`), cinema (`$TYPE_KINO`), sport (`$TYPE_SPORT`), culture/museums (`$ATTRACTIVE_TYPE_MUSEUM`), spiritual (`$TYPE_CHURCH`), tourism/holiday (`$TYPE_HOTEL`)."
- what should have covered it: "Model per-citizen need satisfaction beyond food" AC-3 mentions "Wants (consumer goods, culture, leisure)" only in generic prose, no story/scenario names the specific taxonomy fields (`goods, alcohol, culture, sport, spiritual, recreation` per spec/needs.md:75) or that satisfaction is "spatial and quality-weighted (pollution lowers it, nature/water raise it)"
- why it matters: the spec names a specific `CitizenNeeds`-adjacent wants struct with six named fields and a quality-weighting rule (pollution/nature/water); the extraction only gestures at "wants" generically with no field-level or quality-weighting proof obligation.

## F10 — Aspiration formula (Desire_car) and threshold-crossing-to-demand mechanism not covered
- classification: omission
- source: spec/needs.md:39-47
- verbatim: "Desire_car = MobilityGap + LeisureAccess + WorkAccess + FamilyNeeds\n           + StatusAspiration + ComfortPreference\n           - TransitQuality - CarCost - Scarcity"
- what should have covered it: "Model per-citizen need satisfaction beyond food" AC-3 only asserts aspirations exist as "a separate slower-updating layer whose chronic pressure can cross a threshold and become economic demand" — no scenario exercises an aspiration crossing its threshold and producing an actual demand signal, and no AC covers that fixing an urban-planning input (e.g. transit quality) demonstrably lowers the aspiration (the spec's stated falsifiable causal claim: "fix the tram line → mobility gap falls → car desire drops")
- why it matters: this is the spec's headline example of the aspiration system's causal, non-arbitrary nature ("rather than imposing 'Soviets use transit' by fiat") and it has zero scenario coverage — nothing would catch a regression to an arbitrary/scripted car-desire number.

## F11 — Pantry numeric constants (200 start / -20 per step / +100 per trip) not asserted
- classification: thin-AC
- source: spec/households.md:24 (evidence log row, same page)
- verbatim: "**Pantry** (CS1 CONFIRMED: start 200, −20/step, shop below 200, +100/trip): the terminal demand bucket of the distribution chain; per-household, not per-citizen."
- what should have covered it: "Introduce households as shared-pantry family units" AC-2 only asserts "Consumption by any household member debits the shared household pantry" — it never asserts the numeric drain/refill/threshold constants or the shopping-trigger threshold (below 200)
- why it matters: without asserting the actual constants, a change that keeps the pantry "shared" but silently alters the drain rate or shopping trigger threshold would pass every existing AC.

## F12 — Housing quality (qualityOfLiving) values and default-on heat/electricity requirement not covered
- classification: omission
- source: spec/households.md:48
- verbatim: "`qualityOfLiving` authored per prefab (W&R CONFIRMED: 0.60 khrushchyovka → 1.02 village house) + default-on heat/electricity requirements (W&R CONFIRMED-by-absence: dwellings never declare them; only non-residential opts out via `$HEATING_DISABLE`)."
- what should have covered it: no story / no scenario
- why it matters: this couples directly to the needs taxonomy's `housingQuality` field (spec/needs.md) and to the queue-not-price rule (no dwelling ever prices out heat/electricity, it's a default-on requirement) — neither the authored quality range nor the default-on service requirement is tested anywhere.

## F13 — Coarse age-band thresholds and lifecycle stage transitions (child → student → worker → pensioner) not covered
- classification: thin-AC
- source: spec/citizens.md:70
- verbatim: "Coarse age ticks with threshold graduation (CS1 shape): child → student → worker → pensioner; death by age window widened by poor health + small accident chance (CONFIRMED formulas as tuning priors)."
- what should have covered it: "Progress citizen age and introduce death without deleting the household" AC-1 only asserts "A citizen's age field increases as simulated time passes" — it never asserts the stage-graduation behavior (a citizen actually transitions child→student→worker→pensioner at named age thresholds, à la `AgeGroup thresholds à la CS1 15/45/90/180/240` at spec/citizens.md:32)
- why it matters: age incrementing is necessary but not sufficient — the spec's actual observable claim is that citizens change life-stage (and therefore eligibility for school/work/pension) at threshold boundaries; that transition is untested.

## F14 — Rest/sleep need not covered
- classification: omission
- source: spec/needs.md:28
- verbatim: "Rest / sleep | schedule allows it (short commute helps) | OURS (CS1 has no explicit sleep need)"
- what should have covered it: no story / no scenario names rest/sleep specifically or its commute-length coupling, though "rest" appears listed generically inside AC-1's field list
- why it matters: rest is the one need the spec calls out as wholly original (OURS, no CS1 precedent) with a distinct satisfaction mechanism (schedule/commute-length, not a delivered good or a building visit) — its satisfaction mechanism is never asserted as a proof obligation, only its existence as a struct field name.

## F15 — "No RCI bar / queue-not-price" falsifiable claim not directly asserted
- classification: thin-AC
- source: spec/households.md:16
- verbatim: "No RCI bar — queue length *is* the demand signal; per the one rule, nothing moves in because money exists."
- what should have covered it: "Allocate housing through an explicit, player-visible queue" covers queue growth and eviction-to-queue, but no AC/scenario asserts the negative claim — that no price/affordability field or RCI-style demand bar exists or gates housing assignment
- why it matters: this is the project's core "clearing by queue not price" binding constraint applied to housing; the extraction proves the queue exists but never proves the absence of a price gate, which is the falsifiable half of the claim.

TOTAL FINDINGS: 15
