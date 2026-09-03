# Research methodology

**Kind:** standard
**Authority:** operational for research pages
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

Historical research discovers **mechanisms**, never an ideological verdict. Technical research
discovers **models and their cost**, never a runtime dependency by itself.

## Evidence classes, strongest first

| Class | Examples | Weight |
|---|---|---|
| PROJECT BINDING | charter, glossary, active specs, accepted decisions | binds; not evidence about history |
| CURRENT SUBSTRATE | fact-sheets, current-substrate page, source at a commit | observational; dated |
| PRIMARY / ARCHIVAL | statutes, constitutions, enterprise reports, technical manuals, statistical yearbooks | strong for *formal* design; weak for lived practice; Soviet statistics were falsified at enterprise level (CIA found grain claims up to 53 % high) |
| CIA ANALYTICAL ASSESSMENT | DI reports, JEC compendium papers, NIEs | strong on trends; written to find weakness; Moscow/Leningrad-heavy; thin on the 1960s |
| CIA FIELD REPORT | emigre interviews, HUMINT | anecdotal; the Soviet Interview Project sample was emigre, urban, educated |
| CIA-HOSTED TRANSLATED SOVIET SOURCE | translated press and journals | a Soviet source with CIA provenance; judge by the original |
| ACADEMIC SECONDARY | Kornai, Berliner, Weitzman, Ledeneva, Andrusz, Feshbach, Filtzer, Bater, Kibita | strongest for mechanism and periodisation |
| ENGINEERING REFERENCE | Treiber (IDM), Kesting (MOBIL), Daganzo (CTM), BPR, Gawron, EPANET, SWMM, HEC | model definitions; validation oracles, not runtime deps |
| GAME COMPARISON | W&R (installed reference), CS2, Victoria 3, Factorio, DF, Frostpunk | what is possible and what players expect; never mechanism authority |
| DESIGN INFERENCE | the design thread's proposals; lane §3 sketches | hypotheses until ratified |

Not every CIA Reading Room file is equal: label which class a document is.

## Confidence labels

`CONFIRMED` (a cited source or the code proves it) · `PLAUSIBLE` (consistent, unproven) ·
`HYPOTHESIS` (design proposal) · `UNSUPPORTED` (no source found) · `WRONG` (contradicted) ·
`DOCUMENTED` / `PLAUSIBLE-NO-DOC-FOUND` (Lane B2's CIA-specific pair). Copy the label the source
page gave; never upgrade it.

## Historical scope

State the system and the period: USSR 1930s–50s (Stalinist planning; ratchet strongest), 1950s–60s
(the game's fixed era: Khrushchev housing, sovnarkhozy, Kosygin), late-Soviet (1970s–80s: consumer
frustration, alcohol, maintenance crisis); Yugoslavia; Hungary; Poland; China (danwei). Late-Soviet
behaviour is not the 1950s. Formal institutions (a 1936 constitution) are not lived power.

## Known biases to correct for (Lane B2 §4)

Emigre bias; dependence on falsified official statistics; Cold-War framing that documents failure
better than success; urban focus (Moscow ≫ province ≫ small town ≫ rural); temporal clustering in
the late 1950s and early 1980s. The baseline experience was functional but constrained; crises
emerge from failures, not from the default state. The republic must be allowed to work.

## Numbers

Only numbers with a document ID or a citation are used. The only sanctioned calibration set is
Lane B2 §3 (27 parameters). The design thread's illustrative tables (72/141/24/18 corridor
utilisation; 61/67/82/91 credibility) are fabrications and are banned. Treat 1954 numbers as
bounds, not constants.

## From evidence to mechanic

A research page may end with **Possible mechanic** and **Scope status** (1.0 / candidate / hook /
Post-1.0). It never states the mechanic as a rule. The design page cites the research; the
specification, if any, states the rule.

## Technical research

Verify a crate on crates.io and its repository (version, date, licence, maintenance); check
whether it is already a transitive dependency; state the integration cost; prefer a validation
oracle to a runtime dependency for engineering models. Context7 is the documentation source for
library API questions; an assertion from training memory about an API is not evidence.

## Related

- [Research index](index.md)
- [Document authority](../meta/document-authority.md)
- [Dependency standard](../engineering/dependencies.md)
