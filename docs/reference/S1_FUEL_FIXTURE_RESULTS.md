# Private fuel-exhaustion fixture qualification

Status: scoped FINAL ACCEPT for the private fixture and captured evaluator API.
This does not qualify persistent Game callbacks, records, slots or G1.

Only the private guest-counter fixture Cargo.toml and src/lib.rs changed: an
exclusive fuel feature enters a deliberate loop at counter >=3. Ordinary hooks,
client, SDK/WIT, default/trap/reject/invalid behavior and dependency locks stay
unchanged. Private checkpoint bf8b33b preceded native authoring; source author
s1_c0a_packet_design, source/tool reviewer s1_fuel_fixture_review, different
results reviewer s1_fuel_results_review. Both tracked files are included in this
normal private checkpoint; no source or artifact was published to the engine fork.

The external native consumer depends on accepted C1fa source and exact mods tree
1e2d42f88c86dc1b2fb025ba46fceae5cde7afa1. It uses its own recorded offline lock
e159fda79f827447181505e336b9763625bfe7561a3748740d0cb61802dba062, rather than
the main Game artifact. Source/tool review preceded all compilation and execution.

Actual release fuel-only Wasm core718ac259830f87532a281f474cdde49862ad7d0039cceb45e0b768122812b5d3
was standard-componentized. The new five-file captured pair preserves the pinned
H09 client, descriptor and content; only server component155579c3b8dfa7e52e54fc67fd3b2ad6a69c6a34ecb1a31858b92d83627949ed
differs. Capture contents and executable identities remain stable before/after.

Actual controls passed:

- Both component imports zero and canonical counter ABI accepted.
- Ordinary client/server activation, exact capture identity and live evaluator selector.
- Complete +1 proposals for inputs 0, 1 and 2.
- Input3 traps in the Wasm step: all fuel consumed by WebAssembly.
- Input0 succeeds again through the evaluator's fresh Store path.
- Requests remain immutable and ordinary output queues remain empty.

The diagnostic is observational evidence. Current Game classification remains
GuestFailure::Evaluation; no String parsing creates typed Fuel authority.

An ignored SDK dependency input was caught before execution. The reviewed driver
now enumerates the whole WIT tree, rejects unexpected files/symlinks and pins its
accepted Git provenance. Fixture copyecc68691 equals canonical Git blob0680cce9;
native SDKrawa19 differs only by837CRLF. No WIT input replacement occurred.

Buildreceiptcdb453ecfe085ac589b53464b6f249c0bee311ba36cdaecc08d41eed423c0b63;
qualificationreceipt410f56d7a5077bfa878f107781e47deed9850546a9eb8c2a04d34bb9b085582e.
Frozeninput48076b991f993a40e7df6da194c0895708646cd5ebe219f9b60265c3ccb2cb9f;
summary8bf9f6d184ae41270a6b7749366c5c28d66d3c1f6a0d64651cd7ff54b8958fc9.
Different reviewer verified26indexed artifacts and17source inputs. Native Linux
core/consumer identities rely on guarded receipts and Cargo artifact provenance;
the reviewer ran no tools through WSL. Python-I is Root's recorded historical
invocation, not a dynamically measured receipt field. Optimization rejection and
environment/data isolation were reviewed. No reviewer execution or writes.

Real persistent Guest tests must still prove original counter/slot/item ownership,
failure latching, stopped recovery and ordinary lifecycle on the actual host Game.
No optimized Game, performance, publication or G1 acceptance follows here.
