# Causal loops catalogue

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Representative cross-system feedback loops. Each names its status: **implemented** (the code
closes the loop), **partial** (a link exists), **target** (specified or designed), **research**
(evidence only). Today exactly one link is implemented: electricity blackout stops production
(`simulation/src/souls/goods_company.rs`, `CompanyEnt::productivity`). Everything else is target or
research. The general principle — every important system participates in other systems — is
[coupling](concepts/index.md).

## Reliability spiral — target (CONFIRMED historically: Kornai 1980)

```text
unreliable delivery
→ enterprises and households raise buffers and requests
→ central availability falls
→ other actors face shortage
→ more defensive buffering; emergency dispatch and queueing rise
→ congestion
→ delivery less reliable
```

And in reverse: reliable delivery → buffers shrink → stock released → emergency dispatch falls →
congestion falls → reliability improves. **The code has a floor**: an enterprise cannot buy above
its storage (`recipe_should_produce`, `storage_multiplier`), so hoarding is bounded by warehouse
size. → [reliability](concepts/reliability.md), [reliability and buffering](planned-economy/reliability-and-buffering.md)

## Storming loop — target (CONFIRMED: shturmovshchina)

```text
quota risk near period end
→ overtime, maintenance deferral, larger input requests
→ freight pulse; dock queues
→ rail congestion
→ inputs arrive late
→ more quota risk; quality and rework risk
```

Propagates upstream recursively: space → electronics → wire → copper → mine → rail. Needs plan
periods, which do not exist. → [storming](planned-economy/storming.md)

## Space electronics cascade — target

```text
Space Programme quota risk → emergency electronics requirement → plants over-request and storm
→ copper and precision-component demand → rail dispatch pressure → consumer output loses inputs
→ congestion → components later → more storming → quality/rework → replacement demand → risk worsens
```

Worker layer: storming → overtime → fatigue → safety complaints → institutional pressure →
maintenance or welfare allocation → short-term production trade-off. → [national projects](national-projects/index.md)

## Coal → electricity → water → sewage → heat — target (each link specified in a draft utility spec)

```text
coal train delayed → thermal generation reserve falls → electricity curtailment
→ pump service constrained → water tank drains → sewage buffer fills
→ district service restriction → heating circulation degrades
→ household time, health and warmth consequences
```

Different networks respond at different speeds; the delay is the mechanic. → [infrastructure](infrastructure/index.md), [phase lag](concepts/phase-lag.md)

## Housing → labour → production — research (CONFIRMED: Feshbach on tekuchest'; the monotown death spiral)

```text
industrial expansion → labour demand → housing queue and long commute
→ turnover and recruitment difficulty → lower effective staffing
→ production shortfall → construction materials scarcer → housing construction slows
```

→ [housing](society/housing.md), [labour](society/labor.md)

## Retail shortage → labour — research (CONFIRMED: CIA 1979, 1982; time-budget studies)

```text
store shortage → household search and queue time rises
→ sleep and discretionary time fall → lateness, fatigue, absence
→ workplace effective capacity falls
```

Consumer logistics is an industrial input. → [time](society/time.md), [provisioning](society/provisioning.md)

## Social reproduction — research (CONFIRMED: Zaslavskaya)

```text
Plan → industrial production → goods, housing, services, infrastructure
→ household life: health, education, time, migration, family formation
→ labour force → enterprise capacity → Plan
```

→ [social reproduction](concepts/social-reproduction.md)

## National-project privilege — target

```text
strategic project receives housing, specialists and freight priority
→ project performance improves
→ other districts lose exactly those resources
→ queues, service strain, labour shortage elsewhere
→ local and institutional pressure rises
```

No "national project penalty" modifier is ever needed. → [priorities](planned-economy/priorities.md)

## Ratchet — target (CONFIRMED: Weitzman 1980; strong in the 1950s–60s era)

```text
heroic overfulfilment → next quota raised → revealed slack becomes obligation
→ enterprises conceal capacity → the Planner's information degrades
```

→ [reliability and buffering](planned-economy/reliability-and-buffering.md)

## Blat displacement — research (CONFIRMED: Ledeneva 1998) — Post-1.0

```text
formal supply fails → a connected household obtains through a contact
→ a real unit leaves a real store → the next citizen in the formal queue goes without
→ inventory depletes faster than throughput explains → the Planner sees an anomaly
```

→ [social networks](society/social-networks.md)

## Related

- [Concepts index](concepts/index.md)
- [Mechanics index](../reference/mechanics-index.md)
- [Current substrate](../architecture/current-substrate.md)
