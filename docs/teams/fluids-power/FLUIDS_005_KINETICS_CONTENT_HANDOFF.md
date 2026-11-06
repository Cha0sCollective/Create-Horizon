# FLUIDS-005 pump kinetics and selected content handoff

**Worker scope:** isolated `codex/worker-fluids-005-kinetics-content`, based on
accepted private source `55339ce3347b2ce667281e8a3e6318ec856144e4`.
This is an uncomposed source proposal. No host, shared pin, catalog, manifest,
runtime hook, generated bundle index or accepted source has changed here.

## Kinetic contract

`Kind::Pump` is a small cog, contributes 4 SU per absolute RPM to network
stress and has no shaft port on any face. The runtime node for
`create:mechanical_pump` must set `Node.axis` from its six-value `facing`
property. The node's fluid front/back are unrelated to kinetic shaft ports.
The focused tests drive an X-axis Pump from a side small cog at both RPM signs,
check 64 stress at 16 RPM, reject a mismatched cog axis, and refuse an axial
motor or shaft connection. They do not run until the shared build lease.

Pinned Java evidence: `PumpBlock.java:39-56` implements `ICogWheel` and sets
rotation axis from `FACING`; `KineticBlock.java:51-53` supplies no shaft port;
`AllBlocks.java:929-938` sets stress impact 4. These observations come from the
clean `references/create` revision `7ff2760e` (Create 6.0.11).

## Content contract

The new `create:mechanical_pump` block declaration has only `facing` with all
six direction names, default east. The new `create:fluid_pipe` has only
`axis=x|y|z`, default x, and one straight tube shape along that axis. The
finite line places the Pipe at Pump + `facing`, with matching axis. Neither
declaration has branches, connection toggles, per-face flow data, animation
NBT or a third fluid-volume owner. Model, item and loot JSON are supplied for
these two IDs. The provisional geometry is authored here, using only vanilla
andesite, copper and iron texture identifiers. It copies no original Create
art or binary. Its visual accuracy, waterlogging and pipe flow display remain
unqualified. The separate Create 6.0.10 source at `sources/create` revision
`ac0c444d9828da3453ae8cc65338e8de063286fb` was checked for the two IDs
and loot item identities. Its item-model parents differ from these authored
provisional models; the new JSON follows the accepted native content format.
No original resource was imported wholesale.

`native/create/tools/fluid_pump_content.py` reproducibly generates only the
12 new pump/pipe JSON files. This worker did not alter existing language,
creative-tab, catalog or bundle-index files.

## Shared composition proposed for Operations

1. Add `mechanical_pump` and `fluid_pipe` to `FLUIDS` in
   `native/create/tools/catalog.py`, and include the authored
   `mechanical_pump/block` and `fluid_pipe/straight` model names in `PARTS`
   if the shared renderer catalog requires them. The generic `BLOCKS` loop
   then includes both items in the creative tab. **Compose the asset-import
   rule in the same change:** `assets/src/lib.rs:537-550` imports original
   blockstates and item models for every `BLOCKS` ID. The pinned 6.0.10
   `fluid_pipe.json` is multipart over six connection booleans, which cannot
   match this packet's axis-only native state. Do not import that original
   Pipe blockstate unchanged. Either explicitly keep the authored bundled
   axis variants and item model by excluding Pipe from those original-art
   imports, or add a bounded axis-compatible selected mapping as the existing
   Track import does. This is a shared composition decision, not a claim that
   catalog registration alone yields a matching Pipe model.
2. In `native/create/tools/content.py`, import
   `fluid_pump_content.generate` and call it near the other selected feature
   generators **after** the generic `BLOCKS` loop and before the final
   `files = { ... CONTENT.rglob(...) }` bundle-index capture. That call
   overrides generic pump/pipe JSON with the restricted declarations here.
   Add `block.create.mechanical_pump = Mechanical Pump` and
   `block.create.fluid_pipe = Fluid Pipe` in the generated English language
   file, with suitable Russian labels in the Russian file, before capture.
   Rebuild generated `assets/src/catalog.rs` and `data/content.json` in the
   serial shared composition; do not hand-edit those generated outputs.
3. Runtime/placement composition must recognize the two IDs, set Pump
   `Node.axis` from `facing`, assign `Kind::Pump`, and keep the Pipe out of
   the kinetic graph. Targeted Pump wrench behavior flips `facing` to its
   opposite, matching pinned Java `PumpBlock.java:47-50`. Restrict placement
   and use to the declared four-block profile; no general pipe network is
   implied. Operations owns the shared manifest/catalog/pin reconciliation.

## Validation and limits

All 12 new JSON files parsed with Node; `git diff --check` passed. The
local Windows Python alias was inaccessible, so the standalone generator
was not executed here; its authored outputs were written from the same
data structures and need a regeneration diff check during shared composition.
No Cargo, stage, Java or game process ran. The added Rust tests are meaningful
but unexecuted pending the one shared heavy lease. There is no gameplay or
save/reopen claim from this worker.
