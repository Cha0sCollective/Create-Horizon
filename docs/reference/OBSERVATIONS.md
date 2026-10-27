# P01 first observation cases

Status: the initial and expanded corpora have run twice; outcomes are recorded in
`S0_RESULTS.md` and the added schema-2 cases in `EDGE_CASES.md`. This document
preserves the original six-case schema and schedules below.
This narrow adapter uses NeoForge GameTest and real server block entities. It is
not a replacement simulation, a generic experiment framework or a native parity
runner. See [environment](ENVIRONMENT.md) and the original Java adapter under
`tools/reference/java/dev/ch4os/reference/PortReferenceTests.java`.

## Inputs and coordinate conventions

The original empty test volume is 16x8x16 with structure data version 3955. Tests
place an upward-facing motor at relative `(2,2,2)`, a drive at `(2,3,2)` and the
direct-drive/milling load at `(2,4,2)`. Shafts and cogs have Y axes. Coordinates in
traces are fixture-relative, never raw runtime registry IDs or network IDs.

| Fixture | Schedule and samples | Initial settled invariant |
| --- | --- | --- |
| direct_stop_reverse | Samples ticks 1..70; tick 10 checks +16 then requests -16; tick 30 checks -16 then removes connecting shaft; tick 70 checks stopped | +16 and -16 reach the load; removal stops it |
| small_to_small | Two small cogs at `(2,3,2)` and `(3,3,2)`; samples ticks 1..20 | Target -16 RPM for +16 drive |
| large_to_small | Large driver at `(2,3,2)`, small target `(3,3,3)`; samples ticks 1..20 | Target -32 RPM for +16 drive |
| small_to_large | Small driver at `(2,3,2)`, large target `(3,3,3)`; samples ticks 1..20 | Target -8 RPM for +16 drive |
| milling_16 | Shaft below millstone; tick 10 sets +16 and drops one cobblestone at relative `(2.5,5.2,2.5)` with zero velocity; samples 1..300 | One gravel, no input remaining, no other output |
| milling_256 | Same layout and schedule, +256 RPM | Same conservation invariant; observe actual completion tick |

Sampling is a GameTest scheduled callback, using that test's tick counter. At
action ticks one callback samples first and then mutates or asserts. Independent
same-tick callbacks have no defined registration order in this Minecraft version.
Record this ordering and inspect actual observations; do not label samples as a proven
beginning/end-of-world-tick boundary. The server's world seed is included. The
adapter does not replace random sources or call machine tick methods manually.

The motor helper sets the existing speed behavior directly. It does not establish
the player's normal control gesture or zero clamping. The dropped-item case does
exercise actual entity collision and server inventory intake. Player empty-hand
retrieval and overflow are not covered by these first fixtures.

## Initial trace schema

Each console row starts with `CREATE_HORIZON_TRACE ` followed by one JSON object:

- `schema = 1`, fixture name, integer relative tick, phase, dimension and world seed.
- Ordered machine observations with relative position, namespaced block ID and
  sorted state properties. A removed shaft remains an observation of air.
- For kinetic block entities: theoretical/effective signed RPM and network
  presence, stress and capacity in reference stress units. Absence is distinct
  from a zero-valued active network.
- For millstones: remaining recipe work and each of one input/nine output slots,
  including inventory owner, slot index, namespaced item and count.

These first fixtures use only cobblestone/gravel without custom components. The
schema must add canonical item components before it is used to establish arbitrary
item equivalence. It intentionally omits unstable raw network IDs. Millstone work
is read directly from the reference timer; completion is the first output change,
not an assumed `processing_time` tick duration.

The per-run metadata records source pins, exact JDK and hashes of the adapter.
Resolved artifacts and generated configurations must agree across repeat runs.
Keep raw console logs, configurations and worlds local; commit only a concise
result report and non-game fixture source. Missing rows, duplicate ticks, partial
logs, failed assertions or mismatched environments invalidate a comparison.

## Expansion and later observations

The edge corpus now covers finite stress/recovery, interruption/reversal,
changed/removed input, one/all full output slots, retrieval/overflow and motor
value settings. The owner also exercised the physical motor gesture. The following
list preserves the originally identified coverage needs; remaining lifecycle and
contention cases belong to subsequent packets, not an implied pass:

- Finite-capacity source at stress equality and overload, followed by recovery;
  the creative motor does not establish survival capacity behavior.
- Mid-recipe power removal/reconnection and reversal; altered/removed input at
  completion; full/partially full output slots; multiple competing dropped stacks.
- Empty-hand retrieval with output, input-only fallback and full player inventory;
  count all host inventories and dropped remainders.
- Real motor UI gesture, supported signed values, zero clamp, and client/server
  acceptance; an internal setter is insufficient.
- Removal/replacement, chunk deactivate/unload/reload, restart and cross-boundary
  network contributions, including both identity and item accounting.
- Negative control against a deliberately changed result/ratio/work decrement in
  a comparison adapter, with a useful first-difference report. Do not alter the
  reference mechanics to manufacture its expected outcome.

Repeat-reference equality establishes reproducibility of those observations. It
does not establish a Rust port, host implementation, performance budget or G0 pass.

The next finite-capacity case can use a reference-only hand crank; this does not
add it to the native S3 component set. At the pinned source, `AllBlocks` declares
capacity 8 for the crank and impact 4 for each millstone, with stress enabled by
default. The crank turns at magnitude 32 RPM and its `turn` method refreshes a
10-tick use interval. Proposed layout: downward-facing crank `(2,2,2)`, supported
by the shaft `(2,3,2)` above it; Y-axis small drive cog `(2,4,2)` and millstone
`(2,5,2)`. Small target cogs at `(3,4,2)` and `(2,4,3)` can power millstones above.
Turn every five ticks, add the second then third load, remove the third, then stop
turning. The source-derived candidate boundaries are 128/256/384 stress units
against capacity 256, but actual signed speeds and transition ticks must be read
from the reference. Empty millstone inventories do not remove their declared
kinetic stress. Record the real configured values rather than inferring them only
from source defaults. This layout has now passed and repeated in the expanded corpus.

For two completed captures, run `python3 tools/compare_reference.py <run-a> <run-b>`.
The comparator requires all six successful fixtures, all 730 unique samples,
finite speeds/stress, complete declared inventory slots and matching environment,
dependency and configuration records. Configuration receipts retain raw hashes;
only Java's generated date-comment line in `server.properties` is excluded from
configuration equivalence. Actual settings and other bytes are preserved. The
manifest covers every file under config/serverconfig/defaultconfigs, including
FTB's SNBT files. Exit 0 means equality, 1 a first difference,
and 2 invalid/incomplete evidence. It refuses to compare one directory to itself.
