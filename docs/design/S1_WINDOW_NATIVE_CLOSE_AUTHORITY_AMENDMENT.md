# C0e2c original in-frame native-close authority

Status: HELD architectural supplement. No source release until a fresh independent
Sol High review accepts this exact file, a private checkpoint is synced, and the
root gate records it. Existing eleven-path budget and all prior scope limits stay.

## Actual failure and role distinction

At engine parent bd0647a6, candidate source manifest47385401, the real Linux window
rendered its original world twice, resized natively and admitted world-leave.
The same live endpoint reported Blocked with the actual message `native close
identity preparation requires the authoritative owner`. This is a failed window
qualification, preserved under .local/s1wc1-attempt22; it is no completed close.

game/execution_role.rs explicitly distinguishes presentation role from in-frame
simulation: Client may simulate a world in-frame. The accepted dormant C0d native
close path in game.rs rejects every Client role before preparing native UUIDs.
Earlier native-close fixtures used server roles; they did not establish in-frame
authority. Do not relabel this window IntegratedServer, infer authority from
remote=None, an option, a path, UUID bytes, counts or an enum, or broaden ordinary
client/domain/guest authority to make the smoke pass.

## Fixed constructor-issued capability

Add a private, non-Clone NativeCloseAuthority in game/world_loading.rs. Only the
successful opaque ColdPreparation factory issues it after exact original entry
and complete original cold-transfer checks. Return a distinct ColdPreparedWorld
containing BOTH the original PreparedWorld and its capability; fields private.
No constructor from a Game, observation, caller boolean, raw identity/path, closure
or an arbitrary PreparedWorld is exposed. The existing test-only transfer keeps
its old raw PreparedWorld signature and does not issue production authority.

ColdPreparedWorld may supply a fixed borrowed read-only view for the app's original
construction association. Game's private InitialGameWorld::Retained consumes the
entire ColdPreparedWorld and installs its actual world/storage/session/streamer,
NativeCheckpointOrigin and NativeCloseAuthority together before running the view.
No swap into a legacy Game or second original Box. Ordinary Game construction has
no such capability. Cold failures/refused entries retain their original guard and
never obtain a capability. Source changes remain within game.rs, world_loading.rs,
window_close.rs and window_close_tests.rs, all already in the eleven-path budget.

The capability binds the exact original SaveDir allocation (Weak/Arc identity),
identified WorldSession/world ID, original root and dimension, and the genuine
nonforgeable NativeCheckpointOrigin from that same factory. Its fixed validation
must reject foreign original allocations/sessions and current mode changes. It
cannot be cloned, exported to guests, reconstructed from matching bytes, or reused
to authorize another Game. Actual private constructor flow, not addresses/counts
or a free status flag, establishes uninterrupted original-world association.

Permit the Client role ONLY at these three C0d native-close gates when this same
original Game carries a valid capability: prepare_close_identity_fields_avoiding,
prepare_retained_level_metadata_from, and preflight_retained_level_metadata.
Validate original provenance and current mode at EACH use. These encompass UUID
preparation, native Level preparation and final native Level metadata preflight;
passing only the first gate would still leave the actual close blocked. Existing
remote, integrated proxy
and lan_acting refusal checks remain unconditional. Keep the server-role path's
behavior and all other authoritative simulation, mutation, package, replication,
guest and host permission checks unchanged. This is solely permission to prepare
the actual original native close identities and Level projection within C0d's
existing frozen snapshot and checked native-save protocol, including its final
unchanged snapshot/receipt preflight. It is no general simulation authority API.

The capability is not a producer/writer/LOD/network/UI completion witness. All real
worker fences, original IO outcomes, exact Retry and typed native/client handoffs
remain required. Preserve the old dormant disposal guard. Do not bypass preflight,
forge receipts, accept a foreign closure or waive a missing helper to obtain it.

## Defining evidence and release gates

Fresh read-only Sol High DESIGN acceptance of this supplement, private checkpoint
and explicit gate amendment precede source. Preserve all eleven paths and every
top-level packet-prefix artifact before each correction. Renew exact SOURCE/tool
review before formatter/compiler/runtime; a different FINAL still gates use and
public sync. Bind this supplement and review/checkpoint in the source gate and
qualification guard. No standing team or operating permission expansion.

Add a focused real-resource capability case using actual cold constructors: exact
original Game validates; another genuine cold Game/allocation is rejected; ordinary
legacy/in-frame client observations cannot obtain the capability and the existing
close authority guard still refuses them without preparing UUIDs. Current-mode
changes must refuse too. The positive actual capability case exercises native
Level preparation AND final metadata preflight, then completes checked close.
Close every actual original native owner through the checked protocol before ending the case.
Use fixed production factory APIs, not the test-only cold transfer. Missing input
or unresolved owners fail. Add no generic executor or broad test framework.

Rerun the three real-window scenarios on the revised application: original world
close must now progress into the actual held native IO, preserve the same RPC
through Blocked/exact Retry, then complete original C0d/LOD/control/client handoffs.
Native control-final Retry/failure Exit, stable exact cleanup cause, persistent
frame-error route/minimization/signals remain required. Reuse the prior eighteen
normal cases and fatal control only at their actual59297333 pin where new changes
do not affect their declared inputs; separately qualify new capability behavior
and revalidate affected constructor/close paths as independent SOURCE review selects.

No ordinary/default integrated, dedicated, external authentication, general world
mutation, C1/live S2, G1, upstream contribution, PR, merge, tag or release follows.
