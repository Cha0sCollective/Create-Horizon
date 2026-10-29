# H08 idle-close active-dimension restoration

DESIGN ACCEPTED by a DIFFERENT fresh independent read-only Sol High review of
draft SHA256 f334544c167d436c6c501327ea87bce5beef118bdd62d696f86ac36559388b19.
Implementation remains FROZEN until the coordinator releases the exact fully
qualified parked-save successor. This acceptance is not a runtime result.
Pregen eligibility may remain explicitly source-supported in this bounded packet;
do not add process-global test controls merely to manufacture executed coverage.

Inspected engine HEAD is 3ded3a2f8b7e012eae08dfaf06e6985686e95912, tree
cffabea129c8e95fa74c19f4ac489a7c41fb2ac4. The coordinator is qualifying its parked-save
successor on codex/h08-parked-save-restoration. At inspection, levels.rs contains
that separately assigned save_parked_levels correction and has SHA256
7215b3c0e28f6722ff015d989fc0275208c1692fd800f49311ab1c9029db288a.
close_idle_levels itself is unchanged. Freeze the qualified SUCCESSOR revision
before beginning this packet; do not reset, combine or relabel the active work.
This draft uses inspected native Horizon ownership/save seams, not Create sources.

## Concrete defect and smallest boundary

close_idle_levels selects idle parked levels, enters a selected dimension, saves
it, returns to the original dimension, retires that successfully closed level's
counter and removes its Parked record. A save error currently returns through
question mark before the return transition. The failed dimension remains active,
although the original World/Streaming/SaveDir/entities still exist in Parked.
The hub's current dimension/encoder context also names the failed dimension.

Change ONLY game/levels.rs::close_idle_levels and add a gated declaration for NEW
game/levels/idle_close_tests.rs. This two-file surface can inspect Parked's private
owners in its defining-module child. Preserve every other production/helper/test
body, including save_parked_levels, swap_level, enter_level, levels_to_tick,
retire_world_counter, the new parked-save tests and all qualified owner/raid tests.
No new field, guard service, callback, injected fault, ledger, journal, public API,
owner reconstruction, forced swap, save schema or successful-close policy change.

## Eligibility and existing native side effects

Keep the current eligibility pass and its iteration order EXACTLY:

- levels_to_tick observes ANY Guest record's dimension membership, each level's live
  Streaming.forced set and pregen_level. When none applies it returns Overworld.
  It does not filter guest placement/loading like the PLAYER preflight, and it
  does not mean every parked level is occupied.
- Each occupied parked level has idle reset to zero and is excluded from closing.
  Each other parked level has idle incremented once. Only non-Overworld levels
  reaching IDLE_TICKS (300) enter the captured closing vector. An unoccupied
  Overworld still never closes through this method.
- A pregen task protects its dimension even when paused: pregen_level returns the
  task's dimension without testing paused. Do not change that to pregen_runs_here.
- No newly eligible level is added during the close loop; preserve the captured
  HashMap-derived order without sorting or retrying as an incidental change.
- If closing is empty, keep the existing early Ok with no transition/save. That
  is not a byte-for-byte no-op for idle counters: the eligibility pass already
  reset/incremented them. Do not move the pass or roll those counters back.

There is a further important native side effect. enter_level sets the departing
Parked record's idle to zero. After entering a failed level and restoring the
starting level, the failed level therefore has idle zero. This is ordinary native
restoration behavior; preserving it is part of this small fix. A subsequent call
usually increments to one and does NOT retry the failed save immediately. An
immediate Ok after filesystem repair is not proof of a successful close/retry.
Advance the normal idle eligibility through the required native calls before
claiming retry. Not-yet-processed selected owners may retain their incremented
threshold idle count, while a failed entered owner resets through restoration.

## Exact result, restore and partial-close ordering

Retain the eligibility pass and empty-closing return. Capture the starting dim.
Run the CURRENT loop inside a local Result-producing block/closure, stopping at
the FIRST error. For EACH selected dimension preserve this exact order:

1. enter_level(None, selected_dim).
2. save_level(true).
3. enter_level(None, starting_dim).
4. Under the existing paired-packages gate, borrow the selected Parked record and
   call the unchanged retire_world_counter(world, "idle-level-close").
5. Remove ONLY that selected Parked record.

Do not move step 3 out of this loop. Counter retirement and removal must still
follow successful saving AND native restoration for that individual level.
A saved level whose return transition hypothetically fails has not completed a
close and must not be retired/removed merely because its save succeeded.

After the loop yields either Ok or Err, ALWAYS evaluate one final native
enter_level(None, starting_dim). On the ordinary success path this is a no-op
because each completed close already returned. On a failed save it restores the
original owner and hub context before the error propagates.

Preserve result precedence: return the first loop error if present; otherwise
return the final restoration result. If final restoration fails, log that failure
with the REQUESTED original dimension and ACTUAL current self.dim. If both failed,
keep the original error and report the secondary one. Do not change the error into
a generic string, hide it behind a later save, or claim restoration succeeded.

Earlier COMPLETED closures remain closed. The operation is not transactional:
files/producer side effects from successful earlier levels stay, their counters
were retired, and their Parked records were removed. The failed selected owner
and every not-yet-processed owner remain in their actual native locations; after
normal restoration the failed owner is parked again. Never re-open/rebuild an
already closed level to manufacture all-or-nothing retention. Do not retire or
remove a failed/not-yet-processed owner, clear a sticky writer error, replace its
SaveDir, or continue the remaining saves after the first failure.

Use a local loop_result and final restore_result with ordinary first-error
precedence. Do not factor this loop together with save_parked_levels: this path
has intentional successful retirement/removal which that save-only path lacks.

## Restoration-error reachability at the inspected base

Natural final restoration failure remains UNREACHABLE under this synchronous
method's current invariants. enter_level only fails by opening a target absent
from Levels.parked. The first successful entry parks the original dimension;
completed loop iterations return to it before removing only their selected
non-original level. The original active key was never in the initial parked
closing list. save_level and retire_world_counter do not remove that original
Parked record. No callback/tick/pregen/control pump interleaves with this loop.

Thus a failed save leaves the original parked and available for a nonfallible
native swap; an error before any swap leaves the original already active. The
inner step-3 restoration failure is likewise unreachable at this base. Preserve
Result handling/accurate defensive diagnostics for future native failure changes,
but qualify these branches by source review only. Do not delete an original owner,
poison unrelated state, inject a callback or mock a transition to claim a real
double-error restoration test. There is no panic/abort recovery guarantee.

## Meaningful two-file test surface

Use ignored actual resource-backed tests in the NEW levels/idle_close_tests.rs,
with the existing pinned Fixture/new/game and unchanged entity_owner_tests::ready,
players/rich as appropriate. Preserve the synthetic local body outside Game and
hold both channel-backed external guests/receivers and participating SaveDir Arcs
through assertions. Existing helpers remain unmodified and private fields stay
private. New small setup/assertion helpers belong only in the new test child;
qualified parked_save_tests helpers must not be widened just to reuse them.

Obtain actual parked levels via enter_level, restore the intended starting level,
then seed defining-module idle thresholds for a controlled first close attempt.
Keep fixtures settled, with distinct resident blocks/rich native drops, stable
UUID/raw entries and empty pending loaded rows. Compare owners by logical Dim,
SaveDir/chunk/item-metadata identity and payload values, not stack addresses of
moved World/Parked records. Allow ordinary save dirty/metadata/cache side effects.
Compare exact directory/file entries and unrelated bytes against a finite native
write whitelist; no broad subtree permission merely because a root is Overworld.
Independent disk evidence uses NEW SaveDir readers at exact dimension roots after
the native writer completion observation appropriate to the tested outcome.

The EARLY metadata fault must not defeat idle eligibility: a nonempty forced set
would itself keep the target occupied! Keep Streaming.forced EMPTY and, AFTER
opening the parked level, write a valid existing chunk_tickets.dat saved-data
record. For example retain an ordinary opaque non-forced ticket/root marker.
The actual save_forced_chunks(empty) then reaches its normal write path because
the file exists. Create the owned directory at THAT dimension's
data/minecraft/chunk_tickets.dat_new. This is the real first save producer's
write_dat failure without force-loading the dimension. Do not obstruct another
level's path or infer a reached save from a refusal value alone.

Minimum concrete cases:

1. One eligible parked level with the early ticket obstruction. Actual close must
   return the native IO error, restore starting dim/SaveDir/hub context, retain the
   same failed World/drop/UUID/raw/counter/slot owner values and writer, and leave
   disk entries/bytes unchanged. Assert the actual post-restoration idle value
   rather than the old threshold. A separate failed-owner variant may hold a
   nonempty pending World slot retirement and compare its key/payload values.
2. Filesystem-only repair in a fixture with ordinary drops and no unsupported
   pending item slot to be destroyed by successful closure. Invoke the native
   method through the normal idle-count interval; assert no premature closure or
   producer writes. At the real retry require successful close, original active
   restoration, selected key removal and exact independent block/entity/ticket
   readback. Do not label the first post-repair no-op call a successful close.
3. Later sticky entity-writer failure using a file at the eligible parked root's
   entities path, with nonempty supported entities and dirty chunks. Restore and
   retain the SAME SaveDir/failed owner. Removing the obstruction does not reset
   the writer error. After the native idle interval, a retry still returns the
   same sticky error and restores the starting level. Native queueing continues
   despite that error: after count-zero flush still reports it, a NEW disk reader
   may/properly should see the repaired retry's actual entity records. Do not
   assume an Err means nothing was written or replace/reset the native writer.
4. Two selected parked levels: observe the native pre-call closing order without
   imposing a sorted policy. Let the first complete and obstruct the second.
   Require the first removed with independent disk records, second retained with
   same writer/payload, and original dimension/hub restored. Earlier successful
   closure is allowed and must not be reconstructed or rolled back.
5. Obstruct the FIRST of two selected levels instead. The failed and unprocessed
   owners both remain; the second's producer files and counter remain unchanged.
   Check the distinction between failed-owner idle reset and unprocessed-owner
   eligibility increment. Stop after the original error; no later save/retirement.
6. Eligibility/ordinary regression: occupied-by-guest and nonempty forced levels
   reset idle and are not closed, below-threshold levels increment without IO,
   no eligible levels preserve owner/disk state apart from those idle counters,
   and Overworld remains excluded. An ordinary successful eligible close retains
   the current save-return-counter-retire-remove ordering and its disk records.

For pregen, inspected PregenTask fields/constructor are private to commands/pregen.
Do NOT widen them or add a task factory. The existing native public
set_horizon_commands plus run_command("pregen start 0 square"), "pregen pause"
and "pregen cancel" can construct an otherwise real task without ticking/pumping
workers. That path temporarily changes the process-wide native command-tree flag;
it is NOT a per-Game fixture field. A coordinator-approved single-test fresh serial
process could use it, restoring the flag and cancelling the task, to prove both
running and PAUSED tasks keep their dimension occupied. If that exact isolated
native setup is not adopted in review, disclose pregen eligibility as source-only
coverage in this two-file packet. No environment/global command changes may be
smuggled into an ordinary shared test process or existing Fixture.

A native counter can be enabled with the existing World counter API and local
metadata configuration, with no item payload or persistence claim. Failure must
not retire it. Successful close intentionally makes the selected World disappear;
its existing HORIZON_COUNTER_RETIRE stderr receipt can be checked by the coordinator
if captured, alongside source ordering and removal. Do not add an injected sink or
claim an in-test inspection of a World already removed. Snapshot Debug + SlotKey
shows retained World-slot key/payload VALUES, not the private SlotProjection
payload allocation address. Prior defining-module tests/source support the latter;
do not widen the slot surface to manufacture address evidence.

## Successful-close and outer-lifetime limits

retire_world_counter takes the transient COUNTER trial and emits a best-effort
receipt. It does not serialize machine state or retire/drain every World slot.
Preserve this successful-close policy; do not silently turn this failure correction
into a new whole-slot close/conservation service. Never use a successfully closed
item-bearing unsupported slot as evidence that its payload was captured. The
failed-save owner retention and an actual successful ordinary drop close are
separate claims and need appropriate separate fixtures.

This fixes close_idle_levels restoration only. tick_server has other selection/
error exits, and later shutdown/caller lifetimes may destroy a failed Game. No new
failed-owner service, replay queue, global checkpoint readiness, producer
quiescence, block/container/spill consistency, scalar/codec qualification,
world-history/reconnect retention, in-frame capture adapter, machine persistence,
immutable publication/recovery or G1 acceptance follows. server-hold still pumps
control work and Integrated::call timeout does not cancel an owner operation.

## Qualification and stop

Coordinator owns exact qualified base/branch, stable source review, all formatting,
feature-on/off compile/lints, actual isolated resource execution and appropriate
ordinary/native/broad/both-mode regressions. Serialize Cargo. Preserve failed
attempts and distinguish source-only branches and prior evidence from executed
new cases. Obtain a DIFFERENT fresh independent source/receipt review before
qualification. Stop after the bounded idle-close failure/restoration correction
and named evidence pass. No public push, merge, release, upstream contact,
permission expansion, standing team or operating-rule change is proposed.
