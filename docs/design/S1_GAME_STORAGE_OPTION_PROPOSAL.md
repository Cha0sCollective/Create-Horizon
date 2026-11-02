# Game storage absence: held representation proposal

Status: HELD design only. No source, execution or actual no-storage client
authority. S1/G1 stay open; no retained remote Game activation.

Parent: accepted engine7d443a1bf41a83db6ff5b8426460a224a4bf612a, tree
d37a5faec2740407c945ace5352c2e7d5caa1cea. FINALcd4090f3/index36a4925c,
reviewed public syncb18671df and private results60d508cd/sync01dafb2b are
accepted and immutable. Proposed branchcodex/s1-game-storage-option, prefix
.local/s1gs1, same stable checkout and its own warm target-s1nc1. No copying.

## Concrete change

Game currently requires Arc<SaveDir>; legacy remote creates horizon-remote-unused.
That placeholder cannot establish a retained client without local storage.
Represent absence with Game.save: Option<Arc<SaveDir>>, using standard Rust
rather than another enum/module/framework or a fake SaveDir/writer.

EVERY current production Game constructor/replacement installs Some(original_save),
including unchanged legacy remote setup. No production None Game initializer,
setter, command/environment switch or retained remote installer. Current streaming
factories stay Existing. Later activation must honestly adopt both fields and
guard reachable native-only operations before mutation/owned-input loss. None
does not prove native writer closure.

Use field-local Option borrows for split Game-field access. Assertions apply only
to audited current Some factories; no default path for absence. Clone the contained
original Arc for native records, not the Option. PreparedWorld/PrepareOwner,
NativeStreaming/Parked, dimension parent saves, captured payloads and expected
native records stay raw Arc<SaveDir>.

Authority/identity queries fail closed on None: authorizes/matches false,
optional native lookup None, checked native validation refusal. Preserve ALL
Some allocation/session/root/dimension/origin/role/provenance checks. Keep
NativeCheckpointOrigin::validate's raw &Arc<SaveDir> interface and exact predicate.
Some/presence/labels never grant native authority.

At levels.rs:1188 onward, preflight Existing streaming AND present save before
clearing provenance or moving World. Swap the contained original Arc with raw
Parked.save at its original position. No normal swap take(), temporary substitute,
new SaveDir/Arc allocation, changed successful swap/disposal order or new actor.
Current replacement wraps its original Arc at the same original boundary.

## Exact maximum whitelist

24 existing paths relative to crates/horizon-client/src/:

```
game.rs
game/integrated.rs
game/integrated/transition_ownership_tests.rs
game/lan.rs
game/levels.rs
game/levels/capture_integration.rs
game/levels/capture_integration_tests.rs
game/levels/counter_restore_integration_tests.rs
game/levels/idle_close_tests.rs
game/levels/native_write_fence_tests.rs
game/levels/parked_save_tests.rs
game/levels/session_checkpoint.rs
game/levels/session_checkpoint_registered_tests.rs
game/levels/session_checkpoint_tests.rs
game/levels/slot_advance_capture_tests.rs
game/nether_portal.rs
game/persist.rs
game/raids/metadata_tests.rs
game/raids/lifetime_tests.rs
game/remote.rs
game/saving.rs
game/window_close.rs
game/world_loading.rs
game/window_close_tests.rs
```

The first23 are actual Game.save sites/aliases; the last permits the focused
absence assertion in an existing native test. No new engine module or wildcard.
No lib streaming/net/decoder, dependency/lock, launcher, reference/playable or
public documentation edits. Stop for exact scope review before another path.

Distinguish raw records: NativeLevelExpected.save in game.rs1776/1811,
RetainedDimensionPreparation.parent_save in world_loading.rs1394 onward, and
CoherentImageContext.save in levels/session_checkpoint.rs stay raw. Multiline
session.game.save/s.game.save aliases require deliberate migration, not global
replacement of every similarly named field.

## Meaningful checks and limits

No tests of standard Option mechanics. Extend the existing exact Game-bin case
game::window_close::tests::cold_native_close_authority_binds_original_and_all_three_gates.
After positive actual cold factory authority, temporarily move its original Some
Arc into a local test owner, verify authority/checked native admission refuses
absence, then restore THAT SAME Arc before normal native work/close. Preserve
foreign-owner/legacy/mode and all-three-gate checks. Do not drop the original,
fabricate a replacement or claim supported no-storage Game. Use already visible
checked queries; no visibility/identity getter expansion just for a test.

Run that ONE exact process plus explicit-file format, paired all-target lint/
compile, feature-off compatibility and public-tree checks. Bind actual artifact/
profile/features/binary and pinned28176 resources, isolated data; missing fixture,
timeout/assertion/nonzero exit FAIL. Add the existing parked-save success case
only for actual successful swap/disposal changes or concrete concern. No full
Game/window/network/save/Java/owner/fatal replay; counts are estimates, not cap.

No actual None-bearing Game runs here. Later activation still must address
lan.rs2742 (save.tables used for packet encoding) and persist.rs1444 onward
(entity ownership changes before save access). Presence assertions are not safe
remote dispatch. Session/decoder adoption, immutable or revision-aware mesh
configuration, backpressure, original client construction/close, server startup/
Hub, scoped joining and LOD-before-server-storage ordering remain separate.
No native writer, whole-client, G1/S2 or performance claim follows.

## Gates

Fresh independent SolHigh DESIGN acceptance of this proposal/assignment, synced
private checkpoint and exact root release precede source. Root owns serial
execution/Git writes; bounded authors native TEXT only and reviewers native
procedural READONLY under inherited workspace-write. Preserve every packet path
and prefix before corrections. Exact SOURCE/tool before formatter/compiler/
runtime, then post-format binding renewal. Archive mutable execution/config/
recipe/instruction inputs and compare qualified live/archive/Git; prior freezes
stay immutable. DIFFERENT FINAL/privacy before use/reviewed public sync, fresh
visibility/exact origin and normal hooks. No PR/merge/tag/release/default
activation, G1 or live S2 permission from this proposal.
