# H09b.1 frozen inputs and bounded wire data

PROPOSED, NOT RELEASED. H09a must receive final acceptance before this successor
source release; current H09a qualification is pending. This packet remains dormant
DATA ONLY. No live startup, configuration sender, hub, session or admission caller
uses these values yet. Existing external/world/package/build refusal stays intact.

## Exactly four proposed source paths

Relative crates/horizon-client/src:

1. NEW lan/peer_contract.rs: bounded unauthenticated wire claims, codec, mismatch
   comparison and defining unit controls.
2. lan/mod.rs: module declaration only.
3. game/mod_packages.rs: binary-private frozen holder, capture and wire projection.
4. game/peer_contract_snapshot_tests.rs: frozen owner/actual captured-input controls.

No lan/data.rs, transport/startup/world/activation/SDK/dependency/H11/H08 edits.
No receipt, nonce/session identity, admission Boolean or permission capability.
Concrete extra path/API dependency or positive bound/identifier conflict requires
reviewed amendment, no silent limits increase or fallback.

## Trust and ownership

Binary-private FrozenHostInputs owns retained Packages, exact Arc<SyncData>, exact
immutable &'static BlockRegistry, captured PeerSnapshot and projected WireContract.
Private fields; no Clone/Default/Deserialize/mutable getters/into_parts/public
constructor/host-construction trait. Packages::freeze_peer_inputs(sync:Arc<SyncData>,
blocks:&'static BlockRegistry) returns Result<Option<FrozenHostInputs>,String>.

It first invokes accepted self.peer_snapshot(&sync,blocks). None stays None;
unsupported profiles/errors propagate. Only AFTER successful bounded capture clone
Packages (one bounded identity/build map plus shared captured Catalog), move SAME
Arc<SyncData>, keep exact block pointer and convert the captured snapshot to wire
data. Read-only accessors expose wire/sync/blocks, not owner replacement. Retaining
already-loaded Arc data does not copy its unrelated commands/recipes/advancements.

No input parameter accepts a build string, package digest, asserted snapshot,
WireContract/decoded claim, or mutable source path. Local build comes from retained
startup current_exe SHA256, package from retained actual Catalog/PreparedBundle.
Projection checks validated hex into raw32 arrays. The decoder cannot construct
the holder. This describes/retains supplied actual objects, not a live network
freeze or proof that a current sender/hub already uses them. Later H09b wiring
must establish that identity and liveness independently.

Library WireContract is explicitly unauthenticated DATA. Private bounded fields;
from_claims(WireClaims borrowed strings/fixed data), decode(bytes), encode(),
compare(other)->component mismatch enum. Syntax/equality grants NOTHING. A caller
may make false claims; no constructor or equality reaches FrozenHostInputs or
ValidatedPeer/network/world authority. Keep this warning beside the type. No
cross-crate trusted-host constructor or generic protocol/normalization framework.

## Proposed wire v1

Fixed sequential big-endian encoding; separate reviewed format, not serialization
of H09a internal hashing bytes/domain or accidental prior protocol adoption:

| Field | Encoding |
| --- | --- |
| Magic | exact8bytes HZPCONTR |
| Wire version | u16=1 |
| Profile | u16=1, onepair/paired-preview-1/current one-machine schema |
| Snapshot semantics | u16=1, accepted H09a component algorithms |
| Package identity scheme | u16=1, paired-folder-sha256-1 |
| Minecraft protocol | i32 |
| Game version | u16 UTF8 byte length + nonempty <=1024bytes |
| Startup executable SHA256 | raw32bytes |
| Package ID | u16 byte length + [a-z0-9_]{1,64} ASCII owner ID |
| Captured package/content identity | raw32bytes |
| Machine ID | u16 byte length + nonempty <=1024bytes |
| Machine schema | u32=1 |
| Registry components | five raw32byte digests: builtin, blocks/defaults, synchronized, tags, features |

Fixed overhead254bytes; grammar-constrained maximum254+1024+64+1024=2366bytes. Additional complete
input/output ceiling8192bytes includes all magic/framing; it authorizes no padding
or extension. Protocol/game can be well-formed incompatible values; compare reports
the component mismatch. Package owner ID is exactly ASCII [a-z0-9_]{1,64}, matching Manifest::parse; machine ID
must be valid namespaced ID owned by that package. No case/Unicode/separator repair.
Game-version syntax must admit the pinned actual version without normalization.

Check whole input ceiling BEFORE decode. Borrowed checked offsets/lengths, reject
every truncation, invalidUTF8/identifier, zero/oversized string, unknown version/
profile/semantics/scheme, unsupported schema and trailing data. Validate ALL fields
before allocating at most three owned strings. Encode validates complete checked
length first, allocates bounded buffer once and emits canonical bytes. Digests
have no alternate hex/algorithm representation. No lists/extensions/maps/NBT/tables/
executable bytes/content/path fields. Future versions require a separately reviewed
branch, no downgrade/best-effort/automatic upgrade. Codec ceiling does NOT claim
to limit earlier network framing/decompression allocation or total I/O time.

## Evidence and stop condition

Fresh independent design acceptance then accepted H09a parent/private checkpoint
before source release. Root alone formats/builds/tests, serial across all worktrees.
Freeze exact sources/names; fresh independent source clearance before execution;
appropriate three-feature lint/fmt/scope/public-tree/ordinary baseline/regression
checks and different final review. No gameplay/network smoke for dormant data only;
any actual startup/transport change needs its own design packet first.

Codec controls: literal independently expected golden bytes/offsets/length, decode
field expectations, deterministic roundtrip, every truncated prefix, whole8KiB
rejection, oversized declarations, invalidUTF8/IDs, unknown versions/codes/schema,
trailing bytes and EACH component mismatch. Self-roundtrip alone is insufficient.

Actual-resource controls reuse pinned resources/read-only prebuilt Rust pair, no
guest compiler/counter evaluator/socket. Two independent captures/materializations
freeze equal contracts. Independently hash current frozen test executable and
compare its retained startup bytes. Assert exact Arc<Catalog>/Arc<SyncData>/block
pointer identity even after caller handles drop; Arc::get_mut cannot mutate shared
retained SyncData. Fresh changed tables/captured scratch content produce relevant
wire mismatch; old holder stays stable after source edits. Existing real data/tag/
feature/default/order controls reach wire mismatch or capture refusal. Decoder-
created equal claims still cannot bypass external_network/world pins. No-pair None
avoids registry capture; existing unsupported profiles refuse. Synthetic test-only
build edits remain labeled synthetic, not two actual executable captures.

Retain exact fixture provenance, isolated scratch homes and before/after file
evidence. No owner saves/Prism/session environment. Preserve failures/source before
fixes; no generic artifact framework. Completion claims bounded wire DATA derived
from/retained alongside local captured inputs ONLY. Live input freeze, external
agreement, configuration validation, receipt/admission, attestation, H10/G1 and
public release remain open.

DESIGN ACCEPTED at revisedproposalSHA1a61082e1d9f6c03717d4b59798d02cf353c2faaeed41c1e5916f70a6ace5854 by fresh independent read-only Sol High follow-up. Original103d7698 withheld solely for package-owner grammar/max-length conflict. No source release until H09a FINAL acceptance/private checkpoint; current H09a LOCAL3ac2d4c targetedchecksPASS/broadactive. Exact-source review/qualification/final review remain required. No transport or admission acceptance.
