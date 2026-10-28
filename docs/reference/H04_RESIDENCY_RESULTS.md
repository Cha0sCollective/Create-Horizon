# H04c passive residency seam

Local engine commit `ddbeb8d4bdda9af13093b0f9f3ce6ff098cf6f55`, tree
`50606a7ff488dba756823740294a2873a132e0e0`, captures the two ordinary authoritative
streaming membership writes through World wrappers. Default/feature-off worlds
preserve ordinary map behavior. No automatic observer or guest API is introduced.

The separate observer accepts at most 64 supplied coordinates and 4,096 queued
events, with immutable scope, captured game time and checked sequence. Enable
returns sorted current membership without inventing history. A read-only current
snapshot does not drain history and remains available after a queue fault. Initial
snapshots stay immutable. Same-key admission is replaced; missing removal is a
no-op. Overflow/exhaustion faults drain persistently while chunk writes still
commit. No Chunk payload is retained.

Replica insert/forget and temporary SDK map lending stay separate. Scope/queue
travel with World swaps; destruction ends observation without invented Drop or
per-chunk unload delivery. Coordinate history cannot reconstruct the contents of
a chunk which loads/unloads before consumption. Future record consumers need
synchronous admission or appropriate bounded snapshots.

Pinned Rust 1.99.1 checks, serialized after actual game testing:

- Five new ordinary focused tests pass. The name filter also passes one existing
  end-sky test; that sixth result is not a new residency test.
- Explicit owned-resource streaming seam regression passes. It uses generated/
  saved final-chunk fixture inputs through real update admission, replacement,
  far removal/reload and replica-separation paths. This is not disk-load or real
  Game-controller qualification. Missing resources fail explicitly.
- Eight existing mutation tests and the existing World regression pass; the World
  regression also passes without the paired feature.
- Final library/test Clippy with warnings denied passes with feature on/off;
  formatting and whitespace checks pass. The current-snapshot getter delta reruns
  focused tests and both lints. Earlier unchanged resource evidence is reused.

Raw logs are `.local/h04c-*.log`. Fresh independent read-only Sol High review found
no actionable issue in source or the getter delta. Review performed no builds,
tests, imports, launches or writes under its behavioral read-only restriction;
actual runtime permissions were workspace-write.

Coordinator verified exact copied SHA256 values before committing with the normal
public-tree hook:

| File | SHA256 |
| --- | --- |
| residency_observer.rs | `bcfc9ec537e6d287d06673b5a0acd5ea84856092d3fd5cee665acbe58f3417e3` |
| world.rs | `a8a803ed7a28b9f3062289e732f8297d2919072a351feaa66e4bcedbec3f327d` |
| streaming.rs | `8a0ebb8d798a62959e919122e1c59fd4bcbda00ca0f32e788dc78e1d8dfb773f` |
| lib.rs | `21c7b8e37edab993cae5ac573f23f403770425fa2c8f3b245f589c35d474f725` |

Next: `docs/design/H04_RESIDENCY_GAME_PACKET.md` adds explicit strict-authority
controls and real unload/reload/SDK-read probes in both game modes. Those now pass
in `docs/reference/H04_RESIDENCY_GAME_RESULTS.md`. Eligibility,
pause/parked activity, lifetime retirement, records, generations, inventories and
coherent recovery remain open. No engine push or G1 promotion occurred.
