# Social networks — blat as an alternative allocation topology

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| Blat | Post-1.0 |
| Degree-bounded social graph | Post-1.0 |
| Non-monetary inequality | Post-1.0 |

All content on this page is Post-1.0. The design bible (§7.8) labels blat Post-1.0.

## What this is

Blat is the system of personal favours through which Soviet citizens obtained scarce goods
outside the formal allocation system. It is NOT corruption or bribery — it operates through
personal relationships and reciprocity, not money. A citizen who knows a shop worker can
obtain deficit goods through that contact. The goods are real: they are physically removed from
the store's inventory, and the next citizen in the formal queue goes without.

Blat is the game's most ambitious social mechanic. It creates non-monetary inequality: a
well-connected citizen has better access to goods than an isolated one, without any wealth
difference.

## Target design

### The degree-bounded graph — PLAUSIBLE (B1 §3c; Ledeneva 1998)

The design proposes a sparse social graph per citizen (B1 §3c):

```text
BlatGraph:
  edges: Vec<SmallVec<[BlatEdge; MAX_TIES]>>   -- indexed by CitizenID

BlatEdge:
  other: CitizenID
  relationship: BlatRelation   -- Kin, Coworker, Neighbour, Favour
  reciprocity_balance: i8      -- positive = they owe me
  last_activated: GameTick
```

Degree bound: MAX_TIES = 5–8 per citizen. Ledeneva documents that blat networks were small
and personal, not diffuse. This bound also controls performance: 250,000 citizens × 6 edges
= 1.5 million edges, each ~16 bytes = ~24 MB. Affordable.

### Physical custody chain — answering G-17

The design pillar says "nothing teleports." Blat must satisfy this:

1. The contact must have **physical access** to the needed item — they work at a place that
   has stock.
2. The reciprocity balance must be within bounds — net favours owed below threshold.
3. **Physical stock must be available** to divert.
4. The item is physically removed from the store's inventory — the same `Resources` debit
   that a normal retail purchase would make.
5. The requesting citizen **travels physically** to the contact's location to receive the
   goods.

The social graph says *whom to ask*, never *how goods move*. A blat transfer that requires
goods to cross town without a carrier is impossible. The physical chain is identical to a
normal retail transaction — the only difference is who gets served.

### Displacement is the game mechanic

The Planner sees that a store's inventory depleted faster than its throughput should allow.
The causal chain:

```text
blat diversion
  → faster inventory depletion
  → longer formal queue
  → more time poverty for non-connected citizens
  → observable anomaly: depletion rate exceeds expected throughput
```

The Planner sees **aggregate anomalies** (depletion faster than throughput), never individual
favours. This matches Ledeneva's observation that blat was invisible to the state.

### Non-monetary inequality — CONFIRMED (B1-11; B1-27; Zaslavskaya 1988; CIA 1982)

Soviet inequality was not wealth-based. Zaslavskaya identified stratification through access:

- Workplace privileges (enterprise welfare, closed distributors)
- Housing allocation priority
- Geographic location (Moscow vs province)
- Special distribution channels (zakrytye raspredeliteli)
- Educational credentials
- Party membership
- Informal connections (blat)

The design proposes modelling access channels rather than a single social-class variable.
A citizen's effective access to goods depends on their workplace, housing allocation,
geography, and social contacts — not on accumulated wealth.

### Komandirovki and information — Post-1.0

Business trips to better-supplied cities return with deficit goods and fresh shop knowledge.
This connects the social graph to the [citizen information model](provisioning.md).

## Current substrate

No social network exists. No informal economy. No blat. No access channels. All citizens are
identical in access. `BuyFood` places a market buy order matched globally by distance with
perfect knowledge. No social transmission of information.

## Research basis

- Ledeneva (1998), *Russia's Economy of Favours: Blat, Networking and Informal Exchange*,
  Cambridge University Press. Definitive source: blat is zero-sum for physical goods, sparse,
  reciprocal, pervasive, and invisible to the state.
- Zaslavskaya (1988), social-stratification framework: access-based inequality.
- Connor (1977), "The Soviet Worker: Social Stratification and Political Perceptions",
  Wilson Center.
- Grossman (1977), "The Second Economy of the USSR": ~33 % of Soviet citizens earned a
  quarter of their income informally.
- B1 §3c designed the graph structure; bible §7.8 resolved the physical-custody concern.

## Open questions

- Should the Planner see individual blat transactions (unrealistic but gameplay-useful) or
  only aggregate anomalies (realistic but harder to act on)? (B1 §6.5.)
- How do blat edges form and decay? Through workplace proximity, neighbourhood, kinship?
- What is the interaction between blat and the [deficit-goods list](provisioning.md)?

## Related

- [Provisioning](provisioning.md)
- [Housing](housing.md)
- [Labour](labor.md)
- [Households](households.md)
- [Time](time.md)
- [Migration](migration.md)
- [Physical causality concept](../concepts/physical-causality.md)
- [Glossary: blat](../../reference/glossary.md)
