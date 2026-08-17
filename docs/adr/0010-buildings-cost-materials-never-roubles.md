# A building's placement cost is its material bill, never a rouble price

**Status:** decided 2026-08-17, **not yet built** — R1, [#117](https://github.com/caioniehues/soviet-simulator/issues/117).

The charter's R1 line asks the ghost preview to show a placement's cost, and building
placement turned out to have none — no funds gate exists, only zoning siting. The obvious
fix, giving each building kind a rouble price, is rejected: the rouble is explicitly the
*foreign* currency spent at the border, and the domestic economy is allocated rather than
traded, so charging cash to raise a domestic building would contradict the identity the
charter refuses to re-litigate. The cost the ghost shows is instead the construction site's
**material bill** — tonnes demanded, and whether yards within supply range can actually
meet it. This already exists on the road side, where `pay_gravel` drains nearby yards
all-or-nothing and refuses when the material is absent, so the decision unifies buildings
and roads under one cost model instead of splitting them across cash and matter. It is
also the more useful preview: "18 t cement, nearest supply out of range" is a siting
decision, where a price is only a number.
