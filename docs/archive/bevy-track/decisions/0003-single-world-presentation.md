# Single-world presentation; the invariant is direction, not a mirror layer

The carried architecture mirrors active pawns into separate presentation entities. In a
single-world Bevy app that bookkeeping buys nothing yet: pawns carry `Transform` directly and a
`PostSimEasing` set in `Update` eases rendered transforms toward authoritative positions. The
load-bearing rule is directional — presentation reads sim state and never writes it — enforced by
set ordering. The mirror layer stays documented as the upgrade path and becomes mandatory only if
the sim moves to its own world/thread (ADR 0001's open door).

**The test for what presentation may write, added after the rule was misapplied twice.**
"Player intent, not sim state" is not decidable and produced two violations that each
defended themselves in a comment: `drive_recruitment_controls` writing `RecruitmentPlan`
and `drive_band_tuning` writing `StoragePolicies`, both claiming the standing of
`SimSpeed`. The decidable test is two questions — **does a sim system read it as a
decision input, and does it persist in the save?** `SimSpeed` fails both: it only gates
how many ticks the clock runs, and it is not saved. `RecruitmentPlan` passes both, and so
does `StoragePolicies`. Anything passing both is sim state and goes through a queue,
whatever the player's relationship to it feels like.

That gives three categories, not two. **Sim state** is queued and applied at the barrier.
**Player controls** (`SimSpeed`, camera, tool mode) are written freely and never saved.
**Policy** — player-authored values a sim system reads and the save persists — is queued
like sim state, through its own `PolicyEditQueue`, so the category is visible at the call
site rather than only in this document.
