# H04b bounded dimension-probe prerequisite

The integrated dimension run `24c6ec1e-822f-48ea-b197-b80766e3be1e` passes.
In-frame run `32759998-39ab-4249-8ac6-37206d00a7be` is a retained failure, not a
pass: after returning to the Overworld, the player remains in travelling while
lighting never satisfies its existing 5x5 arrival gate.

Read-only audit identifies a concrete generator defect. Streaming lights the
outer finished ring using decorated proto neighbours that do not become resident.
TestTerrain's proto already contains a complete Chunk, but it does not implement
the trait's optional light_view. Its default None leaves corner lighting waiting
with no runnable job at radius 2. The Nether's VanillaTerrain supplies this view,
so outward travel completes. FlatTerrain has the same complete-Chunk omission.

Add immutable Chunk snapshots as light_view overrides to these two generators.
Keep arrival conditions, finishing ranges, player construction and timeouts.
This exposes existing proto data for lighting; it must not promote neighbours to
resident gameplay chunks, consume the proto or mutate it through shared sections.

Tests: both generator views preserve coordinates, blocks/biomes and normal finish;
editing the view cannot edit its source. Add an explicit owned-resource radius-2
streaming regression requiring the full 5x5 arrival square to become lit while the
outer corner remains nonresident. Then freeze a reviewed build and repeat actual
in-frame and integrated round trips with the same strict driver. No superflat
real-game claim follows from unit tests alone.

Coordinator owns these separate engine files and integration. H03c's isolated
worker retains exclusive Cargo ownership until it reports completion. Require
fresh independent Sol High review and focused checks before the new game build.
No public push or larger world-loading/save redesign is authorized by this fix.
