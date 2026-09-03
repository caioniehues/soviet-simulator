# Alternate socialist institutions — the "who decides" questions

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| "Who decides" parameter set | Post-1.0, HYPOTHESIS |
| Sovnarkhoz 1957–65 | Post-1.0, CONFIRMED historically |
| Kosygin reform 1965 | Post-1.0, CONFIRMED historically |
| Danwei | Post-1.0, CONFIRMED historically |
| Wartime rationing | Post-1.0, CONFIRMED historically |

All content on this page is Post-1.0. Each subsection describes a different institutional
answer to the same questions. These are future game-mode rulesets, not base-game mechanics.

## What this is

Every socialist system must answer the same seven questions about authority. The answers differ
by system and period. Each combination is a potential game mode — a different institutional
configuration over the same physical economy.

## Target design

### The seven "who decides" questions — HYPOTHESIS (bible §8.14; Lane H)

| Question | Default (branch ministry) | Determines |
|---|---|---|
| 1. Who proposes quotas? | Gosplan via ministries | What gets produced |
| 2. Who sets reserves? | State + enterprise | How much is buffered |
| 3. Who selects management? | Party appointment | Who runs the enterprise |
| 4. Who allocates surplus? | Plan | Where excess goes |
| 5. Who controls housing/welfare? | Enterprise + union + district | Who gets housed |
| 6. Who approves overtime/norms? | Management + profkom | How hard people work |
| 7. Who coordinates inter-enterprise inputs? | Allocation orders from Gosplan | How supply chains run |

Lane H and bible §8.14 reached the same conclusion independently: game modes are
institutional-parameter presets over one physical simulation. Each mode changes ~4–8 of these
parameters. No separate simulation is needed.

### Sovnarkhoz 1957–65 — CONFIRMED (Kibita 2013)

Khrushchev replaced ~30 central industrial ministries with 105 regional economic councils
(sovnarkhozy), later consolidated to 47. Reversed by Brezhnev/Kosygin in 1965.

**What changed:** Resource allocation shifted from vertical (ministry controls all steel plants
nationwide) to horizontal (regional council controls all industry in its territory).

**What it teaches:** Intra-regional allocation improves (shorter transport, better local
information). Inter-regional allocation degrades (regions hoard resources for local use —
mestnichestvo). The Planner manages a different shape of the same hoarding problem.
Decentralisation trades vedomstvennost' (departmentalism) for mestnichestvo (localism).

**Parameter changes:** allocation authority → regional by territory; information flow →
bottom-up to regional council; reserve rules → regional + enterprise; priority classes → by
regional need; housing allocator → regional council.

### Kosygin reform 1965 — CONFIRMED (Encyclopedia.com; Britannica)

Brezhnev and Kosygin restored central ministries but gave enterprises limited autonomy. The
number of obligatory plan indicators was reduced from dozens to 4–8: output volume, assortment,
profit, quality. Enterprises could retain profit for self-financing investment funds.

**What failed:** Ministries reasserted control. "Changing the names on doors." Prices remained
fixed, so "profit" was meaningless as a signal. The reform was quietly abandoned by the early
1970s.

**What it teaches:** Fewer controls can produce better or worse outcomes depending on whether
indicators carry information. In this game, price signals cannot carry information (clearing is
by queue, never price). The player discovers *why* the Kosygin reform failed.

**Parameter changes:** Planner's quota levers reduced to 4–8 aggregate indicators; enterprises
gain profit-retention autonomy; quality becomes a plan indicator alongside volume.

### Danwei — CONFIRMED (Chinese work unit, 1950s–1990s)

The danwei was the foundational cell of urban Chinese life. Each unit provided employment,
housing, ration coupons, medical care, pensions, childcare, and permission to marry or travel.
A "small society" with little need for inter-unit exchange.

**What it changes:** Welfare provision bundles into enterprises. The Planner does not allocate
housing, healthcare, or childcare separately — each enterprise provides its own.

**What it teaches:** Simplified planning (no district-level service coverage) creates new
problems: large enterprises become welfare monopolies; closing a factory displaces not just
workers but their entire social infrastructure; enterprise inequality becomes lived inequality.

**Parameter changes:** housing allocator → work unit (bundled); welfare → unit only; reserve
rules → unit only.

### Wartime rationing by labour category — CONFIRMED (July 1941)

From July 1941, Soviet rationing was tiered by labour category: defence workers received the
most, followed by industrial workers, then office workers, then dependants.

**What it changes:** Allocation by labour category replaces allocation by queue position or
enterprise request. This is a physical allocation mechanism that determines who eats.

**What it teaches:** Rationing is a political decision with physical consequences. The Planner
decides which categories eat well and which go without. No market, no queue — direct
allocation by administrative classification.

In three months (July–October 1941), GOSPLAN relocated 1,360 factories: 455 to the Urals, 210
to Western Siberia, 250 to Central Asia. Over 10 million people were evacuated.

### Systems mentioned, not elaborated

**Hungarian reform socialism** (the New Economic Mechanism, 1968): a different set of answers
involving enterprise autonomy, limited market pricing, and a role for profit signals. Distinct
from the Kosygin reform in scope and duration. Requires its own research before any game-mode
design.

**Polish workplace politics:** workers' councils existed from 1956 alongside enterprise
directors and party committees. A three-way institutional tension different from both the
Soviet and Yugoslav models. Requires its own research.

Both are noted as distinct systems with their own institutional answers. They are not variants
of the Soviet or Yugoslav models.

## Current substrate

No institutional parameters exist. `Government` holds only `money: Money`
(`simulation/src/economy/government.rs:9-11`). No allocation authority, no information
topology, no reserve rules, no priority classes. The `Market` is a single global market with
`BTreeMap<ItemID, SingleMarket>`. No concept of regional, branch, or enterprise-level
allocation authority.

## Research basis

- Sovnarkhoz: Kibita (2013), *Soviet Economic Management Under Khrushchev: The Sovnarkhoz
  Reform* (Routledge); Wikipedia, "1957 Soviet economic reform."
- Kosygin: Encyclopedia.com, "Kosygin Reforms"; Britannica, "Aleksey Kosygin."
- Danwei: Lvivcenter; MDPI Sustainability 12:4; Grokipedia.
- Wartime rationing: GlobalSecurity.org; Soviet History MSU.
- "Who decides" framework: bible §8.14; Lane H §3.

## Open questions

- Should modes be switchable mid-save? Historically they were (Sovnarkhoz → branch
  ministries). Mid-save switching requires institutional transition mechanics.
- How much enterprise AI is needed? Sovnarkhoz and Kosygin require enterprises that respond
  to changed parameters. Self-management requires enterprises that make autonomous decisions.
- Should Hungarian and Polish systems receive game-mode research, or are they beyond scope?

## Related

- [Worker self-management](worker-self-management.md)
- [Soviet workplaces](soviet-workplaces.md)
- [Trade unions](trade-unions.md)
- [Local Soviets](local-soviets.md)
- [Labour](../labor.md)
- [Workplaces](../workplaces.md)
- [Game modes](../../../product/game-modes.md)
