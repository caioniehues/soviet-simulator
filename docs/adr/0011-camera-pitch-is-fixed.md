# The RTS camera has no pitch control, deliberately

**Status:** decided 2026-08-17, **not yet built** — R1, [#117](https://github.com/caioniehues/soviet-simulator/issues/117).

The rig exposes pan, yaw and zoom; pitch is fixed at −45° and no input system touches it.
This reads as an omission and is not one, so it is recorded before someone "completes" the
controller. A single pitch guarantees every building is seen from the angle its procedural
geometry and the R0.1 grounding pass — SSAO, contact shadows, the four-cascade shadow
config and the 260–950 m fog band — were tuned against, and it removes the failure mode
where a player tilts to the horizon and concludes the game looks broken. Adding a pitch
axis is not one input system: it means re-validating the cascade distances and the fog
closure at every reachable angle, which is R14 polish work if it is ever wanted. R1's
camera pass therefore ships easing and zoom-to-cursor only, and edge-pan is cut outright
as the control players most often turn off, with middle-drag already covering it.
