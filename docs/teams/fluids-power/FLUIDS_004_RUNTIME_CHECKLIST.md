# FLUIDS-004A — ready-to-schedule Tank lifecycle check

This is an **unrun** packet for the project lead's shared game reservation.
Use a new disposable world/instance and the exact frozen accepted `.11` source
`55339ce3347b2ce667281e8a3e6318ec856144e4`, source-input map SHA-256
`b6b913501fdfc3fe4326dcafb53fc8c47a8eb7dccc697615ba0a5d984e18aa4b`,
game binary SHA-256
`edba263e154f198b4a62033d3c468ffa6567c371c5201eb5ccb8577cc0ba807b`
and control binary SHA-256
`170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`.
A different candidate needs its own identified, reviewed source and binary
receipts. The mutable physical integration worktree is not an input identity.
Preserve cohort003 evidence and do not run its full Tank suite again.

The packet-local driver `tools/packets/fluids004_tank_lifecycle.py` runs only
against an already launched control session. It verifies the frozen identity
above, records each control call and authoritative query, and stops on a failed
assertion. It does not start, stop, rebuild or clean any game process. Supply
the session JSON, exact `horizon-ctl` path and a new output directory. Run its
`normal` and `sneak` cases once each at separate pads; never rerun over a
partly used pad. The project lead records process IDs, session path, source
hashes, raw driver receipts, screenshots and all process exits in its own
runtime packet. No launch command or reservation is implied by this checklist.

## Finite sequence and expected census

| Moment | Actor inventory | Block and fluid owner | World drop/peer |
| --- | --- | --- | --- |
| Ordinary Creative setup for each pad | One Tank placed; one water bucket used without consumption. Setup items are cleared before Survival checks. | The ordinary placed one-block controller is valid and contains exactly 1,000 mB plain water from one real click. No NBT seed. | Peer may show the window, but 1,000 mB is below the visible front; server NBT is the content evidence. |
| Normal wrench on the first filled Tank | Wrench remains held. | Window block state and `Window` NBT flip, while the **same** block entity retains 1,000 mB. | Peer state screenshot, not a visible-water claim. |
| Survival normal break | Before: one plain diamond pickaxe and zero Tank items. After: pickaxe plus one plain Tank item only after ordinary pickup. | Old block becomes air; no old Tank block entity or local world water remains. | `break` receipt reports exactly one Tank drop. Query the dropped `Item` compound before pickup: one `create:fluid_tank`, no block-entity content. Peer sees removal. |
| Re-place normal drop | The one Tank item is consumed in Survival. | New position has one empty, valid Tank controller and `TankContent:{}`; old position remains air. | No second item or local water. |
| Survival Shift-wrench on the second filled Tank | Before: one wrench and zero Tank items, with free inventory. After: wrench plus exactly one plain Tank item returned directly. | Old controller gone and no local world water. | No nearby dropped item. Peer sees removal. |
| Re-place wrench return | Returned Tank item is consumed in Survival. | New position has one empty valid Tank controller; old position remains air. | No second item or local water. |

The driver samples the isolated pads around each removal and replacement, including
all loaded blocks in a bounded 7×7 horizontal area around each removed Tank,
the actor's complete nonzero inventory counts and stack data, the dedicated
peer's initially empty and unchanged inventory, the normal dropped item before
pickup, and authoritative `data get block` replies. It records the break result,
pre-replacement census and replacement state separately. The normal-drop pickup and replacement occur
only after the dropped item is captured. A cold reopen of a removed or replaced
Tank is an optional separate persistence claim; this packet does not count
cohort003's earlier filled cold reopen again.
The project lead must inspect the peer PNGs; peer `block` queries establish
replica state, not that the transition rendered visibly.

Do not reinterpret Survival bucket use as direct Tank exchange. Pinned Java
passes a regular Tank's Survival use through; the host's vanilla bucket can
place world water beside it. A separate short pass-through check may be
scheduled later with its own actor/bucket/world-fluid census, but it is not a
prerequisite for this filled-removal packet. A usable Survival transfer path
needs a later source-backed producer or consumer of the already exposed
one-block fluid capability.

## Stop and reporting rules

An observed Tank, item or fluid mismatch is a **FALSE gameplay observation**,
not an accepted repair or a reason to reset the same pad. A source-identity,
teleport, chat-query or other fixture/control failure is a **setup failure**;
preserve its receipt but do not diagnose Tank behavior from it. Keep the raw
receipt and world, identify the exact differing owner or state, and ask the
project lead to reconcile any source repair. Do not change Tank interaction semantics, make a broad fluid
network, alter schema or host, or expand to connected/boiler tanks. The final
result must separate source expectations, game observations, any failed
attempt, actor/drop ownership and whether the peer actually rendered the
state transition. No playable save, launcher setting or public artifact is
involved.
