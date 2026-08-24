# Brief — finish STORY-0149 AC-4: make a real truck carry the goods

You are working in `/home/caio/soviet-simulator` (Rust, hard fork of Egregoria — an ECS
city-builder). Branch `main`, clean at commit `35ce342`. Report findings to the lead in this
terminal when done. **Do not commit.** Leave changes in the working tree.

## Why this task exists

ITER-0000 is the walking skeleton: *"one mill hoards coal delivered by one truck over one road,
and the planner catches it from observable state."*

The ledger half is done and correct. The physical half is not. Today
`simulation/src/economy/market.rs` drives delivery off `const DISPATCH_TRAVEL_TICKS: u32 = 3` —
a blind countdown. No vehicle, no road, no distance. So "nothing teleports" currently holds as
bookkeeping and **not** as physics. You are closing that.

Two agents have already been here. Neither failed; both stopped honestly and left you a map.
**Trust the map below over any assumption about how this "should" work.** The first brief for this
task was wrong on its central premise, and the agent that caught it did so by reading the movement
code rather than believing the brief. Do the same to this document.

## Verified ground truth — established in source, do not re-derive

**Baseline:** `cargo test -p simulation -- --test-threads=1` → **22 passed, 0 failed**, ~21s.
ALWAYS pass `--test-threads=1`. The binary segfaults ~1-in-5 under parallel threads because
`init.rs` pushes into `static mut` globals unsynchronized. Pre-existing, reproduced on an
unmodified tree, ticket `sov-test-race-initfuncs-qt6`. Not yours — do not chase it.

**Trucks are NOT shaped like trains. This is the trap that ate the last attempt.**
- `TrainEnt` has no parking concept, so `souls/freight_station.rs` can simply assign
  `train.it = Itinerary::route(..)` and the train moves.
- `VehicleEnt` carries `Vehicle.state: VehicleState { Parked(SpotReservation) | Driving |
  Panicking(_) | RoadToPark(spline, ..) }`.
- `transportation/road.rs:55-58` moves a vehicle **only** when
  `matches!(vehicle.state, VehicleState::Driving | VehicleState::Panicking(_))` AND it holds a
  `Transporter` collider in the `TransportGrid`.
- **Therefore: setting `.it` on a parked truck is a no-op.** The truck will not move.

**Un-parking is deferred, not synchronous.**
- `transportation/vehicle.rs:107` — `pub fn unpark(sim: &mut Simulation, vehicle: VehicleID)`.
  It frees the `ParkingManagement` spot and inserts a `Transporter` into `TransportGrid`.
- `Market::advance_dispatches` and its only caller `economy::market_update` run as
  `(world: &mut World, resources: &mut Resources)` systems — **no `Simulation` handle.**
- The established pattern for calling `unpark` from such a system is the command buffer, already
  used at `map_dynamic/router.rs:217`:
  `cbuf_vehicle.exec_ent(vehicle, move |sim| unpark(sim, vehicle));`
  It is deferred — the state transition and the actual unpark land on **different ticks**. Your
  state machine must tolerate that.
- **There is no `park()` counterpart.** Vehicles re-park via the `RoadToPark` spline machinery in
  `road.rs:vehicle_state_update`, driven once an itinerary naturally ends near a parking spot.
  Nobody has traced that path fully for a truck driven directly rather than by a human `Router`.

**Dispatcher state (just fixed, commit `35ce342`):**
- `Dispatcher::update()` now registers `VehicleKind::Truck` vehicles as
  `DispatchID::SmallTruck(VehicleID)`. This previously did nothing — the block was commented out.
- `DispatchKind::SmallTruck` maps to `LaneKind::Driving`.
- Query API, per `souls/freight_station.rs` (your ONLY correct prior art — read it in full):
  - `let mut dispatch = resources.write::<Dispatcher>();`  (:76)
  - `dispatch.query(map, DispatchKind::FreightTrain, DispatchQueryTarget::Pos(destination), ..)` (:145-148)
  - `dispatch.free(v)` when released (:132)
  - mirror its `FreightTrainState::Arriving/Moving` shape for arrival detection.

**The mechanism you are replacing** (`simulation/src/economy/market.rs`):
- `const DISPATCH_TRAVEL_TICKS: u32 = 3`
- `enum DispatchState { ToSource, Loading, ToDestination, Unloading }`
- `struct Dispatch { buyer, seller, kind, qty, state, ticks_left }`
- `Market::dispatches`, `Market::advance_dispatches()` (called from `economy/mod.rs`)
- `make_trades` reserves into `reserved: BTreeMap<SoulID,u32>` and pushes a `Dispatch` in
  `ToSource` instead of transferring capital at match time.
- Seller debited on entering `Loading`; buyer credited on entering `Unloading`; quantity held by
  the dispatch in between and counted in neither bucket.

**Company delivery path:** `souls/goods_company.rs:244-263` pops `c.sold.0.pop()` and builds
`WorkKind::Driver { deliver_order, .. }` driven through `HumanDecisionKind::DeliverAtBuilding`.
This path never touches `Market` today.

## The map left by the previous agent — start here

- Add `truck: Option<VehicleID>` to `Dispatch`.
- New signature for `Market::advance_dispatches`, taking `&mut World`, `&Map`, `&BuildingInfos`,
  `&mut Dispatcher`, `&ParCommandBuffer<VehicleEnt>`, `Tick`. Call site is
  `economy/mod.rs:market_update`, which already has `world: &mut World` and reads `Map`; it needs
  to additionally read `BuildingInfos`, write `Dispatcher`, and read `ParCommandBuffer<VehicleEnt>`.
- Reserve a truck with `Dispatcher::query(.., DispatchKind::SmallTruck,
  DispatchQueryTarget::Pos(seller_door_pos), ..)`, then unpark it via the `cbuf_vehicle.exec_ent`
  pattern before routing.
- Drive `ToSource` / `ToDestination` off `world.vehicles[id].it.has_ended(0.0)` — mirroring
  `freight_station.rs` — instead of `ticks_left`.
- Keep `ticks_left` ONLY as a `Loading`/`Unloading` dwell. A dwell at a stop is physically real; a
  travel timer is not. Rename `DISPATCH_TRAVEL_TICKS` → `DISPATCH_DWELL_TICKS`.
- Re-parking after `Unloading` / `free()`: the simplest defensible option is to NOT re-park —
  leave the truck `Driving` with `Itinerary::NONE` where it stopped, and tag it with a
  `ponytail:` comment naming the ceiling. `Dispatcher::update()` will still track and re-query it
  from wherever it sits. **Confirm this does not wedge traffic before you ship it.**

## Tests you must rewrite

`test_dispatch_gates_stock_not_match` and `test_inflated_request_hoards_honest_does_not` currently
live in `market.rs`'s `#[cfg(test)] mod tests` and call `advance_dispatches()` with fabricated
`SoulID`s and no `Map`/`World`/`Dispatcher` at all. Any real signature change forces rewriting
them onto `crate::tests::TestCtx` with real roads, buildings and a truck.

Move them to a NEW file `simulation/src/tests/scenarios/hoarding.rs`, declared with
`mod hoarding;` in `simulation/src/tests/scenarios/mod.rs`. Name each test with its corpus ID
(`scenario_0082_...`, `scenario_0083_...`, `scenario_0151_...`) so the behavior corpus can address
them and the sentinel runner can find them. This traceability was explicitly requested by the user.

Harness available (`simulation/src/tests/mod.rs`): `TestCtx::new()`, `build_roads(&[Vec3])`,
`build_house_at(&mut self, p: Vec2) -> BuildingID` (explicit placement, no dependence on
`map().lots()`), `advance_ticks(&mut self, n: u32)`, `tick()`, `check_determinism(&self)`.
`SimulationOptions` has `seed: u64` threaded to `RandProvider`.
`transportation::spawn_parked_vehicle` is how a truck gets created.

## Hard constraints

- **Never weaken the determinism check.** `TestCtx::tick()` bincode-round-trips the whole
  `Simulation` and hash-compares every key each tick. Any new field must serialize. A failure
  there is the harness working — fix the cause, never the check.
- **Do NOT touch** `simulation/src/tests/scenarios/recipe_provided.rs` — five passing proofs.
- **Do NOT touch** the external-trade teleport at `market.rs:367-369` (buyer credited before
  `find_external` is consulted). Deliberately out of scope, fenced in tests via `optout_exttrade`.
- Match existing style. This is a fork with a live upstream; gratuitous reformatting costs merges.
- Minimum code. No logistics framework, no trait hierarchy, no abstraction layer over
  `Dispatcher`. Extend what exists.

## Acceptance criteria

1. `cargo test -p simulation -- --test-threads=1` — all 22 existing tests green, plus yours.
2. `cargo check -p simulation --tests` exit 0, no new warnings.
3. **The proof that the truck is real:** with zero trucks available, NO debit and NO credit
   happens — goods do not move. With a truck present, they do. A test that fails if you delete the
   vehicle. This is the entire point; a passing state machine with no truck is what you are
   replacing.
4. `test_dispatch_gates_stock_not_match`'s guarantee survives the move: restoring the match-time
   transfer in `make_trades` must still fail a test. Demonstrate it — mutate, run, paste the real
   failure, revert.

## Reporting

Paste REAL command output; "tests pass" is not evidence. State plainly whether AC-4 is met,
partially met, or not met, with the source-level reason. Report every deviation.

**If you conclude it cannot be done safely within budget, do not half-land it.** Say where you
stopped, what works, what doesn't, and what the next agent needs. The last two agents did exactly
that and it was the right call both times — an honest partial with an accurate map beats a broken
whole. Say so early rather than at the end.
