# Transport-001 — Depot peer clear candidate

Status: reviewed local candidate for project-lead integration/acceptance. The
original bare-belt checkpoint remains HOLD until the project lead reconciles
this focused repair with its historical evidence. This packet does not complete
S6 or qualify additional transport behavior.

## Exact source

- Lane `codex/team-transport`; operations-adopted base
  `f00d04a3249b53bee453a0191d65285b0e7f6538`, held checkpoint
  `2adc50a962fd804bbc29c139d863641c19e84af7`, held native source
  `43cef7dd7e8568e3e5a28cddb5b96a38196e1a0a`, accepted plain-water
  ancestor `02d029f3bea09c62480e07e315e23d4868f9180a`.
- Candidate head `0df826c3459f3eae7c0e9c8619c3fc2a97b0b204`, clean worktree.
  Only `native/create/mixins/create/depot_changed.rs` changed (+9/-2).
- On empty canonical Depot slot, remove `HeldItem` as before and queue block
  entity sync only if that render record existed. The host publishes the current
  state later; a same-tick refill need not show an intermediate empty frame.
  The existing portable/Basin notifications remain first and unchanged.
- This is source-informed reuse/alignment with the retained MIT upstream
  `dd0f3141a33851ac20fa98b807a6532471dba727` Depot callback pattern,
  retaining the private accepted portable callback. This is a local snapshot
  comparison, not a remote-head assertion. No host/API/schema/pin change.

## Checks and evidence

- Fresh independent read-only Sol High source review CLEAR; comment delta
  CLEAR. Independent completion evidence audit CLEAR for the narrow claim.
- Frozen new lane map SHA-256
  `10a868c01d1e87156eb9620181fbaeab71ab26426f0444e1f62163102469030a`;
  all 634 tracked native files rechecked. Horizon
  `9c7f9b2215dfed497aba5522c44f38926628648d`, Mixins
  `e8cd0aa38cffef49cb2e088ef716baa107148c02`, Rust 1.99.1, native `.9`.
- Focused format check PASS (0.620 s); new stage plan 65 injections/30 modules
  PASS (200.904 s); Clippy PASS with 37 warnings (224.356 s); locked build
  PASS (388.423 s). Binary SHA-256: client
  `63047934df11951c3c42303fd7da2664a2cb3465ee7279f11593eb711ab5426d`,
  ctl `7b2a25cb1b134addedae199c4091188e3b1198c284d2590da4d05cc4ba145816`.
  No duplicate historical 17-test battery.
- New disposable dedicated seed-5 Survival world with server, actor and actual
  rendered peer; isolated homes/resources/config. Supplied ordinary Depot gold1
  above a locked stock hopper and chest. Peer image visibly shows gold, then
  empty after ordinary redstone unlock and hopper extraction without reconnect
  or Depot replacement. Final live Depot/hopper empty, chest gold1, actor gold0,
  no loose items. Manual iron placement/collection also shows peer occupied to
  empty, with actor iron1 and chest gold1. Saved fixture chunk decodes to the
  same owners; actor iron1, peer incidental leaf litter22, default player empty,
  equipment/Ender no gold/iron. All three successful process exits 0.
- Raw packet `G:\Create Rusty\.local\native-s6-transport-team-001`;
  successful run `runs/dedicated-o7g9ushy`; `completion-evidence.json` SHA-256
  `0f2863fc9512fdbabf8fc01eab46fad03915c1e37d5b21c2e7d917c6e46e0f42`
  indexes 19 checked artifacts. Actual peer images are
  `peer-gold-occupied-final-camera.png`, `peer-gold-empty-after-hopper.png`,
  `peer-iron-occupied.png`, `peer-iron-empty-after-collection.png` in that run.

## Boundaries and operations

The first launch failed before controls because G-drive Unix sockets are
unsupported (server exit 0, actor/peer exit 1); it is retained as
`runs/dedicated-pxjuhkiu`. Initial setup and camera attempts in the successful
run are retained and superseded by the clear image pair. Fixture drops were
cleared before the accepted occupied capture, with gold1 still confirmed on
Depot. The successful visual pair is about 29 seconds apart, so it demonstrates
eventual peer presentation, not notification latency. This run supplied gold
directly by ordinary Depot interaction; earlier unchanged belt delivery and
ownership evidence is attributed to the held checkpoint. No new integrated,
partial-extraction, same-tick refill, cold-reopen or broader belt claim.

The root-granted heavy lease was returned after normal exits and save decode.
No game is running. Temporary lane tool/cache/target links were unlinked after
validation; the primary cache and all raw runs remain intact. No additional
resource is requested for this packet. Project lead owns integration and any
acceptance update, central registry/accounting and private synchronization.

Nested read-only task paths: `/root/trace` (source trace and focused harness
assessment) and `/root/validation_review` (Sol High plan, source/delta and
completion evidence reviews). Agent phase records are under
`G:\Create Rusty\.local\team-operations\transport\phases`; strict reviewer
reported 47.709 s, 78.067 s, 0.301 s and 117.520 s intervals without writes.
Operations collects actual lead/descendant token counters; unavailable counters
remain unknown, never estimated.
