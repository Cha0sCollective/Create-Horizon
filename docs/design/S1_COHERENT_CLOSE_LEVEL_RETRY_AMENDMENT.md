# Coherent close: fixed native Level retry input

Status: PROPOSED; no new source release until fresh independent design acceptance
and private checkpoint. Extends the accepted coherent-close assignment at
485d4b1914c639d2a6cfc68f6fa7191ae5adbdaf within its existing fourteen paths.

## Concrete gap

Native Level::write reads unknown level.dat fields, conditional sidecar existence,
world clocks and wall-clock LastPlayed during each invocation. Repeating a clone
of the modeled Level after partial backup rotation can lose original unknown data
or change the output. The staged writer's fixed final records do not solve these
separate native metadata writes. Do not duplicate the native codec or invent a
journal, transaction, rollback guarantee or fifteenth path.

## Bounded save.rs extension

The existing save.rs author may add NativeLevelWriteInput, constructed purely from
an owned final Level, world root and actual ClockControls. It is Clone, with clones
sharing one private retained native plan. Its capture_native operation runs only
on Root's actual owned IO worker, before ANY final metadata or Level mutation.
Capture all required original native reads/existence/legacy decisions, complete
merged unknown fields, exact player output, native sidecar outputs and one fixed
LastPlayed value before publishing a complete memo. A failed capture publishes no
partial plan and performs no writes; retries may finish capture before final writes.
After successful capture, no retry rereads, resamples or regenerates these outputs.

write_native reuses that complete memo and the actual native per-file codecs and
write order: weather, clocks, conditional world generation, conditional rules,
player data, level.dat. Each attempt retains truthful partial/unknown file effects;
an error is not rollback. The memo lives in the original input outside disposable
worker returns, including panic or lost return. Any memo mutex is worker-only;
owner polls never acquire it, and poison refuses progress. No public setter or
worker-returned Level replacement grants live authority.

Factor native pure formatters where required. Ordinary Level::write must retain
its existing read/write ordering, guards, UUID/world-spawn assignment and partial
failure behavior. It must not become an eager read-all operation. Existing
write_world_clocks callers retain their current behavior. The retained plan uses
the actual frozen clock controls as well as final day/time, preserving unknown
clock fields from capture even if an earlier core Clock write partially succeeds.

## Composition and owner projection

Root captures this plan before the fifteen-family metadata write. It then writes
the prepared final Level after core metadata, using the same plan on explicit
retry. The final modeled Level comes from original Level plus aggregate raid extra
FIRST, then native warden/sync/actual owner projections. Prepare required UUID and
world_spawn before native entity/leash payload capture; use final actual owner
spawn when world_spawn is absent. No post-projection arbitrary Level mutation.

For a genuine integrated owner, reuse existing session_player_level to include
actual body motion, food exhaustion and food timer in the retained-only owner
projection. Ordinary store_owner remains unchanged. Acknowledge only individually
captured and preflighted fields, including these three when applicable. Original
Level, participant roster/body/raw item fields and actual resource/save/tree
allocations remain frozen and revalidated before aggregate CPU acknowledgment.

The game.rs/lan.rs author may bind the library input inside the existing private
Level input, preserving private attempt association and assignment-only commit.
Root retains the original input independently of worker clones. Native completion
still does not prove global CloseCommit or authorize disposing transport owners.

## Focused evidence and limits

Add defining native tests for frozen LastPlayed and unknown fields across actual
partial publication/repair, missing versus existing conditional sidecars, clock
controls plus unknown fields, capture refusal before writes and unchanged ordinary
partial-error ordering. Use existing gated controls and actual native codecs.
Root chooses minimal affected regressions; unchanged predecessor evidence remains
at its real revision. No broad replay or general fault framework.

Fresh independent design acceptance/checkpoint precedes this source extension.
Preserve ALL14 sources and s1cl1-prefix artifacts before author corrections;
freeze exact hashes. Whole-candidate SOURCE/tool review precedes compiler/runtime,
and a different FINAL reviewer remains required. No public unfinished source,
ordinary activation, C0e/C1, connected G1, live S2, PR, merge or release is granted.
