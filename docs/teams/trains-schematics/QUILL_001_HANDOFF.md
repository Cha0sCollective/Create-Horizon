# QUILL-001 source handoff

Status: bounded client-only source candidate, independent read-only review
CLEAR. Root integration, build and game qualification remain outstanding.

The isolated worker branch `codex/team-trains-schematics-quill-001` at
`eb8f0a3610ff3e247a00ba7d0fca8434ebf51b40` is clean. It starts at
accepted private source `55339ce3347b2ce667281e8a3e6318ec856144e4`
(`.11/cache14`); its two commits are `313f2d85136143bd0e2ba2aa37dfe280566d9d6f`
and `eb8f0a3610ff3e247a00ba7d0fca8434ebf51b40`. The team lane remains
separate so the project lead can compose shared files serially.

Changed files in the worker, all new relative to the accepted base:

- `native/create/mixins/create/engine/src/core/quill.rs`
- `native/create/mixins/create/quill_client.rs`
- `native/create/mixins/create/quill_key.rs`
- `native/create/mixins/create/quill_click.rs`
- `native/create/mixins/create/quill_hud.rs`
- `native/create/mixins/create/quill_frame.rs`
- `native/create/mixins/create/quill_unload.rs`
- `docs/teams/trains-schematics/QUILL_001_PACKET.md`

The pure core normalizes two corners and reports inclusive dimensions,
refuses an axis beyond 16 blocks while retaining the first corner, and
handles cancel, no target and a completed third click. The client module
claims only a selected main-hand `create:schematic_and_quill` right-click,
queues each right-mouse press edge through the existing native `key` hook,
and cancels repeated held-use callbacks without advancing a corner. Its HUD
shows corner coordinates and dimensions; the frame/unload seams forget state
when the item, screen or world exits. No original art bytes, cache, catalog,
manifest, schema, export, upload, server packet or world mutation is in the
worker commit.

The project lead must export the pure module, add the distinct Game module
and bind the five thin helpers to `key`, `interaction_key`, `hud`, `frame` and
`client_level_unload`. The shared content/catalog integration must declare a
one-stack item, placeholder model, name, creative entry, private local-JAR
art import and cache/version key. The worker packet records the exact Create
6.0.10 JAR/art hashes and Create 6.0.11 Java references. Keep the HUD seam
separate from Experience's `presentation.rs`.

Evidence: scoped Rust formatting succeeded; `git show --check` and
`git diff --check` were clean. A standalone dependency-free `rustc --test`
run passed all five pure selection tests, including held-use repetition.
The first strict read-only Sol High source review found a held-click repeat
bug (HOLD, 149.0 seconds); the repair delta received a fresh strict read-only
Sol High CLEAR with no findings (60.1 seconds). Both reviews made no writes
or builds. No shared Cargo/staging/linking/Java/game reservation was used.
Team phase records are under `.local/team-operations/trains-schematics/phases/`;
per-session token counters are unknown here.

On the project lead's host-wide lease, compile the composed helpers/core and
run focused tests. Then verify in one disposable integrated/dedicated
two-client UI milestone: hold the first click through repeated-use intervals,
release and click the second corner, quick separate clicks, 16/17-axis limit,
coordinates/size, cancel, item/screen/world reset, another held item's
interactions, unchanged blocks and item counts, and peer selection isolation.
The ordinary host hit range replaces Java's 75-block ray; midair targeting,
world-space outline, save/export/upload, placement and train/graph behavior
remain outside this packet. Context-sensitive replaceability is a remaining
in-game check because the client uses the host's raw replaceable flag.
