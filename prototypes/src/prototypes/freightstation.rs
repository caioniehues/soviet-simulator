use crate::{get_lua, get_lua_opt, Money, NoParent, Prototype, PrototypeBase, RenderAsset, Size2D};
use mlua::Table;
use std::ops::Deref;

use super::*;

/// FreightStationPrototype is a freight station
#[derive(Clone, Debug)]
pub struct FreightStationPrototype {
    pub base: PrototypeBase,
    pub id: FreightStationPrototypeID,
    pub asset: RenderAsset,
    pub price: Money,
    pub size: Size2D,
    /// Station-owned road fleet (sov-2uv): trucks spawned parked at the
    /// station door by `freight_station_soul`, mirroring `GoodsCompanyPrototype`
    /// `n_trucks` for factories. Absent in older data: defaults to 0, which
    /// keeps the station train-only.
    pub n_trucks: u32,
}

impl Prototype for FreightStationPrototype {
    type Parent = NoParent;
    type ID = FreightStationPrototypeID;
    const NAME: &'static str = "freight-station";

    fn from_lua(table: &Table) -> mlua::Result<Self> {
        let base = PrototypeBase::from_lua(table)?;
        Ok(Self {
            id: Self::ID::new(&base.name),
            base,
            asset: get_lua(table, "asset")?,
            price: get_lua(table, "price")?,
            size: get_lua(table, "size")?,
            n_trucks: get_lua_opt(table, "n_trucks")?.unwrap_or(0),
        })
    }

    fn id(&self) -> Self::ID {
        self.id
    }

    fn parent(&self) -> &Self::Parent {
        &NoParent
    }
}

impl Deref for FreightStationPrototype {
    type Target = PrototypeBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
