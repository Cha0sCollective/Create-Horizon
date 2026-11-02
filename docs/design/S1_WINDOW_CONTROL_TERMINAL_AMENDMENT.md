# C0e2c terminal application control phase amendment

Status: HELD, requires fresh independent Sol High acceptance together with
S1_RETAINED_WINDOW_PROPOSAL.md. No source/activation authority by itself.
This is an explicit architectural amendment, not a claim that older promises
were tested or that native control already fulfilled them.

The immutable S1_PAIRED_CONFIGURATION_ADMISSION_PROPOSAL.md385-391/748-759/
899-914 keeps the same retry endpoint through every fallible close stage and
plans no Blocked after final listener closure. Its terminal path assumes a
no-thread start_polled control service and treats unlink errors as diagnostics.
Accepted C0e2b instead retains actual listener/client/cleanup workers and checked
native cleanup with visible failure. begin_close seals requests/sockets; no same
endpoint reconnect remains possible after that operation. Uninterruptible cleanup
or unfinished original workers can legitimately retain Blocked then. A status
getter, original path or client count cannot establish otherwise.

For the explicit C0e2c profile ONLY, distinguish these actual scopes:
- WORLD close: original control remains Running, restricted to fixed status,
  exact-attempt Retry and idempotent close through every LOD/native-save/image/
  world-owner Blocked. New clients can reconnect to that same endpoint. It cannot
  be toggled off/replaced while it is the usable retry surface. WorldLeave ends
  only in an inert closed view; no mutable NativeReady recovery.
- APP terminal control seal: admitted only after WORLD completion and a usable
  original native UI retry surface. Deliberately call the same RetainedControl's
  close, preserving its original handles/receivers/cleanup owner. After this
  irreversible RPC seal, native UI status and exact Retry remain; RPC availability
  is no longer promised. Socket reopening/rebinding or replacement workers are
  forbidden. Actual native quiescence and cleanup success are reported separately.
- Terminal handoff/exit: requires actual checked actor completion and the accepted
  original world/client handoffs. Native cleanup failure/ambiguous release remains
  visible; never mint successful endpoint release. The design must explicitly
  classify a native-quiescent leftover endpoint versus still-active work, preserve
  the original outcome/error. Select the following concrete policy: a
  native-quiescent Failed/Ambiguous/Replaced/refused-publication outcome consumes
  the original ClosedControl as a FAILURE handoff, retains the original outcome
  and error in an inert native terminal-error view/title, and requires explicit
  fixed native Exit acknowledgement before owning-thread disposal and returning
  that original failure. No completed-cleanup Retry is shown or promised; existing
  retry_close refuses Quiescent and does not rerun cleanup. No respawn/rebind or
  foreign-path deletion. Still-active work retains Blocked and exact Retry of the
  wait, with all actual owners intact. Successful release or never acquired without
  failure permits ordinary final exit. No hidden diagnostic-only success or
  dropped unresolved owner is allowed.

A renderer failure before terminal seal keeps original RPC live. Native retry
input must be fixed and actually usable; rendering failure is not permission to
drop its renderer/window or forge a native closure witness. Indivisible GPU/OS
operations cannot carry a hard responsiveness bound. If qualification shows no
valid retry surface at the terminal boundary, retain the pre-seal owner and revise
scope; do not call a missing surface an accepted close. A stronger independent
management service would need its own original acquisition/lifecycle budget.

This supersedes ONLY the incompatible no-thread/nonwaiting terminal assumption
and post-seal same-endpoint availability for this opt-in component. Other profiles
and the full ordinary S1/G1 gate retain their unmet promises. No completed G1,
ordinary activation, native-control memory/IO budget waiver, new stage, permission
expansion, public release or upstream contribution is implied. The proposal must
preserve or explicitly re-review concrete control bounds under the real worker
architecture. Existing frozen documents/indices remain unchanged.
