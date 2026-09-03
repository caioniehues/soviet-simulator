# Provisioning — Food, Meat, and the adaptive sequence

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Food and Meat as separate needs | 1.0 — charter row *Resources and production* |
| Adaptive sequence | 1.0 — charter row *Households and citizens* |
| Citizen knowledge and search | Post-1.0 hook |
| Deficit-goods list | Post-1.0 |
| Household plots | Post-1.0 |

## What this is

Provisioning is how households obtain food. Food and Meat are separate 1.0 dwelling needs: a
citizen who has bread but no meat is partially provisioned, not fully satisfied. The path from
need to satisfaction passes through search, travel, queuing, and sometimes informal channels.
Each step costs time.

## 1.0 requirement

The charter says "Food and Meat are separate dwelling needs." The draft needs specification
defines the contract:

- Food and Meat are distinct; a record of one never satisfies the other
  (`SPEC-NEEDS-001`).
- A need is satisfied only by authoritative consumption after physical stock has reached the
  permitted point of use (`SPEC-NEEDS-002`).
- Domestic clearing is non-price (`SPEC-NEEDS-003`).
- An unmet need persists with age and an explicit outcome: waiting, approved substitution, or
  going without (`SPEC-NEEDS-004`).
- Going without degrades the affected need and is inspectable; never game over
  (`SPEC-NEEDS-005`).

## Target design

### The adaptive sequence — 1.0, charter row *Households and citizens* (SPEC-NEEDS-003/004 extended; bible §7.6)

When a household needs food or meat, the design proposes a stepped sequence:

1. **Preferred available** — the household's known reliable store has stock. Acquire.
2. **Not available** — the preferred store is empty. Search: try an alternative known store.
   Cost: travel time.
3. **Still not available** — approved substitute if the need policy permits. (Open question:
   which substitutions are allowed, `needs.md:115-116`.)
4. **Still not available** — household reserve (pantry buffer), informal route (blat,
   Post-1.0), or plot production (Post-1.0).
5. **Still not available** — going without. The need ages; the outcome is inspectable.

Each step costs more time and yields less certainty. The sequence is an instance of
[adaptation under scarcity](../concepts/adaptation.md).

### Citizen knowledge and search — PLAUSIBLE (B1 §3d; bible §7.5)

The design proposes that citizens do not know every store's inventory. A household maintains
knowledge about at most 3–5 shops:

```text
ShopKnowledge:
  shop: BuildingID
  last_seen_stock: Option<(ItemID, quantity, GameTick)>
  last_visit_tick: GameTick
  heard_from: Option<CitizenID>
  estimated_queue_time: u16
```

Learning channels:
- **Direct visit** — on arrival at the shop, stock knowledge updates to the current actual.
- **Neighbourhood observation** — a citizen within visual range observes a delivery. This
  requires physical presence: information is local.
- **Social transmission** — a household member or blat contact shares their shop knowledge
  with a time decay. Information is stale by the time it reaches you.
- **Rumour** — a lossy, delayed, sometimes wrong channel (B1-MISSED-10). The only information
  channel that propagates without physical proximity to a shop. It is lossy (degrades),
  delayed (multiple social hops), and sometimes false.

### How a delivery becomes a crowd — PLAUSIBLE (B1 §3d)

The design proposes emergent crowd formation with no scripted events:

```text
delivery arrives at shop
  → citizens in visual range observe
  → they tell household members (same tick, if at home)
  → household members tell blat contacts (next social tick)
  → informed citizens travel to shop
  → queue forms
  → stock depletes
```

The Planner sees: delivery → crowd → depletion → empty shelves → longer queues at other shops.

### Deficit-goods list — Post-1.0 (B1-MISSED-11)

The design proposes that specific goods cycle in and out of deficit status rather than
"everything is equally scarce." Historically deficit categories included meat, butter, coffee,
fruit, quality shoes, clothing, household appliances, and electronics. Deficit status varied
by city (Moscow better supplied than provinces) and period (shortages worsened in the 1980s).
A good's deficit status drives queue length and blat activation for that item.

### Household plots — Post-1.0 (B1-09; CONFIRMED)

Private plots on ~3 % of agricultural land produced 64 % of potatoes, 43 % of vegetables,
40 % of meat, 39 % of milk, and 66 % of eggs (1966 figures; CIA DOC_0000496622; Wädekin
1973). Plots buffer formal food failures at the cost of labour, transport, tools, seed/feed,
and leisure time. Food must be physically grown, never conjured.

Plots depend on state-sector inputs: feed grain, fertiliser, veterinary services, transport
to market. The plot is not an independent system — it is a physical buffer that costs
household time and requires formal-sector inputs.

## Current substrate

Citizens have one need: bread, via `BuyFood` (`simulation/src/souls/desire/buyfood.rs`). The
BuyFood state machine places a market buy order and is matched globally by distance — the
citizen has perfect knowledge of the matched seller's location. There is no search, no
queueing time, no multi-store search, no household pantry, no Meat, no substitution sequence,
no citizen knowledge model.

On arrival the citizen verifies the live retail claim and settles it at eat-time:
`Market::settle_retail` removes the claim, debits seller capital, and releases the
reservation (`simulation/src/souls/desire/buyfood.rs:157-168`;
`simulation/src/economy/market.rs:480-491`). `last_ate` advances only on successful
settlement; if the claim expired during the walk, the citizen goes without and `last_ate`
does not advance.

## Research basis

- CIA, "Consumer Frustrations and the Soviet Regime" (August 1979): the "shopping rat race."
- CIA, "Selected Information on Consumer Welfare in the USSR" (March 1955): 33.65 h/week for
  a basic food basket (1954 Moscow).
- Wädekin (1973), *The Private Sector in Soviet Agriculture*: private-plot data.
- Ledeneva (1998) on blat-mediated provisioning (see [social networks](social-networks.md)).
- Lovell (2003), *Summerfolk: A History of the Dacha*: plot culture and economics.
- B1-MISSED-11 on the deficit-goods list; B1-MISSED-10 on rumour.

## Open questions

- Which substitutions between Food and Meat (or between specific goods) does the need policy
  permit? (`needs.md:115-116`.)
- How many shops does a household track knowledge for? 3–5 is the design proposal.
- Should the citizen information model exist in 1.0 as a simplified version, or is it entirely
  Post-1.0?

## Related

- [Households](households.md)
- [Time](time.md)
- [Social networks](social-networks.md)
- [Needs specification](../../reference/specifications/needs.md)
- [Housing](housing.md)
- [Adaptation concept](../concepts/adaptation.md)
