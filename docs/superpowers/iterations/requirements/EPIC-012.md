# EPIC-012 — Foreign trade catalogue

**Summary:** Foreign trade catalogue
**Stories:** STORY-0055, STORY-0056, STORY-0057
**Primary sources:** `spec/trade.md`
**Status:** 0/3 done

## STORY-0055

**Epic:** EPIC-012 — Foreign trade catalogue
**Title:** Let the plan set standing import/export contracts fulfilled by logistics

**As a** planner
**I want** the plan to set standing import/export contracts (quotas) rather than negotiate every shipment order manually
**So that** foreign trade quotas are a plan artifact and logistics dispatch fulfils them physically, consistent with domestic deficit-driven dispatch

**Acceptance criteria:**
- AC-1: A standing import or export contract set by the plan specifies a good, a quantity/quota, and a direction, and logistics dispatch fulfils it over time via the same physical border-crossing path as any other trade order, rather than requiring the player to place a fresh order for every shipment. [SUBSTRATE: ABSENT — greenfield; no plan-contract or quota mechanism exists, spec/trade.md:60-61] · impact:`journey` · seam:`integration`

**Sources:**
- `spec/trade.md:60-61`

**Status:** pending

## STORY-0056

**Epic:** EPIC-012 — Foreign trade catalogue
**Title:** Gate the import/export catalogue by era and bloc

**As a** planner
**I want** which goods and vehicles are importable/exportable to change with the campaign timeline and bloc alignment
**So that** trade is a geopolitical lever, not a static shop

**Acceptance criteria:**
- AC-1: Each tradeable item/vehicle prototype carries an availability window (era range) and an origin/bloc tag, and an order for an item outside its current availability window or bloc alignment is rejected at customs. [SUBSTRATE: ABSENT — ItemPrototype today is only {base, id, optout_exttrade}, no era or country fields exist, prototypes/src/prototypes/item.rs:8-12, docs/egregoria-substrate-audit.md:47-48] · impact:`journey` · seam:`unit` · scenario:`SCENARIO-0027`

**Sources:**
- `spec/trade.md:42-44`

**Status:** pending

## STORY-0057

**Epic:** EPIC-012 — Foreign trade catalogue
**Title:** Depreciate exported vehicles by condition on resale

**As a** planner
**I want** an exported vehicle's sale value to depend on its wear/condition, not just its base price
**So that** players cannot churn new vehicles across the border for free money

**Acceptance criteria:**
- AC-1: Exporting a used vehicle yields strictly less hard currency than exporting an equivalent new vehicle, as a function of a tracked condition/wear value. [SUBSTRATE: ABSENT — greenfield; Vehicle is a bare kinematic shell today with no wear/condition/owner economic fields at all, transportation/vehicle.rs:34-44, docs/egregoria-substrate-audit.md:22] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0028`

**Sources:**
- `spec/trade.md:46-48`

**Status:** pending