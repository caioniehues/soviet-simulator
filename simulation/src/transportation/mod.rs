use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

use flat_spatial::grid::GridHandle;
use flat_spatial::storage::CellIdx;
use serde::{Deserialize, Serialize};

use egui_inspect::InspectVec2Rotation;
use geom::{Transform, Vec2};
pub use pedestrian::*;
pub use vehicle::*;

use crate::map::BuildingID;
use crate::utils::resources::Resources;
use crate::world::VehicleID;
use crate::{Simulation, World};

pub mod pedestrian;
pub mod road;
pub mod testing_vehicles;
pub mod train;
mod vehicle;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    Outside,
    Vehicle(VehicleID),
    Building(BuildingID),
}
debug_inspect_impl!(Location);

#[derive(Clone, Default, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Speed(pub f32);
debug_inspect_impl!(Speed);

impl Debug for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}m/s", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Inspect)]
pub enum TransportationGroup {
    Unknown,
    Vehicles,
    Pedestrians,
}

#[derive(Copy, Clone, Serialize, Deserialize, Inspect)]
pub struct TransportState {
    #[inspect(proxy_type = "InspectVec2Rotation")]
    pub dir: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub height: f32,
    pub group: TransportationGroup,
    pub flag: u64,
}

impl Default for TransportState {
    fn default() -> Self {
        Self {
            dir: Vec2::X,
            speed: 0.0,
            radius: 1.0,
            height: 0.0,
            group: TransportationGroup::Unknown,
            flag: 0,
        }
    }
}

pub type TransportGrid = flat_spatial::Grid<TransportState, Vec2>;

/// Canonical form of one grid object: handle, then every f32 field as its bit pattern
/// (so that NaN or -0.0 cannot claim equality where the bytes differ), then the plain fields.
type CanonObject = (GridHandle, [u32; 7], TransportationGroup, u64);
/// Canonical form of the sparse cell map: ordered cells, each with its objects ordered.
type CanonCells = BTreeMap<CellIdx, (Vec<(GridHandle, [u32; 2])>, bool)>;

/// Order-insensitive equality for a `TransportGrid`: the sparse cell map is an `FnvHashMap` whose
/// iteration (and therefore bincode) order is not preserved across a decode.
pub fn transport_grid_equal(a: &TransportGrid, b: &TransportGrid) -> bool {
    fn objects(g: &TransportGrid) -> Vec<CanonObject> {
        let mut objs: Vec<CanonObject> = g
            .handles()
            .filter_map(|h| {
                let (pos, s) = g.get(h)?;
                Some((
                    h,
                    [
                        pos.x.to_bits(),
                        pos.y.to_bits(),
                        s.dir.x.to_bits(),
                        s.dir.y.to_bits(),
                        s.speed.to_bits(),
                        s.radius.to_bits(),
                        s.height.to_bits(),
                    ],
                    s.group,
                    s.flag,
                ))
            })
            .collect();
        objs.sort_unstable_by_key(|o| o.0);
        objs
    }

    fn cells(g: &TransportGrid) -> CanonCells {
        let storage = g.storage();
        #[allow(clippy::iter_over_hash_type)] // collected into a BTreeMap, so order is irrelevant
        storage
            .cells
            .iter()
            .map(|(idx, cell)| {
                let mut objs: Vec<(GridHandle, [u32; 2])> = cell
                    .objs
                    .iter()
                    .map(|&(h, pos)| (h, [pos.x.to_bits(), pos.y.to_bits()]))
                    .collect();
                objs.sort_unstable();
                (*idx, (objs, cell.dirty))
            })
            .collect()
    }

    a.len() == b.len()
        && a.storage().cell_size == b.storage().cell_size
        && objects(a) == objects(b)
        && cells(a) == cells(b)
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Transporter(pub GridHandle);
debug_inspect_impl!(Transporter);

impl Transporter {
    pub fn destroy(self) -> impl FnOnce(&mut Simulation) {
        move |sim| {
            let cw = &mut sim.write::<TransportGrid>();
            cw.remove_maintain(self.0);
        }
    }
}

pub fn transport_grid_synchronize(world: &mut World, resources: &mut Resources) {
    profiling::scope!("physics::transport_grid_synchronize");
    let mut transport_grid = resources.write::<TransportGrid>();

    world.query_trans_speed_coll_vehicle().for_each(
        |(trans, kin, coll, v): (&Transform, &Speed, Transporter, Option<&Vehicle>)| {
            transport_grid.set_position(coll.0, trans.pos.xy());
            let (_, po) = transport_grid.get_mut(coll.0).unwrap(); // Unwrap ok: handle is deleted only when entity is deleted too
            po.dir = trans.dir.xy();
            po.speed = kin.0;
            po.height = trans.pos.z;
            if let Some(v) = v {
                po.flag = v.flag;
            }
        },
    );

    transport_grid.maintain_deterministic();
}
