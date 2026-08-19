# Memory Index

- [Building catalogue layout](catalogue-layout.md) — where each per-kind value lived before Phase 1, and the trap in each
- [BuildingSpec field design](catalogue-field-design.md) — why demands split into three small types instead of one, and what got left out
- [Catalogue test traps](catalogue-test-traps.md) — the tautology every deletion phase creates, and how to drive one production frame headless
- [Customs imports](customs-imports.md) — StoragePolicies "no band" vs "(0,0) band" trap; how buy_imports/sell_exports avoid ping-pong
- [SimPlugins/GamePlugins group shape](sim-plugins-group-shape.md) — ticket #118 group order, per-bench disable lists, why ConstructionSimPlugin breaks most benches
- [Plugin-group dedup trap](plugin-group-dedup-trap.md) — deleting a duplicate resource registration silently breaks in-module test app() builders at runtime, not compile time
