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
