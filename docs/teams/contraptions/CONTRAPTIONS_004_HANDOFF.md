# Contraptions 004 — inert entity-owner foundation

Status: isolated source candidate for independent review and root-owned validation. This packet implements the owner data and guard seam selected in `CONTRAPTIONS_003_ENTITY_OWNER_DESIGN.md`; it does not create a carrier, remove a world block, run a bearing, or claim a playable assembly.

## Exact source

Base private revision: `55339ce3347b2ce667281e8a3e6318ec856144e4` (.11/cache14). Pinned Horizon: `9c7f9b2215dfed497aba5522c44f38926628648d`. Pinned Mixins: `e8cd0aa38cffef49cb2e088ef716baa107148c02`. Java Create reference: `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`. All code in this packet is new private source under `native/create/mixins/create/`; no host, Mixins, Java reference, piston, or shared root-owned operations file was edited.

The `create_assembly_owner` module declares an independent `AssemblyId`, a carrier UUID field, a versioned canonical `AssemblyRecord`, and a smaller synced `AssemblyView`. Only two state-only inert cells are accepted by the first codec: front slime and one adjacent oak log. Each cell retains its original source state and exact planned target state, so a rotated log's axis is verified at landing. Unknown versions, fields, payloads, duplicate cells, invalid poses, malformed properties and inconsistent verified progress are rejected. The record is `sync = false`; the view is `sync = true`. A marker in the carrier's raw NBT plus an exact carrier UUID and decoded record identify a loaded owner; a malformed candidate is treated as guarded, not ordinary.

The per-level post-tick hook invokes a read-only scan of loaded `block_display` carriers. It rejects duplicate assembly IDs, overlapping cell claims, out-of-chunk cells, unloaded positions and a carrier whose saved dimension disagrees with the level. It currently discards the scan result: there is no transient index or diagnostic output, and no state progression. Five private cancellable HEAD guards target `/kill`, `/tp`, native remove, native move and game-test area discard for the active `OtherEntity` carrier. The last guard cancels the entire area discard if it includes an active owner; this can stall test cleanup. Normal chunk unload and saved level clear are outside those targets. The guard functions use immutable reads; a fully settled record permits later ordinary removal. These are source-level Mixins claims until transformed output and runtime behavior are checked.

## Root-owned composition and proof

Integrate the candidate serially against the current private branch after reviewing the exact diff. Keep the pinned host and Mixins revisions unchanged. Run the shared queue's focused Mixins stage/compile and the module's codec tests, then inspect generated injection output for all five cancellable HEADs and the level-tick hook. A later synthetic, inert runtime fixture must provision a real saved display carrier and valid record/view before cold-reopen, dedicated-peer sync and `/kill`, `/tp`, native remove/move and game-test discard refusal can be observed. This packet contains no provisioning call or renderer; the view's transport policy is encoded but peer delivery is unobserved.

Before any source-block capture, verify every explicit deletion and relocation path at runtime, normal unload/reopen, duplicate/corrupt-owner refusal and owner retention on obstruction. A command may still print success after its inner carrier mutation is cancelled. The native display-transform API can alter visuals but does not move the carrier position. No source material is under this packet's custody, so a passing codec or guard unit test cannot satisfy the live bearing gate. No public release or remote push is authorized here.

## Local checks

Pinned `rustfmt` ran on all six new Rust files. `git diff --check` passed. The worker record and guard commits were independently composed from the accepted base. No Cargo, Mixins stage, game, cold-reopen or peer test has run in this team lane; the project lead owns that shared heavy queue.
