# Sewage specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** utilities
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Purpose

Sewage is a separate finite physical network for collecting, buffering, treating, and discharging effluent. It is neither Electricity nor Water and is not inferred from road links or Market state.

## Invariants

- `SPEC-SEWAGE-001` — Sewage SHALL be sole authority for sewage topology, endpoint attachment, buffers, pipe/pump capacity, transfer progress, treatment result, and discharge record. It MUST NOT reuse Electricity connectivity or Water quantity as sewage state.
- `SPEC-SEWAGE-002` — Every accepted effluent quantity enters one named endpoint buffer. Per tick movement is bounded by connected path and pipe/pump rate; treatment/discharge removes quantity only via named result and any residue has a named physical holder.
- `SPEC-SEWAGE-003` — Full buffers, disconnected path, zero rate, unavailable treatment, or blocked discharge retain observable backlog. A producing water use is restricted only through a declared interface; backlog MUST NOT disappear or end the plan.
- `SPEC-SEWAGE-004` — Treatment receives named sewage and emits only a declared result. A Water handoff uses one immutable `SewageWaterHandoffReceiptID`: its single atomic acceptance debits the named Sewage treatment result and credits only the Water-owned accepted result; replay is a no-op. Sewage cannot mutate Water topology, quantity, quality, or meter.
- `SPEC-SEWAGE-005` — Priority is explicit and non-price-based. Buildings, Water, and Production reference results and MUST NOT copy sewage buffers, graph, or discharge state.
- `SPEC-SEWAGE-006` — Each treatment or discharge application SHALL have one immutable `SewageTreatmentDispositionID` and apply at most once. For accepted sewage quantity `Q`, treated output `T` plus physical residue `R` plus named loss `L` SHALL equal `Q`: `Q = T + R + L`. A replay is a no-op; no treatment/discharge may yield more than its accepted sewage input.

## Model and state

Sewage owns `SewageNodeID`, pipe/pump graph, source buffer, treatment queue/capacity, `SewageTransferID`, treatment result, discharge record, residue holder, blockage age, `SewageTreatmentDispositionID`, and `SewageWaterHandoffReceiptID`. An endpoint references a Water service result only to calculate declared effluent. A named result may be accepted by Water or Waste, whose authority alone records receiving state.

## Failure behavior and observability

Backpressure fills physical buffers and exposes a sewage queue; future service degrades only through its declared interface. Recovery drains only at later connected capacity. The Planner can inspect source, buffer, path, pipe/pump budget, treatment queue, discharge/residue, blockage reason/age, and resulting restriction.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure; the current serial suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-SEWAGE-001` | `cargo test -p simulation evid_sewage_separate_graph_rate_buffers -- --test-threads=1` — separate connected finite-rate graph/buffers. | Share Water/Electricity graph or exceed rate. | Inspected graph/buffer timeline. |
| `EVID-SEWAGE-002` | `cargo test -p simulation evid_sewage_backpressure_persists -- --test-threads=1` — blocked treatment retains backlog and declared restriction. | Delete backlog or bypass full buffer. | Inspected backlog capture. |
| `EVID-SEWAGE-003` | `cargo test -p simulation evid_sewage_treatment_discharge_conservation -- --test-threads=1` — one `SewageTreatmentDispositionID` proves `Q = T + R + L` and applies once; its named Water handoff uses one `SewageWaterHandoffReceiptID` to atomically debit the treatment result and credit one Water-accepted result. Both replays are no-ops and Sewage cannot mutate Water topology, quantity, quality, or meter. | Replay either ID, yield `T + R + L > Q`, delete residue/loss, credit Water without acceptance, or mutate Water state from Sewage. | Inspected accepted quantity, treatment/discharge IDs, ledger, and Water-handoff capture. |
| `EVID-SEWAGE-004` | `cargo test -p simulation evid_sewage_nonprice_priority -- --test-threads=1` — declared sewage priority orders equal-capacity demand identically despite reversed rouble balances. | Rank or debit/credit by roubles. | Inspected queue inputs, order, and unchanged-money capture. |
| `EVID-SEWAGE-005` | `cargo test -p simulation evid_sewage_authority_references_not_copies -- --test-threads=1` — consumers cannot mutate sewage state. | Add a consumer-owned sewage buffer. | Inspected authority links. |

## Substrate and decisions

No sewage kind or registered system exists (`simulation/src/map/objects/building.rs:17-37`; `simulation/src/init.rs:52-70`). Electricity's road cache (`simulation/src/map/electricity_cache.rs:203-279`) cannot establish sewage. The [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2c--utilities-electricity-water-sewage-heating-and-waste) records no sewage save/UI/test surface. Legacy `spec/sewage.md` is rewrite input, not mechanism authority.

## Deferred behavior

Treatment tiers, tanker sewage, storm/rain load, and a pollution mechanism have no 1.0 acceptance criteria.

## Open questions

- Which endpoints generate sewage and by what Water-service relation?
- Which treatment/discharge endpoints are 1.0?
