# H02c: immutable package identity before activation

Implemented and reviewed locally after the owner explicitly resumed work beyond
the earlier cutoff; see ../reference/H02_SNAPSHOT_RESULTS.md for exact checks and
limits. This document retains the bounded implementation scope after H02b. The
complete loader and S1 remain open. The earlier run's baseline evidence remains
in ../reference/BASELINE_D0.md.

## Concrete seam

At local engine commit `2f1041`, `paired::PreparedBundle` captures descriptor/code
bytes and parsed content declarations, but `Content::source` is still a directory
path. `Content::reader` opens resource files later. A successful typed code check
therefore cannot identify the content that will actually be registered.

Implement a narrow immutable paired-folder snapshot first, keeping it behind the
experimental feature. Capture descriptor, both components and accepted content
bytes once, applying existing content path/namespace/file/total limits. Define a
versioned deterministic digest over relative paths, lengths and bytes, with
unambiguous framing; include every gameplay-relevant declaration and resource.
Do not hash just parsed TOML or code, and do not reread the original folder when
using the snapshot. Keep raw content/code out of Git.

Reconcile the existing `Content` reader rather than inventing a second content
validator. Prefer a small immutable reader/source variant usable by existing
registration code; preserve legacy folder/zip behavior. Review memory cost against
the current 512 MiB content limit before choosing an in-memory representation.
An immutable staged-file design needs an explicit ownership/lifetime story and
must not reuse mutable user paths as though a digest made them immutable.

Acceptance checks: unchanged inputs produce the same identity regardless of
directory enumeration order; changing descriptor, either component, a declaration,
or a resource changes it; post-capture edits/removals do not alter snapshot reads;
bad paths, symlinks, size/count overflow and namespace collisions fail closed.
Use the exact captured bytes for later type preflight and registration. Obtain a
fresh read-only independent review of the substantive change before committing.

Stop at a reusable immutable snapshot and authored tests. Do not wire discovery,
world loading, a network handshake or executable activation into this packet.

## Subsequent activation work and known traps

- `manifest::find` and `Package::read` currently select a single legacy manifest
  and code module. `game/mods.rs::content_root` also uses that legacy path before
  resources load. Discovery must make one explicit package-kind decision and
  register a paired content owner once alongside legacy owners.
- `runtime::Entry` has one `Option<Running>`. Giving each half a separate installed
  mod row duplicates ownership; replacing it with one arbitrary half loses the
  other side. Design one package status with separate side execution state.
- `game/mods.rs::mod_runs` selects client/server for the default split-thread
  paths, and both for the in-frame fallback. The latter requires two separate
  Stores. Unsupported roles must fail explicitly before admission.
- The Mods page currently derives permission descriptions from one manifest.
  Display both executable capability sets accurately; the content owner's
  `registries` permission must not hide server world/player access.
- Keep per-side settings/storage separate without turning guest storage into
  authoritative world inventory. Preserve existing fuel, memory and storage quotas.
- Validate optional UI/browser exports before their activation too. H02b validates
  the required base client/server callback interfaces only.
- H09 identity negotiation must precede registry-dependent packets and editable
  world admission. Equal hashes are package matching, not peer authentication.
- A saved counter machine still depends on lifecycle, transactions and coherent
  persistence packets. An inventory sidecar cannot substitute for that work.

No upstream draft is merged by this plan, no new API version is assigned, and no
public publication is authorized. G1/G2/G3 remain open.
