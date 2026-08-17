//! The Plan (G1.1 #76, revised by G1.2 #80): the pressure loop. The state
//! hands down quota targets per plan period; fulfillment at rollover scales
//! the next rouble tranche — under-delivering plants get under-allocated,
//! never game over. The rouble (W&R-style, pulled forward from B10) replaces
//! fiat purchases: trucks, buses, machines, and recruited households all
//! draw from the same treasury, so every quota has a cost behind it. The
//! authored First Five-Year Plan ladder arrives with G1.5; until then
//! quotas scale by period.

use bevy::prelude::*;

use super::buildings::Building;
use super::clock::{FRAMES_PER_GAME_DAY, FrameIndex};
use super::resources::{Inventory, ResourceKind};
use super::stages::{SimStage, SimTick};

/// Game days per plan period — five 12-day climate years: a Five-Year Plan.
pub const PLAN_PERIOD_DAYS: u32 = 60;
pub const PLAN_PERIOD_FRAMES: u32 = PLAN_PERIOD_DAYS * FRAMES_PER_GAME_DAY;

/// Rouble prices. One truck ≈ recruiting two-and-a-half households:
/// machinery is dear, labour the state sends more freely.
pub const TRUCK_COST: f32 = 10.0;
pub const BUS_COST: f32 = 12.0;
pub const MACHINE_COST: f32 = 15.0;
pub const RECRUIT_COST: f32 = 4.0;

/// The opening grant, and the rollover tranche: a floor the state always
/// sends plus a bonus scaled by fulfillment — the loop that makes a missed
/// plan a leaner next period instead of a game over.
pub const STARTING_ROUBLES: f32 = 120.0;
pub const BASE_TRANCHE: f32 = 40.0;
pub const FULFILLMENT_BONUS: f32 = 140.0;

/// What a quota measures. Stockpile counts the resource across every
/// building inventory (the republic's shelves, not one warehouse); Housed
/// counts households actually assigned a flat — recruitment alone earns
/// nothing.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum QuotaKind {
    Stockpile(ResourceKind, f32),
    Housed(u32),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quota {
    pub kind: QuotaKind,
    /// Fulfillment 0..1, refreshed by the daily measure pass.
    pub progress: f32,
}

impl Quota {
    pub fn new(kind: QuotaKind) -> Self {
        Self {
            kind,
            progress: 0.0,
        }
    }
}

/// The standing plan: current period, its deadline, its quotas, and what
/// last rollover concluded (both feed the ledger screen in G1.3).
#[derive(Resource)]
pub struct StatePlan {
    pub period: u32,
    pub period_started: u32,
    pub quotas: Vec<Quota>,
    pub last_fulfillment: Option<f32>,
    pub last_tranche: Option<f32>,
}

impl Default for StatePlan {
    fn default() -> Self {
        Self {
            period: 1,
            period_started: 0,
            quotas: default_quotas(1),
            last_fulfillment: None,
            last_tranche: None,
        }
    }
}

impl StatePlan {
    pub fn deadline(&self) -> u32 {
        self.period_started.wrapping_add(PLAN_PERIOD_FRAMES)
    }
    pub fn days_left(&self, frame: u32) -> u32 {
        self.deadline().wrapping_sub(frame) / FRAMES_PER_GAME_DAY
    }
    /// Mean quota progress — the number the tranche is paid on.
    pub fn fulfillment(&self) -> f32 {
        if self.quotas.is_empty() {
            return 1.0;
        }
        self.quotas.iter().map(|q| q.progress).sum::<f32>() / self.quotas.len() as f32
    }
}

/// Placeholder ladder until the authored First Plan (G1.4): each period asks
/// for more coal on the shelves and more households under a roof.
pub fn default_quotas(period: u32) -> Vec<Quota> {
    let p = period as f32;
    vec![
        Quota::new(QuotaKind::Stockpile(ResourceKind::Coal, 60.0 * p)),
        Quota::new(QuotaKind::Housed(8 * period)),
    ]
}

/// The republic's purse. Every purchase path spends from here.
#[derive(Resource)]
pub struct Treasury {
    pub roubles: f32,
}

impl Default for Treasury {
    fn default() -> Self {
        Self {
            roubles: STARTING_ROUBLES,
        }
    }
}

impl Treasury {
    pub fn try_spend(&mut self, cost: f32) -> bool {
        if self.roubles >= cost {
            self.roubles -= cost;
            true
        } else {
            false
        }
    }
}

/// A refused spend, for HUD feedback ("NO ALLOCATION"): what was refused and
/// the frame it happened, so the notice can fade.
#[derive(Resource, Default)]
pub struct AllocationFeedback(pub Option<(u32, &'static str)>);

pub struct PlanSimPlugin;

impl Plugin for PlanSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StatePlan>()
            .init_resource::<Treasury>()
            .init_resource::<AllocationFeedback>()
            .add_systems(
                SimTick,
                (measure_quotas, rollover_period)
                    .chain()
                    .in_set(SimStage::AccountingAndCausalHistory),
            );
    }
}

/// Daily measure pass: refresh every quota's progress from live state.
fn measure_quotas(
    frame: Res<FrameIndex>,
    mut plan: ResMut<StatePlan>,
    inventories: Query<&Inventory, With<Building>>,
    households: Query<&super::households::Household>,
) {
    // Frame 0 included so a fresh world starts measured, not blank.
    if !frame.0.is_multiple_of(FRAMES_PER_GAME_DAY) {
        return;
    }
    let housed = households.iter().filter(|h| h.dwelling.is_some()).count() as u32;
    for quota in &mut plan.quotas {
        quota.progress = match quota.kind {
            QuotaKind::Stockpile(kind, target) => {
                let held: f32 = inventories.iter().map(|i| i.amount(kind)).sum();
                if target <= 0.0 {
                    1.0
                } else {
                    (held / target).clamp(0.0, 1.0)
                }
            }
            QuotaKind::Housed(target) => {
                if target == 0 {
                    1.0
                } else {
                    (housed as f32 / target as f32).clamp(0.0, 1.0)
                }
            }
        };
    }
}

/// Period rollover: pay the tranche on fulfillment, hand down the next
/// quotas. Runs right after the measure pass so the payout reads the same
/// numbers the player last saw.
fn rollover_period(
    frame: Res<FrameIndex>,
    mut plan: ResMut<StatePlan>,
    mut budget: ResMut<Treasury>,
) {
    if frame.0.wrapping_sub(plan.period_started) < PLAN_PERIOD_FRAMES {
        return;
    }
    let fulfillment = plan.fulfillment();
    let tranche = BASE_TRANCHE + FULFILLMENT_BONUS * fulfillment;
    budget.roubles += tranche;
    plan.last_fulfillment = Some(fulfillment);
    plan.last_tranche = Some(tranche);
    plan.period += 1;
    plan.period_started = frame.0;
    let period = plan.period;
    plan.quotas = default_quotas(period);
    info!(
        "plan period {} closed: fulfillment {:.0}%, tranche {tranche:.0} pts",
        period - 1,
        fulfillment * 100.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tranche_scales_with_fulfillment_and_next_quotas_are_handed_down() {
        let mut app = App::new();
        app.add_plugins((super::super::SimPlugin, PlanSimPlugin));
        app.insert_resource(Time::<()>::default());
        // Pin a plan about to expire with half its quotas met.
        {
            let world = app.world_mut();
            let mut plan = world.resource_mut::<StatePlan>();
            plan.quotas = vec![
                Quota {
                    kind: QuotaKind::Housed(1),
                    progress: 1.0,
                },
                Quota {
                    kind: QuotaKind::Stockpile(ResourceKind::Coal, 100.0),
                    progress: 0.0,
                },
            ];
            plan.period_started = 0;
            world.resource_mut::<FrameIndex>().0 = PLAN_PERIOD_FRAMES + 1;
            world.resource_mut::<Treasury>().roubles = 0.0;
        }
        // Run the rollover directly against the pinned frame (the measure
        // pass only rewrites progress on day boundaries, which this isn't).
        let _ = app
            .world_mut()
            .run_system_cached(rollover_period)
            .expect("rollover runs");
        let plan = app.world().resource::<StatePlan>();
        let budget = app.world().resource::<Treasury>();
        assert_eq!(plan.period, 2);
        assert_eq!(plan.last_fulfillment, Some(0.5));
        assert!((budget.roubles - (BASE_TRANCHE + FULFILLMENT_BONUS * 0.5)).abs() < 1e-3);
        assert_eq!(plan.quotas, default_quotas(2));
    }

    #[test]
    fn budget_refuses_past_zero() {
        let mut budget = Treasury { roubles: 10.0 };
        assert!(budget.try_spend(10.0));
        assert!(!budget.try_spend(0.1));
        assert_eq!(budget.roubles, 0.0);
    }
}
