# Heating

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R base-game heat network + CS1 Snowfall's temperature-driven demand and electric fallback
**Evidence:** see [research/utilities.md](../research/utilities.md) §B4/§E/§G/§H.

> District heating network — especially relevant to the cold-climate socialist setting. Warmth is a citizen need (spec/needs.md); heat is produced by burning real fuel and pumped through real pipes.

## Purpose

Heat plants burn coal into `heat`, pumped through trunk-and-branch pipe networks to endstations serving building clusters. Cold weather raises demand; a cold home is an unmet need with real consequences. District heating is core infrastructure — base-game, never a DLC afterthought (CS1 gates it behind Snowfall; W&R ships it base — we follow W&R).

## Draft model

### Production (from W&R — CONFIRMED)

Heat is a produced resource: `heating_plant_big.ini` — 30 workers, `$CONSUMPTION coal 0.28` → `$PRODUCTION heat 350`, medium pollution (research/utilities.md §E2). Ordinary recipe building per spec/production.md: no coal or staff ⇒ no heat. **Waste-to-heat:** W&R's `incinerator_heat` burns waste for `$PRODUCTION heat 450` — a direct spec/waste.md coupling, adopted (§F2).

### Distribution (from W&R — CONFIRMED grammar)

Third explicit pipe network, two gauges: `$CONNECTION_HEATING_BIG` (trunk) / `_SMALL` (branch), with typed chain `plant → pumping station → endstation → buildings` (§E1–E2). Pumping stations carry `$ENGINE_SPEED` — heat must be actively pumped, hinting at distance loss (INFERRED; solver native). OURS, as with power/water: explicit pipe capacity + per-km heat loss, making plant siting matter.

**Dropped:** CS1's heating-as-a-pulse-layer on the shared water grid (`m_conductivity2` — §B4).

### Demand: temperature-driven (from CS1 Snowfall — CONFIRMED, the good idea we keep)

CS1 scales a building's heat demand with weather: `(20°C − temperature) × 8`, clamped 0–400% (`CommonBuildingAI.cs:1907-1910` — §B4). Adopted: seasonal/weather temperature drives heat load continuously — no separate "winter mode."

### Fallback: unmet heat burns electricity (from CS1 — CONFIRMED, adopted)

CS1's sharpest heating mechanic: a shortfall in district heat converts into extra electricity draw (electric heaters), unless policy forbids it — then a real heating problem with production penalties (§B4). Adopted with W&R-era texture: electric heating is the *expensive* fallback; buildings with neither pipe heat nor spare power go cold → warmth need unmet (spec/needs.md) → health/wellbeing consequences.

### Who needs heat

W&R defaults most buildings to `$HEATING_DISABLE` (101 vs 3 enable — §E1, INFERRED: opt-in). We invert: in our cold-climate setting all inhabited/workplaces have temperature-driven load (CS1 model); only unheated structures (sheds, storage) opt out.

### Data (draft)

```
HeatPlant → ProductionBuilding emitting `heat` (coal- or waste-fired)
HeatPipe { gauge: trunk|branch; capacity; lossPerKm }   PumpStation { pushRate }
Endstation { served cluster }
Building += { heatDemand(t) = base × clamp((20 − T(t)) × k); heatBuffer; electricFallback: policy }
```

## Open questions
- ~~Seasonality: curve or multiplier?~~ → continuous temperature-driven demand (CS1 formula shape).
- Combined heat-and-power (CHP): neither game has it (CS1's incinerator makes power *or* the heat variant makes heat; W&R splits plants too). Very period-authentic — add as OURS building type?
- Return pipes / closed loop: real district heating is two-pipe; model one-way flow in v1?
- Coal stoves in old housing as pre-network era heating (spec/buildings.md era ladder)?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R: heat is a produced resource from coal + workers, base-game | CONFIRMED | `heating_plant_big.ini:3-25` | §E2 |
| W&R: third explicit network, trunk/branch gauges, pump/endstation chain | CONFIRMED tokens, INFERRED topology | `$CONNECTION_HEATING_BIG/SMALL`, `heating_pumpingstation.ini` | §E1-E2 |
| W&R: waste incinerator produces district heat | CONFIRMED | `incinerator_heat.ini` (`heat 450`, waste 2.5) | §F2 |
| W&R: most buildings opt out of heat by default | CONFIRMED counts, INFERRED meaning | `$HEATING_DISABLE` 101 vs `_ENABLE` 3 | §E1 — we invert |
| CS1: heating is Snowfall DLC, own conductivity layer on water grid | CONFIRMED | `HelperExtensions.cs:51`, `WaterManager.cs:30` | §B4 — grid rejected, DLC-gating rejected |
| CS1: heat demand scales with temperature `(20−T)×8`, clamp 0-400 | CONFIRMED | `CommonBuildingAI.cs:1900-1910` | §B4 — adopted |
| CS1: unmet district heat → extra electricity draw (fallback) | CONFIRMED | `CommonBuildingAI.cs:1940-1943` | §B4 — adopted |
| Pipe capacity + heat loss per km; CHP | OURS | — | §H |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/utilities.md · ../spec/electricity.md · ../spec/waste.md · ../spec/needs.md · ../spec/buildings.md
