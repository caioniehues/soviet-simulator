# Persistence

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

- `SimulationSer { world, version: String, res: map<name, bytes> }` (`simulation/src/lib.rs`).
  Each resource is bincode-encoded separately; disk saves use `CompressedBincode` (`miniz_oxide`);
  replays use JSON (`common/src/saveload.rs`, `Encoder` trait).
- `VERSION` is `0.6.1`. On load, a major-version mismatch (or minor, while major is 0) **warns**
  and proceeds; a resource that fails to decode is replaced by its default. A loaded world can
  therefore be a hybrid of saved entities and fresh resources.
- There is **no migration mechanism**: nothing transforms a version-N payload into version N+1.
- `slotmapd` keeps key generations stable across a save cycle, which is why it exists.

## Why this blocks the target

Nearly every structural proposal — record/body split, SoA cores, `EnumMap` stock, cadence bands,
`DispatchManager` extraction, keyed randomness — changes serialised layout. Without a migration
path each one means "new game required". The charter allows explicit version-gated hard breaks
during development and requires compatibility from the 1.0 release candidate; "one continuous
save" is a product value. The design thread specified an envelope and forgot the migration (Lane
C2 §4.1).

## Target design

**Envelope** for released saves:

```text
magic · format version · schema/game version · codec · uncompressed size · compressed size · checksum · payload
```

Old saves without a header are version 0. Thirty to fifty lines around the existing bincode
payload; the first need is the envelope, not a codec change.

**Migration seam.** A `SaveMigration` trait that transforms the `res` map (and world bytes) from
version N to N+1, applied in sequence by `VERSION`. Every structural change ships its migration or
declares a hard break explicitly.

**Hard reject** on major mismatch instead of warn, once migrations exist.

**Codec** is an open conflict: keep bincode 1 (current) with the envelope; `postcard + zstd`
(design thread) later if measured worthwhile; `rkyv` for internal snapshots only. Lane C1's prior
survey: "save risk is versioning and bounds, not encoding speed or syntax."

## Migration

1. Envelope + version-0 fallback.
2. `SaveMigration` trait with one no-op migration to prove the seam.
3. The first real migration rides the first structural change (likely `EnumMap` stock or the
   citizen record).

## Open decisions

- Is "new save required" acceptable pre-1.0, or does the seam come first? (Recommended: first.)
- Codec change, and when.

## Related

- [Entity identity](entity-identity.md)
- [Serialization standard](../engineering/serialization.md)
- [Technical stack research (2026-08-24)](../explanation/research/technical-stack-upstream-2026-08-24.md) — the bincode-2 and envelope findings
- [Lane C1 §2 C1-22](../research/conversation-mining-2026-08-28/C1-rust-crates.md)
