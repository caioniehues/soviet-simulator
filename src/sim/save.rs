//! Save/load first cut (ticket #28, ROADMAP § persistence): a custom
//! serde-column format per the ecosystem survey — each entity table becomes a
//! plain row vector sorted by its stable u64 id (ADR 0004), cross-references
//! stored as indices into the sorted sibling tables. Runtime `Entity` ids
//! never touch the disk, which is also what makes the state hash well-defined:
//! hash the serialized columns, not the world.
//!
//! First-cut boundaries, all self-healing on load: commuter pawns and vehicle
//! pawns are transient and dropped (in-transit citizens are normalized to
//! at-home; the shuttle dispatcher respawns pawns for saved assets, in-transit
//! cargo is lost); player edit queues are not saved (at most one frame of
//! input); derived road geometry (length, lanes) is recompiled, never stored.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::buildings::{Building, BuildingId, BuildingIds, BuildingKind, PowerOutput, Powered};
use super::citizens::{Citizen, CitizenId, CitizenIds, CitizenLocation, EducationTier};
use super::clock::{FrameIndex, TickIndex};
use super::commute::CommuterPawn;
use super::households::{
    Dwelling, Household, HouseholdId, HouseholdIds, HouseholdSpawnQueue, HousingQueue,
    RecruitmentLedger, RecruitmentPlan, SpawnHousehold,
};
use super::labour::Staffing;
use super::needs::CitizenNeeds;
use super::resources::{Inventory, ResourceKind, TransportClass};
use super::roads::{
    LastCut, NodeId, RoadBuildFeedback, RoadClass, RoadIds, RoadNode, RoadSegment, SegmentId,
};
use super::dispatch::{
    DeficitBoard, DispatchQueue, FreightJob, FreightOrder, FreightPhase, OrderId,
};
use super::storage::{StorageBand, StoragePolicies};
use super::vehicles::{ActivePawn, ActiveVehicle, VehicleAsset, VehicleId, VehicleIds};
use super::wires::{PoleId, SpanId, WireIds, WirePole, WireSpan};

/// Bumped whenever a column layout changes — postcard is not self-describing,
/// so an old file must be rejected, never misparsed. v2: VehicleRow gained
/// home_depot + class (M3.2). v3: storage-policy bands per building, the
/// freight order queue, and mid-trip vehicle state (M3.5).
pub const SAVE_VERSION: u32 = 5;
/// Quicksave path, relative to the working directory.
pub const QUICKSAVE_PATH: &str = "saves/quicksave.sav";

// ---------------------------------------------------------------------------
// Rows — plain serde types only ([f32;3] positions, u8 discriminants, u32
// indices). Every vector is sorted by its stable id before writing.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SaveGame {
    pub version: u32,
    pub frame: u32,
    pub tick: u32,
    pub next_building: u64,
    pub next_citizen: u64,
    pub next_household: u64,
    pub next_node: u64,
    pub next_segment: u64,
    pub next_pole: u64,
    pub next_span: u64,
    pub next_vehicle: u64,
    pub next_order: u64,
    pub match_cursor: u32,
    pub recruit_target: u32,
    pub recruited: u32,
    /// The Plan + treasury (G1.5): period state and the rouble balance.
    pub plan: PlanRow,
    pub last_cut: Option<([f32; 3], [f32; 3], u8)>,
    pub buildings: Vec<BuildingRow>,
    pub households: Vec<HouseholdRow>,
    pub citizens: Vec<CitizenRow>,
    /// FIFO of household indices still waiting for a flat.
    pub housing_queue: Vec<u32>,
    /// Pending plan-recruited spawns (already counted in `recruited`).
    pub pending_spawns: Vec<u8>,
    pub nodes: Vec<NodeRow>,
    pub segments: Vec<SegmentRow>,
    pub poles: Vec<PoleRow>,
    pub spans: Vec<SpanRow>,
    pub vehicles: Vec<VehicleRow>,
    pub orders: Vec<OrderRow>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PlanRow {
    pub period: u32,
    pub period_started: u32,
    /// (tag, resource discriminant, target, progress): tag 0 = Stockpile,
    /// tag 1 = Housed (resource unused, target is the household count).
    pub quotas: Vec<(u8, u8, f32, f32)>,
    pub last_fulfillment: Option<f32>,
    pub last_tranche: Option<f32>,
    pub roubles: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BuildingRow {
    pub id: u64,
    pub kind: u8,
    pub pos: [f32; 3],
    pub capacity: f32,
    pub amounts: [f32; ResourceKind::COUNT],
    pub power_output: Option<f32>,
    pub powered: Option<bool>,
    pub dwelling: Option<(u32, u32)>,
    /// (assigned citizen indices in roster order, present tally).
    pub staffing: Option<(Vec<u32>, u32)>,
    /// Per-resource storage bands (min_pct, max_pct), dense over ResourceKind.
    pub bands: Vec<Option<(f32, f32)>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HouseholdRow {
    pub id: u64,
    pub members: Vec<u32>,
    pub dwelling: Option<u32>,
    pub pantry: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CitizenRow {
    pub id: u64,
    pub household: u32,
    pub home: Option<u32>,
    pub work: Option<u32>,
    pub health: u8,
    pub wellbeing: u8,
    pub education: u8,
    pub location: u8,
    pub food: f32,
    pub rest: f32,
    pub attends: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NodeRow {
    pub id: u64,
    pub pos: [f32; 3],
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SegmentRow {
    pub id: u64,
    pub a: u32,
    pub b: u32,
    pub class: u8,
    pub modification_index: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PoleRow {
    pub id: u64,
    pub pos: [f32; 3],
    /// `NetKind` discriminant (save v4): 0 power, 1 water, 2 heat.
    pub kind: u8,
}

/// A span endpoint by table, not by entity.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum EndRef {
    Building(u32),
    Pole(u32),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SpanRow {
    pub id: u64,
    pub a: EndRef,
    pub b: EndRef,
    pub kind: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct VehicleRow {
    pub id: u64,
    /// Building idx of the depot whose slot this asset occupies.
    pub home_depot: u32,
    /// Transport-class discriminant.
    pub class: u8,
    /// Mid-trip state, if the truck is out on an order.
    pub job: Option<JobRow>,
    /// Still driving in from the customs border until this frame (G1.5).
    pub border_arrives: Option<u32>,
}

/// A truck's in-flight trip: enough to respawn the pawn where it was with the
/// cargo it carried. The route is derived state — recomputed on load from the
/// pawn's position (self-healing after road edits).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct JobRow {
    pub order: u64,
    pub phase: u8,
    pub pos: [f32; 3],
    pub heading: [f32; 3],
    pub cargo: [f32; ResourceKind::COUNT],
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OrderRow {
    pub id: u64,
    pub resource: u8,
    pub qty: f32,
    pub from: u32,
    pub to: u32,
    pub priority: f32,
    /// Index into the sorted vehicle table.
    pub assigned: Option<u32>,
    pub issued_tick: u32,
}

// -- discriminant maps (append-only; order is serialized) -------------------

fn net_kind_to_u8(kind: super::wires::NetKind) -> u8 {
    match kind {
        super::wires::NetKind::Power => 0,
        super::wires::NetKind::Water => 1,
        super::wires::NetKind::Heat => 2,
    }
}

fn net_kind_from_u8(v: u8) -> super::wires::NetKind {
    match v {
        1 => super::wires::NetKind::Water,
        2 => super::wires::NetKind::Heat,
        _ => super::wires::NetKind::Power,
    }
}

fn kind_to_u8(kind: BuildingKind) -> u8 {
    match kind {
        BuildingKind::Mine => 0,
        BuildingKind::Quarry => 1,
        BuildingKind::PowerPlant => 2,
        BuildingKind::Factory => 3,
        BuildingKind::Dwelling => 4,
        BuildingKind::Warehouse => 5,
        BuildingKind::Depot => 6,
        BuildingKind::BusStop => 7,
        BuildingKind::ConstructionOffice => 8,
        BuildingKind::WaterPump => 9,
        BuildingKind::SewagePlant => 10,
        BuildingKind::HeatPlant => 11,
        BuildingKind::CustomsOffice => 12,
    }
}

fn kind_from_u8(v: u8) -> Option<BuildingKind> {
    Some(match v {
        0 => BuildingKind::Mine,
        1 => BuildingKind::Quarry,
        2 => BuildingKind::PowerPlant,
        3 => BuildingKind::Factory,
        4 => BuildingKind::Dwelling,
        5 => BuildingKind::Warehouse,
        6 => BuildingKind::Depot,
        7 => BuildingKind::BusStop,
        8 => BuildingKind::ConstructionOffice,
        9 => BuildingKind::WaterPump,
        10 => BuildingKind::SewagePlant,
        11 => BuildingKind::HeatPlant,
        12 => BuildingKind::CustomsOffice,
        _ => return None,
    })
}

fn class_to_u8(class: RoadClass) -> u8 {
    match class {
        RoadClass::Dirt => 0,
        RoadClass::Paved => 1,
    }
}

fn class_from_u8(v: u8) -> Option<RoadClass> {
    Some(match v {
        0 => RoadClass::Dirt,
        1 => RoadClass::Paved,
        _ => return None,
    })
}

fn location_to_u8(location: CitizenLocation) -> u8 {
    match location {
        CitizenLocation::AtHome => 0,
        CitizenLocation::ToWork => 1,
        CitizenLocation::AtWork => 2,
        CitizenLocation::ToHome => 3,
    }
}

fn location_from_u8(v: u8) -> CitizenLocation {
    match v {
        2 => CitizenLocation::AtWork,
        _ => CitizenLocation::AtHome,
    }
}

fn resource_to_u8(kind: ResourceKind) -> u8 {
    ResourceKind::ALL.iter().position(|&k| k == kind).unwrap() as u8
}

fn resource_from_u8(v: u8) -> Option<ResourceKind> {
    ResourceKind::ALL.get(v as usize).copied()
}

fn transport_class_to_u8(class: TransportClass) -> u8 {
    match class {
        TransportClass::Bulk => 0,
        TransportClass::Covered => 1,
        TransportClass::Passenger => 2,
        TransportClass::Machine => 3,
    }
}

fn transport_class_from_u8(v: u8) -> Option<TransportClass> {
    Some(match v {
        0 => TransportClass::Bulk,
        1 => TransportClass::Covered,
        2 => TransportClass::Passenger,
        3 => TransportClass::Machine,
        _ => return None,
    })
}

fn phase_to_u8(phase: FreightPhase) -> u8 {
    match phase {
        FreightPhase::ToPickup => 0,
        FreightPhase::Loading => 1,
        FreightPhase::ToDropoff => 2,
        FreightPhase::Unloading => 3,
        FreightPhase::ReturnToDepot => 4,
    }
}

fn phase_from_u8(v: u8) -> FreightPhase {
    match v {
        1 => FreightPhase::Loading,
        2 => FreightPhase::ToDropoff,
        3 => FreightPhase::Unloading,
        4 => FreightPhase::ReturnToDepot,
        _ => FreightPhase::ToPickup,
    }
}

// ---------------------------------------------------------------------------
// Snapshot: world → columns
// ---------------------------------------------------------------------------

/// Capture the whole sim state as sorted columns. `&mut World` only for query
/// construction; nothing is mutated.
pub fn snapshot(world: &mut World) -> SaveGame {
    // Sorted (id, entity) tables first; every cross-reference resolves
    // through these.
    let mut buildings: Vec<(u64, Entity)> = world
        .query::<(Entity, &Building)>()
        .iter(world)
        .map(|(e, b)| (b.id.0, e))
        .collect();
    buildings.sort_unstable();
    let mut citizens: Vec<(u64, Entity)> = world
        .query::<(Entity, &Citizen)>()
        .iter(world)
        .map(|(e, c)| (c.id.0, e))
        .collect();
    citizens.sort_unstable();
    let mut households: Vec<(u64, Entity)> = world
        .query::<(Entity, &Household)>()
        .iter(world)
        .map(|(e, h)| (h.id.0, e))
        .collect();
    households.sort_unstable();
    let mut nodes: Vec<(u64, Entity)> = world
        .query::<(Entity, &RoadNode)>()
        .iter(world)
        .map(|(e, n)| (n.id.0, e))
        .collect();
    nodes.sort_unstable();
    let mut poles: Vec<(u64, Entity)> = world
        .query::<(Entity, &WirePole)>()
        .iter(world)
        .map(|(e, p)| (p.id.0, e))
        .collect();
    poles.sort_unstable();

    let index_of = |table: &[(u64, Entity)], entity: Entity| -> Option<u32> {
        table
            .iter()
            .position(|&(_, e)| e == entity)
            .map(|i| i as u32)
    };
    let building_index: HashMap<Entity, u32> = buildings
        .iter()
        .enumerate()
        .map(|(i, &(_, e))| (e, i as u32))
        .collect();
    let citizen_index: HashMap<Entity, u32> = citizens
        .iter()
        .enumerate()
        .map(|(i, &(_, e))| (e, i as u32))
        .collect();
    let household_index: HashMap<Entity, u32> = households
        .iter()
        .enumerate()
        .map(|(i, &(_, e))| (e, i as u32))
        .collect();
    let node_index: HashMap<Entity, u32> = nodes
        .iter()
        .enumerate()
        .map(|(i, &(_, e))| (e, i as u32))
        .collect();

    let building_rows: Vec<BuildingRow> = buildings
        .iter()
        .map(|&(id, entity)| {
            let building = world.get::<Building>(entity).unwrap();
            let inventory = world.get::<Inventory>(entity).unwrap();
            let mut amounts = [0.0; ResourceKind::COUNT];
            for (i, kind) in ResourceKind::ALL.into_iter().enumerate() {
                amounts[i] = inventory.amount(kind);
            }
            BuildingRow {
                id,
                kind: kind_to_u8(building.kind),
                pos: building.pos.to_array(),
                capacity: inventory.capacity,
                amounts,
                power_output: world.get::<PowerOutput>(entity).map(|o| o.0),
                powered: world.get::<Powered>(entity).map(|p| p.0),
                dwelling: world.get::<Dwelling>(entity).map(|d| (d.flats, d.occupied)),
                staffing: world.get::<Staffing>(entity).map(|s| {
                    let roster = s
                        .assigned
                        .iter()
                        .filter_map(|&worker| citizen_index.get(&worker).copied())
                        .collect();
                    (roster, s.present)
                }),
                bands: ResourceKind::ALL
                    .into_iter()
                    .map(|r| {
                        world
                            .get::<StoragePolicies>(entity)
                            .and_then(|p| p.band(r))
                            .map(|b| (b.min_pct, b.max_pct))
                    })
                    .collect(),
            }
        })
        .collect();

    let household_rows: Vec<HouseholdRow> = households
        .iter()
        .map(|&(id, entity)| {
            let household = world.get::<Household>(entity).unwrap();
            HouseholdRow {
                id,
                members: household
                    .members
                    .iter()
                    .filter_map(|&member| citizen_index.get(&member).copied())
                    .collect(),
                dwelling: household
                    .dwelling
                    .and_then(|d| building_index.get(&d).copied()),
                pantry: household.pantry,
            }
        })
        .collect();

    let citizen_rows: Vec<CitizenRow> = citizens
        .iter()
        .map(|&(id, entity)| {
            let citizen = world.get::<Citizen>(entity).unwrap();
            let needs = world
                .get::<CitizenNeeds>(entity)
                .copied()
                .unwrap_or_default();
            // In-transit citizens are normalized home: the pawn is transient
            // and not saved, so the trip simply never happened.
            let location = match citizen.location {
                CitizenLocation::ToWork | CitizenLocation::ToHome => CitizenLocation::AtHome,
                other => other,
            };
            CitizenRow {
                id,
                household: household_index
                    .get(&citizen.household)
                    .copied()
                    .expect("citizen's household is a live entity"),
                home: citizen.home.and_then(|h| building_index.get(&h).copied()),
                work: citizen.work.and_then(|w| building_index.get(&w).copied()),
                health: citizen.health,
                wellbeing: citizen.wellbeing,
                education: citizen.education.0,
                location: location_to_u8(location),
                food: needs.food,
                rest: needs.rest,
                attends: needs.attends,
            }
        })
        .collect();

    let housing_queue: Vec<u32> = world
        .resource::<HousingQueue>()
        .0
        .iter()
        .filter_map(|&h| household_index.get(&h).copied())
        .collect();
    let pending_spawns: Vec<u8> = world
        .resource::<HouseholdSpawnQueue>()
        .0
        .iter()
        .map(|s| s.members)
        .collect();

    let node_rows: Vec<NodeRow> = nodes
        .iter()
        .map(|&(id, entity)| NodeRow {
            id,
            pos: world.get::<RoadNode>(entity).unwrap().pos.to_array(),
        })
        .collect();
    let mut segment_rows: Vec<SegmentRow> = Vec::new();
    {
        let mut q = world.query::<&RoadSegment>();
        for segment in q.iter(world) {
            segment_rows.push(SegmentRow {
                id: segment.id.0,
                a: *node_index
                    .get(&segment.a)
                    .expect("segment endpoint is a live node"),
                b: *node_index
                    .get(&segment.b)
                    .expect("segment endpoint is a live node"),
                class: class_to_u8(segment.class),
                modification_index: segment.modification_index,
            });
        }
    }
    segment_rows.sort_unstable_by_key(|row| row.id);

    let pole_rows: Vec<PoleRow> = poles
        .iter()
        .map(|&(id, entity)| {
            let pole = world.get::<WirePole>(entity).unwrap();
            PoleRow {
                id,
                pos: pole.pos.to_array(),
                kind: net_kind_to_u8(pole.kind),
            }
        })
        .collect();
    let end_ref = |entity: Entity| -> Option<EndRef> {
        if let Some(&i) = building_index.get(&entity) {
            return Some(EndRef::Building(i));
        }
        index_of(&poles, entity).map(EndRef::Pole)
    };
    let mut span_rows: Vec<SpanRow> = Vec::new();
    {
        let mut q = world.query::<&WireSpan>();
        for span in q.iter(world) {
            // A span to a despawned endpoint cannot exist (removal cleans up),
            // but stay defensive: drop rather than corrupt.
            if let (Some(a), Some(b)) = (end_ref(span.a), end_ref(span.b)) {
                span_rows.push(SpanRow {
                    id: span.id.0,
                    a,
                    b,
                    kind: net_kind_to_u8(span.kind),
                });
            }
        }
    }
    span_rows.sort_unstable_by_key(|row| row.id);

    let mut vehicle_rows: Vec<VehicleRow> = Vec::new();
    let mut vehicle_index: HashMap<Entity, u32> = HashMap::new();
    {
        let mut assets: Vec<(u64, Entity)> = world
            .query::<(Entity, &VehicleAsset)>()
            .iter(world)
            .map(|(e, v)| (v.id.0, e))
            .collect();
        assets.sort_unstable();
        for (i, &(_, entity)) in assets.iter().enumerate() {
            vehicle_index.insert(entity, i as u32);
        }
        for (id, entity) in assets {
            let asset = world.get::<VehicleAsset>(entity).unwrap();
            // An asset whose depot vanished cannot be re-homed on load; the
            // sim never removes buildings yet, so this is only defensive.
            let Some(&home_depot) = building_index.get(&asset.home_depot) else {
                continue;
            };
            let class = transport_class_to_u8(asset.cargo_class);
            // Mid-trip state: the job on the asset plus the live pawn's
            // position and cargo (the route is derived, recomputed on load).
            let job = world.get::<FreightJob>(entity).and_then(|job| {
                let pawn = world.get::<ActivePawn>(entity)?.pawn()?;
                let vehicle = world.get::<ActiveVehicle>(pawn)?;
                let mut cargo = [0.0; ResourceKind::COUNT];
                for (i, kind) in ResourceKind::ALL.into_iter().enumerate() {
                    cargo[i] = vehicle.cargo.amount(kind);
                }
                Some(JobRow {
                    order: job.order.0,
                    phase: phase_to_u8(job.phase),
                    pos: vehicle.pos.to_array(),
                    heading: vehicle.heading.to_array(),
                    cargo,
                })
            });
            let border_arrives = world
                .get::<super::customs::InTransitFromBorder>(entity)
                .map(|t| t.arrives);
            vehicle_rows.push(VehicleRow {
                id,
                home_depot,
                class,
                job,
                border_arrives,
            });
        }
    }

    let queue = world.resource::<DispatchQueue>();
    let order_rows: Vec<OrderRow> = queue
        .orders
        .iter()
        .filter_map(|order| {
            Some(OrderRow {
                id: order.id.0,
                resource: resource_to_u8(order.resource),
                qty: order.qty,
                from: building_index.get(&order.from).copied()?,
                to: building_index.get(&order.to).copied()?,
                priority: order.priority,
                assigned: order
                    .assigned
                    .and_then(|a| vehicle_index.get(&a).copied()),
                issued_tick: order.issued_tick,
            })
        })
        .collect();

    SaveGame {
        version: SAVE_VERSION,
        frame: world.resource::<FrameIndex>().0,
        tick: world.resource::<TickIndex>().0,
        next_building: world.resource::<BuildingIds>().next,
        next_citizen: world.resource::<CitizenIds>().next,
        next_household: world.resource::<HouseholdIds>().next,
        next_node: world.resource::<RoadIds>().next_node,
        next_segment: world.resource::<RoadIds>().next_segment,
        next_pole: world.resource::<WireIds>().next_pole,
        next_span: world.resource::<WireIds>().next_span,
        next_vehicle: world.resource::<VehicleIds>().next,
        next_order: world.resource::<DispatchQueue>().next_order,
        match_cursor: world.resource::<DispatchQueue>().cursor as u32,
        recruit_target: world.resource::<RecruitmentPlan>().target_households,
        recruited: world.resource::<RecruitmentLedger>().recruited,
        plan: {
            use super::plan::{QuotaKind, StatePlan, Treasury};
            let plan = world.resource::<StatePlan>();
            PlanRow {
                period: plan.period,
                period_started: plan.period_started,
                quotas: plan
                    .quotas
                    .iter()
                    .map(|q| match q.kind {
                        QuotaKind::Stockpile(kind, target) => {
                            (0, resource_to_u8(kind), target, q.progress)
                        }
                        QuotaKind::Housed(target) => (1, 0, target as f32, q.progress),
                    })
                    .collect(),
                last_fulfillment: plan.last_fulfillment,
                last_tranche: plan.last_tranche,
                roubles: world.resource::<Treasury>().roubles,
            }
        },
        last_cut: world
            .resource::<LastCut>()
            .0
            .map(|(a, b, class)| (a.to_array(), b.to_array(), class_to_u8(class))),
        buildings: building_rows,
        households: household_rows,
        citizens: citizen_rows,
        housing_queue,
        pending_spawns,
        nodes: node_rows,
        segments: segment_rows,
        poles: pole_rows,
        spans: span_rows,
        vehicles: vehicle_rows,
        orders: order_rows,
    }
}

// ---------------------------------------------------------------------------
// Restore: columns → world
// ---------------------------------------------------------------------------

/// Replace the live sim state with `save`. Despawns every sim entity (render
/// dressing goes with them — meshes live on the sim entities), then respawns
/// entity shells per table and fills components with remapped references.
/// The `Added`/`Changed` render-sync systems re-dress everything next frame.
pub fn restore(world: &mut World, save: &SaveGame) {
    let stale: Vec<Entity> = world
        .query_filtered::<Entity, Or<(
            With<Citizen>,
            With<Household>,
            With<Building>,
            With<RoadNode>,
            With<RoadSegment>,
            With<WirePole>,
            With<WireSpan>,
            With<CommuterPawn>,
            With<VehicleAsset>,
            With<ActiveVehicle>,
        )>>()
        .iter(world)
        .collect();
    for entity in stale {
        // linked_spawn may already have taken a vehicle pawn with its asset
        if world.get_entity(entity).is_ok() {
            world.despawn(entity);
        }
    }

    // Entity shells per table, so cross-references can point anywhere.
    let building_ents: Vec<Entity> = save
        .buildings
        .iter()
        .map(|_| world.spawn_empty().id())
        .collect();
    let household_ents: Vec<Entity> = save
        .households
        .iter()
        .map(|_| world.spawn_empty().id())
        .collect();
    let citizen_ents: Vec<Entity> = save
        .citizens
        .iter()
        .map(|_| world.spawn_empty().id())
        .collect();
    let node_ents: Vec<Entity> = save
        .nodes
        .iter()
        .map(|_| world.spawn_empty().id())
        .collect();
    let segment_ents: Vec<Entity> = save
        .segments
        .iter()
        .map(|_| world.spawn_empty().id())
        .collect();
    let pole_ents: Vec<Entity> = save
        .poles
        .iter()
        .map(|_| world.spawn_empty().id())
        .collect();

    for (row, &entity) in save.buildings.iter().zip(&building_ents) {
        let Some(kind) = kind_from_u8(row.kind) else {
            continue;
        };
        let mut inventory = Inventory::new(row.capacity);
        for (i, kind) in ResourceKind::ALL.into_iter().enumerate() {
            inventory.add(kind, row.amounts[i]);
        }
        let mut target = world.entity_mut(entity);
        // Optional components go in BEFORE `Building`: the `Add<Building>`
        // observer and `Added<Building>` systems guard on their presence, so
        // the saved values must already be there when Building lands.
        if let Some(output) = row.power_output {
            target.insert(PowerOutput(output));
        }
        if let Some(powered) = row.powered {
            target.insert(Powered(powered));
        }
        if let Some((flats, occupied)) = row.dwelling {
            target.insert(Dwelling { flats, occupied });
        }
        if let Some((roster, present)) = &row.staffing {
            target.insert(Staffing {
                assigned: roster
                    .iter()
                    .filter_map(|&i| citizen_ents.get(i as usize).copied())
                    .collect(),
                present: *present,
            });
        }
        let mut policies = StoragePolicies::default();
        for (i, resource) in ResourceKind::ALL.into_iter().enumerate() {
            if let Some(Some((min, max))) = row.bands.get(i) {
                policies.set(resource, Some(StorageBand::new(*min, *max)));
            }
        }
        target.insert(policies);
        target.insert((
            Building {
                id: BuildingId(row.id),
                kind,
                pos: Vec3::from_array(row.pos),
            },
            inventory,
        ));
    }

    for (row, &entity) in save.households.iter().zip(&household_ents) {
        world.entity_mut(entity).insert(Household {
            id: HouseholdId(row.id),
            members: row
                .members
                .iter()
                .filter_map(|&i| citizen_ents.get(i as usize).copied())
                .collect(),
            dwelling: row
                .dwelling
                .and_then(|i| building_ents.get(i as usize).copied()),
            pantry: row.pantry,
        });
    }

    for (row, &entity) in save.citizens.iter().zip(&citizen_ents) {
        let household = household_ents
            .get(row.household as usize)
            .copied()
            .expect("citizen row references a household row");
        world.entity_mut(entity).insert((
            Citizen {
                id: CitizenId(row.id),
                household,
                home: row
                    .home
                    .and_then(|i| building_ents.get(i as usize).copied()),
                work: row
                    .work
                    .and_then(|i| building_ents.get(i as usize).copied()),
                health: row.health,
                wellbeing: row.wellbeing,
                education: EducationTier(row.education),
                location: location_from_u8(row.location),
            },
            CitizenNeeds {
                food: row.food,
                rest: row.rest,
                attends: row.attends,
            },
        ));
    }

    for (row, &entity) in save.nodes.iter().zip(&node_ents) {
        world.entity_mut(entity).insert(RoadNode {
            id: NodeId(row.id),
            pos: Vec3::from_array(row.pos),
            segments: Vec::new(),
        });
    }
    for (row, &entity) in save.segments.iter().zip(&segment_ents) {
        let Some(class) = class_from_u8(row.class) else {
            continue;
        };
        let (Some(&a), Some(&b)) = (node_ents.get(row.a as usize), node_ents.get(row.b as usize))
        else {
            continue;
        };
        let mut segment = RoadSegment {
            id: SegmentId(row.id),
            a,
            b,
            class,
            length: 0.0,
            modification_index: row.modification_index,
            lanes: Vec::new(),
            curve: Vec::new(),
        };
        let (a_pos, b_pos) = (
            Vec3::from_array(save.nodes[row.a as usize].pos),
            Vec3::from_array(save.nodes[row.b as usize].pos),
        );
        // Chord-straight for now; the DirtySegment below re-derives the
        // node-fan curve on the first compile pass (geometry is never stored).
        let chord = (b_pos - a_pos).normalize_or_zero();
        segment.recompile(a_pos, b_pos, chord, chord);
        world
            .entity_mut(entity)
            .insert((segment, super::roads::DirtySegment));
        for node in [a, b] {
            world
                .get_mut::<RoadNode>(node)
                .unwrap()
                .segments
                .push(entity);
        }
    }

    for (row, &entity) in save.poles.iter().zip(&pole_ents) {
        world.entity_mut(entity).insert(WirePole {
            id: PoleId(row.id),
            pos: Vec3::from_array(row.pos),
            kind: net_kind_from_u8(row.kind),
        });
    }
    let resolve_end = |end: EndRef| -> Option<Entity> {
        match end {
            EndRef::Building(i) => building_ents.get(i as usize).copied(),
            EndRef::Pole(i) => pole_ents.get(i as usize).copied(),
        }
    };
    for row in &save.spans {
        if let (Some(a), Some(b)) = (resolve_end(row.a), resolve_end(row.b)) {
            world.spawn(WireSpan {
                kind: net_kind_from_u8(row.kind),
                id: SpanId(row.id),
                a,
                b,
            });
        }
    }

    let mut vehicle_ents: Vec<Entity> = Vec::with_capacity(save.vehicles.len());
    for row in &save.vehicles {
        let (Some(&home_depot), Some(cargo_class)) = (
            building_ents.get(row.home_depot as usize),
            transport_class_from_u8(row.class),
        ) else {
            continue;
        };
        // Kind derives from the class: only buses carry passengers, so no
        // extra save column is needed.
        let kind = if cargo_class == TransportClass::Passenger {
            super::vehicles::VehicleKind::Bus
        } else {
            super::vehicles::VehicleKind::Truck
        };
        let asset = world
            .spawn(VehicleAsset {
                id: VehicleId(row.id),
                kind,
                home_depot,
                cargo_class,
            })
            .id();
        if let Some(arrives) = row.border_arrives {
            world
                .entity_mut(asset)
                .insert(super::customs::InTransitFromBorder { arrives });
        }
        vehicle_ents.push(asset);
        // Mid-trip truck: respawn the pawn where it was with its cargo; the
        // route is recomputed on the next drive tick (self-healing).
        if let Some(job) = &row.job {
            let mut vehicle =
                super::vehicles::ActiveVehicle::at(Vec3::from_array(job.pos));
            vehicle.heading = Vec3::from_array(job.heading);
            for (i, kind) in ResourceKind::ALL.into_iter().enumerate() {
                vehicle.cargo.add(kind, job.cargo[i]);
            }
            world.spawn((vehicle, super::vehicles::PawnOf(asset)));
            world.entity_mut(asset).insert(FreightJob {
                order: OrderId(job.order),
                phase: phase_from_u8(job.phase),
                stalled: 0,
            });
        }
    }

    {
        let orders: Vec<FreightOrder> = save
            .orders
            .iter()
            .filter_map(|row| {
                Some(FreightOrder {
                    id: OrderId(row.id),
                    resource: resource_from_u8(row.resource)?,
                    qty: row.qty,
                    from: *building_ents.get(row.from as usize)?,
                    to: *building_ents.get(row.to as usize)?,
                    priority: row.priority,
                    assigned: row
                        .assigned
                        .and_then(|i| vehicle_ents.get(i as usize).copied()),
                    issued_tick: row.issued_tick,
                })
            })
            .collect();
        let mut queue = world.resource_mut::<DispatchQueue>();
        queue.orders = orders;
        queue.next_order = save.next_order;
        queue.cursor = save.match_cursor as usize;
    }
    world.resource_mut::<DeficitBoard>().0.clear();

    world.resource_mut::<FrameIndex>().0 = save.frame;
    world.resource_mut::<TickIndex>().0 = save.tick;
    world.resource_mut::<BuildingIds>().next = save.next_building;
    world.resource_mut::<CitizenIds>().next = save.next_citizen;
    world.resource_mut::<HouseholdIds>().next = save.next_household;
    {
        let mut road_ids = world.resource_mut::<RoadIds>();
        road_ids.next_node = save.next_node;
        road_ids.next_segment = save.next_segment;
    }
    {
        let mut wire_ids = world.resource_mut::<WireIds>();
        wire_ids.next_pole = save.next_pole;
        wire_ids.next_span = save.next_span;
    }
    world.resource_mut::<VehicleIds>().next = save.next_vehicle;
    world.resource_mut::<RecruitmentPlan>().target_households = save.recruit_target;
    world.resource_mut::<RecruitmentLedger>().recruited = save.recruited;
    {
        use super::plan::{Quota, QuotaKind, StatePlan, Treasury};
        let mut plan = world.resource_mut::<StatePlan>();
        plan.period = save.plan.period;
        plan.period_started = save.plan.period_started;
        plan.quotas = save
            .plan
            .quotas
            .iter()
            .filter_map(|&(tag, resource, target, progress)| {
                let kind = match tag {
                    0 => QuotaKind::Stockpile(resource_from_u8(resource)?, target),
                    _ => QuotaKind::Housed(target as u32),
                };
                Some(Quota { kind, progress })
            })
            .collect();
        plan.last_fulfillment = save.plan.last_fulfillment;
        plan.last_tranche = save.plan.last_tranche;
        world.resource_mut::<Treasury>().roubles = save.plan.roubles;
    }
    world.resource_mut::<LastCut>().0 = save.last_cut.and_then(|(a, b, class)| {
        Some((
            Vec3::from_array(a),
            Vec3::from_array(b),
            class_from_u8(class)?,
        ))
    });
    world.resource_mut::<RoadBuildFeedback>().0 = None;
    world.resource_mut::<HousingQueue>().0 = save
        .housing_queue
        .iter()
        .filter_map(|&i| household_ents.get(i as usize).copied())
        .collect();
    world.resource_mut::<HouseholdSpawnQueue>().0 = save
        .pending_spawns
        .iter()
        .map(|&members| SpawnHousehold { members })
        .collect();
}

// ---------------------------------------------------------------------------
// Bytes, files, hash
// ---------------------------------------------------------------------------

pub fn to_bytes(save: &SaveGame) -> Vec<u8> {
    postcard::to_stdvec(save).expect("save serialization is infallible for plain rows")
}

pub fn from_bytes(bytes: &[u8]) -> Result<SaveGame, String> {
    let save: SaveGame = postcard::from_bytes(bytes).map_err(|e| e.to_string())?;
    if save.version != SAVE_VERSION {
        return Err(format!(
            "save version {} unsupported (expected {SAVE_VERSION})",
            save.version
        ));
    }
    Ok(save)
}

/// The sim state hash the round-trip test compares: a hash of the canonical
/// serialized columns.
pub fn state_hash(world: &mut World) -> u64 {
    let mut hasher = DefaultHasher::new();
    to_bytes(&snapshot(world)).hash(&mut hasher);
    hasher.finish()
}

pub fn save_to_file(world: &mut World, path: &Path) -> Result<(), String> {
    let bytes = to_bytes(&snapshot(world));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

pub fn load_from_file(world: &mut World, path: &Path) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let save = from_bytes(&bytes)?;
    restore(world, &save);
    Ok(())
}

// ---------------------------------------------------------------------------
// Requests + plugin
// ---------------------------------------------------------------------------

/// Set by the hotkey layer (F5/F9); consumed between ticks so a save always
/// lands on a tick boundary with every command flushed.
#[derive(Resource, Default)]
pub struct SaveLoadRequests {
    pub save: Option<PathBuf>,
    pub load: Option<PathBuf>,
}

pub struct SaveSimPlugin;

impl Plugin for SaveSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveLoadRequests>()
            // Plan state rides the save; init here so plugin-subset test
            // apps snapshot/restore without PlanSimPlugin.
            .init_resource::<super::plan::StatePlan>()
            .init_resource::<super::plan::Treasury>()
            .add_systems(Update, process_requests.after(super::clock::drive_sim));
    }
}

/// Exclusive: runs after the sim driver, i.e. exactly on a tick boundary.
fn process_requests(world: &mut World) {
    let (save_to, load_from) = {
        let mut requests = world.resource_mut::<SaveLoadRequests>();
        (requests.save.take(), requests.load.take())
    };
    if let Some(path) = save_to {
        match save_to_file(world, &path) {
            Ok(()) => info!("saved to {}", path.display()),
            Err(err) => error!("save failed: {err}"),
        }
    }
    if let Some(path) = load_from {
        match load_from_file(world, &path) {
            Ok(()) => info!("loaded {}", path.display()),
            Err(err) => error!("load failed: {err}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingSimPlugin};
    use super::super::commute::CommuteSimPlugin;
    use super::super::dispatch::DispatchSimPlugin;
    use super::super::storage::StorageSimPlugin;
    use super::super::households::HouseholdSimPlugin;
    use super::super::labour::LabourSimPlugin;
    use super::super::needs::NeedsSimPlugin;
    use super::super::roads::{RoadEdit, RoadEditQueue, RoadSimPlugin};
    use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
    use super::super::wires::{WireEdit, WireEditQueue, WireSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            HouseholdSimPlugin,
            LabourSimPlugin,
            CommuteSimPlugin,
            NeedsSimPlugin,
            StorageSimPlugin,
            VehicleSimPlugin,
            DispatchSimPlugin,
            WireSimPlugin,
            SaveSimPlugin,
        ));
        a
    }

    fn ticks(app: &mut App, n: u32) {
        for _ in 0..n {
            app.world_mut()
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
            app.update();
        }
    }

    /// A whole town: roads, dwelling, mine, plant, factory, wires, a shuttle,
    /// recruited households — every table non-empty.
    fn full_town(app: &mut App) {
        for (from, to) in [
            (Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0)),
            (Vec3::new(100.0, 0.0, 0.0), Vec3::new(200.0, 0.0, 0.0)),
        ] {
            app.world_mut()
                .resource_mut::<RoadEditQueue>()
                .0
                .push(RoadEdit::Place {
                    from,
                    to,
                    class: super::super::roads::RoadClass::Dirt,
                });
        }
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 15.0)),
                (BuildingKind::Mine, Vec3::new(100.0, 0.0, 15.0)),
                (BuildingKind::PowerPlant, Vec3::new(115.0, 0.0, 30.0)),
                (BuildingKind::Factory, Vec3::new(200.0, 0.0, 15.0)),
                (BuildingKind::Depot, Vec3::new(60.0, 0.0, 60.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 3;
        app.world_mut()
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::Place {
                kind: crate::sim::wires::NetKind::Power,
                from: Vec3::new(115.0, 0.0, 30.0),
                to: Vec3::new(200.0, 0.0, 15.0),
            });
        ticks(app, 3);
        // shuttle mine → plant needs live building entities
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        let mine = q
            .iter(world)
            .find(|(_, b)| b.kind == BuildingKind::Mine)
            .unwrap()
            .0;
        let plant = q
            .iter(world)
            .find(|(_, b)| b.kind == BuildingKind::PowerPlant)
            .unwrap()
            .0;
        let depot = q
            .iter(world)
            .find(|(_, b)| b.kind == BuildingKind::Depot)
            .unwrap()
            .0;
        let mut edits = world.resource_mut::<VehicleEditQueue>();
        edits.0.push(VehicleEdit::BuyTruck {
            depot,
            class: TransportClass::Bulk,
        });
        edits.0.push(VehicleEdit::CreateShuttle {
            from: mine,
            to: plant,
            resource: ResourceKind::Coal,
        });
    }

    #[test]
    fn round_trip_preserves_the_sim_state_hash() {
        let mut source = app();
        full_town(&mut source);
        ticks(&mut source, 200); // mid-morning: production, commutes, hauling
        let saved = snapshot(source.world_mut());
        let hash_before = state_hash(source.world_mut());

        let mut target = app();
        ticks(&mut target, 1); // schedules initialized
        restore(target.world_mut(), &saved);
        let hash_after = state_hash(target.world_mut());
        assert_eq!(hash_before, hash_after, "load reproduces the saved state");

        // the loaded world must keep simulating without panicking
        ticks(&mut target, 100);
        let world = target.world_mut();
        assert!(world.query::<&Citizen>().iter(world).count() > 0);
    }

    #[test]
    fn restore_into_the_same_world_is_identical() {
        let mut app = app();
        full_town(&mut app);
        ticks(&mut app, 350);
        let saved = snapshot(app.world_mut());
        let hash_before = state_hash(app.world_mut());
        restore(app.world_mut(), &saved);
        assert_eq!(state_hash(app.world_mut()), hash_before);
    }

    #[test]
    fn mid_commute_save_normalizes_travellers_home() {
        let mut app = app();
        full_town(&mut app);
        ticks(&mut app, 170); // morning departures under way
        {
            let world = app.world_mut();
            assert!(
                world.query::<&CommuterPawn>().iter(world).count() > 0,
                "someone must be mid-walk for this test to bite"
            );
        }
        let saved = snapshot(app.world_mut());
        restore(app.world_mut(), &saved);
        let world = app.world_mut();
        assert_eq!(world.query::<&CommuterPawn>().iter(world).count(), 0);
        let in_transit = world
            .query::<&Citizen>()
            .iter(world)
            .filter(|c| {
                matches!(
                    c.location,
                    CitizenLocation::ToWork | CitizenLocation::ToHome
                )
            })
            .count();
        assert_eq!(in_transit, 0, "travellers were normalized home");
        // snapshot of the restored world matches the saved columns exactly
        assert_eq!(snapshot(app.world_mut()), saved);
    }

    #[test]
    fn file_round_trip_and_version_gate() {
        let mut app = app();
        full_town(&mut app);
        ticks(&mut app, 50);
        let dir = std::env::temp_dir().join("soviet_sim_save_test");
        let path = dir.join("quicksave.sav");
        save_to_file(app.world_mut(), &path).unwrap();
        let hash_before = state_hash(app.world_mut());
        ticks(&mut app, 100); // diverge
        assert_ne!(state_hash(app.world_mut()), hash_before);
        load_from_file(app.world_mut(), &path).unwrap();
        assert_eq!(state_hash(app.world_mut()), hash_before);

        let mut bad = snapshot(app.world_mut());
        bad.version = 99;
        assert!(from_bytes(&to_bytes(&bad)).is_err());
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn mid_trip_save_keeps_the_cargo_and_the_order() {
        let mut app = app();
        full_town(&mut app);
        // Stock the mine so the dispatcher has real tonnage to move (the
        // unstaffed mine produces nothing on its own this early).
        ticks(&mut app, 1);
        {
            let world = app.world_mut();
            let mine = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .find(|(_, b)| b.kind == BuildingKind::Mine)
                .unwrap()
                .0;
            world
                .get_mut::<Inventory>(mine)
                .unwrap()
                .add(ResourceKind::Coal, 40.0);
        }
        // Run until some truck is out with a freight job (the sugar policies
        // feed the plant from the mine).
        let mut out = false;
        for _ in 0..40 {
            ticks(&mut app, 50);
            let world = app.world_mut();
            let loaded = world
                .query::<&super::super::vehicles::ActiveVehicle>()
                .iter(world)
                .any(|v| v.cargo.total() > 1.0);
            if loaded {
                out = true;
                break;
            }
        }
        assert!(out, "a loaded truck must be mid-trip for this test to bite");
        let saved = snapshot(app.world_mut());
        assert!(
            saved.vehicles.iter().any(|v| v.job.is_some()),
            "trip state saved"
        );
        assert!(!saved.orders.is_empty(), "orders in flight saved");
        let hash_before = state_hash(app.world_mut());
        restore(app.world_mut(), &saved);
        assert_eq!(
            state_hash(app.world_mut()),
            hash_before,
            "mid-trip round trip is hash-identical"
        );
        // The restored world keeps hauling: cargo aboard is not lost.
        let world = app.world_mut();
        let carried: f32 = world
            .query::<&super::super::vehicles::ActiveVehicle>()
            .iter(world)
            .map(|v| v.cargo.total())
            .sum();
        assert!(carried > 1.0, "cargo stayed aboard through the load");
        ticks(&mut app, 100);
    }

    #[test]
    fn loaded_world_resumes_production_and_commutes() {
        let mut source = app();
        full_town(&mut source);
        ticks(&mut source, 100); // staffed, pre-morning
        let saved = snapshot(source.world_mut());

        let mut target = app();
        ticks(&mut target, 1);
        restore(target.world_mut(), &saved);
        ticks(&mut target, 250); // morning: workers commute back in
        let world = target.world_mut();
        // Presence — not stock — is the signal that commutes resumed (the
        // shuttle drains yards, and which workplace the planning pass filled
        // first is arbitrary).
        let present: u32 = world
            .query::<&Staffing>()
            .iter(world)
            .map(|s| s.present)
            .sum();
        assert!(present > 0, "restored citizens report to work again");
    }
}
