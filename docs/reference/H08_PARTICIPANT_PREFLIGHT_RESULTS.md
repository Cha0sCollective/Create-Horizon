# H08 native participant roster/player preflight results

Local engine candidate 78c5a0d70824ce397d1a26bd6736a149000d99db, tree
13d6c9ad9d22f006f9e08b5883f38b56aebbc2eb, based on accepted H08a14 b3e0d356.
Four files add 954 lines, including 594 resource-test lines, above the upstream
soft size target. The normal public-tree commit hook passes; no public engine push.
Acceptance boundary: H08_PARTICIPANT_PREFLIGHT_PACKET.md.

The dormant paired-packages methods observe native host/sender Arc identities,
exact order-independent conn/UUID/dimension populations and canonical integrated-
owner/external record mapping. Strict validation rejects missing/duplicate/invalid
mapping, unsupported player owners/activity, acting guest turns, transitions and
server-local owners. Defining-module predicates borrow private fields without
exposing them. No new Game field, IO, projection, callback, normalization, payload
take, SDK/control command or gameplay enabling callsite is added.

The expected roster is not refreshed by preflight. Native marker allocation is
compared with Arc::ptr_eq; mutable pending byte counts are not compared. Cloned
senders share their marker, while production connections allocate a fresh one.
This is not global conn uniqueness or a persistent incarnation ledger; deliberately
reconstructed same-marker/same-tuple guests remain indistinguishable.

Fresh independent read-only Sol High design/source reviews find no blocking issues.
Initial no-run compilation caught a test-only Vec versus borrowed-Vec assertion;
one extra borrow fixes it before any resource execution. Original compiler output,
source and hashes remain in workspace-root .local/h08p1-build-attempt1. Production
sources were unchanged by that correction.

Frozen precommit test binary SHA256:
eed9dd4e7b53b65b2d7929f5385aebae411dec375d342acfb6b37301abdbde18.
Receipts bind all four formatted source hashes. It predates the final source commit;
no clean-commit test-binary build is claimed. The isolated runner verifies source,
binary/resource hashes and all seven exact names, creates fresh cwd/HOME/data/
config/cache/temp, removes session and preserves inherited /run/user/1000.
Seven resource cases pass, zero ignored, exit 0, no timeout, in 23.74 seconds:

- Real owner/external canonical mapping with different account/player UUIDs,
  reorder independence and unchanged native values.
- Missing/changed/duplicate/invalid mappings refuse without repair/default creation.
- Host replacement and same-tuple fresh sender allocation refuse; cloned sender
  identity remains valid without treating numeric pending counts as identity.
- Cursor/crafting/inbox/body/teleport and other supported refusal cases retain
  original ownership and raw recursive BundleSelected metadata.
- Actual guest turns and native table/station/merchant openings are refused as
  nonidle owners; orphan play state also refuses.
- Authority, transitions and server-local ownership refuse.
- Matching idle players can still coexist with an off-resident entity: success is
  explicitly PLAYER-only, never global checkpoint readiness.

Independent read-only resource follow-up verifies those receipts and unchanged
source hashes. Not every guarded transition/private field is individually toggled
by resource tests; concrete borrowed source checks plus two local predicate tests
cover the stated native contract without claiming exhaustive field mutation.

Native validation passes: 175 ordinary bin tests (33 ignored), including the two
new predicate cases, all three feature lint matrices, formatting/public-tree and
normal hook. The seven explicitly executed resource cases are not silently counted
as ordinary passes. Broad --quick passes at the exact candidate: 369 Python tests,
1,357 client tests, zero doc tests, fmt/Clippy/public-tree and unchanged locks.
Untouched content-root/theme/launcher checks and vanilla parity are explicitly
skipped. Soft scope warning reports four files/+954 lines. Broad wrapper exits 0.
Broad log SHA256:
7b1fc7bb88457cc27a7083f85a0aa877362008a7fdd96ed0a03e55c3fba0e3cd.
Final independent read-only receipt review accepts this bounded PLAYER observation
with no blocking findings and confirms unchanged source/commit/tree and all receipts.

No new ordinary Game execution is claimed for this unconnected helper; unchanged
normal-path full-counter coverage is reused from H08a14. Historical failed Left,
queued transport events, global/parked/entity/World-slot owners, serialization,
all-dimension writers, immutable publication/recovery and lasting admission remain
required later work. A later capture must recheck inside the SAME exclusive owner
operation. This does not establish player authentication/replay, machine durability,
external paired networking, retained animation, performance, Create conformance or G1.
Raw build/resource/native/broad evidence stays outside Git at .local/h08p1-*.
