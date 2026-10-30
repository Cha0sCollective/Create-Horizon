# H09a bounded peer compatibility snapshot

DESIGN ACCEPTED, NOT RELEASED. Fresh independent read-only Sol High review accepts
revised proposalSHA d17bb651e2bcd07bf826af9c578c73d40ddd3750333b1db2ea8cf720c8e8733d,
no remaining blocking findings. Exact snapshot .local/h09a-compatibility-proposal2.md.
Native fence has different FINAL acceptance at cleanLOCALea96036ce48c269225169e4865e02e4860182b47,
tree7828a3f89b775f36503070175f3c8784d7c65cf3. Three-path release follows this private
checkpoint push and clean pinned codex/h09-peer-compatibility-snapshot creation.
Original proposal0c79a78d WITHHELD for unspecified pre-conversion resource limits
and silent native packet-writer NBT fallback. Its immutable snapshot remains
.local/h09a-compatibility-proposal1.md. This revision requires fresh review.
Release requires its accepted clean successor pin, private checkpoint and fresh
independent Sol High design acceptance. Source audit: H09_PEER_ADMISSION_AUDIT.md.
This is a concrete input prerequisite to H09 negotiation, not admission or H10.

## Objective and three proposed paths

Build a bounded, owned compatibility SNAPSHOT for the supported S1 profile from
actual captured startup package identities and actual registry/configuration data.
The value describes one capture time; it is not a readiness/admission receipt,
remote-code attestation, permanent freeze of mutable tables or network capability.
Keep Packages::external_network refusal and ALL existing network callers unchanged.
No packet exchange, wire format, login state machine, world open/write or activation.

Relative crates/horizon-client/src:

1. lan/data.rs: registry/configuration snapshot value and bounded digest builder,
   focused defining tests. Existing loader/sending behavior unchanged.
2. game/mod_packages.rs: private package/build/schema snapshot composition and
   component-specific comparison, plus new test-child declaration. Existing read,
   activation/world pins/external_network bodies unchanged.
3. NEW game/peer_contract_snapshot_tests.rs: actual captured-input/resource cases.

No lib/module registry beyond that declaration, SDK/dependency changes, renderer,
save/fence code or networking startup edits. Report concrete extra dependency
before expansion; source-level review must check actual encoder/builder APIs.

## Supported capture and identity

No selected pair returns None without changing legacy behavior. Paired profile is
exactly ONE successfully captured/prepared pair with ONE current machine schema;
refuse enabled legacy/unknown/failed extra owners or unsupported profile counts.
Use the retained Catalog/PreparedBundle identity and startup executable SHA256,
not re-reading mutable source paths or accepting a caller-supplied claimed build.
Capture the actual machine id/save-schema declaration. Preserve package-v1 markers.

An internal versioned, domain-separated snapshot includes protocol/game version,
exact executable identity, captured package/content identity, machine schema tuple
and the registry/configuration component fingerprints below. Compare components
with specific mismatch reasons. Matching values MUST NOT bypass external_network
or grant any writable-world/peer authority. No cryptographic attestation claim.
This internal representation does not adopt an on-wire schema or stable public API.

The registry part must cover actual mapping used for numeric lookup and configuration:

- Built-in registry names and names by numeric ID, with reverse lookup consistency.
- Runtime BlockRegistry states by actual state ID, complete names/properties and
  default-state mappings, checked against lookup round-trips.
- Synchronized registry entry ORDER (numeric IDs) and full data, not known-pack
  omission placeholders; check Dynamic lookup consistency with registry_packets(false).
- Actual configuration tags and enabled feature set. Preserve meaningful IDs/order;
  canonicalize only semantically unordered map/set traversal with explicit rules.

Do not substitute mod-content cache stamps/declarations for these actual tables.
Reuse existing native packet/NBT encoders when suitable; do not invent a generic
NBT codec or use lossy JSON normalization. Reject inconsistent/duplicate/unsupported
inputs instead of silently repairing them. Keep loaded native data unchanged.

Use checked arithmetic and the concrete preflight policy below. A positive pinned-
resource conflict must be reported for reviewed amendment; do not trim inputs or
silently change constants. No hard I/O timing guarantee.

### Concrete pre-conversion resource and ordering policy

Before cloning, sorting, converting JSON/Tag or building whole packets, walk the
existing borrowed loaded inputs with checked counters. NO early call to
registry_packets(false): it recursively allocates all JSON-derived NBT first.
Preflight lengths of names/json arrays before zip, and validate complete mapping
cardinalities/both directions, including extra reverse-map entries.

Limits fixed for this revision:

-128MiB aggregate fingerprint bytes INCLUDING domain tags, count/length framing,
 numeric IDs, names, properties and encoded data; checked counters never wrap.
-1,000,000 aggregate table entries: count every forward/reverse builtin entry,
 runtime state/default, dynamic entry, emitted entry, property pair, tag definition,
 tag member, feature, package and schema tuple (even repeated captured views).
-262,144 elements per collection/container and64 properties per runtime state.
-1,024 UTF-8 bytes per identifier; require valid supported nonempty names.
-32,767 UTF-16 code units per source string/key, AND65,535 maximum modified-UTF-8
 bytes per native string/key. Count actual surrogate/NUL expansion losslessly.
-64 source JSON/Tag nesting levels,500,000 source nodes per entry and8,000,000
 aggregate effective native nodes. Count each scalar/container/key-bearing data
 value; account conservatively for synthetic compounds/fields introduced by mixed
 list wrapping. Effective converted entry <=1,000,000 nodes and depth<=512.
-Each complete native NBT entry <=64MiB encoded bytes, <=1,000,000 nodes, depth<=512,
 matching actual outgoing writer DEFAULT limits. The128MiB aggregate does NOT
 enlarge an entry limit. Include root/header/end/list/compound/string/array framing
 and synthetic mixed-list wrappers in preflight size/depth/node upper bounds.

After preflight, convert/encode ONE entry at a time; reuse existing native JSON→NBT
semantics and FALLIBLE horizon_world::nbt::write_network under those default entry
limits. Propagate failure: protocol::Writer::nbt is forbidden for fingerprinting
because it silently emits End on encoding error. Check produced size against the
preflight upper bound and remaining aggregate before hashing/retaining it. No
whole unbounded packet or general new NBT codec. Borrowed Java strings stay
lossless; do not route native NBT through JSON or lossy UTF conversion.

Preserve builtin/state/synchronized-entry numeric order. Sort only unordered
registry-map keys/property maps and features; duplicate names/features refuse.
Preserve synchronized registry group order as currently emitted. For tags, sort
registry/tag names and numeric member IDs (membership is a set), rejecting duplicate
registry/tag names, duplicate members and invalid/out-of-range IDs. Do not mutate
actual emission or loaded inputs. Hash with versioned component-domain separation
and unambiguous checked fixed-width count/length framing; this is INTERNAL identity
encoding, not an adopted network format.

Dynamic map keys/names/order must exactly match the full emitted registry entries.
Existing None data means an omitted cached payload and is valid, but hash the FULL
emitted data rather than None. If Dynamic contains Some(Tag), its fallible native
encoding must exactly match that emitted full entry; otherwise refuse. Preflight
and charge both traversals/comparison buffers. No unknown/extra dynamic keys.

Snapshot is owned and stable after its source is later changed; a new snapshot
must reflect that change. This does NOT freeze actual future registry emission or
detect mutation automatically. H09b must separately bind its validated contract to
the same immutable inputs used for registry transmission/Joined and guard changes.
No peer/admission or safe-live-reload claim is possible from this packet alone.

## Required evidence and limits

Defining tests cover deterministic map insertion order, meaningful numeric-order
changes, block state/default/reverse lookup mismatch, dynamic data/tag/feature
changes, bounds/exhaustion/inconsistency, missing/multiple/extra package profiles,
package/build/content/schema mismatch, and frozen-value independence. Prove all
no-pair and selected-pair external-network behavior is unchanged even for equal
snapshots. No global allocator mutation or fabricated successful admission.

Actual-resource cases use the pinned resources and read-only prebuilt Rust pair,
capture independently constructed values, exercise selected current schema and
compare before/after source/table/fixture bytes. A declared digest or cache stamp
alone cannot be the positive expected value. Controls must change one real input
and observe the relevant fingerprint/comparison change. Do not compile guests or
invoke counter evaluators; retain old prebuilt provenance. Fresh scratch homes,
session environment removal; no owner saves/accounts/Prism writes.

Root owns all integration/format/Cargo/tests, serialized. Freeze exact names/hashes,
fresh independent full-source clearance before execution, relevant package/content/
snapshot tests, three feature lints/fmt/tree/scope, ordinary baseline and appropriate
broad checks, then different final review. This is new dormant data-only code:
fresh gameplay smoke is needed only if reviewed scope changes actual startup/
gameplay behavior; such a change first requires a design amendment. Keep all current
H08 acceptance/guards untouched and retain supported prior package refusal tests.

H09b actual negotiation and H10 server commit/tracking/replicas remain separate.
H11 renderer ownership may be designed alongside these; H12 needs H10 timestamped
committed input. No ordinary activation/recovery, G1 or public release follows.
