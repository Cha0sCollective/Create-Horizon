# H03c captured machine type metadata

The next original S1 counter needs package-owned type/schema identity before
instances or inventories exist. Add bounded immutable declaration/binding metadata
to captured paired content. Preserve the existing static state preview's explicit
no-machine behavior. This packet is not a working counter or full H03/H05.

Recognize `data/<package>/horizon/machine/<path>.json`, with format 1 and
save-schema 1, rejecting unknown/duplicate fields and unsupported values. Derive
the type ID from the declaration's primary owning package namespace and checked
path. Do not accept arbitrary owner IDs or a separate identity hash. Bound each
declaration to 64 KiB and this preview to one machine type and one bound block per
captured paired package. Resolve declarations before bindings, independent of file
order; reject dangling/foreign bindings and alias namespace declarations.

The bound block adds `machine-type: "<package>:<path>"`, must use the exact iron
block template and cannot also use static `state-definition`. Binding metadata is
immutable after validation and available through read-only catalog/type getters.
One optional unbound declared type may remain metadata; no block may bind a missing
type. Uncaptured legacy content and feature-off builds must refuse these fields
and declaration paths. Ordinary legacy blocks, static states and vanilla IDs stay.

Implement in the mods content parser and a small `content/machines.rs` module;
client materialization should need no physical-table changes for the full cube.
Use existing captured-content hashes and exact world pins. Tests cover valid
counter identity, malformed IDs/JSON, schema/version/ownership/duplicate/cap errors,
dangling bindings, uncaptured/feature-off refusal and static combination refusal.
Use a focused real-resource regression to compare vanilla prefix/appended state
IDs and verify existing geometry does not change for metadata-only binding.

There are no payloads, slots, instances, callbacks, ticks, generations, migrations
or save guarantees in this packet. In particular the observer's transient scope
is not saved placement identity. Complete load/unload/activity lifecycle precedes
an inventory-free record experiment; coherent save qualification precedes G1.
The eventual slot/callback interface still needs comparison with pinned PR1189;
this content metadata does not adopt or claim that draft interface.

Coordinator owns integration and commits; one isolated worker owns parser/module,
focused tests and only necessary mechanical fixture changes. Fresh independent
read-only Sol High review covers the stable candidate. Keep Cargo stopped until
the coordinator releases it after game testing. No public push or broad framework.
