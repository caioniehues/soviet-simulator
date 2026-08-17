//! Resource ontology stage 1 (spec/resources.md): stored commodities only.
//! Electricity is a flow, not a stored commodity — it lives in the wire/power
//! module, never in an inventory.

/// M1's commodity set. Extend by appending; index order is not serialized yet.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourceKind {
    Coal,
    Gravel,
    Goods,
}

/// Transport class (spec/resources.md §B1): the hard compatibility gate CS1
/// lacks. A resource moves only on a vehicle of its class — a mismatch is
/// simply not an edge. Two classes cover the M1 commodity set; the W&R
/// vocabulary has sixteen, added as the resources arrive.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TransportClass {
    /// Loose bulk on a tipper: ore, coal, gravel (W&R `GRAVEL`).
    Bulk,
    /// Dry boxed goods on a covered bed (W&R `COVERED`).
    Covered,
    /// People on a transit vehicle (W&R `PASSANGER`, spec/vehicles.md § bus).
    /// No `ResourceKind` maps to it, so the freight dispatcher can never
    /// match a bus — transit drives its own fleet (`sim/transit.rs`).
    Passenger,
    /// Construction machinery (B6): carries no cargo class at all — the
    /// vehicle *is* the tool. Never matched by the freight dispatcher.
    Machine,
}

impl ResourceKind {
    pub const COUNT: usize = 3;
    pub const ALL: [ResourceKind; Self::COUNT] = [
        ResourceKind::Coal,
        ResourceKind::Gravel,
        ResourceKind::Goods,
    ];
    fn index(self) -> usize {
        match self {
            ResourceKind::Coal => 0,
            ResourceKind::Gravel => 1,
            ResourceKind::Goods => 2,
        }
    }

    pub fn transport_class(self) -> TransportClass {
        match self {
            ResourceKind::Coal | ResourceKind::Gravel => TransportClass::Bulk,
            ResourceKind::Goods => TransportClass::Covered,
        }
    }
}

/// Typed inventory: physical stock in tonnes. Never negative; `take` returns
/// what was actually available (partial fulfilment is normal, never an error —
/// capacity exhaustion is a planning signal, spec README's one rule).
#[derive(bevy::prelude::Component, Clone, Debug)]
pub struct Inventory {
    amounts: [f32; ResourceKind::COUNT],
    pub capacity: f32,
}

impl Inventory {
    pub fn new(capacity: f32) -> Self {
        Self {
            amounts: [0.0; ResourceKind::COUNT],
            capacity,
        }
    }

    pub fn amount(&self, kind: ResourceKind) -> f32 {
        self.amounts[kind.index()]
    }

    pub fn total(&self) -> f32 {
        self.amounts.iter().sum()
    }

    /// Add up to `amount`, bounded by shared capacity; returns what fit.
    pub fn add(&mut self, kind: ResourceKind, amount: f32) -> f32 {
        let space = (self.capacity - self.total()).max(0.0);
        let added = amount.min(space);
        self.amounts[kind.index()] += added;
        added
    }

    /// Take up to `amount`; returns what was actually there.
    pub fn take(&mut self, kind: ResourceKind, amount: f32) -> f32 {
        let taken = amount.min(self.amounts[kind.index()]);
        self.amounts[kind.index()] -= taken;
        taken
    }
}
