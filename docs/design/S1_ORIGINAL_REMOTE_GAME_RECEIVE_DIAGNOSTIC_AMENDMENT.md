# Held amendment: receive diagnostics during deliberate remote stop

Status: HELD for fresh independent DESIGN acceptance, private checkpoint and
coordinator release. No source change follows from this document alone.
This amends only s1ogrc1's operation-error presentation policy within its existing
24 paths, parent e52f0def9f4cfd297133c9c8d018e8061a7fe328. No library contract,
permission, default activation, C1, native storage or publication scope expands.

## Concrete issue and missing fact

The actual retained Session reader reports Receive(String) when an incomplete
Login or Play read is interrupted by stop. It also uses that variant for other
receive failures. Current Game close promotes every non-Closed terminal into an
application failure requiring acknowledgement, including deliberate socket stop.
The Session contract cannot establish the cause of a Receive outcome. A snapshot
does not remove races. Keep its raw terminal evidence and do not label a receive
as caused by cancellation, clean IO or proof of an error-free network operation.

The current source at manifest 3a6110dc has passed focused checks/build and the
seven selected groups (three earlier cases explicitly reused at identical inputs).
The incomplete-Login test now correctly proves only the original reader exists;
the writer is acquired after LoginFinished. No live window case has run yet.

## Exact presentation decision proposed

Record an explicit deliberate parent stop request for user leave/quit, OS close
or signal and an intentionally requested frame limit. An existing application,
decoder, stream, send, protocol or startup failure retains its error and required
genuine acknowledgement. Timeout remains a failure. A later deliberate request
must never erase an already observed failure.

Before requesting Session stop, preserve and inspect already observed terminals,
pending disconnect/decode errors, refused sends and existing Game/Application
errors. Original events drained for that snapshot move into the original IO's
pending queue without processing, replay or input loss. Preserve the terminal
snapshot boundary; no atomicity or causal ordering with native IO is claimed.

Every original terminal remains in its original Session/ClosedSession. A Reader
Receive first observed after the explicit deliberate stop request may be shown
as an operation diagnostic with unresolved cause, instead of requiring error
acknowledgement by itself. Say "first observed after deliberate stop request",
never "occurred after the fence" or "caused by stop". Raw outcomes and all queued
payloads remain owned; diagnostic presentation may clone text only.

ServerDisconnect, Panic, Spawn, Io, Encode, refusals and every other failure stay
failures regardless of observation time. A Receive already observed before stop
stays a failure. Use one classification in IO poll_close and Game failure queries.
Preserve original_error and its acknowledgement requirement across subsequent
diagnostics, retries and final control sealing.

Show the receive diagnostic in closing/ready status, title and control replies.
Keeping it only inside an opaque closed owner is insufficient. Deliberate leave
success means actual checked actor/control closure with retained operation
diagnostics. It proves neither receive success nor absence of network errors.
No worker restart, replay, replacement owner or false native world completion.

## Bounded validation and gates

Extend the existing real partial-Login group: deliberate stop must retain actual
Receive evidence and expose the diagnostic while the original actors join; a
second real peer ending receive before stop must remain an application failure.
Keep conditional Retry, exact attempts, original owner custody and native joins.

Use the actual isolated server/client window cases for normal move/render/leave
and server-sent disconnect. Add an abrupt owned fixture-server exit if needed to
establish unexpected Receive presentation through genuine acknowledgement. Its
intentional abrupt death is a failure fixture, never retained server closure.
Keep the smallest affected bin/native regressions. Reuse only the two already
successful library skin cases if the actual new library binary is byte-identical;
the former three-bin-case reuse is invalidated by this production close change.

Preserve all 24 source paths and all prefix artifacts before every correction.
All source authors stay serial; the earlier disjoint draft exception has ended.
Fresh exact SOURCE/tool and post-format binding precede qualification. Review the
modified window driver and bind actual binaries/results before game execution.
Different FINAL/privacy and fresh repository metadata still precede public push.
Do not claim C0 composite, retained server closure, whole-client/S1/G1 or S2.
