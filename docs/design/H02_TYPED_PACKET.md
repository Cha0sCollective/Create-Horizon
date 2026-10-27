# H02b: paired code type checks without execution

After H02a's reviewed local engine commit
`06545594bad9e67982048b3e70aff40ee26ba53b`, add a narrow runtime preflight for
captured client/server bytes. Keep it behind the existing experimental
`paired-packages` feature plus runtime; do not wire discovery or world admission.

Compile both components with the existing engine, enforce each side's declared
imports/capabilities and supported API functions, and check required export types
against the generated client/server bindings. Use Wasmtime pre-instantiation and
generated `*Pre` validation, with no Store, guest callback, init, world mutation or
renderer. Return immutable compiled components tied to the prepared declarations.
Malformed/core modules, missing exports, wrong worlds and undeclared imports fail
before a checked pair is returned.

Use the current WIT to make authored dummy components for focused tests. A valid
pair must type-check without invoking its unreachable dummy functions; invalid
code and permission/world mismatches must fail. Preserve legacy runtime behavior.
Stop after relevant tests/lints/self-check and fresh independent review, or record
a concrete blocker at the owner's 7 p.m. Central cutoff.

Success remains short of full H02/G1. Content identity/immutability, peer admission,
side-specific instantiation/state/storage and actual integrated-server execution
are still required. This step must not certify package content/digests or claim a
saved counter machine. It uses only Horizon's own API and generic runtime code.
