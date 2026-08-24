# Undo is "rescind the order", available only before ground is broken

**Status:** decided 2026-08-17, **not yet built** — R1, [#117](https://github.com/caioniehues/soviet-simulator/issues/117).

The charter's one rule is that nothing teleports and every effect has a physical cause,
with fiat allowed only as an explicitly marked bootstrap. A conventional editor undo —
remove the last placement, refund its cost, whatever state it had reached — is a fiat
rewind and breaks that rule. So R1 ships **rescind** instead: it acts on a construction
site only while no materials have been delivered and no work has been done, the window in
which nothing physical has yet happened and removing the site therefore costs and teleports
nothing. Past that point rescind refuses with the reason, and the player demolishes at real
cost. The guard condition is the entire difference in implementation cost against a plain
undo, and it buys a refusal that *teaches* — "cannot rescind: 12 t gravel delivered" states
the game's central rule at the moment the player is trying to break it. The accepted risk
is that the window feels short; the mitigation is on the same rung, since a ghost preview
carrying the material bill and the refusal reason is what stops misplacements before the
click.

**Constraint on any future extension: rescind must never rewind an id counter.**
`DockIndex` is keyed on `(next_node, next_building, node_count)` and the `PathService`
snapshot on `(next_node, next_segment, count, mod_sum)`; both assume the counters only ever
grow. Rewind one and a stale cache key can match a topology it no longer describes, which
misroutes freight silently, with no error anywhere. Rescind as specified is safe because it
only ever removes a site that never broke ground, so nothing is re-added and no counter
moves — but the obvious "redo" someone adds later would break exactly this. Rescind is also
keyed on `BuildingId`, never on a runtime `Entity`: `BuildingEdit::Demolish` already stores
a raw `Entity`, which goes stale as soon as anything else despawns.

Rescind stays distinct from the road tool's existing **rebuild** (`RoadEdit::RebuildLast`,
restoring the last cut segment and re-paying its gravel). The two share an intuition and
nothing else: rebuild re-pays material and acts on something that physically existed,
rescind refunds nothing and acts only on something that never broke ground. Collapsing them
onto one key would leave the player unable to predict which rules apply before pressing it,
which defeats the point of a refusal that teaches.
