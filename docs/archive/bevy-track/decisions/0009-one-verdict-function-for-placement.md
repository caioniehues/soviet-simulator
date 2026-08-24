# Placement rules live in one verdict function, called by both the ghost and the commit

**Status:** decided 2026-08-17, **not yet built** — R1, [#117](https://github.com/caioniehues/soviet-simulator/issues/117).

R1's ghost preview must state a refusal *before* the click, which puts the same siting
rules on two paths: the preview, running every frame on the presentation side, and
`apply_building_edits`, running at the sim's `ApplyCommands` barrier. Duplicating the
checks is the obvious shortcut and is rejected — a preview that approves what the sim then
refuses is worse than no preview at all, and the two copies drift the moment anyone adds a
rule to only one of them. Instead a single pure verdict function takes a kind and a
position and returns either approval or a refusal carrying its reason; the ghost calls it
to colour and label itself, and the apply path calls it as the actual gate. The payoff is
in later rungs, not this one: R4's new building kinds, R8's slope and grade gates and R10's
basin edges each add one variant to one function and appear in the preview for free.
Presentation still never mutates — it only asks.
