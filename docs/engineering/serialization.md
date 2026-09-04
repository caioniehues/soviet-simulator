# Serialization standard

**Kind:** standard
**Authority:** operational; rules marked *(target)* depend on the persistence decision
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-04

## Rules

1. **Must:** every serialised resource is registered through `init.rs` (`register_resource`) so
   `SimulationSer` and `hashes()` see it; a resource that must not persist uses
   `register_resource_noserialize`.
2. **Must:** entity identity survives a save cycle — `slotmapd` exists for this; never replace it
   with upstream `slotmap` without proving the round-trip invariant another way.
3. **Must:** a change that alters a serialised layout says so in its `bd` issue and its commit,
   and either ships a migration *(target)* or declares an explicit version-gated hard break (the
   charter allows hard breaks during development; none after the 1.0 release candidate).
4. **Should** *(target)*: released saves carry an envelope — magic, format version, schema/game
   version, codec, sizes, checksum, payload — and load through a `SaveMigration` sequence
   ([persistence](../architecture/persistence.md)).
5. **Should** *(target)*: a major-version mismatch is rejected, not warned past. Today it warns and a
   resource that fails to decode silently becomes its default, producing a hybrid world.
6. **Must:** the round-trip determinism check in `TestCtx` keeps passing after any serialisation
   change; a new resource adds itself to that check by registration.
7. **Should:** keep internal snapshots (`ArcSwap`-published views, caches) on a separate, faster
   path from the released save contract; `rkyv` is a candidate there only.
8. **Must** (sov-bdr): all `bincode` access goes through `common/src/saveload.rs`
   (`Bincode` / `CompressedBincode`, varint `DefaultOptions` config). Never call
   `bincode::` free functions or paths from any other file — they use a different
   (fixed-int, allow-trailing) config whose streams this file cannot read, so the
   failure mode is a corrupt save, not a compile error. Enforced by the
   `no_direct_bincode_use_outside_saveload` unit test in `saveload.rs` (clean as of
   2026-08-28: zero direct uses outside that file).
9. **Version gate** (sov-bdr): `Simulation`'s `Deserialize` impl warns — rather than
   refuses — on a major/minor version mismatch, and a resource that fails to decode
   falls back to its default, which can produce a hybrid world. Warn-is-enough during
   development because the charter allows hard save breaks pre-1.0 and every break is
   declared in its `bd` issue and commit (rule 3); refusing would add no signal while
   developers iterate daily. After the 1.0 release candidate the gate must reject
   (rule 5, target envelope + `SaveMigration`).


## Related

- [Persistence (architecture)](../architecture/persistence.md)
- [Determinism standard](determinism.md)
- [Current substrate — persistence](../architecture/current-substrate.md#persistence-and-determinism)
