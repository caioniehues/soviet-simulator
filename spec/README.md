# Specifications

One file per subsystem. Each is a living document that evolves from stub → draft model → validated model as Phase 1 research proceeds.

## Evidence levels

Every non-trivial claim in a spec **must** be tagged, so we never confuse what a game appears to do with how it actually works internally.

| Level | Meaning |
|---|---|
| **CONFIRMED** | Officially documented (dev diary, official modding docs, patch notes). |
| **OBSERVED** | Measured directly in a controlled experiment. |
| **INFERRED** | Best-fit explanation of measurements; not directly observed. |
| **SPECULATIVE** | A plausible internal implementation; unverified. |
| **OURS** | A deliberate design decision or departure — not reverse-engineered from either game. |

## File template

Each stub follows the same shape: status header, purpose, open questions, draft model, an **evidence log** table, and related links. Keep the evidence log honest — an untagged formula is a liability.

## Index

**Root layer**
- [needs.md](needs.md) — needs / wants / aspirations (design this first; the economy hangs from it)

**Citizens & households**
- [citizens.md](citizens.md) · [households.md](households.md) · [vehicles.md](vehicles.md)

**Movement**
- [roads.md](roads.md) · [traffic.md](traffic.md) · [pathfinding.md](pathfinding.md) · [zoning.md](zoning.md) · [buildings.md](buildings.md)

**Physical economy**
- [resources.md](resources.md) · [production.md](production.md) · [logistics.md](logistics.md) · [construction.md](construction.md) · [trade.md](trade.md)

**Utilities**
- [electricity.md](electricity.md) · [water.md](water.md) · [sewage.md](sewage.md) · [heating.md](heating.md) · [waste.md](waste.md)

**Services**
- [education.md](education.md) · [healthcare.md](healthcare.md) · [crime.md](crime.md)
