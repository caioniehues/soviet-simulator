#!/usr/bin/env python3
"""Rebuild and validate the Wave 3 requirements and STORY migration ledger.

This is deliberately a small, deterministic generator.  The legacy STORY corpus is read only to
enumerate identities and titles for migration coverage; its acceptance criteria never feed the
new requirements.  The requirement catalogue below is derived from the charter and stable SPEC
anchors only.
"""
from __future__ import annotations

import argparse
import re
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
LEGACY = ROOT / "docs/superpowers/iterations/requirements"
DEFAULT_OUTPUT_DIR = Path(__file__).resolve().parent
DEFAULT_MIGRATION_OUTPUT = ROOT / "docs/plan/traceability/story-migration.md"


def ids(values: str) -> set[int]:
    """Expand comma-separated numbers and inclusive ranges."""
    result: set[int] = set()
    for value in values.split(","):
        if "-" in value:
            start, end = map(int, value.split("-"))
            result.update(range(start, end + 1))
        else:
            result.add(int(value))
    return result


CATALOGUE = {
    "REQ-CONSTRUCTION-001": {
        "file": "built-world.md",
        "title": "Physical construction and activation",
        "scope": "Charter §1.0 — Planner interaction; Resources and production",
        "anchors": "SPEC-CONSTRUCTION-001, SPEC-CONSTRUCTION-002, SPEC-CONSTRUCTION-003, SPEC-CONSTRUCTION-004, SPEC-CONSTRUCTION-005, SPEC-CONSTRUCTION-006, SPEC-CONSTRUCTION-007, SPEC-CONSTRUCTION-008, SPEC-BUILDINGS-002, SPEC-BUILDINGS-003",
        "criteria": [
            "A proposal records its footprint, material bill, verdict, and refusal reason before it creates one non-operating Site.",
            "Only received physical material and recorded work make a Site ground broken or complete; completion publishes one Buildings result and never activates an asset early.",
            "Partial delivery, interruption, rescind, and refusal conserve the material bill and remain inspectable.",
        ],
        "evidence": "Mutation-proven executable construction conservation guard plus inspected Ghost/Site capture",
    },
    "REQ-BUILDINGS-001": {
        "file": "built-world.md",
        "title": "Declared buildings and observable activation",
        "scope": "Charter §1.0 — Planner interaction; Agriculture and services",
        "anchors": "SPEC-BUILDINGS-001, SPEC-BUILDINGS-002, SPEC-BUILDINGS-003, SPEC-BUILDINGS-004, SPEC-BUILDINGS-005, SPEC-BUILDINGS-006",
        "criteria": [
            "A building declaration owns its identity, declared capability, connection declarations, and operating-state prerequisites.",
            "A completed Site activates exactly once through Buildings and remains observable when a declared prerequisite blocks operation.",
            "Planner inspection shows the declaration, Site, activation state, and every blocking prerequisite.",
        ],
        "evidence": "Mutation-proven activation-once guard plus inspected building-state capture",
    },
    "REQ-ZONING-001": {
        "file": "built-world.md",
        "title": "Planner land-use intent and siting feedback",
        "scope": "Charter §1.0 — Planner interaction",
        "anchors": "SPEC-ZONING-001, SPEC-ZONING-002, SPEC-ZONING-003, SPEC-ZONING-004, SPEC-ZONING-005, SPEC-ZONING-006, SPEC-ROADS-005",
        "criteria": [
            "Planner land-use intent is an inspectable boundary record that Construction consults for Ghost verdicts.",
            "Changing intent never spawns, activates, demolishes, or deletes Sites or buildings; automatic lot spawning is not accepted as target placement.",
            "Shortage indicators are decision support only and expose their inputs without becoming an autonomous placement loop.",
        ],
        "evidence": "Mutation-proven non-spawn and siting-verdict guards plus inspected zoning capture",
    },
    "REQ-ELECTRICITY-001": {
        "file": "utilities.md",
        "title": "Finite non-price electricity service",
        "scope": "Charter §1.0 — Resources and production; Terrain and environment",
        "anchors": "SPEC-ELECTRICITY-001, SPEC-ELECTRICITY-002, SPEC-ELECTRICITY-003, SPEC-ELECTRICITY-004, SPEC-ELECTRICITY-005, SPEC-ELECTRICITY-006, SPEC-ELECTRICITY-007",
        "criteria": [
            "Electricity owns explicit topology, bounded generation/storage/service, and a once-only allocation result.",
            "Shortage sheds declared non-price priority loads with visible served, curtailed, and unmet rates; it neither creates energy nor activates a disconnected building.",
            "Every offered generation result is bounded by a once-accepted Production result and its declared physical source/input.",
        ],
        "evidence": "Mutation-proven energy-conservation, shortage, and authority guards plus inspected network capture",
    },
    "REQ-HEATING-001": {
        "file": "utilities.md",
        "title": "Finite thermal service without electric substitution",
        "scope": "Charter §1.0 — Resources and production; Agriculture and services",
        "anchors": "SPEC-HEATING-001, SPEC-HEATING-002, SPEC-HEATING-003, SPEC-HEATING-004, SPEC-HEATING-005, SPEC-HEATING-006, SPEC-HEATING-007",
        "criteria": [
            "Heating owns a rate-bounded thermal graph, buffer, declared loss, and once-only allocation result.",
            "Thermal shortfall remains visible unmet heat and can make homes colder; Electricity never substitutes for it.",
            "Variable temperature demand requires a ratified Weather observation; otherwise the result declares static demand.",
        ],
        "evidence": "Mutation-proven heat-flow conservation, shortage, and Weather-prerequisite guards plus inspected thermal capture",
    },
    "REQ-LOGISTICS-001": {
        "file": "movement.md",
        "title": "Physical, finite, cancellable freight fulfillment",
        "scope": "Charter §1.0 — Transport and border; Resources and production",
        "anchors": "SPEC-LOGISTICS-001, SPEC-LOGISTICS-002, SPEC-LOGISTICS-003, SPEC-LOGISTICS-004, SPEC-LOGISTICS-005, SPEC-LOGISTICS-006, SPEC-LOGISTICS-007, SPEC-LOGISTICS-008, SPEC-LOGISTICS-009, SPEC-LOGISTICS-010, SPEC-LOGISTICS-011, SPEC-VEHICLES-001, SPEC-VEHICLES-002, SPEC-VEHICLES-003, SPEC-VEHICLES-005, SPEC-VEHICLES-006",
        "criteria": [
            "A finite compatible vehicle traverses one ordered compatible itinerary to source before pickup and to destination before delivery; elapsed time or route creation alone proves neither transition.",
            "Allocation and reservation do not transfer stock. Pickup, custody, delivery, cancellation, and recovery preserve one accountable quantity and vehicle identity without teleporting it.",
            "Target-stock demand uses declared non-price deficit, route distance, stable tie-break, and bounded docks; unavailable capacity waits with a visible reason.",
        ],
        "evidence": "Mutation-proven same-vehicle traversal, timer-delivery, cancellation/recovery, deficit-ordering, and dock-rate guards plus inspected haul timeline",
    },
    "REQ-TRADE-001": {
        "file": "economy.md",
        "title": "Physical border clearance and the single rouble",
        "scope": "Charter §1.0 — Transport and border",
        "anchors": "SPEC-TRADE-001, SPEC-TRADE-002, SPEC-TRADE-003, SPEC-TRADE-004, SPEC-TRADE-005, SPEC-TRADE-006, SPEC-TRADE-007, SPEC-TRADE-008, SPEC-RESOURCES-003, SPEC-RESOURCES-004",
        "criteria": [
            "Domestic matching, allocation, reservation, dispatch, production, and needs use no money or price gate.",
            "A fixed per-kind rouble amount settles exactly once only after physical customs clearance of the declared order.",
            "Non-Water orders use a completed Logistics haul; Water clears only after a completed Water-owned metered transfer and never enters freight custody.",
        ],
        "evidence": "Mutation-proven border-clearance, one-settlement, Medicine, and tagged-transport guards plus inspected customs capture",
    },
    "REQ-ROADS-001": {
        "file": "movement.md",
        "title": "Planner-authored roads and physical parking",
        "scope": "Charter §1.0 — Transport and border; Planner interaction",
        "anchors": "SPEC-ROADS-001, SPEC-ROADS-002, SPEC-ROADS-003, SPEC-ROADS-004, SPEC-ROADS-005, SPEC-ROADS-006",
        "criteria": [
            "Road topology is an authoritative Planner-authored typed physical network with declared capacity inputs and refusal feedback.",
            "Road placement or alteration preserves or explicitly invalidates affected route and parking references.",
            "Roads alone reserves physical parking; no consumer or dispatcher instant-parks a vehicle.",
        ],
        "evidence": "Mutation-proven topology, invalidation, and parking-authority guards plus inspected road verdict capture",
    },
    "REQ-PATHFINDING-001": {
        "file": "movement.md",
        "title": "Compatible route derivation",
        "scope": "Charter §1.0 — Transport and border",
        "anchors": "SPEC-PATHFINDING-001, SPEC-PATHFINDING-002, SPEC-PATHFINDING-003, SPEC-PATHFINDING-004, SPEC-PATHFINDING-005, SPEC-PATHFINDING-006, SPEC-TRAFFIC-007, SPEC-TRAFFIC-008",
        "criteria": [
            "A route records origin, destination, compatible lane types, topology revision, and Traffic-derived damped cost.",
            "Invalid, blocked, or absent paths leave a recoverable reason and never transfer custody or satisfy a request.",
            "New routes exclude Traffic-blocked lanes and consume Traffic's published cost rather than copying congestion state.",
        ],
        "evidence": "Mutation-proven route compatibility, blocked-lane, and no-transfer guards plus inspected route/reason capture",
    },
    "REQ-TRAFFIC-001": {
        "file": "movement.md",
        "title": "Observable congestion and physical recovery",
        "scope": "Charter §1.0 — Transport and border",
        "anchors": "SPEC-TRAFFIC-001, SPEC-TRAFFIC-002, SPEC-TRAFFIC-003, SPEC-TRAFFIC-004, SPEC-TRAFFIC-005, SPEC-TRAFFIC-006, SPEC-TRAFFIC-007, SPEC-TRAFFIC-008",
        "criteria": [
            "Moving vehicles remain on compatible physical lanes while queue, pressure, and stall age remain durable state.",
            "Stall recovery reroutes through Pathfinding or exposes a Planner-visible bottleneck; it never deletes a vehicle or clears a domestic request.",
            "Traffic publishes EMA load, BPR cost, Gawron damping, and blockage while retaining authority over the dynamic inputs.",
        ],
        "evidence": "Mutation-proven stall, no-deletion, EMA/BPR/Gawron, and authority guards plus inspected congestion capture",
    },
    "REQ-CITIZENS-001": {
        "file": "settlement.md",
        "title": "Persistent citizens under shortage and death",
        "scope": "Charter §1.0 — Agriculture and services; Presentation and audio",
        "anchors": "SPEC-CITIZENS-001, SPEC-CITIZENS-002, SPEC-CITIZENS-003, SPEC-CITIZENS-004, SPEC-CITIZENS-005, SPEC-CITIZENS-006, SPEC-CITIZENS-007",
        "criteria": [
            "Each citizen retains one persistent identity through save/load, assignment, unmet need, and death lifecycle.",
            "Planner policy allocates eligible labour and study work by declared non-price criteria; unreachable work or shortage remains an observable outcome.",
            "Citizens alone publishes the death transition consumed once by dependent systems.",
        ],
        "evidence": "Mutation-proven identity, allocation, shortage, and death-once guards plus inspected citizen capture",
    },
    "REQ-NEEDS-001": {
        "file": "settlement.md",
        "title": "Distinct dwelling needs and going without",
        "scope": "Charter §1.0 — Resources and production; Agriculture and services",
        "anchors": "SPEC-NEEDS-001, SPEC-NEEDS-002, SPEC-NEEDS-003, SPEC-NEEDS-004, SPEC-NEEDS-005, SPEC-NEEDS-006",
        "criteria": [
            "Food and Meat remain distinct dwelling needs and satisfaction follows one authoritative compatible consumption event after physical availability.",
            "Domestic need clearing uses no roubles or prices; unsatisfied need persists as waiting, substitution, or inspectable going without.",
            "Each consumption ID changes compatible Resources stock once and cannot be replayed.",
        ],
        "evidence": "Mutation-proven distinct-need, non-price, persistent-shortage, and once-consumption guards plus inspected need capture",
    },
    "REQ-HOUSEHOLDS-001": {
        "file": "settlement.md",
        "title": "Households, housing, and shared pantries",
        "scope": "Charter §1.0 — Agriculture and services",
        "anchors": "SPEC-HOUSEHOLDS-001, SPEC-HOUSEHOLDS-002, SPEC-HOUSEHOLDS-003, SPEC-HOUSEHOLDS-004, SPEC-HOUSEHOLDS-005, SPEC-HOUSEHOLDS-006, SPEC-HOUSEHOLDS-007, SPEC-HOUSEHOLDS-008",
        "criteria": [
            "A household has a persistent member set, a residence queue, and a capacity-bounded residence assignment.",
            "Food and Meat pantries are distinct shared physical records and cannot be credited by request, allocation, or a duplicated fulfillment.",
            "Housing priority is observable Planner policy and a Citizens death result removes membership once without deleting household history.",
        ],
        "evidence": "Mutation-proven capacity, pantry, fulfillment-once, and death-consumption guards plus inspected household capture",
    },
    "REQ-PRODUCTION-001": {
        "file": "economy.md",
        "title": "Input-bounded production and observable dishonest enterprises",
        "scope": "Charter §1.0 — Resources and production",
        "anchors": "SPEC-PRODUCTION-001, SPEC-PRODUCTION-002, SPEC-PRODUCTION-003, SPEC-PRODUCTION-004, SPEC-PRODUCTION-005, SPEC-PRODUCTION-006, SPEC-PRODUCTION-007, SPEC-PRODUCTION-008, SPEC-PRODUCTION-009",
        "criteria": [
            "A run consumes only delivered compatible input and remains bounded by declared recipe, labour, utilities, capacity, and storage, with its binding constraint visible.",
            "Run IDs apply input/output/byproduct changes atomically and once through Resources; domestic money never gates production.",
            "Underperformance retains an observable, allocation-eligible enterprise without conjured stock; request, receipt, consumption, surplus, and age discrepancies let the Planner infer hoarding without an honesty flag.",
        ],
        "evidence": "Mutation-proven delivered-input, atomic-run, soft-budget, and dishonest-enterprise guards plus inspected discrepancy capture",
    },
    "REQ-RESOURCES-001": {
        "file": "economy.md",
        "title": "Physical resource catalogue and accountable stock",
        "scope": "Charter §1.0 — Resources and production; Transport and border",
        "anchors": "SPEC-RESOURCES-001, SPEC-RESOURCES-002, SPEC-RESOURCES-003, SPEC-RESOURCES-004, SPEC-RESOURCES-005, SPEC-RESOURCES-006",
        "criteria": [
            "The catalogue declares the charter resource identities, units, handling compatibility, and import-only Medicine before use.",
            "Resources alone mutates on-hand balances; request, allocation, reservation, custody, delivery, and consumption remain separate accountable records.",
            "Failure and cancellation preserve quantity, Water never becomes cargo, and Medicine enters domestic stock only after physical import clearance.",
        ],
        "evidence": "Mutation-proven conservation, cancellation, Water restriction, and Medicine restriction guards plus inspected stock capture",
    },
    "REQ-EDUCATION-001": {
        "file": "settlement.md",
        "title": "Capacity-limited school and technical education",
        "scope": "Charter §1.0 — Agriculture and services",
        "anchors": "SPEC-EDUCATION-001, SPEC-EDUCATION-002, SPEC-EDUCATION-003, SPEC-EDUCATION-004, SPEC-EDUCATION-005",
        "criteria": [
            "The 1.0 catalogue contains exactly School and Technical education, with persistent enrolment, seat reservation, queue, and progress records.",
            "Progress requires attendance at a staffed operating compatible facility; absence of seat, staff, building, or route remains a visible queue or going-without result.",
            "Planner policy orders scarce seats by explicit non-price criteria.",
        ],
        "evidence": "Mutation-proven capacity, attendance, shortage, and non-price-priority guards plus inspected enrolment capture",
    },
    "REQ-HEALTHCARE-001": {
        "file": "settlement.md",
        "title": "Finite healthcare with physical Medicine",
        "scope": "Charter §1.0 — Agriculture and services",
        "anchors": "SPEC-HEALTHCARE-001, SPEC-HEALTHCARE-002, SPEC-HEALTHCARE-003, SPEC-HEALTHCARE-004, SPEC-HEALTHCARE-005, SPEC-HEALTHCARE-006",
        "criteria": [
            "Care requests retain citizen, reason, queue, priority, and outcome under Healthcare authority.",
            "Treatment requires finite staffed capacity, physical arrival or declared remote care, and compatible on-hand Medicine consumed once.",
            "Scarcity uses declared health priority, leaves waiting or worsening outcomes visible, and never clears by domestic price.",
        ],
        "evidence": "Mutation-proven capacity, Medicine-once, arrival, and non-price guards plus inspected care capture",
    },
    "REQ-WATER-001": {
        "file": "utilities.md",
        "title": "Metered, finite Water transfer",
        "scope": "Charter §1.0 — Resources and production; Transport and border; Terrain and environment",
        "anchors": "SPEC-WATER-001, SPEC-WATER-002, SPEC-WATER-003, SPEC-WATER-004, SPEC-WATER-005, SPEC-WATER-006, SPEC-TRADE-007, SPEC-TRADE-008",
        "criteria": [
            "Water owns a connected compatible topology, quality, buffers, finite tick capacity, transfer progress, and directional border meter.",
            "Disconnected, zero-capacity, or partial paths do not deliver or clear; they retain visible unmet transfer without tanker, cargo, early rouble settlement, or created water.",
            "Trade clears once only after the complete Water-owned metered transfer; each transfer application is idempotent and quantity-conserving.",
        ],
        "evidence": "Mutation-proven disconnected, zero-capacity, partial-flow, meter, clearance, and replay guards plus inspected water timeline",
    },
    "REQ-SEWAGE-001": {
        "file": "utilities.md",
        "title": "Finite sewage buffering, treatment, and discharge",
        "scope": "Charter §1.0 — Agriculture and services; Terrain and environment",
        "anchors": "SPEC-SEWAGE-001, SPEC-SEWAGE-002, SPEC-SEWAGE-003, SPEC-SEWAGE-004, SPEC-SEWAGE-005, SPEC-SEWAGE-006",
        "criteria": [
            "Sewage owns a separate finite graph, buffers, transfer/treatment/discharge records, and non-price priority.",
            "Blocked capacity retains physical backlog and a declared service restriction; no other system copies or mutates sewage state.",
            "Treatment/discharge and an optional Water handoff apply once and conserve accepted quantity into named output, residue, and loss.",
        ],
        "evidence": "Mutation-proven graph, backpressure, disposition-conservation, and authority guards plus inspected sewage capture",
    },
    "REQ-WASTE-001": {
        "file": "utilities.md",
        "title": "Physical waste collection and single disposition",
        "scope": "Charter §1.0 — Agriculture and services",
        "anchors": "SPEC-WASTE-001, SPEC-WASTE-002, SPEC-WASTE-003, SPEC-WASTE-004, SPEC-WASTE-005, SPEC-WASTE-006, SPEC-WASTE-007",
        "criteria": [
            "Waste owns typed finite containers, collection requests, overflow, processing disposition, and landfill retention.",
            "Collection requests exactly one compatible Logistics haul and uses a once-only receipt for container-to-custody pickup.",
            "Each delivered quantity has one conserving disposition through landfill or a bound Production result; blockage retains waste instead of deleting it or direct-crediting outputs.",
        ],
        "evidence": "Mutation-proven compatible-haul, overflow/recovery, disposition-conservation, and non-price guards plus inspected waste capture",
    },
}


# A current requirement is only a charter-plus-SPEC contract.  The numbers below intentionally
# classify legacy stories by the replacement contract rather than by legacy EPIC or line citation.
ASSIGNMENTS = {}
for key, covered in {
    "REQ-CONSTRUCTION-001": "1-11,15",
    "REQ-BUILDINGS-001": "12",
    "REQ-ZONING-001": "13-14,16",
    "REQ-ELECTRICITY-001": "17-18,21-22,24-25",
    "REQ-HEATING-001": "27-30,32",
    "REQ-LOGISTICS-001": "34-35,39,140-141,145-149",
    "REQ-TRADE-001": "47-52",
    "REQ-ROADS-001": "59,63",
    "REQ-PATHFINDING-001": "58,60",
    "REQ-TRAFFIC-001": "65-71",
    "REQ-CITIZENS-001": "72-77",
    "REQ-NEEDS-001": "81,83",
    "REQ-HOUSEHOLDS-001": "84-86",
    "REQ-PRODUCTION-001": "93-107,113",
    "REQ-RESOURCES-001": "36,108,111-112",
    "REQ-EDUCATION-001": "78,121-123",
    "REQ-HEALTHCARE-001": "73,124-126",
    "REQ-WATER-001": "127-129",
    "REQ-SEWAGE-001": "131-133",
    "REQ-WASTE-001": "114-115,134-136",
    "REQ-VEHICLES-001": "33,143",
}.items():
    for number in ids(covered):
        ASSIGNMENTS[number] = key

# Vehicles is declared late because its requirement belongs in the freight contract while Roads
# remains the exclusive parking authority.
CATALOGUE["REQ-VEHICLES-001"] = {
    "file": "movement.md",
    "title": "Finite freight vehicles and fixed rail consists",
    "scope": "Charter §1.0 — Transport and border",
    "anchors": "SPEC-VEHICLES-001, SPEC-VEHICLES-002, SPEC-VEHICLES-003, SPEC-VEHICLES-004, SPEC-VEHICLES-005, SPEC-VEHICLES-006, SPEC-LOGISTICS-003, SPEC-LOGISTICS-007",
    "criteria": [
        "Every operational freight vehicle has durable identity, state, compatible finite capacity, depot/recovery reference, and a Roads-owned parking reference.",
        "A missing vehicle or failed route preserves the haul and physical recovery state rather than spawning or deleting a substitute.",
        "The 1.0 rail catalogue is one locomotive type and one wagon type in a fixed compatible consist; passenger rail is not a requirement.",
    ],
    "evidence": "Mutation-proven vehicle identity, capacity, recovery, parking-authority, and fixed-consist guards plus inspected fleet capture",
}

FULL_DEFERRED = {
    19: ("Post-1.0 direction: voltage tiers and grid depth including transformers", "Charter §Explicit cuts — voltage tiers and grid depth"),
    20: ("Post-1.0 direction: voltage tiers and grid depth including transformers", "Charter §Explicit cuts — voltage tiers and grid depth"),
    26: ("Post-1.0 direction: vehicle fuel lifecycle", "Charter §Explicit cuts — vehicle fuel lifecycle"),
    31: ("Post-1.0 direction: electric-heating fallback", "Charter §Explicit cuts — electric-heating fallback"),
    38: ("Post-1.0 direction: passenger rail", "Charter §Explicit cuts — passenger rail"),
    45: ("Post-1.0 direction: a dynamic world-market price model is not a charter commitment", "Charter §Scope discipline — domestic clearing is non-price"),
    53: ("Post-1.0 direction: dual currency", "Charter §Explicit cuts — dual currency"),
    54: ("Post-1.0 direction: dual currency and per-currency loans", "Charter §Explicit cuts — dual currency"),
    56: ("Post-1.0 direction: era calendar", "Charter §Explicit cuts — era calendar"),
    57: ("Post-1.0 direction: vehicle lifecycle", "Charter §Explicit cuts — vehicle fuel lifecycle"),
    88: ("Post-1.0 direction: crime", "Charter §Explicit cuts — crime"),
    89: ("Post-1.0 direction: crime", "Charter §Explicit cuts — crime"),
    90: ("Post-1.0 direction: crime", "Charter §Explicit cuts — crime"),
    91: ("Post-1.0 direction: crime", "Charter §Explicit cuts — crime"),
    92: ("Post-1.0 direction: crime", "Charter §Explicit cuts — crime"),
    110: ("Post-1.0 direction: perishables and refrigerated transport", "Charter §Explicit cuts — perishables and refrigerated transport"),
    120: ("Post-1.0 direction: vehicle lifecycle and era calendar", "Charter §Explicit cuts — vehicle lifecycle; era calendar"),
    142: ("Post-1.0 direction: vehicle manufacture", "Charter §Explicit cuts — vehicle manufacture"),
    144: ("Post-1.0 direction: vehicle lifecycle", "Charter §Explicit cuts — vehicle fuel lifecycle"),
    # Existing stories that were not marked deferred nevertheless make an explicit charter cut.
    116: ("Post-1.0 direction: machinery/vehicle lifecycle condition", "Charter §Explicit cuts — vehicle fuel lifecycle"),
    119: ("Post-1.0 direction: era calendar", "Charter §Explicit cuts — era calendar"),
    130: ("Post-1.0 direction: voltage tiers and substations", "Charter §Explicit cuts — voltage tiers and grid depth"),
}

RETIRED = {
    23: ("Border electricity transformers are not a stable in-scope SPEC contract; retain no 1.0 acceptance criterion.", "Charter §Scope discipline"),
    37: ("Fixed conveyance edges have no charter-plus-SPEC contract; retain no 1.0 acceptance criterion.", "Charter §Scope discipline"),
    40: ("Domestic cash conflicts with non-price domestic clearing and the border-only rouble.", "Charter §Identity — domestic clearing; rouble"),
    41: ("Domestic wages in cash conflict with non-price domestic clearing.", "Charter §Identity — domestic clearing"),
    42: ("Domestic enterprise settlement accounts conflict with non-price domestic clearing.", "Charter §Identity — domestic clearing"),
    43: ("Two-circuit conversion rules are inapplicable after the domestic-money cut.", "Charter §Identity — domestic clearing"),
    44: ("Administered domestic prices and shadow-price settlement conflict with non-price domestic clearing.", "Charter §Identity — domestic clearing"),
    46: ("Starter warehouse stock is not a stable charter-plus-SPEC requirement.", "Charter §Scope discipline"),
    55: ("Standing-contract scheduling has no ratified contract beyond individual Trade orders.", "Charter §Scope discipline"),
    61: ("Async solver and per-tick budget are performance design choices without a stable SPEC claim.", "Charter §Scope discipline"),
    62: ("An eight-segment intersection cap has no charter-plus-SPEC contract.", "Charter §Scope discipline"),
    64: ("Compound-internal routing has no charter-plus-SPEC contract.", "Charter §Scope discipline"),
    75: ("Birth mechanics are not yet a stable Citizens contract; only demographics including death is scoped.", "Charter §1.0 scope — Agriculture and services"),
    79: ("A labour-pool office has no charter-plus-SPEC contract.", "Charter §Scope discipline"),
    80: ("Named human driver assignment has no stable Logistics or Vehicles claim.", "Charter §Scope discipline"),
    87: ("Population-performance mechanics require a dedicated ratified performance contract.", "Charter §Scope discipline"),
    109: ("Legacy economic tiers are not a charter-plus-Resources catalogue requirement.", "Charter §Scope discipline"),
    117: ("Profession-specific skilled-labour gating has no stable Education/Production claim.", "Charter §Scope discipline"),
    118: ("Weather-linked renewable modulation awaits a ratified Weather contract.", "Charter §Scope discipline"),
    137: ("Attractiveness and sickness effects from waste need a ratified cross-system contract.", "Charter §Scope discipline"),
    138: ("Weather has no current stable specification anchor.", "Charter §Scope discipline"),
}

SPLITS = {
    82: ("REQ-NEEDS-001; DIR-POST-LOYALTY-001 (legacy AC-4)", "SPEC-NEEDS-001, SPEC-NEEDS-002, SPEC-NEEDS-004, SPEC-NEEDS-005", "The in-scope distinct-need and going-without direction is rewritten; the loyalty/legitimacy AC is Post-1.0 direction only.", "Charter §Identity — persistent identities; §Explicit cuts — loyalty, legitimacy, broadcast, monuments"),
    139: ("REQ-VEHICLES-001; DIR-POST-VEHICLE-FUEL-001 (legacy AC-1)", "SPEC-VEHICLES-001, SPEC-VEHICLES-002, SPEC-VEHICLES-005, SPEC-VEHICLES-006", "Finite identity and cargo compatibility are rewritten; tagged legacy AC-1 fuel lifecycle is Post-1.0 direction, and the untagged condition detail is not retained because lifecycle is an explicit cut.", "Charter §Transport and border; §Explicit cuts — vehicle fuel lifecycle"),
}


def legacy_stories() -> dict[int, str]:
    found: dict[int, str] = {}
    for path in sorted(LEGACY.glob("EPIC-*.md")):
        text = path.read_text()
        blocks = re.split(r"(?=^## STORY-)", text, flags=re.M)
        for block in blocks:
            match = re.match(r"## STORY-(\d{4})\n.*?^\*\*Title:\*\* ([^\n]+)$", block, re.M | re.S)
            if match:
                found[int(match.group(1))] = match.group(2).strip()
    return found


def legacy_flagged_ids(marker: str) -> set[int]:
    """Read legacy-only flags so their migration cannot disappear silently."""
    found: set[int] = set()
    for path in sorted(LEGACY.glob("EPIC-*.md")):
        for block in re.split(r"(?=^## STORY-)", path.read_text(), flags=re.M):
            match = re.match(r"## STORY-(\d{4})", block)
            if match and marker in block:
                found.add(int(match.group(1)))
    return found


def generated_requirement_names() -> set[str]:
    return {item["file"] for item in CATALOGUE.values()}


def write_requirements(output_dir: Path) -> None:
    grouped: dict[str, list[tuple[str, dict]]] = {}
    for requirement_id, item in CATALOGUE.items():
        grouped.setdefault(item["file"], []).append((requirement_id, item))
    for filename, entries in grouped.items():
        body = [
            f"# Wave 3 {filename.removesuffix('.md').replace('-', ' ').title()} requirements",
            "",
            "**Kind:** requirements",
            "**Authority:** operational",
            "**Status:** draft",
            "**Owner:** project lead",
            "**Last verified:** 2026-08-24",
            "",
            "These requirements are proposed implementation contracts. Scope comes from the charter; "
            "mechanism comes only from the stable SPEC anchors named in each block. Every evidence "
            "status is intentionally unimplemented while the specifications remain draft.",
            "",
        ]
        for requirement_id, item in entries:
            body.extend([
                f"## {requirement_id} — {item['title']}",
                "",
                "**Kind:** requirement",
                "**Status:** proposed",
                "**Owner:** project lead",
                "**Scope link:** " + item["scope"],
                "**Specification anchors:** " + item["anchors"],
                "**Evidence intent:** " + item["evidence"],
                "**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.",
                "",
                "### Acceptance criteria",
                "",
            ])
            body.extend(f"- {criterion}" for criterion in item["criteria"])
            body.append("")
        (output_dir / filename).write_text("\n".join(body))


def row(number: int, title: str) -> tuple[str, str, str, str, str]:
    story = f"STORY-{number:04d}"
    if number in SPLITS:
        identity, anchors, rationale, citation = SPLITS[number]
        return "split", rationale, citation, anchors, identity
    if number in FULL_DEFERRED:
        rationale, citation = FULL_DEFERRED[number]
        return "deferred", rationale, citation, "—", f"DIR-POST-{number:04d}"
    if number in RETIRED:
        rationale, citation = RETIRED[number]
        return "retired", rationale, citation, "—", f"RET-{number:04d}"
    requirement_id = ASSIGNMENTS.get(number)
    if requirement_id is None:
        raise ValueError(f"{story} ({title}) has no migration rule")
    item = CATALOGUE[requirement_id]
    rationale = f"Re-derived as the current {item['title'].lower()} contract; legacy acceptance detail is not authority."
    return "rewritten", rationale, item["scope"], item["anchors"], requirement_id


def write_migration(migration_output: Path) -> None:
    stories = legacy_stories()
    if set(stories) != set(range(1, 150)):
        raise ValueError(f"expected STORY-0001..0149, found {len(stories)} stories")
    body = [
        "# Legacy STORY migration ledger",
        "",
        "**Kind:** traceability",
        "**Authority:** operational",
        "**Status:** active",
        "**Owner:** project lead",
        "**Last verified:** 2026-08-24",
        "",
        "This ledger accounts for every legacy STORY identity. It does not make the legacy corpus "
        "authoritative: scope comes from the charter and mechanisms from the stable SPEC anchors. "
        "`deferred` rows are Post-1.0 direction and intentionally have no 1.0 acceptance criteria; "
        "`retired` rows have no replacement requirement. The two `split` rows preserve their named "
        "in-scope contract while recording the cut acceptance criterion only as direction.",
        "",
        "| Legacy story | Legacy title | Disposition | Rationale | Charter citation | Current SPEC anchors | New requirement identity |",
        "|---|---|---|---|---|---|---|",
    ]
    for number, title in sorted(stories.items()):
        disposition, rationale, citation, anchors, identity = row(number, title)
        body.append(f"| STORY-{number:04d} | {title} | {disposition} | {rationale} | {citation} | {anchors} | {identity} |")
    body.extend([
        "",
        "## Direction-only records",
        "",
        "The following legacy meanings remain available only as explicit Post-1.0 direction: voltage "
        "tiers/grid depth, vehicle fuel lifecycle and manufacture, passenger rail, electric-heating "
        "fallback, dual currency, era calendar, perishables/refrigerated transport, and crime. "
        "No requirement file grants them a 1.0 acceptance criterion.",
        "",
        "Water delivery is deliberately rewritten to `REQ-WATER-001`: Water is tradable but never "
        "cargo, and a completed connected finite-rate Water transfer precedes Trade clearance. "
        "The obsolete tanker premise is not retained. Dispatch is deliberately rewritten to "
        "`REQ-LOGISTICS-001`: a finite same vehicle must traverse both compatible itineraries; a "
        "timer, reservation, or route result is insufficient evidence of pickup or delivery.",
    ])
    migration_output.parent.mkdir(parents=True, exist_ok=True)
    migration_output.write_text("\n".join(body) + "\n")


def fail(message: str) -> None:
    print(f"FAIL: {message}")
    raise SystemExit(1)


def display(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def validate(output_dir: Path, migration_output: Path) -> None:
    stories = legacy_stories()
    migration = migration_output.read_text() if migration_output.exists() else ""
    rows = re.findall(r"^\| (STORY-\d{4}) \| .*? \| (retained|rewritten|split|deferred|retired) \| .*? \| .*? \| (.*?) \| (.*?) \|$", migration, re.M)
    found = [story for story, _, _, _ in rows]
    expected = [f"STORY-{n:04d}" for n in range(1, 150)]
    if sorted(found) != expected or len(found) != 149:
        fail(f"migration coverage is {len(found)} rows, expected exactly STORY-0001..0149")
    if len(found) != len(set(found)):
        fail("migration contains duplicate STORY IDs")
    dispositions = {int(story.removeprefix("STORY-")): disposition for story, disposition, _, _ in rows}
    full_deferred = legacy_flagged_ids("**Deferred:** true")
    if len(full_deferred) != 19:
        fail(f"legacy full-defer inventory is {len(full_deferred)}, expected 19")
    if any(dispositions[number] != "deferred" for number in full_deferred):
        fail("an explicit legacy full-defer story is not direction-only deferred")
    ac_cuts = legacy_flagged_ids("POST-1.0 AC")
    if ac_cuts != {82, 139}:
        fail(f"legacy AC-level cut inventory is {sorted(ac_cuts)}, expected [82, 139]")
    if any(dispositions[number] != "split" for number in ac_cuts):
        fail("an explicit legacy AC-level cut is not a split row")
    for story, disposition, anchors, identity in rows:
        if disposition in {"rewritten", "split"} and (anchors == "—" or identity == "—"):
            fail(f"{story} {disposition} row lacks anchors or replacement identity")
        if disposition in {"deferred", "retired"} and (identity == "" or identity == "—"):
            fail(f"{story} {disposition} row lacks explicit identity/disposition")
    required_header = ["**Kind:** requirements", "**Authority:** operational", "**Status:** draft", "**Owner:** project lead", "**Last verified:**"]
    seen_ids: list[str] = []
    expected_names = generated_requirement_names()
    actual_names = {path.name for path in output_dir.glob("*.md") if path.name != "README.md"}
    if actual_names != expected_names:
        fail(f"{display(output_dir)} contains generated requirement files {sorted(actual_names)}, expected {sorted(expected_names)}")
    for path in sorted(output_dir.glob("*.md")):
        if path.name == "README.md":
            continue
        text = path.read_text()
        for field in required_header:
            if field not in text:
                fail(f"{display(path)} lacks {field}")
        if "[SUBSTRATE:" in text or "root `spec/`" in text or "spec/" in text:
            fail(f"{display(path)} contains a forbidden legacy authority marker")
        blocks = re.split(r"(?=^## REQ-)", text, flags=re.M)[1:]
        if not blocks:
            fail(f"{display(path)} has no requirement blocks")
        for block in blocks:
            match = re.match(r"## (REQ-[A-Z]+-\d{3})", block)
            if not match:
                fail(f"{path.relative_to(ROOT)} has malformed requirement ID")
            seen_ids.append(match.group(1))
            for field in ["**Kind:** requirement", "**Status:** proposed", "**Owner:**", "**Scope link:** Charter", "**Specification anchors:** SPEC-", "**Evidence intent:**", "**Evidence status:** UNIMPLEMENTED", "### Acceptance criteria"]:
                if field not in block:
                    fail(f"{match.group(1)} lacks {field}")
            if len(re.findall(r"^- .+", block, re.M)) < 2:
                fail(f"{match.group(1)} has fewer than two acceptance criteria")
            anchors = re.search(r"\*\*Specification anchors:\*\* (.+)", block)
            if not anchors:
                fail(f"{match.group(1)} has no anchors")
            for anchor in re.findall(r"SPEC-[A-Z]+-\d{3}", anchors.group(1)):
                if not any(anchor in p.read_text() for p in (ROOT / "docs/reference/specifications").glob("*.md")):
                    fail(f"{match.group(1)} references unknown {anchor}")
    if len(seen_ids) != len(set(seen_ids)):
        fail("requirements contain duplicate IDs")
    migration_identities = {identity.strip() for _, disposition, _, identity in rows if disposition in {"rewritten", "split"} for identity in identity.split(";")}
    for identity in migration_identities:
        if identity.startswith("REQ-") and identity not in seen_ids:
            fail(f"migration replacement {identity} has no requirement artifact")
    print(f"PASS: 149 STORY rows, {len(seen_ids)} live requirements, {len(CATALOGUE)} stable requirement identities")


def byte_compare(generated_dir: Path, generated_migration: Path, output_dir: Path, migration_output: Path) -> None:
    for name in sorted(generated_requirement_names()):
        generated = generated_dir / name
        tracked = output_dir / name
        if not tracked.exists() or generated.read_bytes() != tracked.read_bytes():
            fail(f"generated bytes drift from {display(tracked)}")
    if not migration_output.exists() or generated_migration.read_bytes() != migration_output.read_bytes():
        fail(f"generated bytes drift from {display(migration_output)}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT_DIR, help="directory for generated requirement Markdown")
    parser.add_argument("--migration-output", type=Path, default=DEFAULT_MIGRATION_OUTPUT, help="path for generated story-migration Markdown")
    parser.add_argument("--check", action="store_true", help="regenerate in a temporary directory, byte-compare, and validate without writing")
    args = parser.parse_args()
    if args.check:
        validate(args.output_dir, args.migration_output)
        with tempfile.TemporaryDirectory(prefix="wave3-requirements-") as tmp:
            temporary_root = Path(tmp)
            temporary_requirements = temporary_root / "requirements"
            temporary_requirements.mkdir()
            temporary_migration = temporary_root / "story-migration.md"
            write_requirements(temporary_requirements)
            write_migration(temporary_migration)
            byte_compare(temporary_requirements, temporary_migration, args.output_dir, args.migration_output)
    else:
        args.output_dir.mkdir(parents=True, exist_ok=True)
        write_requirements(args.output_dir)
        write_migration(args.migration_output)
        validate(args.output_dir, args.migration_output)


if __name__ == "__main__":
    main()
