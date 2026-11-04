# Durable OWN archive uniqueness correction

PROPOSED amendment within s1own1 and the SAME eight existing source paths.
Accepted base design fbf5ca4ff235682d8c678a3db426ed14a0194a9fffa2efd8a2dba634fc8f2706;
held raw source1 map123254bb385b8db15d8714afaf9fd798a1ea7b7fd184ad5d97f44463abdda6b2.
Fresh source review HOLD: two new UUIDs can bind the same empty caller-selected
archive, then mix epochs and make the older world's complete image inaccessible.

Change explicit NewOwnedRoute to accept a canonical separate ARCHIVE PARENT.
Only actual successful new native identity/namespace construction derives its
archive as parent/<actual logical UUID>. Create that fresh child with create_dir
(no adoption/overwrite), sync child and parent before immutable OWN record/exposure.
Record the canonical actual child in the existing OWN association; on every strict
record read/recheck require the archive basename equal its actual logical UUID.
Thus two different world UUIDs cannot share the same canonical archive, including
across distinct configured namespaces. Existing per-UUID/native exclusion handles
same logical identity; no archive registry, pointer chain, journal or new format.
Existing parents may contain other UUID archives; do not demand an empty parent.
Original process-owned routes and all existing image/frame formats remain strict.
Any refusal stays with the original preparation/native/namespace owner; no cleanup.

Use the SAME two LOCAL worlds/serial A-B child fixture. In A construct BOTH worlds
before either closes, using one shared archive parent; assert actual OWN children
are distinct UUID-derived paths, then proceed with Unplaced/Claimed complete close.
B requests still derive authoritative archive exclusively from actual acquired OWN;
any decoded archive path in the test is an observation for expected-result checks.
Preserve the actual shared parent under the unplaced fixture's archive directory.
No extra worlds, client modes, supported profiles, ingress or general framework.

Close source-review coverage gaps with focused actual owned recovery refusals for
altered strict association, profile, native UUID and PIN digest, retaining negative
input bytes and checked original closure. On actual recovered Unplaced World,
verify wrong-World, duplicate recovery admission and post-placement stale unused
admission refusal without cloning/extracting/recreating its session grant. Reuse
existing LOCAL original partial-preparation cancellation/Retry regression unchanged.

Root coordinator adopts only after independent read-only Sol High design acceptance,
private amendment checkpoint and exact correction gate. Source1/tools remain frozen;
no s1own1 format/compiler/runtime executed. No additional owner scope decision needed:
this repairs the already-required one-world/one-stable-archive invariant.
