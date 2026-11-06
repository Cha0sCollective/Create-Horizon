# QUILL-003 runtime-r2 status — visual HOLD

Status: **HOLD despite four automated green receipts**. All runtime
processes exited normally and the exclusive heavy lease was released before
evidence review. Root must inspect the raw frames directly; no complete
Quill gameplay PASS is claimed.

Fresh disposable run IDs were `integrated-9vkqdlrp` and
`dedicated-8_3c4gcp`, each with full session 00 and cold-reopen session 01.
Their raw driver receipts are under the matching
`.local/native-team-runtime-005/<run>/quill/` folder. The full receipts
contain 14 integrated and 19 dedicated HUD frames; cold reopen contains one
and two frames respectively. Every driver command/state assertion returned
`AUTOMATED_CHECKS_PASS_HUD_INSPECTION_PENDING`. The two integrated actor
exits plus two dedicated sessions with server, actor and peer each make
**eight** process exits; every code was 0. The earlier final message's count
of six was incorrect. Exact-name process check found no Horizon client
afterward.

Source `9651dba12d163896d45a8cc4ba1b0b3b0da1feff` and all 711 files,
both frozen binary hashes and the Create 6.0.10 JAR hash matched before and
after; no production source, pin, package or schema changed. The repaired
fixture cleanup from `cdc2194` kept complete actor/peer item counts stable
and selected-line blocks unchanged in these automated receipts. Same-process
unload/rejoin and cold reopen completed in both modes. Dedicated actor/peer
HUD frames show independent first corners, with the peer retaining its own
corner after the actor's cancel/rejoin.

Direct team inspection of the integrated `cancel` and `quick-edges` PNGs
found the completed `(56,65,20)` to `(41,65,20)` selection still shown in
both. The dedicated equivalents show a successful cancel followed by a
`1 x 1 x 1` quick-edge pair. The integrated driver's Shift press, mouse
click and Shift release were sent consecutively before the queued use
callback ran. `control.rs` routes the mouse press to `play_mouse`, which
queues `use_clicks`; Quill reads
`ShiftLeft` when the later interaction callback executes. Dedicated network
latency happened to leave Shift down long enough, so its success does not
validate the old input sequence. This is a **driver timing finding**, with
the original screenshots and automated status preserved rather than
reinterpreted.

Driver-only repair `f4356ce` holds Shift for three game ticks after the
right click, asserts the local sneak state before and after, and releases
the key in `finally`. Python AST parsing and `git diff --check` passed;
fresh scoped independent Sol High source review returned CLEAR (33.6
seconds). No game run has
qualified this repair. After a new root lease, rerun the affected
integrated cancel/quick sequence with actual HUD-frame inspection. Root may
reuse other unchanged phase receipts only after directly inspecting their
frames. No Save/export/upload/placement or train behavior was exercised.

UTC phase intervals are under `.local/team-operations/trains-schematics/phases/`;
per-session token counters are unknown.
