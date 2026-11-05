# Native S6 portable item interface — PASS within the declared scope

Candidate completion of `docs/design/NATIVE_S6_PORTABLE_ITEM_PACKET.md`: one
east-facing one-pole piston carries one depot and one portable storage interface
one cell, with one plain iron ingot. An opposing stationary interface exposes
the mounted item to one vanilla hopper/chest. Fresh independent read-only Sol High
completion review is CLEAR; this does not complete S6 as a whole.

## Source and validation

Private branch `codex/native-s6-portable-interface` starts at accepted mounted
depot `a77af67`; the audit/packet is `f80fca0`, and source checkpoint is
`6376c3adc4630f1695ad4a10655f3f8de1b5f45c`. The fresh upstream audit found no
existing moving-interface implementation to reuse. Host pins remain official
Horizon `fc021038`, Mixins `a1e13326`, native API359, Rust1.99.1 and Minecraft26.3.
The selected Java reference remains Create6.0.11 `7ff2760e`; imported base/local
art remains6.0.10. Native/content manifests agree on `0.5.0-ch4os.4`.

The piston holds the new profile's sole canonical item in slot0. The stationary
interface has no inventory. A private mutable source preflight validates the
live reciprocal owner, geometry, power and timer, creates a stack-local view of
that slot, and delegates extraction to Horizon's stock `suck_from_slots` path.
It introduces no engine API or global container alias. A wire-only occupied/empty
snapshot derives from the slot on every owner update and is not saved as a
second payload. Existing stone and single-depot formats remain readable.

Independent read-only Sol High source/delta reviews cleared capture/restoration,
the private adapter, client snapshot reading, rendering axes and the exact
three-box/four-final-bound clearance admission. Four focused core tests cover
admission, timeout/reset/departure and malformed/reversed motion; four host tests
cover retained custody/installation checks and explicit wire clearing. All pass.
Formatting, manifest consistency, diff checks and final Clippy pass (27 nonfatal
warnings). Final locked build passes. The earlier focused tests precede the
reviewed trace/axes/clearance deltas; the latter received Clippy/build and the
actual movement/refusal/regression checks below. No historical full battery ran.

Final input map SHA-256 is
`74a642c6ec94818e1c3b6d6003472884d1c9f3a947571d5e2ccca686db58a847`;
game is `45e083365649a25adbbf35901245095953401dc32c1984f2cc8c7f90e804f7d3`,
ctl is `507b23e1a8f68a3f11da9c677fd2ec2bf52f93e73d7ebfe2affcc9f7a7863500`.
Every launcher verifies source, prepared-resource and binary hashes. No source
changed during these final-game sessions.

## Integrated Survival

Disposable seed5 normal mob-free world `integrated-x_gx74cb` used fixture terrain,
motor/shaft and supplied items. The player ordinarily placed the piston, pole,
depot, both interfaces, hopper and chest, and inserted/collected the ingot.
The initial piston orientation was corrected by ordinary pickup/placement.
Static stone at the front refused assembly without consuming the source item.

At piston Speed-64RPM, stress256SU/capacity1,048,576SU, the final build captured
three moving parts, connected/stalled at offset0.125, and transferred exactly one
ingot through the vanilla hopper to the chest. Owner `PortableTransferred=1`
and an empty slot distinguished extraction from missing custody. Timer expiry
released the stall; phase2/offset1 restored an empty depot and east-facing
interface. Normal nonsticky return restored the pole and left both cargo blocks
forward. Ordinary chest interaction collected exactly one ingot. Actual client
render traces changed from occupied to explicit empty, and the connection
screenshot shows the native parts. No item entity appeared.

With all five hopper slots and all27 chest slots full of incompatible64-count
cobblestone stacks, the owner retained ingot1/Transferred0 while all2048 fillers
stayed unchanged. Stopping restored ingot1 to the depot. A redstone-powered
stationary interface exposed no connection/inventory; empty destinations stayed
empty and the ingot restored. Ordinary empty-hand collection recovered that one.

One explicitly constructed partial-capture state assigned the sole ingot to the
piston while the original interface/pole remained in the world. A stone in the
depot restoration cell produced `capture-restore-blocked`, retaining owner1.
Removing only that obstacle allowed retry to restore depot1, clear owner custody
and report `capture-rolled-back`, with unchanged interface/pole and no drop. This
is a synthetic intermediate-state check, not an injected engine-write failure or
crash guarantee. A focused stone-only extension/nonsticky return still passed.
Both integrated processes exited0.

## Dedicated, peer and connected cold reopen

The completed dedicated run `dedicated-1ectugww` used an isolated loopback server
and two matching rendered Survival clients. Initial ordinary placement/insertion
passed, and both clients saw the correctly oriented blocks. Full hopper/chest
fixtures retained the one ingot while connected at slow power. A normal shutdown
saved that actual moving owner; both clients and server exited0.

The final connected-save copy was fully NBT-decoded before reopening. Region
SHA-256 `e481eed6864b5582d335c46f511c2b5f5c34942589d44968565eb123e55bcc46`
contains one ingot payload in the piston's slot0, phase1, offset0.14453125,
CapturedDepot/Interface/Pole1, linked/working1, timer4 and Transferred0. Neither
`DepotItem` nor `PortableItemWire` is saved in this owner. The complete isolated
world snapshot is retained, not only a token search.

Cold reopening the same world/server and both clients retained owner ingot1 at
offset0.18945313 with an active link. The peer's renderer observed occupied motion.
Removing the diagnostic filler allowed stock extraction; the owner became empty
and Transferred1, timer55/offset0.40820313 showed the renewed connection stall,
and the chest held exactly one ingot. Both client render logs changed occupied
to explicit empty at world time3836. Timeout/resume, empty depot/interface
settlement and normal nonsticky return passed. The peer opened the actual chest
menu and collected the ingot from slot301; its inventory held1, the actor held0,
and depot/hopper/chest held0 ingots. One diagnostic cobblestone forwarded during
unblocking remained in chest slot0; it was not collected or counted as product.
No dropped item appeared. All three processes exited0.

One additional focused positive run re-added the empty diagnostic hopper after
the already ticking piston, without changing code or supplying another ingot.
It passed the same transfer/empty settle/return/ordinary peer collection. This
covered the other actual tick-list order rather than inferring it from placement:

| Actual source lookup | Owner tick | World time | Result |
| --- | ---: | ---: | --- |
| Hopper before owner, cold-reopened run | 3825 | 3826 | One ingot transferred |
| Owner before hopper, final focused run | 6239 | 6239 | One ingot transferred |

`portable-peer-owner-first-occupied.png` shows the moving depot's ingot and
interface parts; the corresponding empty screenshot shows the ingot removed
and arms extended. The source trace also records empty state in both orders.
All12 processes across this completed run's four sessions exited0.

## Preserved failures and limits

The first integrated build (`6c35f7c5`, pre-cap map
`4b4cb3fc2516bccf5d200f7075c5a29bb105e4712e2f9f95e1ad629ab7c6d6e4`)
transferred an ingot but the old two-box clearance cap rejected three-part
movement. That trial is FALSE for motion. It quit normally; the corrected build
safely restored its retained empty assembly before a successful retest. Original
source reconstruction, map and logs remain. The reviewed fix only raises the
private bounds to the declared three moving boxes/four final bounds.

The first full-destination setup filled the hopper before the chest, allowing
one cobblestone to forward before the chest refill. Its319/320 hopper count is
insufficient evidence; the corrected fill order and verified2048 counts above
are the accepted blocked case. A peripheral depot wrench pickup attempt did
not remove the depot; the verified-empty block was fixture-reset. No depot
wrench behavior is qualified or repaired here.

Earlier dedicated `dedicated-p2dhhvxb` saved connected normally (three exits0),
but its restart recipe sent a command before the control socket existed and
ended without final process-exit receipts. That restart is unqualified. The
replacement recipe waits for both actual sockets and retains the driver on a
command error. An aim at an invisible proxy also failed; it is preserved, and
that clean-reopened ingot was ordinarily collected before reinsertion. Final
captures use a real stationary block or a point target. Wrapper errors are not
gameplay passes. The accepted integrated/dedicated sessions total17 exits0.

Raw maps, checks/builds, exact commands, screenshots, snapshots, failed attempts
and exits remain outside Git in `.local/native-s6-portable-01` and the three
packet-owned Linux runs. `evidence-complete/` is a hash-indexed exact command/exit
mirror with compact sessions and line-numbered log traces for native read-only
review. No game is running. No engine checkout, playable save, launcher or Java
setting changed; nothing was publicly published.

Only the packet's finite one-pole/one-cell/plain-ingot/one-station extraction
profile is qualified by these observations. Survival interface crafting,
arbitrary inventories/items/assemblies, insertion, fluid interfaces, riders,
trains, external inventory-event position parity and interrupted-crash atomicity
remain outside this result. S6 remains OPEN for separately audited families.
