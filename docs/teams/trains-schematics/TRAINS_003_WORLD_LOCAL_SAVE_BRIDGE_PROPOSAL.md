# TRAINS-003 — bounded Quill to world-local Save bridge proposal

Status: read-only design for the owner's existing schematic authority decision
gate. No source, manifest, schema, pin, package, runtime or gameplay change is
authorized by this document. It does not alter QUILL-001/002 or delay their
project-lead runtime qualification.

## Decision to make before source work

The smallest useful Save result is a **world-local native structure template**
at `<world>/generated/create/structures/quill/<slug>.nbt`. It is GZIP NBT
written by pinned Horizon's vanilla template writer, under the server's
world, with no Create schematic item. It is not Java Create's client export,
an upload, a portable/external import, a placement operation or a complete
Create schematic workflow. If the owner wants the first Save to produce a
Create schematic item or interoperable Java `.nbt`, that is a different
format/ownership decision and this bridge should not be implemented as a
substitute.

Proposed authority profile: only a living Creative player with operator level
at least 2, holding `create:schematic_and_quill` in the selected main-hand
slot, may save. Keep QUILL-001's inclusive maximum of 16 blocks on each
axis; take blocks and air only, exclude entities, and reject any selected
block entity or structure void. This initial bridge stores a snapshot only;
it does not promise that dynamic Create blocks can later be placed safely.
The proposed visible name is 1..35 lower-case ASCII characters from
`[a-z0-9_-]`. The server builds the fixed `create:quill/<slug>` identifier;
the client never sends a namespace, path or filename. Refuse an existing
name, including any existing resource or non-regular filesystem entry, before
calling the writer. These are project policy choices, not Create Java 6.0.11
limits. The owner must approve this narrower Save profile and the collision
policy before implementation.

## Reusable host behavior and its boundary

Pinned Horizon `9c7f9b2215dfed497aba5522c44f38926628648d` in
`.local/evaluations/horizon-heating-refresh/source` has
`game/structure_blocks.rs:314-351` `Game::save_structure_area`, visible to
an injected `game` child module as `pub(super)`. It fills a box from the
current `Game.world` with air when `ignore_air=false`, captures block-entity
NBT when present (`:190-245`), and writes a GZIP NBT template in the world
directory (`:158-184`). Its name/path check only rejects empty, `.` and `..`
slash segments (`:48-53`); it does not check permission, held item,
dimension, world bounds, loaded chunks, size, existing names or symlinks.
`save_structure_area` inserts a template into its in-memory cache even if a
disk write fails (`:343-351`). The actual structure-block UI and packet
already require Creative plus permission >=2 for a guest
(`structure_blocks.rs:568-582`), but that path requires a placed structure
block in Save mode (`:586-628`) and cannot be repurposed as a Quill action.

The output is Horizon's native `StructureTemplate` palette/block/entity/size
and DataVersion tree (`horizon-world/src/worldgen/structure/template.rs:179-215`),
not Java Create's schematic item/file contract. Pinned Create Java
`SchematicAndQuillHandler.java:93-127` opens a name prompt on a third click;
`SchematicPromptScreen.java:45-74,95-110` allows a 35-character name and
Save/Cancel; `FilesHelper.java:27-42` slugifies and adds a suffix to avoid a
client-file collision. `SchematicExport.java:38-63` writes a separate client
file. Its instant server path re-captures server blocks after metadata upload
(`ServerSchematicLoader.java:269-309`), while Schematic Table upload has a
different validation path. Neither provides this proposed permission policy.

## Smallest Create-facing path on the pinned host

1. Extend the already reviewed client-only Quill state only after its runtime
   milestone. A third ordinary click on a completed selection opens a
   35-character name prompt using Create's existing native mod-screen owner
   and screen text/key hooks. Copy normalized bounds into transient prompt
   state **before** opening the screen: QUILL-001 intentionally clears its
   gameplay selection when a screen opens. Cancel discards the prompt. Save
   builds the fixed lower-case slug and six decimal coordinates; no block
   data, NBT, path, namespace or client file is sent.
2. Prefer Horizon's existing server-command route over a custom untrusted
   request packet. Register an operator-level-2 command through
   `native_register_commands` / `NativeCommand::op` with a fixed usage such
   as `createquill save <name:word> <x1:integer> <y1:integer> <z1:integer>
   <x2:integer> <y2:integer> <z2:integer>`. The prompt invokes
   `Game::click_command` (`game/chat.rs:1181-1192`), which uses
   `send_unattended_command` (`game/remote_chat.rs:298-331`) to send a normal
   player command with no signed message argument. The host filters it by
   command level, then calls `native_perform_command` with the acting player's
   state swapped in (`game/native_hooks.rs:3835-3857`,
   `game/native_server.rs:975-1007`, `game/lan_actions.rs:3274-3300`). The
   handler parses the raw words strictly and independently rechecks
   `player.permission >= 2`, `player.game_mode == 1`, live health, the selected
   main-hand item through `native_held`, client-supplied coordinate bounds,
   and a fixed safe slug. Manual command use must pass the same checks.
3. Do not capture in the command callback unless the current Game level is
   proven to be the actor's. Put a small transient request keyed by player,
   world session and dimension in a bounded server queue. In the matching
   `level_tick_post` hook, `Game.world` is the named level after `enter_level`
   (`game/levels.rs:442-479`). Recheck that the player is still placed,
   living, Creative/op >=2 and in that dimension using a fresh
   `native_server_players` snapshot. Resolve that UUID's `entity_id` through
   `native_entity`, then `native_living` and
   `native_living_hand_item(living, Hand::Main)` to recheck the current
   selected Quill. The entity/living handle path reads the guest's live
   selected slot and fails closed for a missing or stale handle
   (`game/native_server.rs:202-225`, `game/native_entity_refs.rs:13-43`,
   `game/native_living_refs.rs:65-114`).
   A change of item, dimension or world session drops the request. Limit to
   one pending request per player and clear it on world close.
4. Normalize in i64 with checked arithmetic; reject a zero side, any axis
   above 16, or out-of-range coordinates. Require x/z in
   `[-30_000_000,30_000_000)`, y inside the current level height, and every
   covered chunk loaded; the host's checked read boundary is
   `game/native_world_read.rs:121-139`. Preflight all at most 4096 cells
   against block entities and structure void. Confirm the destination is
   unused and under the fixed world-local subtree. Then call
   `save_structure_area("create:quill/<slug>", min, size, true, author,
   true, false)`: ignore entities, write to disk, include air. Report success
   only on its true return, with the resolved world-local name; on failure,
   report failure to the actor through `native_tell`/system chat. Do not
   create a schematic item or mutate world cells.

The command route uses no new host API, persistent owner or schematic format.
A custom `native_net_send_to_server` / `net_from_client` request would carry
untrusted bytes, has no acting player, and would still need the same queue,
permission and selected-item checks. It offers no smaller authority path.

## Limits that must be accepted or repaired

The pinned writer uses direct `std::fs::write`, so a preflight collision
check cannot make no-clobber atomic against concurrent filesystem changes;
pre-existing symlinks under `generated/` can also redirect a write. A hard
guarantee of no overwrite and path containment needs a **bounded host writer
repair** (exclusive/atomic file creation with symlink-safe containment),
separately reviewed before Save is implemented. The writer also caches the
new template after a failed disk write, so failure must never be shown as a
saved file; an exact rollback/verified-file policy belongs in that narrow
repair. Recommend requiring that narrow private writer repair before the
first Save; the owner should decide this at the authority gate. The public
native player snapshot and entity/living handles compose a live held-item
check by UUID. There is no dimension-explicit native template save; the
proposed private injected bridge calls the pinned internal Game writer in
the matching level hook.

Qualification after authorization should cover permission/Survival refusal,
wrong or changed held item, malformed and traversal names, duplicate name,
dimension change, unloaded edge, 16/17 boundary, BE/structure-void refusal,
air inclusion, actual file path and GZIP/template readback, disk failure,
normal reopen, no world/item mutation and peer isolation in one disposable
integrated/dedicated two-client milestone. No external file ingestion,
placement, upload, Schematicannon, train graph, cross-world import or public
release follows from this proposal.
