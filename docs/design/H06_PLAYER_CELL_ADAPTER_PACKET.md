# H06d dormant actual-Game player-cell adapter

Bounded G0 packet accepted after fresh independent read-only Sol High design
review with no blocking findings. Review checked draft SHA256
bf2708492a3eca25c02e489cada41417a8b0e610e84ac1d6611cf603b44dac3b
against clean engine 51e2882cd582e862e5e709e7d6d8b7e7978d852e. H06c native,
broad and final independent receipt qualification now passes at that base.
This unit exercises
real Game inventory ownership, not another detached transfer kernel. It enables
no gameplay service and establishes no authentication, replay or persistence API.

## Exact scope and concrete consumer

Production: new paired-packages-gated game/slot_player.rs and its declaration in
game.rs; widen ONLY Game::stack_max_size in game/commands/items.rs from pub(super)
to pub(in crate::game); optionally widen existing slot_retirement::check_counts to
pub(super) for its unchanged borrowed representation walk. Keep commands.items
private. No library/SDK/World algorithm, network, menu, runtime, schema or dependency
change. No persistent player projection, new Game field or general transaction bus.

Tests: new integrated/transition_ownership_tests/slot_player_tests.rs and one gated
declaration in its existing parent. Reuse the private Fixture::game resource loader
and explicit Options literal. Do not change the four qualified producer tests,
six receiver tests or their helper logic. New local test helpers may construct
captured native metadata and World trial setup independently of receiver helpers.

The concrete later consumer is one host-authoritative machine/player interaction
inside a normal player turn, including actual as_guest execution on a server.
This packet supplies its synchronous ownership handoff only. No automatic enabling
callsite, command, packet handling, UI, guest evaluator or ticking is introduced.

## Native address, context and supported values

Propose private Game::transfer_world_slot_inventory_cell(key, index, direction,
requested) -> Result<TransferOutcome, PlayerTransferError>. Outcome distinguishes
native NoMove(reason) and CommittedTransfer; finite errors retain native causes.
Address exactly one existing ordinary inventory cell: index < 36 AND within the
actual inventory vector. Do not use slot_mut, selected-slot aliasing, cursor,
armor/offhand, crafting, station, merchant, container or computed result addresses.

Refuse remote.is_some() or integrated.is_some(), as the receiver does. Require an
actual current player body. An integrated server Game is valid inside as_guest;
execution-role Client alone is not refusal because in-frame play uses that role.
Require the supplied current opaque key and captured configuration scope/dimension
to agree with the active World allocation and Game.dim; never mint observation_scope
while transferring. Let the existing World prepare/commit enforce live binding,
actual resident-column membership, pending/fault inhibition and stale-key refusal.
Knowing this internal key is not permission, reach, player-session or replay proof.

None is an ordinary empty cell. Some(count=0) is unsupported existing data, not
silently normalized to None. Validate both present root stacks as exact registered
non-air IDs and positive counts; reuse the H06c count walk for the root and four
modeled child collections, including i32::MAX representation limits. Keep that
walk's checked reservation behavior and refusal ownership. It is not full arbitrary
NBT/component conformance, a new depth quota or recursive item-ID normalization.

## Reuse the real native limit resolver

commands/items.rs::stack_max_size already implements the component-aware native
rule: Tag::Int minecraft:max_stack_size clamps to 1..99; otherwise use registered
item.max_stack or its existing fallback64. Reuse this exact Game method after the
visibility-only change. Do not duplicate it, reject valid overrides or substitute
stations.rs::max_stack/menu's registry-only helper. Wrong-tag-type fallback remains
the existing resolver's behavior; do not invent new component parsing here.

For this diagnostic plain machine slot and an ordinary player inventory cell, use
the native plain-container ceiling99 (menu.rs::CONTAINER_MAX), then the effective
item limit. No guest supplies limits. For each endpoint, its present stack selects
the item fact; if empty use the transfer source's fact; if no source exists there
is no item-specific limit and the plain-cell ceiling applies. Thus both empty cells
still produce positive ResolvedLimits and the kernel returns EmptySource. Zero or
otherwise unusable resolved native limits refuse explicitly, not silent repair.

Resolve both endpoint limits from the actual borrowed pre-state before lending.
These same facts remain current through prepare and commit: exclusive field borrows,
no callback/tick/IO and no external plan escape prevent any intervening change.
Pass current host-resolved limits to commit, never the proposal's copied limits.
Existing over-limit positive counts are not clamped or deleted: kernel room uses
saturating capacity and can move out; incoming movement cannot exceed its native
destination limit. A source incompatible with an occupied destination is NoMove.

## Concrete ownership sequence

Complete authority/address/context and supported-data/limit checks first. Split
exclusive borrows of World and the selected inventory cell. A small concrete guard
owns the cell reference and a short-lived SlotProjection containing cell.take().
Its Drop restores projection.into_stack() into that same cell on every Result
branch and during ordinary Rust unwinding. Do not clone the original into a second
authoritative owner, reconstruct it from ID/count, or use unsafe restoration.
This is the local cell guard's unwinding contract only: the pre-existing as_guest
wrapper does not itself restore its broader player swap during panic. This packet
does not change that wrapper or claim a complete panic-safe guest turn.

Run World::prepare_world_slot_transfer against that guarded projection. Return
NoMove for supported zero requests, empty sources, incompatible stacks or no room,
with neither owner changed. Existing invalid data/context errors take precedence
over NoMove. Consume a prepared plan immediately in commit_world_slot_transfer;
no plan, projection or temporary cell identity survives this synchronous call.
Every error restores the original cell and leaves the machine unchanged under
the existing kernel/World checks. After success, guard restoration publishes the
committed player value without another fallible operation, callback or IO.

Projection revision zero is fresh for each local lend. Its opaque identity and
machine checks establish this exclusive operation's freshness only: this is not
a global player revision ledger, ABA detection across calls or request deduplication.
Do not add general intent tokens or claim H07 replay qualification in this packet.

## Actual-Game acceptance

Ignored tests require pinned HORIZON_TEST_RESOURCES with expect, isolated owned
cwd/HOME/data/config/cache/temp and session unset; never mutate process globals.
Test actual Game method and actual World trial/transfer APIs, not mock closures.
Cover both transfer directions, partial/exact moves, full destination, incompatible
components, empty source, zero request, over-limit source without normalization,
and damage/opaque components/top-level plus nested BundleSelected raw integers.
Cover ordinary native registry limits and valid Int overrides below/above bounds,
including stack-limit clamping and delegated wrong-tag fallback. Refuse zero-count,
unknown/air root IDs and unrepresentable root/nested counts without either owner
changing. Exercise stale/replaced keys, physical unload/reload, invalid indices,
remote refusal and no-player refusal. Existing receiver/kernel regressions remain.

For a real guest test, call existing Game::lan_host with NetConfig listen=false,
announce=false and online=None; no TCP listener, discovery or authentication. Take
the resulting LanHost, add ordinary channel-backed Guest::new values to its public
hub.guests and mark fixed test guests placed in the source dimension. Invoke actual
Game::as_guest for one connection, then this adapter. Assert that guest's real cell
changes, main Game inventory is restored, another guest is unchanged, and refusal
restores the same raw cell. This uses existing construction/swap APIs; do not widen
LanHost private fields or alter production guest admission. If faithful setup needs
broader changes, stop and report the concrete need before expanding source scope.

Coordinator owns pinned formatting, feature-on/off native/resource tests and lint,
both-mode inherited Game regressions, independent source review and final evidence.
All active/parked World-loss receivers, unloaded refusal retention, shutdown/save
error ownership, player ingress validation and guest/replay tests remain gates
before actual enablement. Coherent H08 checkpoints and S1 acceptance remain separate.
No owner decision is needed for this bounded dormant adapter; do not weaken those
gates or add a recovery/storage architecture as an incidental implementation step.
