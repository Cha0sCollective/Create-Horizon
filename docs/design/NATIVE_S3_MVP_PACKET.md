# S3 — remaining five-type player workflow

The owner authorized progression through S6 on November4 until the next break.
Native S1/S2 are complete; S2 is private `c0905ca68d122fcaec31e4c0e6cba197bb73f3ff`.
This packet applies the already authorized S3 outcome; it does not expand types,
terrain, architecture or previously deferred guarantees. Review this packet before
source changes. Root alone writes/builds/runs; bounded read-only reviewers only.

## Reuse audit

Port main remains `10cb93e6a13b837edc2293d3669ee6cbdb4d5863`: only five documentation
files differ from imported16c5d454. Experimental `stage/parity-06` remains
`64b35ad3da5e72dd21b5979ec9c6ca0ccfd795a2`. It has additional motor settings,
processing and launcher work with a different host pin; assess individual fixes
when an observed gap requires them. It is not a compatible wholesale replacement.
Official main refreshed to `6319cb701eb0d01242110a23668eda0a56e97e16`; the latest
change is player hit outlines, unrelated to this packet. Retain tested hostfc021/
Mixinsa1e133/API359 unless a concrete requirement proves otherwise.

Existing port services: motor settings board/wheel/network request, ordinary
five-type placement, world-item millstone intake and empty-hand extraction.
Existing Horizon services: creative inventory, ordinary drop/interaction, rendered
clients, native compilation/content loading, mod-list/version comparison and clear
missing/different-mod disconnects, normal saves, UI and resource preparation.
Existing upstream tools include source/native package and launcher build helpers.
Use them where compatible; no replacement host or mod-negotiation framework.

## Remaining deliverables

1. Qualify a usable connected five-type chain through ordinary placement, actual
   motor controls (speed and direction), player Q-drop cobblestone intake and
   empty-hand gravel collection. World/profile preparation and stock provisioning
   may use fixture commands; the player actions under test must use ordinary
   inventory/placement/UI/drop/interaction paths, with no NBT/summon substitute.
   Test integrated and dedicated/two matching rendered clients. Reuse unchanged
   S1 custody and S2 arbitration/reopen evidence rather than repeat their batteries.
2. Qualify existing missing/wrong-package handling with fresh isolated client
   homes. Matching play must work; absent/disabled Create and a different declared
   package version must get the existing clear refusal before play. Give our
   private revision an explicit version in existing native/content metadata,
   preserving `create:` IDs and upstream provenance. This uses the host's existing
   version exchange, not a new protocol. Do not claim detection of arbitrary
   modified native code that falsely declares an identical version.
3. Document and verify one reproducible private Linux source-build/install/play
   route with exact compatible pins, owned generated output, isolated instance
   homes, fixed Creative test terrain and spawn_mobs=false. Reuse existing build,
   content and launcher paths; improve a small developer helper only where the
   route needs it. Keep the supplied Create JAR and prepared Minecraft resources
   local; install artifacts/binaries/caches/worlds stay out of Git. No public
   release, launcher registry publication or platform portability claim.

## Execution and limits

Start by observing existing controls/placement and the private install options;
fix concrete failures only. Preserve exact S2 binaries before any cache reuse.
Changes are private mod behavior, existing package metadata, small developer
launch/build helpers or a short guide. A genuine host/API/architecture gap beyond
this packet requires input; do not build a general framework.

Use focused checks for changed code and one combined affected S3 gameplay
milestone. Setup/driver failures remain incomplete and preserved; fix only their
delta and repeat affected work. Correlate server replies; separate authoritative
queries from actual client inventory/block/render observations. Independent Sol
High review covers substantive patches and final bounded completion. Keep raw
runs local and update CURRENT_WORK, results and the private checkpoint.

Supported profile remains the five types, mob-free Creative TestTerrain Overworld,
integrated and dedicated/two matching rendered clients. Open to LAN, Java/world
import, arbitrary generated worlds/modpacks, all imported machinery, performance,
public distribution and custom interrupted-crash/replay guarantees remain outside
S3. Stage4 may introduce its separately authorized survival/ordinary-terrain packet
after S3 completion; do not silently widen this packet.
