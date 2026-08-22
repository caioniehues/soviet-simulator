# EPIC-025 — Resource ontology (greenfield)

**Summary:** Resource ontology (greenfield)
**Stories:** STORY-0108, STORY-0109, STORY-0110, STORY-0111, STORY-0112
**Primary sources:** `spec/resources.md`
**Status:** 0/5 done

## STORY-0108

**Epic:** EPIC-025 — Resource ontology (greenfield)
**Title:** Give every resource item physical handling metadata

**As a** planner and the logistics/construction systems built on top of resources
**I want** each resource to declare mass, volume, a storage class, and a transport class
**So that** logistics can decide which vehicle/container/network can carry a given resource and storage buildings can reject incompatible goods

**Acceptance criteria:**
- AC-1: Today ItemPrototype is exactly {base, id, optout_exttrade} — no mass, volume, storageClass, or transportClass field exists on any item. [SUBSTRATE: ABSENT — prototypes/src/prototypes/item.rs:8-12, whole schema] · impact:`none` · seam:`unit`
- AC-2: Every item prototype must carry a numeric mass (t/unit) and volume (m³/unit) field, loadable from the Lua catalogue like other prototype fields. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0091`
- AC-3: Every item prototype must carry an optional containerClass (aluminium | bio | construction | plastic | steel | toxic | open) for items that ship in typed containers, matching container_big_{aluminium,bio,construction,plastic,steel,toxic} material classes; a container's material determines what it may legally carry. [SUBSTRATE: ABSENT — greenfield; spec/resources.md:67,86] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0091`
- AC-4: Every item prototype must carry a category field with exactly one of {raw, processed-material, construction, consumer-good, liquid, energy, waste}, distinct from and in addition to its tier field. [SUBSTRATE: ABSENT — greenfield; spec/resources.md:56] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0091`

**Sources:**
- `spec/resources.md:48-90`

**Status:** pending

## STORY-0109

**Epic:** EPIC-025 — Resource ontology (greenfield)
**Title:** Give every resource item an economic tier classification

**As a** planner
**I want** each resource to declare whether it is raw, intermediate, finished, or luxury
**So that** planning priority and trade value can be driven by tier without reintroducing floating money-based worth

**Acceptance criteria:**
- AC-1: Every item prototype must carry a tier field with exactly one of {raw, intermediate, finished, luxury}. [SUBSTRATE: ABSENT — greenfield, no such field exists in prototypes/src/prototypes/item.rs] · impact:`local` · seam:`unit`

**Sources:**
- `spec/resources.md:108-115`

**Status:** pending

## STORY-0110

**Epic:** EPIC-025 — Resource ontology (greenfield)
**Title:** Give perishable and hazardous items lifecycle metadata

**As a** planner
**I want** perishable goods (food, meat) to decay past a shelf life outside cold storage, and hazardous goods (uranium, some waste) to be flagged as such
**So that** refrigerated logistics and radioactive handling are physically meaningful rather than cosmetic categories

**Acceptance criteria:**
- AC-1: An item may optionally declare a shelfLife; when stored outside a storageClass=cooled bucket (or its equivalent) past shelfLife, its stored quantity is reduced or removed rather than persisting indefinitely. [SUBSTRATE: ABSENT — greenfield; no decay mechanism exists anywhere in the simulation crate per audit] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0092`
- AC-2: An item may optionally declare a hazardClass (e.g. radioactive, toxic); items with a hazardClass set are distinguishable at the prototype level from ordinary goods (for later systems — construction siting, waste routing — to query). [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0092`

**Sources:**
- `spec/resources.md:48-78`

**Status:** pending

## STORY-0111

**Epic:** EPIC-025 — Resource ontology (greenfield)
**Title:** Replace the inverted opt-out trade flag with an explicit tradeable declaration

**As a** planner and the trade system
**I want** each resource to declare tradeable directly rather than the codebase reasoning about the negation of optout_exttrade everywhere it needs to know
**So that** trade eligibility reads as a positive, unambiguous fact matching the spec's Resource.tradeable field

**Acceptance criteria:**
- AC-1: Today the only trade-related field is optout_exttrade — the inverse of the spec's tradeable — and every caller must negate it to get the spec's semantics. [SUBSTRATE: PARTIAL — prototypes/src/prototypes/item.rs:8-12, optout_exttrade is the sole existing field] · impact:`none` · seam:`unit`
- AC-2: A tradeable accessor (or renamed field) exists such that tradeable == !optout_exttrade for every existing item, verified against every entry in the current catalogue with no behavioural change to which items trade externally. [SUBSTRATE: ABSENT — greenfield naming/accessor, behaviour-preserving] · impact:`none` · seam:`unit`

**Sources:**
- `spec/resources.md:48-78`

**Status:** pending

## STORY-0112

**Epic:** EPIC-025 — Resource ontology (greenfield)
**Title:** Support storage buckets pinned to a single resource and consumer-demand buffer tiers

**As a** planner
**I want** a building's storage to support a bucket reserved for one named resource and separate consumer-demand buffer tiers
**So that** import docks that only accept one commodity and shop demand buffers are representable, not just a generic storageClass bucket

**Acceptance criteria:**
- AC-1: Today storage buckets are undifferentiated by class only; no bucket type exists that pins itself to a single named resource id or that models a tiered consumer-demand buffer. [SUBSTRATE: ABSENT — greenfield; spec/resources.md:84] · impact:`none` · seam:`unit`
- AC-2: A building may declare an import bucket pinned to exactly one resource id and capacity; the bucket accepts only that resource and rejects any other, even one of a compatible storageClass. [SUBSTRATE: ABSENT — greenfield; spec/resources.md:84 ($STORAGE_IMPORT_SPECIAL)] · impact:`local` · seam:`unit`
- AC-3: A shop building may declare one of four consumer-demand buffer tiers (basic | advanced | hotel | prison), each an independent capacity bucket distinct from ordinary import/export storage. [SUBSTRATE: ABSENT — greenfield; spec/resources.md:84 ($STORAGE_DEMAND_BASIC/_ADVANCED/_HOTEL/_PRISON)] · impact:`local` · seam:`unit`

**Sources:**
- `spec/resources.md:84`

**Status:** pending