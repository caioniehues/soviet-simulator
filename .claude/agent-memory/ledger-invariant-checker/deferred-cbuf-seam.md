---
name: deferred-cbuf-seam
description: A ParCommandBuffer closure observes a DIFFERENT world than the guard that queued it — the recurring shape behind unpark refusals freezing a ToSource dispatch and its seller reservation
metadata:
  type: project
---

**Family I — the deferred-command seam.** Confirmed as a shape 2026-08-28 on
branch `fix/sov-wave-souls` (HEAD `5349f34`).

`ParCommandBuffer::exec_ent` (`utils/par_command_buffer.rs:36`) queues a
closure that runs at the buffer flush AFTER the system, and `scheduler.rs`
flushes every buffer after every system. So any guard written as

```rust
let ok = world.vehicles.get(v).is_some_and(|ve| /* state check */);
if ok { cbuf.exec_ent(v, move |sim| do_the_thing(sim, v)); bookkeeping = Some(v); }
```

records bookkeeping from a world the closure may no longer see. The closure
also runs even when the entity is gone (`apply` does not check existence), so
it must be total.

Concrete instance: `market.rs:783-786` checks the truck is `Parked`, queues
`unpark`, and sets `dispatches[i].truck = Some(v)`. After sov-6qx `unpark`
returns `false` instead of forcing the state. A `Parked` vehicle has no
collider, so `vehicle_decision_system` never moves it and its itinerary never
ends; `DispatchState::ToSource` has **no tick countdown**. A refusal therefore
freezes the dispatch, the truck (still in `Dispatcher::reserved_by`) and
`reserved[seller]` permanently — a quantity removed from the economy with no
sink. Not reachable at `5349f34` (see below), fixed in the working tree by
`Market::release_tosource_truck(v)` + `dispatcher.free`.

**Why it was not reachable at HEAD** — the three facts to re-check whenever
this seam moves:
- `Dispatcher::query` reserves synchronously and removes the truck from
  `positions`/`lanes` (`dispatch.rs:130-140`, `:204-210`), so no second
  dispatch can grab the same truck in the same tick.
- Nothing runs between the guard and the flush; the buffer was emptied after
  the previous system.
- A truck killed between the two is caught next tick by the wedge-(b) handler
  (`market.rs:820-830`): frees the truck, releases `reserved`, drops the
  dispatch.

**Rule: whenever a guard and its effect are separated by a command buffer, ask
what rolls back the bookkeeping if the effect refuses — and whether the state
it leaves behind has any timeout at all.** `ToSource` has none; `Loading`,
`Unloading` and `Returning` do.

## The rollback, audited at `cdc8f1d` (2026-08-28) — CONSERVED
`Market::release_tosource_truck(v)` (`market.rs:757-765`) scans `dispatches`
for the first `ToSource` dispatch with `truck == Some(v)` and clears the field.
Three things make the by-truck lookup sound, and each is the thing to re-check
if this seam moves again:
- **Uniqueness.** `Dispatcher::query` reserves atomically, and every
  `dispatcher.free` in `advance_dispatches` either happens before a truck was
  recorded or is paired with `remove = true`. The rollback itself clears
  `truck` *before* calling `free`. So at most one `ToSource` dispatch ever
  holds a given `v`; the loop cannot release a different dispatch's row.
  `Dispatcher::update`'s `register` does NOT check `reserved_by` and re-adds a
  reserved truck to `positions`, but `query` skips `reserved_by`
  (`dispatch.rs:271`), so that re-add cannot hand the truck out twice.
- **Totality.** `ParCommandBuffer::apply` processes `to_kill` first and then
  runs **every** queued `exec_ent` closure with no existence check
  (`par_command_buffer.rs:62-82`). `unpark` returns `false` for an unknown
  entity. `Dispatcher::free` is called unconditionally, ignoring the `bool`
  return of `release_tosource_truck` — so a vanished or `Market::remove`d
  dispatch still frees the truck. No `reserved_by` leak.
- **No double credit.** The rollback deliberately does not touch
  `reserved[seller]`: the dispatch survives in `ToSource` and still owns that
  reservation. Release stays single-sourced (arrival debit, wedge-(b),
  `Market::remove`).

Residual, pre-existing and NOT a quantity break: the market sets
`ve.it = route` *before* queueing the unpark, and the rollback does not restore
the old itinerary. A truck left non-`Parked` near the seller can be re-offered
and re-rejected by the `!parked` guard every tick (sov-7pg shape), starving
that dispatch of a truck while other trucks idle. State is retryable, ledger is
untouched.

## Related: `SpotReservation` leaks silently on drop
`ParkingManagement` only releases a spot through `free()`
(`map_dynamic/parking.rs:27-32`); `SpotReservation` has **no `Drop` impl**, and
`free` even `mem::forget`s it. So any code path that drops a
`VehicleState::RoadToPark(_, _, spot)` or `Parked(spot)` value without calling
`free` leaks that parking spot forever. The pre-sov-6qx `unpark` did exactly
that via an unconditional `mem::replace(&mut v.vehicle.state,
VehicleState::Driving)` in the `if let` scrutinee. Treat parking spots as a
ledger.
