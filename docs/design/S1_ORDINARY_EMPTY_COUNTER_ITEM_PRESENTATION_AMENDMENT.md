# Captured BlockItem presentation amendment

PROPOSED bounded amendment to S1_ORDINARY_EMPTY_COUNTER_PACKET, November 4, 2026.
No engine edit is released by this proposal. Same fixed mob-free Creative
TestTerrain Overworld, single captured pair/type/schema, integrated Memory,
dedicated two-client TCP and in-frame comparison. Open to LAN remains deferred.

## Actual finding and existing services

Engine branch codex/s1-ordinary-empty-counter remains at accepted/public parent
bbf10296491592863d95359c8e97daff4bbe4c7f, with only the original five assigned
Rust files dirty. Current source13 map is
0483cb7a92d750024375c29c3cb79bd5b56027de3a23486906e786834ca5da80.
Original design ad3c2ef0/assignment f23b95f3/private checkpoint62697b94/serial gate
ec0a5106 preceded those edits. Preserve all original gates, snapshots and failures.

Focused CPU test4 passed381.03 seconds, receipt
99e2c4cd554f62d894bb24e8570cc7ce2f07fe46b2abd1c19e76ad39f8bf309f:
six same-executable children cover NEW/Unplaced, fresh Unplaced placement/actual
Wasm/Claimed close, fresh Claimed resume/second-target refusal in all three modes.
Original tests1/2/3 remain FALSE. Normal fast1 passed96.41 seconds; ELF
996943cbc88e1bedf05e4b164639daef888022ec19cc56b79802631c0770bf66.

Actual normal in-frame window1 runner passed70.03 seconds, natural game exit0,
four coherent epochs, no game TERM/KILL. GUI/native selection, ordinary right-click,
physical template cube, Guest/empty-slot images and same-process claimed reentry
worked. Root viewed actual counter hover, number-key hotbar, placed and reopened
screenshots. Visual QA is HOLD: tooltip says counter and count64 occupies the
expected hotbar cell, but its item icon is blank in Creative and the hotbar. The
placed iron-template block is visible. Runner success is not visual acceptance.
Preserve .local/s1ec1-inframe-1* and source13/binaries/resources unchanged.

Read-only caller inspection identifies the missing connection:

- mod_content.rs Plan creates the BlockItem from its actual template item and
  clones/renames the items.json prototype, including minecraft:item_model.
- The existing build fallback supplies assets/<namespace>/blockstates/<id>.json
  from the template, so the physical block already renders.
- That fallback does not supply the client item definition under assets/<ns>/items.
- Existing ItemRenderer::load_definitions and ItemModels::append_layers already
  load/resolve those client definitions. The missing alias definition produces
  empty item layers. No new renderer or H11 implementation is required.

## Smallest proposed repair

Extend the existing content materializer's template fallback for declared block
items. Use the actual Plan-resolved template item, not an assumption that a block
and its vanilla item have identical names. If the mod supplies its own client
item definition, preserve that override. Otherwise write the template's existing
client item definition to the alias's runtime overlay path, retaining its native
model/texture references. Refuse a required missing template definition with a
specific error. Preserve the owner-supplied base and captured package bytes.

This copies runtime resource data from the owner's existing prepared installation;
no Minecraft asset bytes or generated resources enter Git. The current materializer
stamp already includes the actual executable metadata, so a newly built executable
invalidates its previous overlay. Preserve the existing cache contract; do not
introduce a cache framework or unrelated resource repair.

At the existing fresh CPU palette preflight, validate the actual pack's prototype
and referenced client definition before appending the captured entry and before
native/remote acquisition. Pass the existing PackResources reference through the
same helper call. Do not change item identity, components, count, registry IDs,
native packet encoding or authoritative admission to make presentation work.

Maximum seven EXISTING paths, subject to review and a new exact assignment/gate:

1. game.rs: only extend the existing data-only palette preflight to inspect the
   actual PackResources item model; keep all world/input/arming behavior unchanged.
2. game/window_close.rs: only pass that existing pack to the same pre-acquisition
   helper; no constructor/acquisition/error-owner redesign.
3. game/levels/session_checkpoint_registered_tests.rs: corresponding preflight
   helper argument and bounded fixture assertions where needed.
4. game/retained_server.rs: unchanged source13; remains in the exact source map.
5. game/retained_server_tests.rs: unchanged source13; remains in the exact map.
6. mod_content.rs: minimum existing BlockItem client-definition fallback only.
7. mod_content_tests.rs: focused template fallback/resolution and declared-asset
   override regression through the real materializer, using owned prepared inputs
   and small authored fixture declarations. No new general test framework.

The two library paths must be verified untouched, snapshotted and gated BEFORE
their first edit. Existing source13 remains the baseline for the five assigned
paths. No item_render.rs, game/item_draw.rs, shader/model renderer, H11, protocol,
world/domain/slot, source reference or package/schema changes are proposed.

## Validation and boundaries

Fresh independent design acceptance, exact source assignment, private checkpoint/
sync, raw baseline and serial amendment gate precede edits. Fresh substantive
source/tool review precedes compilation/execution; different FINAL/privacy precedes
any reviewed public branch sync. No PR, merge, release or cleanup is authorized.

Run appropriate library/feature/lint checks, the focused materializer regression,
and normal rendered counter scenarios across the three adopted modes. Inspect the
actual Creative icon, hotbar icon, empty cursor, placed cube and reopened scene;
read actual coherent Guest/empty-slot files and immutable OWN/archive. Dedicated
rendered coverage includes fresh Unplaced and Claimed servers with two fresh
matching rendered clients each. Use existing renderer/driver machinery.

Reuse unchanged source13 CPU ownership/Wasm/recovery/refusal results with explicit
attribution. The amendment changes client asset presentation and preflight, not
gameplay authority, native tables/codecs, domain/grant/history, callbacks or saves.
Do not rerun historical full batteries merely for a presentation connection; add
focused checks if review or a new failure identifies an affected behavior.

This completes the already requested captured Creative entry presentation. It
does not add a supported world profile, general addon/asset framework, protocol,
schema, H11 animation or later-stage gameplay. Owner input is required if review
finds that such expansion is actually necessary. Deliverable1/C0/fullC1/G1/S1 stay
OPEN. Filled-slot ingress, departure/removal, replication/H11 and combined G1 remain
the following adopted work.
