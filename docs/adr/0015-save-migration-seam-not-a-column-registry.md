# The save gets a migration seam, not a per-domain column registry

`save.rs` is one 350-line `snapshot` and one 350-line `restore`, eleven row structs and
fourteen hand-paired `to_u8`/`from_u8` functions, with symmetry between the two halves
enforced only by hash-equality round-trip tests. Distributing the columns so each domain
owns its own is the obvious deepening and is deliberately **not** being done, so it isn't
re-proposed as an oversight.

The reason is seam discipline: one adapter means a hypothetical seam, two means a real one.
R1 needs exactly one new field (building rotation), so extracting a single domain would
produce a registry with one entry — structure nothing varies across. The full extraction is
a 1400-line refactor inside a rung whose subject is placement.

What is being built is the part with two known future users: **a migration path, which does
not exist at all today.** `from_bytes` currently rejects any file whose version differs from
`SAVE_VERSION`, so a bump destroys every existing save. R1 bumps to v6 for building rotation
and R8 bumps to v7 for terrain — the ability to load an older save is needed twice
regardless of whether the columns are ever distributed. Build the thing with two callers;
skip the thing with one hypothetical one.

The column registry stays on the table for whichever rung first needs a second domain to
vary — most likely R6 services, which adds four domains at once.
