# QUILL-003 frozen005 UI milestone driver

Status: packet-local preparation only. No game process, heavy build or runtime
lease was used. Root supplies final frozen source/binary/session metadata and
runs the disposable integrated and dedicated sessions. QUILL-001/002 source
and content are already composed into the project lead's source-only005
candidate; this packet does not change Quill behavior or implement Save.

## Input path that the milestone must use

Pinned Horizon's `horizon-ctl interact` calls `agent_mouse` and only
`play_mouse` (`game/agent.rs:889-893,1400-1434`). The native Quill press
counter listens to `native_key("key.mouse.right", pressed)`, which enters
through `extras_mouse` / `native_mouse_event`
(`game/extras.rs:38-49`, `game/native_hooks.rs:10067-10070`). The actual
`horizon-ctl mouse right down|up|click` path calls **both** `extras_mouse`
and `play_mouse` (`game/control.rs:700-733`). Thus generic `interact` may
perform block use without recording a Quill press. Use `look-at block ...
north` for aiming and explicit `mouse` commands for every Quill action. The
driver asserts the crosshair's target block and horizontal face before each
selection, so a vertical-face adjacent-cell rule cannot move the corner.

`tools/quill_ui_milestone.py` in the team lane takes root's actual session
JSON and matching absolute `horizon-ctl` path, verifies both frozen binary
hashes from session metadata and records command transcripts/PNG frames under
the session mirror's `quill/` directory by default. It creates no launcher
or game process. Example after root grants and starts its frozen lease:

```text
python3 tools/quill_ui_milestone.py /path/to/session-00.json /path/to/horizon-ctl full --origin X Y Z
```

Choose `X Y Z` in a disposable Creative Overworld where the whole fixture is
safe to replace. The script fills a 24-by-8 stone floor/air fixture, places
four stone targets at x offsets 0, 1, 15 and 16 and a lever outside the
selection line. It gives the actor one Quill and one stick; on a dedicated
session, the peer also gets one Quill. No other world or inventory writes are
expected after setup except toggling that lever. The root should inspect the
initial session metadata and ordinary server/client startup before invoking
the driver.

The `full` case performs these checks in one session:

1. Hold one real right mouse press for 18 ticks; the rendered first corner
   must remain first until release. Capture the first-corner HUD frame.
2. Aim from x+16 to x+0: capture the 17-block red preview, click and capture
   the refusal while first corner remains. Aim to x+1: capture the green
   16-block preview, click and capture reversed inclusive first/second
   coordinates and size. Third click shows the explicit no-Save message.
3. Shift plus real right click clears the selection. Two quick separate
   press/release edges at one target produce a 1 x 1 x 1 complete selection.
4. A switch to stick and back, and an inventory-screen open/close, each
   remove the prior selection. Stick use toggles an ordinary lever through
   the same physical mouse path.
5. Before and after Quill use, compare the exact selected-line block list
   and actor item counts. On dedicated, capture actor and peer HUD frames
   before and after their independent first corners and an actor cancel.
6. Leave a first corner selected, capture its HUD, then let root close the
   world/session normally. Reopen that same disposable run with a new
   session index and run `reopen` with the same origin; the new HUD must show
   the initial first-corner prompt, and blocks/items must match the prior
   receipt.

The driver checks world target, input results, lever state, block lists,
item counts and screenshot existence. **Actual HUD wording, coordinates,
colors and absence after item switch require visual inspection of the PNGs.**
Each receipt lists the expected text beside every frame and keeps
`inspection: PENDING` until the project lead records a manual comparison.
An automated success status is therefore not a gameplay PASS. The same
visual check must confirm the peer's own first corner survives the actor's
cancel and that the actor never sees the peer's corner.

After the `full` case, root should save/exit the actor and peer normally,
record process exit codes, then restart the same disposable run with the
frozen binaries. Invoke:

```text
python3 tools/quill_ui_milestone.py /path/to/new-session.json /path/to/horizon-ctl reopen --origin X Y Z
```

Keep raw receipts and screenshots outside Git. Report actual source hash,
binary hashes, session indices, command receipt paths, visual frame review,
process exits and any failure without recasting missing observations as a
pass. A bounded runtime milestone does not qualify Save, upload, placement,
Schematicannon, Java/client-file interoperability or train behavior.
