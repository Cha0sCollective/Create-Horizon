# Coherent close: explicit historical Durable deadline renewal

Status: ACCEPTED DESIGN in focused native_read_design_review follow-up at485d4b1;
this records the exact requested/accepted behavior. No SOURCE/tool/runtime clearance.

Old-source IO can fail or expire while staged controllers remain historically
Durable. Existing poll returns Complete immediately and staged retry delegates to
a Blocked-only retry, so a later begin_final inherits the original expired deadline.

On explicit Root retry, AFTER its actual global IO handle has been reaped, staged
retry may renew historical Durable deadline as well as Submitting deadline. Require
a future deadline, nonwaiting original Sealed/Durable admission and identity,
no original native writer/thread or owner fault, zero outstanding/pending work,
intact correlated error/accounting and a current matching verification ledger.
Busy/refusal retains the same owner. Change ONLY close.deadline; retain historical
NativeWriterDurable proof, admission/status, accounting, errors, history and handles.
Do not reopen admission, reset errors, guess repair or renew from poll/begin_final/
another phase transition. Existing Pending/Blocked historical and terminal retry
semantics remain unchanged. Root's reaped-global-IO obligation is external to the
library controller. Extend the same focused real native retry test, no new framework.

Existing save.rs path only, ALL14/prefix preservation before correction, exact
source freeze and whole SOURCE/tool review before compiler/runtime. Different
FINAL and existing publication boundaries remain unchanged.
