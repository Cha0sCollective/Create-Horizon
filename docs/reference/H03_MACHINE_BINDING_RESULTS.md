# H03c machine type metadata checkpoint

Local engine commit `22ef8c5e3ec14b32fe5a6d91bb07a33ff6a662f3`, tree
`386d01de699397387223ebb5cb814f89770d61e3`, adds captured package-owned type/schema
declarations and immutable block bindings. This is metadata for the original S1
counter experiment; placing the block does not create a working machine.

The preview admits at most one type and one bound iron block per captured paired
package. A type derives its primary-owner ID from
`data/<package>/horizon/machine/<path>.json`, accepts only format/save-schema 1,
and is limited to 64 KiB. An unbound type is allowed. Bindings resolve after all
declarations, independent of file order. Null, unknown/duplicate fields, malformed
paths/IDs, alias/foreign ownership, dangling bindings, duplicate/cap excess and
combining a binding with static state-definition reject. Legacy uncaptured content
and feature-off builds refuse the new field/path. Existing captured hashes and
world pins include declaration bytes; no extra hash or API version was invented.

Read-only catalog/type/binding getters retain validated metadata. Under the
experimental paired feature, Block now has a private binding field, so external
Rust struct literals are no longer constructible; no affected external in-tree
literal was found. Feature-off layout and the guest WIT APIs remain unchanged.
This preview does not establish a stable host Rust API.

Validation on pinned Rust 1.99.1, serialized shared Cargo target:

- Content/parser tests: 20 pass with paired feature, nine pass without it. These
  include nested machine-schema and state-plan tests, not just top-level tests.
- Explicit real-resource metadata regression: one pass. Ordinary and bound
  captured cubes materialize into distinct caches with different captured identity
  stamps but identical 14 JSON and six NBT physical tables. Vanilla prefix and
  appended/default IDs remain fixed; cached geometry matches iron and the collision
  loader succeeds. No physical materializer production code changed.
- Mods library/test Clippy with warnings denied passes with feature on/off; client
  library/test Clippy with feature on passes. Formatting/whitespace checks pass.

Raw logs are `.local/h03c-*.log`. Fresh independent read-only Sol High review found
one test defect: both materializations originally used the same output cache,
so the second overwrote the first. The failing run is retained separately as
`h03c-resource-metadata-initial-failed.log`; the corrected test compares distinct
cache trees and separate stamps. Delta review found no remaining finding.

Coordinator verified these exact physical SHA256 values after copying the isolated
worker's three files, then committed with the normal public-tree hook:

| File | SHA256 |
| --- | --- |
| mods content.rs | `c279b9524e2993286f776a8a228ad19ed11fa22137fef5cdf3f07221436355df` |
| content/machines.rs | `b13d39365bb437c401c5e7eda7229871b06259beb387472fcddc3a0b825a1485` |
| client mod_content_tests.rs | `e81aa0fdad439769bf01f001e26dcb37ed5c8356487699cd9e866a595f9c1577` |

Review used behavioral read-only restrictions under workspace-write permissions;
it did not run tests, build or launch the game. No public engine push occurred.

Instances, payloads, ticks, callbacks, slots, persistent placement generations,
migrations and saved records remain later work. Explicit load/unload/activity
lifecycle is still needed before records can follow the world correctly. The
separate bounded generator-lighting fix is needed to complete the optional
in-frame dimension-observer probe; it does not alter these metadata results.
Full H03/H04/H05 and G1 remain open.
