# Game streaming owner: held source assignment

Status: HELD. No engine source or execution authority. Fresh independent Sol High
DESIGN acceptance of this exact assignment/proposal, a synced private design
checkpoint and exact coordinator release gate come first.

Proposed parent is accepted `e72475864c13720fe066d5bd6335d652ce5cd2bf`, tree
`4a5567d16d154f8b720a8d8c6d82bf69a28f2ca5`. Private results checkpoint
`0d5b1859c3739016bcf9064b94eea8b1ee075965` is synced. Predecessor FINAL receipt
`ce3dae78`, accepted 163-entry index `30ae3427`, public sync `8c5315f4` and private
sync `285bfed4` remain immutable. Earlier historical inputs remain unchanged.

Read S1_GAME_STREAMING_OWNERSHIP_MIGRATION_PROPOSAL.md. For fresh DESIGN review,
the coordinator selects its recommended **factory-invariant minimum**: a private,
non-Clone Existing/RetainedRemote owner enum, checked existing borrow/extraction/
replacement/swap, and mechanical adoption by current native and legacy Game
factories/callers. Existing includes legacy remote pools and is never native
provenance. Boxing moves the original owner; it cannot reconstruct its pool.

All production Game factories/replacements must remain Existing. No production
RetainedRemote Game installer, mode, command, environment route or arbitrary
Game/native-owner setter. Known current factory invariants may be asserted
explicitly at field-local accessors. These are not supported remote dispatch or
refusal promises. No Deref/AsRef, placeholder pool, wrapper inner escape, native
writer witness or default-value facade. Before any later remote Game activation,
its reachable assertions must become honest checked dispatch/refusal before
side effects and owned-input loss. The proposal's larger full-guard alternative
and indirect signature changes are not part of this assignment.

Maximum engine whitelist: exactly the 38 existing paths in the proposal's table
and new `crates/horizon-client/src/game/streaming_owner.rs`, also listed in
`.local/s1go1-proposed-paths.json`. Only genuinely affected paths need edits.
Preserve raw PreparedWorld/PrepareOwner/Parked streamers. Check the active variant
before multi-owner level swaps; return original incoming/slot owners on refusal.
Successful replacement leaves the old owner with its caller and preserves its
actual disposal boundary. Preserve all native authority/save/session predicates,
side effects, update order and legacy public behavior. No streaming.rs, net.rs,
world_loading.rs, integrated.rs, dependency, lock, launcher or reference edits.
Stop for exact scope review if another path or behavioral change is necessary.

Proposed logical branch `codex/s1-game-streaming-owner`, prefix `.local/s1go1`,
same stable s1-native-close-composition checkout and its own warm target-s1nc1.
No physical worktree/target copy or target sharing. No remote Game activation,
Session/decoder adoption, storage representation change, mesh configuration,
packet backpressure, scheduling, renderer/window or scoped server composition.
Those remain real client/server work, not outcomes of this representation.

After source release, authors use native TEXT only: no scripts/imports,
formatter/compiler/tests/game/WSL/Git writes/network/escalation/delegation.
Root owns all serial execution and Git writes. Reviewers use native procedural
READONLY under inherited workspace-write, not profile-enforced. No standing team
or concurrent compiler/test/source mutation/Git writes. Preserve every candidate
path and all prefix artifacts before corrections; never alter historic indices.

Use the existing Game-bin Fixture and pinned actual context described in the
proposal; do not rely on lib testcfg modules or add identity getters. Meaningful
new checked-owner scenarios cover original owner movement and unchanged refused
owners. Reuse the named cold native factory/authority/checked-close regression;
add the specific parked-save case only if actual swap/disposal changes justify
it. Appropriate format, paired all-target lint/compile, feature-off and public
tree checks remain. Do not replay whole Game/window/network/save/Java/fatal
batteries without a changed dependency or concrete concern. Counts are estimates.
Missing fixtures/watchdog/assertion failures FAIL; record real artifact/profile/
features/binary/resource bindings. No performance or remote-runtime claim.

Fresh exact SOURCE/tool review precedes formatting/compiler/runtime; renew the
binding after formatting. Archive mutable source/config/recipe/instruction inputs,
compare qualified live/archive/Git bytes, and distinguish unchanged baseline CRLF
canonical comparisons. DIFFERENT FINAL/privacy acceptance precedes use or reviewed
public branch sync. Every private push needs fresh PRIVATE/exact-origin checks;
public pushes need fresh PUBLIC/exact-origin and normal hooks. No PR/merge/tag/
release, default activation, G1 or live S2 follows. Stop this bounded source
assignment at its FINAL/checkpoints; continue the roadmap through separately
reviewed work, not by silently widening this packet.
