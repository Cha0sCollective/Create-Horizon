# CONTRAPTIONS-004 VIEW resync — source and runtime packet

Status: source candidate only. Base is root integration 007 `74344dfeeed38f3038ebb433d26695702bd63303`, with active Horizon `9c7f9b2215dfed497aba5522c44f38926628648d` and private Mixins `ac5d9e766380d575838c8250f2a7982281c1f44e`. The original disposable run `dedicated-79vzhrga` remains **HOLD**: server discovery/native move-remove refusal was seen, but no peer VIEW receipt, raw entity query, final cell comparison or cold reopen was reached. Preserve `G:\Create Rusty\.local\native-owner-foundation-005\dedicated-79vzhrga` unchanged. This source packet does not convert that run into a pass.

## Source boundary

The existing host calls `native_player_start_tracking` after it has collected a real tracking start and before attachment flush. A private Create injection validates the loaded block-display ID, event UUID, active owner record and saved VIEW against `AssemblyView::from_record`. It then uses the existing `Game::attachment_map(Holder::Entity(id))` and `Attachments::mark_sync(&VIEW)` path. Only the saved VIEW is queued. The canonical RECORD remains private; no owner is created and no source block, carrier position, schema, host pin or framework changes. A new tracker, including one after normal cold reopen, gets a fresh queued VIEW through the host's existing delta path. A rejected, retired, malformed, duplicate or mismatched owner is not queued.

The client probe also writes one bounded `peer_view_gate` observation about every five seconds while enabled under the exact disposable client profile. Each line records remote/overworld, exact UUID count and entity ID, display presence, attachment-map presence, VIEW decode/exactness, and canonical RECORD presence. The success probe remains a separate `peer_view_received` line. These are read-only observations and cannot substitute for actual VIEW receipt.

## Operations runtime sequence after integration and heavy lease

Prepare a **new** disposable dedicated server plus one actual `TeamPeer` session helper and binaries pinned to the integrated source containing this patch. Use the same exact fixture, `CREATE_HORIZON_OWNER_PROBE=1`, server/client cwd names and world-name gates as the first packet. Verify all native source-map files, both binary hashes, Create JAR and resources before launch and after final exit. The new helper must write the same `session-NN.json`, `exit-NN.json`, log and stop-request shape under a new mirror parent directly below `/mnt/g/Create Rusty/.local`. Do not reuse `dedicated-79vzhrga` or its saved world. Operations owns the helper, integration and shared build/game queue.

Attach the new driver to the already running session. Substitute the **new integrated** 40-character Git head, prepared packet root and mirror from its JSON:

```text
python3 '/mnt/g/Create Rusty/.local/workers/contraptions-004-view-sync/tools/packets/contraptions004_view_sync.py' '<mirror>/session-00.json' --phase initial --source-head '<new-integrated-head>' --packet-root '<new-packet-root>'
```

Require one server scan with one active assembly and refused native move/remove, a separate `server_view_sync_queued` line for UUID `[5,6,7,8]`, and a real peer `peer_view_received` line with exact VIEW and `canonical_record_present=false`. After session attachment, the driver stores available `peer_view_gate` samples in `owner-check-00/peer-gates.json` on success or any later failure, including a missing server line; inspect the last sample to distinguish a missing UUID/display from a missing map, decode error or projection mismatch. A send/queue log alone is not peer proof. Require the exact raw NBT marker, private RECORD, saved VIEW, unchanged position, three unchanged loaded fixture cells and normal server/peer exit codes.

Only after session 00 passes and exits, cold reopen that same new disposable world with index `01`, no second summon, and attach:

```text
python3 '/mnt/g/Create Rusty/.local/workers/contraptions-004-view-sync/tools/packets/contraptions004_view_sync.py' '<mirror>/session-01.json' --phase cold --source-head '<new-integrated-head>' --packet-root '<new-packet-root>'
```

Require cold server owner discovery/VIEW queue, actual peer VIEW receipt, retained private RECORD and position, fresh dispatched `/kill` and `/tp` feedback with the carrier unchanged, three world cells matching initial, and normal exits. On any HOLD, the driver requests bounded peer/server shutdown and records the original error, gate samples and exit outcome. Preserve raw logs/inputs and stop; do not repeat with a different fixture, run the game-test area-clear route, write world blocks, capture material or broaden the run without a concrete Operations decision. No runtime, Cargo, stage or Java check was run for this source packet.
