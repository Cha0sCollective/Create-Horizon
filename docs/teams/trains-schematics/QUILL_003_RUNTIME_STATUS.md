# QUILL-003 runtime status — first integrated attempt

Status: **HOLD**, preserved failed receipt; runtime lease released after a
normal exit and post-exit integrity checks. No full Quill milestone pass.
Root must grant a new exclusive lease before another game run.

Frozen inputs before launch and after exit: source
`9651dba12d163896d45a8cc4ba1b0b3b0da1feff` at
`source-inputs-r3.json`, all 711 listed files matched; game binary SHA-256
`b862982154b5ad4cde22e95618920fbec3f6d61f11ed8984357254576c139e6e`,
control binary SHA-256
`170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`,
and Create 6.0.10 JAR SHA-256
`ef87fe5709f1ba1f5b8bb20a2925bafb4669e178fd6d8bf10c167759eefe37a`
matched. Driver syntax had a root AST PASS at SHA-256 `e3a95127` before
launch. Source and binary metadata were read dynamically from the session.

Run `integrated-c5ew91f3`, session 00, origin `(40,65,20)` used the reviewed
real-mouse driver. The failed raw receipt is under
`.local/native-team-runtime-005/integrated-c5ew91f3/quill/integrated-00-full.json`;
six PNG HUD frames are beside it. The first hold showed only corner
`(56,65,20)` after 18 ticks. The 17-block preview was red; its click retained
the first corner and displayed refusal. The 16-block reversed preview was
green; its click displayed first `(56,65,20)`, second `(41,65,20)`, size
`16 x 1 x 1`. A third click displayed the explicit unavailable-Save message.
These six frames were visually inspected by this team; root must inspect
them directly before accepting any reused partial evidence.

The driver then failed its first complete inventory equality: actor Quill
count remained 1 and stick remained 1, but 13
`minecraft:leaf_litter` items had been picked up from drops caused by the
disposable `fill` terrain preparation. The selected-line `blocks` reply still
contained exactly the four expected stones. This is a fixture contamination,
not a qualified Quill accounting result. The actor was closed with `quit`;
session 00 exit receipt reports actor code 0. Exact-name process check found
no Horizon client afterward.

Repair `cdc2194646abf2fe33adc4b0b9b873d3d1158971` removes fixture item
entities and clears incidental actor/peer inventory **before** giving test
items or taking baselines. It waits for empty inventory and a clear nearby
item-entity observation, then retains full equality during Quill actions.
Python AST parsing passed. Fresh independent strict read-only Sol High
review of this scoped delta returned CLEAR (54.1 seconds); no game run has
qualified the repair. The original failed receipt and PNGs remain intact.

Next, after root grants a new lease, use a **fresh** disposable integrated
run and then a fresh dedicated run with the repaired driver. Complete
same-process unload/rejoin, cold reopen, peer isolation, actual HUD
inspection, exits and post-run hashes. Do not recast the first attempt as a
pass or run Save/upload/placement. UTC phase intervals remain under
`.local/team-operations/trains-schematics/phases/`; token counters are unknown.
