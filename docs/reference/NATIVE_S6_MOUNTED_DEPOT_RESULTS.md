# Native S6 mounted depot — PASS within the declared scope

This implements the independently reviewed
`docs/design/NATIVE_S6_MOUNTED_DEPOT_PACKET.md`: one plain iron ingot on one
depot, moved one cell east by the accepted one-pole mechanical piston. Fresh
independent read-only Sol High completion review is CLEAR. S6 as a whole remains
OPEN.

## Source and checks

Private branch `codex/native-s6-mounted-depot` starts from the accepted pressing
checkpoint `f6e3d74ad7583f2828da6dd213419f195a2facd5`; audit and packet are in
`edaa8b6`. The pin remains Horizon `fc021038`, Mixins `a1e13326`, native API359,
Rust1.99.1 and Minecraft26.3. The port/base art remains6.0.10; the selected
Java mounted-depot reference is6.0.11 `7ff2760e`. No engine checkout/API changed.

The private piston owner now distinguishes the old stone cargo from a depot
and holds one validated `minecraft:iron_ingot` while its depot is absent. It
refuses occupied processing output, unsupported item/NBT payloads and waterlogged
depots. Only depot-to-proxy capture suppresses Horizon's ordinary container
removal spill. Restoration validates the empty target before item installation,
syncs the real depot, verifies its item, then releases the piston claim. Retry
retains the claim when installation cannot succeed. The existing proxy renderer
draws the moving block; one private return hook draws its owner-held item.

Independent read-only Sol High source review initially held three concrete
issues. The corrected capture flags, restoration precheck/token ordering and
single model projection received delta CLEAR. Three focused Rust tests pass:
old/new cargo flags and item codec, malformed/occupied depot refusal, and empty
known restoration-target validation. Pinned Clippy, rustfmt and diff checks pass;
Clippy reports26 nonfatal warnings. Final locked fast build passes; its exit0
and compiler summary are coordinator-recorded in `final-build-receipt.json`.
The independent reviewer also verified the final binary's live session and
focused check receipts. No historical full battery was repeated.

The primary integrated/dedicated game observations used source map SHA-256
`bdf6761706845f0ac2474f4789055840651c4b7afb2e532536b15ccfe529bdb6`,
game `20e88d08aadd030be549eefe4a5571544effb546da6503eec020d5f9a4bba57d`
and ctl `507b23e1a8f68a3f11da9c677fd2ec2bf52f93e73d7ebfe2affcc9f7a7863500`.
That map is preserved as `inputs-before-source-tests.json`. Only `s5_piston.rs`
changed afterward, to add the three tests and `Cargo`'s `Debug` derive. The final
rebuild and focused integrated retry/positive run use map
`8db54c33f380fb60b0bc9f2f9cf1be7594e7c31573b439ad92f17082759bf037`,
game `7d1d06e653d68c4680121be285de3d49213f5190bf5de77ed85ec699a3226b78`
and ctl `0ebaf05d83facbc363ce9b7476b2db80b9e74ac48889c72743895c2e2e977ca1`.
Launchers verify source/resource and binary hashes and isolate every game home.

## Dedicated Survival and active-motion cold reopen

Disposable normal seed-5 world `dedicated-tx48h8wy` used an isolated loopback
server and two matching rendered clients. Motor/shaft, piston/pole/depot and
the plain ingot were fixtures in this mode. Mob spawning was disabled and the
isolated plot cleared. At piston Speed-64RPM, stress256SU/capacity1,048,576SU,
the owner reported `Phase=1`, `CapturedDepot=1`, one `DepotItem` and a fractional
offset. At extension it reported `Phase=2`, no captured cargo, and the depot at
`8,65,5` held exactly one ingot. No item entity appeared. Nonsticky return
restored the pole and left the depot/ingot at the destination.

A depot with one iron sheet in output slot1 was refused as `assembly-refused`;
the original depot, its held ingot and its output sheet remained intact, with
no item drop. Both clients reported the moving block proxies at slow power.
The actual peer screenshot `peer-mounted-depot-moving.png` shows the moving
depot and ingot; the aimed settled screenshot shows the restored depot item.
An earlier camera attempt pointed at trees and remains unqualified.

Both clients quit during a new slow extension. The final live owner query had
`Phase=1`, `CapturedDepot=1`, one ingot and `Offset=0.009765625`. The server
then stopped normally. All three processes exited0. Before another autosave,
the saved region was copied into `cold-active-r.0.0.mca`; read-only decoding
found the owner saved at `Offset=0.021484375`, `Phase=1`, `CapturedDepot=1`,
`CapturedPole=1`, `CapturedStone=0`, one `DepotItem` and one iron-ingot ID.
This establishes that the normal saved state was captured motion, rather
than inferring it from an earlier live query.

Cold reopening the same server/world and both clients allowed that saved
motion to finish. The restored depot held exactly one ingot, with no dropped
item. The peer's ordinary depot interaction collected exactly one ingot; its
inventory held1, the actor held0 and the depot was empty. The reopened server
and both clients also exited0. This is ordinary clean restart evidence.

## Integrated Survival and focused regression

Disposable normal seed-5 world `integrated-begi2dhw` used fixture motor/shaft
and supplied items, but ordinary player interaction placed the piston, pole
and depot and inserted the one ingot. A static stone at the destination caused
assembly refusal without changing the source depot/item. Initial mob cleanup
left91 unrelated item entities; the first no-drop probe therefore did not pass.
Those test entities were cleared before the successful extension and all
subsequent item accounting. The original failed probe is preserved.

At Speed-64RPM the depot extended to `8,65,5` with exactly one ingot and no
drop. Normal return restored the pole and left the depot in place. Ordinary
interaction emptied the depot and gave the actor exactly one ingot. A focused
old-S5 stone extension/return in the same world still placed stone one cell
east and left it there on nonsticky return. The process exited0.

The final-build integrated reopen then constructed two specific intermediate
states with native commands; these are synthetic recovery checks, not claims
that a real host write failed. First, a depot-to-proxy partial capture with a
pending owner restored the depot/one ingot, reported `capture-rolled-back`,
cleared `DepotItem`/captured flags and produced no drop. Second, an empty depot
with a foreign NBT field blocked installation: the depot stayed empty and the
owner retained exactly one ingot with `capture-restore-blocked`. Removing only
that field allowed retry to install exactly one ingot, clear the owner claim
and produce no drop. The preceding test ingot was explicitly removed through
fixture commands before the second case.

The final build also performed a normal powered extension and nonsticky return
from the recovered depot. After clearing the prior player fixture inventory,
ordinary collection gave the actor exactly one ingot, left the depot empty and
produced no item entity. This second integrated process exited0. These focused
game checks exercise the actual partial-capture/restoration paths in addition
to the pure source tests; they do not inject engine failures or prove crash
atomicity.

## Evidence and limits

Raw source maps, commands, failed attempts, screenshots, saved-region inspection,
build/check receipts and session/exit logs remain outside Git in
`.local/native-s6-mounted-01` and Linux packet-owned runs
`s6-mounted-runs/dedicated-tx48h8wy` and
`s6-mounted-integrated-runs/integrated-begi2dhw`. All eight game processes across
these sessions exited0. An initial hash-freeze mismatch did not start a game;
its prebuild map is preserved. Shared immutable prepared resources are linked
into disposable runs rather than copied. No playable save or launcher changed.

This qualifies only the packet's one-cell, one-pole, one-plain-ingot depot
profile. S5's unchanged collision/stone evidence is reused with the focused
obstruction and stone regression above. Portable interfaces, moving transfer,
arbitrary inventories/items, output-buffer mounting, riders, trains, broad
contraptions and interrupted-crash consistency remain unqualified. The next
S6 packet must define its exact interface assembly, transfer and custody
boundary before implementation.
