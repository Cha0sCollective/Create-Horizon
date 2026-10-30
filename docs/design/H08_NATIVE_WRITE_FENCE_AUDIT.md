# H08 next producer-to-writer prerequisite audit

Planning evidence only; NO implementation release or capture admission. One finite
native read-only audit during bootstrap qualification inspected unchanged sources
below. Current bootstrap candidate is not yet accepted. A future design needs fresh
independent review and an accepted clean base/private checkpoint before release.

`streaming.rs:4105` require_pristine_fixture_capture permanently refuses historical
order!=0 as PreviouslySubmitted. `game/levels/capture_integration.rs:1745` applies
it to every active/parked participant before native saving. Settling a used streamer
does not reopen that gate. Preserve that refusal in the next experiment.

The native-work book at streaming.rs192-212/951 observes queued, worker-held,
returned, reconciling, disposed and unresolved work plus retained faults. It
explicitly excludes storage-writer ownership. Accepted storage handoffs do not
prove completed writes. Existing save reconciliation1918-2009 handles supported
Decorated/Generated/Saved results and defers others; save_in_flight omits ordinary
Generate/Light/Mesh. Normal update1371 schedules future work as well as reconciling,
so repeatedly updating until a zero observation is not a closed producer boundary.

Persistent callbacks save.rs3951-3978 enqueue proto/finished region/POI/entity
snapshots. A finished handoff can partially enqueue before entity failure. Pending
snapshots are inserted before submission2104-2137; ordinary write errors retain
payloads and sticky errors2187-2206. Successful matching sequences retire pending
snapshots, and successful region writes sync1668-1709. flush2212 waits without a
deadline; writes_finished2231 polls completion and preserves errors. Writer panic/
exit can bypass the normal completion tail; conservative bounded polling must
fail at a deadline rather than infer success. Counts are not a cross-directory epoch.

Actual integrated Task closures run serially before later Tasks/ticks
(`game/integrated.rs:589-600`) and provide a useful exclusive Game ownership seam.
Pause/hold alone still services Tasks/connections419-430. Integrated::call's ten-
second observer timeout95-105 neither cancels nor proves operation completion.

Smallest sensible next investigation: a no-rescheduling native write-producer
fence plus deadline-bound polling of every participating SaveDir, proven inside
an actual used-streamer Game Task while capture STILL refuses. Likely four paths:
streaming.rs, save.rs, declaration-only game/levels/slot_epoch_integration.rs and
one new Game child. This is an audit suggestion, not an accepted path release.
Keep queued/worker/returned/late callback handoffs, unrelated deferred results,
partial enqueue, I/O errors, timeout/disconnection/panic and their payload owners
observable. Tests need deterministic native handshakes, real Task sequencing,
preserved owners/source/archive and unchanged capture refusal.

For inspected built-ins, ordinary generation may be read/in-memory-only, but that
must be independently established together with absence of later write-producing
owner reconciliation during any eventual copy. Arbitrary ChunkGenerator trait
implementations provide no such premise. Current work zero and writer batches are
not readiness. Do not add a general plug-in capability or global barrier by inference.

This does not solve post-tick inhabited/native metadata, E1/W1, all participant/
scalar/history limits, mutation after Task return, ordinary capture/recovery or G1.
No owner input is presently required for a bounded design investigation. No new
source or raw artifact was written by the auditor; root records this factual audit.

Root verified unchanged SHA256 pins during bootstrap qualification:

- streaming.rs 5827ecc86bbe87dfef12f911f404d62756379ade335a8c7cfec85d66625ae032
- save.rs 688ae37142aca312fec253d815fb4af8dfb014aad5d808d4b23b08f6aa6abe04
- game/levels/capture_integration.rs d7d294db74943bca9cc7c88493f5549cd87c574d53126493ac00aadfade1f4dc
- game/integrated.rs 80f616dabdb82639573c8dbba56b55e756f97bfa6ffd21cf17c2f608614a4631
- game/levels/used_streamer_tick_tests.rs e4e198af1b10cfff89c544f179d143d88436745d0b7b1729fce0ab1fe9dcde6e
