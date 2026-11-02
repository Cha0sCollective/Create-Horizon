# Coherent close: real initial retained fixture transfer

Status: PROPOSED, existing fourteen paths only. No ordinary entry activation.
Fresh independent design acceptance/checkpoint precedes this source extension.

Existing session test constructors use WorldRequest::prepare and legacy native
SaveDir/Streaming. They are valid Level/image fixtures but cannot prove retained
close. C0a's Game::adopt_retained_prepared retains the original armed owner inside
Game; it deliberately does not install the owner's World/Streaming into Game.
Do not upgrade, relabel or replace a live legacy native owner to make tests pass.

Inside existing world_loading.rs, add an exact test+paired+native-control+debug
gated RetainedPreparedWorld::into_cold_prepared_for_test(self)
-> Result<PreparedWorld,Self>. Check complete genuine Ready, uncancelled,
nonclosing/error-free original owner with all required Level/World/Streaming/
SaveDir/session/random/origin fields present BEFORE taking anything. Transfer
those SAME native owners, plus locator, into existing PreparedWorld; disarm the
old preparation guard only after successful ownership transfer. On refusal return
the same armed owner for its existing explicit close path. No cloned authority,
new SaveDir, native owner restart, close witness forgery or lost partial owner.
No stream close has begun, so transfer changes no already-bound close address.

The existing image/test author may use the real RetainedWorldLoader finished-only
harvest and this transfer to prepare a genuinely fresh cold Game::new candidate.
Require its title-idle placeholder only: no session/native entry/preparation/
transport/control/loading owners, no resident world/items/parked levels or live
native save obligations. Use existing finish_open_world only for this fixed
fixture installation BEFORE native gameplay owners exist. Preserve its native
initialization and actual advancement helper. Never take a populated Fixture.game
and erase or swap its owners to obtain this cold state.

Expose a SEPARATE retained-close fixture path; existing ordinary session/Level
test helpers and all ordinary production constructors/callers remain unchanged.
The accepted opaque populated close-fixture custody then begins CloseOwner after
its fixed actual setup, with receivers and private no-connector provenance intact.
Counterless NativeWorld fixtures use the same initial transfer and existing real
parked constructor. Root remains sole executor; focused test establishes exact
allocation/World/lock custody and refusal before any take. Whole SOURCE/tool
review precedes compiler/runtime. No C0e/C1, normal startup activation, G1 or release.
