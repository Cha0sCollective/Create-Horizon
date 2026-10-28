# Pinned PR1189 reconciliation

Reference: Horizon commit
[`847210b1e764638c97fce84489b2e9c368668388`](https://github.com/horizon-rust/horizon/commit/847210b1e764638c97fce84489b2e9c368668388).
This identifies a design snapshot, not current PR status. The exact commit is
available through the connected GitHub source even though the local object stores
lack it. Compare against our dfcd7e6 engine and H03c metadata at 22ef8c5.

Read-only audit fetched only the generic Horizon SDK WIT/Rust interface, runtime
and manifest. No third-party mod proof-of-concept source was read or adopted.

| Source at the pin | Git blob |
| --- | --- |
| [SDK WIT](https://github.com/horizon-rust/horizon/blob/847210b1e764638c97fce84489b2e9c368668388/crates/horizon-mod-sdk/wit/horizon.wit) | `23352f2bf22f20e7a86afd4a519e6746ecafcc07` |
| [Host runtime](https://github.com/horizon-rust/horizon/blob/847210b1e764638c97fce84489b2e9c368668388/crates/horizon-mods/src/runtime.rs) | `bb72dfa6062bdcf4d3ea5bfeb77e9c12fbec297a` |
| [SDK Rust](https://github.com/horizon-rust/horizon/blob/847210b1e764638c97fce84489b2e9c368668388/crates/horizon-mod-sdk/src/lib.rs) | `5d792801d0ac358c2244ec6554ba12fd0a072e5b` |
| [Manifest](https://github.com/horizon-rust/horizon/blob/847210b1e764638c97fce84489b2e9c368668388/crates/horizon-mods/src/manifest.rs) | `0fbf8048d4d128e4d0dca792867697673c8a2f9a` |

Useful patterns include per-package batched callbacks, host-owned slot views,
ownership checks, separate item/recipe facts, bounded fuel/memory and default SDK
callbacks. These do not establish our game lifecycle, transactions or checkpoint.

| Pinned design | Required reconciliation |
| --- | --- |
| Machine world includes client horizon-mod | Bind only the authoritative paired server component, preserving separate stores and dedicated behavior |
| Executable types() registers blocks/slots after init | Captured H03c catalog owns type/schema identity and binding; no competing mutable registration |
| Any manifest-owned namespace | Retain primary-owner declaration/binding validation |
| Wrapping u32 handles | Checked invocation-local handles mapped to exact records; never substitute for durable keys/generations |
| One tick batch includes all views if any type ticks | Select exact eligible instances at the documented per-level phase |
| Trap returns views including earlier writes | Discard staged gameplay effects on trap/exhaustion/invalid result |
| Slots contain item ID/count and clamp large counts | Preserve canonical components and host stack identity; reject invalid transfers explicitly |
| Invalid/empty/out-of-range results overlap | Distinguish stale access and validation failures for diagnostics |
| Guest save callback supplies bytes | Host owns last committed payload/schema and can checkpoint a faulted guest |
| Reload discards handle ownership | Rebuild guest cache from committed host records without reallocating durable identities |

The trap behavior is explicit in
[runtime.rs around line 600](https://github.com/horizon-rust/horizon/blob/847210b1e764638c97fce84489b2e9c368668388/crates/horizon-mods/src/runtime.rs#L600).
Its isolation between mods' lent views is not all-or-nothing commit. A changed bit
can help dirty tracking but cannot replace validation, versions or accounting.
The inspected interface also lacks our complete dimension/generation keys,
coherent checkpoints, replay protection and replication contract.

Do not merge the draft or adopt its API 0.4/WIT world wholesale. After residency,
eligibility and lifetime proof, an inventory-free native host-record experiment
can join records to H03c. Before guest callbacks, review a narrow server interface:
immutable dimension/tick views, checked call-local handles, bounded privately staged
payload/state operations, commit after success plus ownership/version/quota
validation, and guest cache reset after failure. Add the owned slot separately in
H06/H07 with canonical stack and transfer accounting.

The guest byte save and ordinary chunk saves do not establish G1. H08 still needs
the coherent checkpoint experiment. This comparison resolves the immediate draft
research prerequisite without accepting a new callback/slot API or creating an
owner-input blocker.
