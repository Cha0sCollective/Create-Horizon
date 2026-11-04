# Existing Create port evaluation — November 4, 2026

Status: evaluation complete; owner architecture/acceptance decision pending.
Linux build, integrated gameplay components and dedicated two-client trial verified.
This is an
evaluation under the owner's explicit selection, not an architecture adoption,
migration, changed acceptance requirement, or S1/G1 pass.

## Exact inputs

- Candidate: [Delete228/create-horizon](https://github.com/Delete228/create-horizon),
  detached `16c5d454b1b9860da01b78474ea5206f0664d384`, tree
  `7ce954394753d1a6c490157660ec6eb7a6c0cb8e`, version 0.5.0.
- Candidate Horizon: official `fc02103818c212920fd24aa34a2b2f91ba49e56f`;
  Mixins `a1e13326306c6bc074b11e6a4926deb551c4083e`; native API 359.
- Rust 1.99.1 is already installed. The build uses a separate evaluation target;
  existing warm targets and development candidates remain preserved.
- Fetched candidate main `10cb93e6a13b837edc2293d3669ee6cbdb4d5863` differs from
  the tested source only in five documentation files; gameplay/build source is
  the same. This is a comparison of the fetched snapshot, not an automatic update.
- Candidate Create reference: 6.0.10 `ac0c444d9828da3453ae8cc65338e8de063286fb`.
  Our accepted reference remains 6.0.11 `7ff2760e` (full pin in workspace.toml).
- Original resource JAR: Modrinth `UjX6dr61`, 19,123,767 bytes, SHA-256
  `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
  Downloaded and verified against the published SHA-512 and candidate SHA-256.
  Used only as a private resource input; no Java execution or redistribution.
- Disposable evaluation root: `.local/evaluations/create-horizon-05/` outside
  this private repository. No user instances, saves, Prism or Java settings change.

## Source-confirmed reuse

Two bounded independent read-only reviews inspected gameplay and networking at
the exact candidate and Horizon pins. They did not execute tests or change code.

All five desired types have substantive implementations: creative motor, shaft,
small cog, large cog and millstone. Native adapters construct and tick their block
entities on the server, save machine fields, expose inventory slots, send update
tags and draw animated models. The portable graph implements shaft 1:1, small-cog
reversal, large/small 2:1 ratios and perpendicular large gears. Cobblestone milling
consumes one input and produces one gravel with duration 250 processing units.

Saved fields include motor ScrollValue, millstone Timer, InputInventory,
OutputInventory and Horizon's generic Items. A bridge snapshot reconciles original
inventory compounds with ordinary hopper edits. The authored smoke driver really
quits and starts a new process, but its maintainer-reported results have not been
independently reproduced here. Its documented cold-reopen scenario uses sandstone,
not our cobblestone case. No local runtime claim follows from this source review.

The native dedicated server uses the same Game hooks, so this is more than a
client-only demo. However, VALIDATION.md explicitly excludes multi-client evidence;
the supplied smoke driver launches one client. At the initial source review,
dedicated startup, two matching rendered clients, reconnect and chunk retracking
needed local execution. Our later trial below verifies startup and peer reconnect
with server-query accounting; chunk retracking and independent inventory NBT
replica/resync inspection remain unverified.

## Confirmed differences and unresolved requirements

1. **Full-output behavior differs from our accepted reference.** Both portable
   `engine/src/core/milling.rs` and native `millstone.rs` stop progress if any
   output stack has 64 items. Thus one full slot blocks eight free slots, and nine
   full slots preserve input. Our selected Java observations produce into another
   slot in the first case and consume input/lose output in the second. The port's
   oracle substitutes a handler limit of 64 and its native check explicitly expects
   this pause; its reported comparisons therefore do not resolve the discrepancy.
   Later discarded insertion remainders do not bypass the earlier guard.
2. The bounded 6.0.10-to-6.0.11 source comparison found unchanged selected motor,
   shaft, cog, graph/network, tick/process and cobblestone recipe logic. The one
   millstone change clears outputInv in clearContent. Those differences do not
   explain the output-limit mismatch. This is not a full version-equivalence claim.
3. At candidate Horizon fc021, `--in-frame` and `--no-server-thread` are removed
   and ignored. Singleplayer runs its integrated server thread. Passing an old
   flag cannot satisfy our old two-mode requirement. Owner decision is required
   before changing that acceptance/profile.
4. Native compiled code replaces the previous Wasm architecture. Native callbacks
   do not establish Wasm trap/fuel/output rollback or sandbox isolation. These
   guarantees cannot be silently transferred to the new route.
5. Stock saves write machine/world and player owners sequentially. Dedicated
   panic handling can save world state while preserving an earlier player file.
   Neither the port's collection path nor ordinary restart establishes our
   interrupted cross-owner checkpoint requirement. Useful stock transfer helpers
   do not automatically apply to the port's direct take/give path.
6. Motor commands validate player presence, mode, dimension, range, target and RPM,
   but have no instance generation, expected prior value, session epoch or nonce.
   Delayed replacement and concurrent-player cases remain qualification gaps.
7. Update tags supply initial and changed machine state. A local ModRevision is
   render-cache identity, not a saved/network sequencing protocol. The later trial
   verifies ordinary peer reconnect and authoritative server state, without direct
   client inventory NBT inspection. Chunk retracking and replica/resync correctness
   remain unverified. Animation derives angle from current client time and RPM
   without a retained server phase anchor.
8. Graph size is bounded to 4096 loaded nodes. Conflict/overspeed/overstress stop
   components rather than reproducing Java breakage. Chunk boundaries and large
   networks are outside a five-block playability trial.

These are source findings or missing evidence, not claims of reproduced runtime
failures. The known full-output rule is directly encoded in source and its tests.

## Local build and integrated results

The unchanged candidate applied successfully with the pinned published Mixins:
34 injections, 14 modules, two crates and one data entry. Linux game plus matching
control client built successfully in 4m22s with Rust 1.99.1, four jobs, no default
features and the documented fast-profile game optimization override. No port or
engine source patch was needed. The local target symlink is an untracked build
artifact; tracked candidate source remains unchanged.

Game SHA-256: `95f54040a5a9a3ae13ee8d8d47b41fe91f2b210585671b886c3524969f3a8ae5`.
Control SHA-256: `40022a2fbd3aa26f01d4c0890dd7c01a7e3bb3f7421e4120252a396d5961a691`.
The imported original resource count was 324; loaded tables contained 35,852 block
states and 1,697 items. Prepared 26.3 resources came from the existing private
reference preparation, copied into disposable run folders.

Integrated run `integrated-d1mogp5_` verified these components:

- Ordinary held-item placement and actual right-click interaction for all five
  block types. The subsequent powered layout was built by diagnostic commands;
  do not describe the whole chain as ordinary player construction.
- Motor 64 → shaft 64 → small cog 64 → large cog -32 → small cog 64 → millstone -64
  RPM; motor reversal changes the millstone to +64.
- A diagnostically summoned falling stack of two cobblestone was captured and
  processed into two gravel, then collected into the player's inventory through
  actual empty-hand interaction. Player Q/drop ingress was not tested.
- Stopped progress Timer=208, three cobblestone inputs, motor setting 32 and the
  player's earlier two gravel survived closing the first process and launching a
  second one. Restoring the shaft completed exactly three more gravel. This is a
  real cold-process reopen, not an in-process reload.
- The generated 960x600 screenshot was visually inspected: original Create models
  render in the actual world. A single image does not prove phase synchronization.

The overall run remains FALSE: after those checks, its logger attempted to JSON-
serialize a Path for the successful screenshot command and raised TypeError. Its
raw result records the passed components, the error and clean exits (first process
exit 0, second cleanup exit 0). Full-output control was not reached in this run.
Do not relabel this raw run PASS or replay the qualified components solely for a
reporting error. The image and result are copied to the local evaluation evidence
folder; original raw files/worlds remain in the private Linux run directory.

An earlier run `integrated-o9ekpcd4` remains FALSE: the harness initially named a
nonexistent Vulkan ICD file. Correcting it to installed `lvp_icd.json` was a test
environment fix; it did not change game/mod code. This first attempt loaded the
mod and imported its resources before failing graphics startup.

Dedicated attempts are separately retained. `dedicated-qk2j6o0q` loaded Create and
started the server but admission failed under Horizon's default enabled whitelist.
`dedicated-jmixgmo5` admitted both rendered clients, then stopped on an obsolete
`tab` diagnostic command. Neither qualifies gameplay. The third driver preserves
the whitelist with exactly two test offline UUIDs and uses supported commands.
All are localhost-only, with separate game/config folders and owned-process cleanup.
No playable server's access settings changed.

The final trial `dedicated-11uw45uu` **passed** using frozen driver SHA-256
`3821c761b01deb7a2c7d254a294676539ca8f0bbbc0dc9ae89baa354a630a533`.
Its raw result SHA-256 is
`21496ce2efb88c516c0eb6ca6c514eba8ec40894ca75577dfc1652688ca991fa`.

- Server logs name both actual players, report two of two online, and record the
  peer leaving/rejoining. Two server processes and four fresh rendered client
  processes were used across the cold restart. All six exited cleanly with code 0.
- An actual held-item millstone placement reached the dedicated server and appeared
  on both clients. The remaining oriented chain used explicit server commands.
- Two captured cobblestone became two gravel, collected by the acting player.
  Server inventory queries through both client connections agreed on the count;
  the player's own inventory then held both gravel.
- The peer disconnected/rejoined while the drive was removed. The server retained
  three inputs and Timer=212. Closing both clients and the dedicated process, then
  starting three new processes, preserved that state, motor setting 32, actor's two
  gravel and peer's zero gravel. Restoring the shaft produced exactly three gravel.
- Both actual client screenshots were visually inspected. The original millstone
  is visible in each; the actor image is partly obscured by the other player's
  avatar, so it is not a complete unobstructed view of every component.
- Submitted millstone-inner matrices independently show animation on each client.
  Actor: 779 submitted poses, 557 distinct matrices; peer: 213 poses, 206 distinct
  matrices. Recent normalized Y-axis matrix deltas give median approximately -32
  RPM for both (actor -31.99998, peer -32.00037). This is observed native render
  motion, not a Java phase oracle, fixed-GPU-buffer claim or anchored phase-sync pass.
- The injected nine-full-slot control retained one cobblestone and 576 gravel on
  the authoritative server. This **reproduces the reference mismatch**: the pinned
  Java case consumes the cobblestone. It is not a Java parity pass.

Images, copied small results and pose summary are under the local evaluation's
`evidence/`; complete world/command/render logs remain preserved in its Linux run
directory. No crash injection, chunk untrack/retrack or concurrent collection was
run. The ordinary restart/reconnect result does not establish those guarantees.

The network trial was reviewed independently, including two important fixes:
remote command replies are asynchronous, and the generic client container command
does not expose the millstone's received original inventory NBT. The driver now
correlates server feedback between unique ordered markers and queries the original
InputInventory/OutputInventory paths. These counts are explicitly authoritative
server queries through each connection, not independent client replica inspection.
Client block/player state, images and submitted render poses are separate evidence.

## Proposed next plan — owner decision required

The candidate already supplies the initial gameplay subset and upstream supplies
its host services. Prefer reusing those implementations over continuing the old
Wasm host framework. This remains a recommendation, not an adopted replacement:

1. Establish a private native-adoption branch using the tested official Horizon
   and existing port pins, preserving MIT notices and source provenance. Keep the
   present custom branches and accepted reference fixtures intact. Do not attempt
   to carry all 103 custom commits across the removed Wasm architecture by rebase.
2. Define the native milestone around ordinary playability: mod activation,
   server-authoritative machine behavior, integrated singleplayer, dedicated/two
   clients and normal save/reopen in the existing fixed mob-free Creative profile.
   Open to LAN stays deferred. Explicitly replace the obsolete in-frame and Wasm
   requirements on this route; original G1 history remains accurate.
3. Keep the owner's selected 6.0.11 observations, including full-output item loss.
   Make only the bounded parity corrections required by that reference, starting
   with the full-output guard and the corresponding misleading oracle assumption.
   The candidate's 6.0.10 pin and clearContent difference need an explicit small
   reconciliation; do not infer full-version equivalence.
4. Defer the general cross-owner crash checkpoint framework and extra hardened
   stale/replay guarantees from the playability milestone, if the owner accepts
   native code's trust/failure model. Ordinary restart evidence cannot satisfy
   those stronger requirements. If they remain required, estimate them as separate
   host work after checking current upstream, rather than hiding their cost in
   basic mod integration.
5. Reuse the existing kinetic core and native bindings in later stages. Validate
   actual changed parity cases and game milestones; no historical full battery.
   Before any further host assignment, compare current upstream and the port's
   current branch for an existing solution. The operating-rule adoption still
   needs its required fresh read-only review before changing instructions.

Do not start this migration, waive old requirements or declare S1 complete from
this recommendation. Present the final local results and obtain the owner's
architecture/acceptance decision first.

## Bounded local trial

Use the candidate's unchanged native build route with the exact published Mixins
revision. Keep all generated material local and all worlds disposable. Do not run
the full oracle battery. Check the five block types, a deterministic cobblestone
chain, source speed/ratios, stopping, retained progress and an actual cold reopen.
Then check one dedicated server with two matching rendered clients, observation,
collection and a reconnect. Fixtures and diagnostic commands must be identified
separately from ordinary player placement/collection and actual rendered evidence.

Do not add fault infrastructure or fix gameplay discrepancies merely to improve
the trial result. Report environmental/compatibility blockers and partial results
honestly. Decide adoption and the revised acceptance contract with the owner after
the assessment. Preserve the current custom S1 candidate and original FALSE/HOLD
records; its five dirty files and source-review import HOLD are untouched.

## Sources

- [Candidate source](https://github.com/Delete228/create-horizon/tree/16c5d454b1b9860da01b78474ea5206f0664d384)
- [Candidate pins](https://github.com/Delete228/create-horizon/blob/16c5d454b1b9860da01b78474ea5206f0664d384/compatibility.json)
- [Milling guard](https://github.com/Delete228/create-horizon/blob/16c5d454b1b9860da01b78474ea5206f0664d384/mixins/create/millstone.rs#L327)
- [Reported validation](https://github.com/Delete228/create-horizon/blob/main/VALIDATION.md)
- [Official native mod APIs](https://github.com/horizon-rust/horizon/blob/fc02103818c212920fd24aa34a2b2f91ba49e56f/docs/native-mods.md)
- Local review: create_port_gameplay_audit and create_port_network_audit;
  independent native source reads only, candidate checkout clean after review.
