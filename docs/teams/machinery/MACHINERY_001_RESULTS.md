# Machinery 001 — fan washing over a Depot: HOLD

Packet: `docs/teams/machinery/MACHINERY_001_PACKET.md`. The selected fan path
processed and preserved one-item custody, but the dedicated peer's cold-reopened
view omitted a server-held flour. This packet is **HOLD**, not a complete
machine/replica qualification. S6 remains open. The actual cause of the missing
presentation is not established; no shared Depot or host source was changed.

## Exact inputs and reuse

Lane `codex/team-machinery` at packet/source HEAD
`5c27c6a86149907d8141079ad9eb6e525b2fa494`, based on accepted water
`02d029f3bea09c62480e07e315e23d4868f9180a`. All 612 tracked native files
matched accepted input map SHA-256
`34aa86142346922d5e4e8aa3a86a29aed6eee695fb71415b39ce66e1f45dd83f`.
Held game `69a8537a1623bc3e309d1a8fb58151d9d0de1220bd803071272ec31960e80515`
and control `446f2f1fe8c15ee082a25c2ea8dacd79b66df8bcf4768150373f2c3caf319cda`
were byte-verified before launch. Native/content version remains
`0.5.0-ch4os.8`, Horizon `9c7f9b22`, Mixins `e8cd0aa3`, API 359 and
Rust 1.99.1. No Cargo build, Java execution or production source change occurred.

The selected reference is Create 6.0.11 `7ff2760e` for plain
`create:wheat_flour` -> `create:dough` under water-catalyzed fan splashing.
The supplied creative motor, fan, Depot, flour, waterlogged fence, support
blocks and camera platforms were disposable fixtures in normal seed-5 Survival
Overworlds with `spawn_mobs=false` and `random_tick_speed=0`. Fan and Depot
were ordinarily placed. Geometry: motor `[4,66,11]` facing east at 16 RPM,
fan `[5,66,11]` facing east, waterlogged oak fence `[6,66,11]`, and Depot
`[8,65,11]` one block below the air cell. No material acquisition or ordinary
fence-waterlogging claim is made. Actor actions used game input; server NBT
queries and actual peer screenshots are identified separately.

## Observed machine behavior

In the isolated integrated world, no water catalyst, zero RPM with the
catalyst, and powered unsupported cobblestone each retained their own Depot
input with no dough after 180 ticks. Ordinary empty-hand interaction recovered
each input. Powered flour paused at `FanProcessingType=create:splashing` and
`FanProcessingTime=107`; 35 stopped ticks did not change the timer or held
flour. Resuming yielded exactly one dough, collected ordinarily by the actor,
with no flour/dough in the Depot or dropped entities. The actor retained the
separately tested cobblestone. The process exited 0.

The first dedicated flour converted while a failed client-side `container`
assertion was being investigated. Authoritative server NBT showed one dough;
the actor collected that dough. A new supplied flour was then paused at
`FanProcessingTime=111`, and 35 stopped ticks left that timer and flour
unchanged. Normal shutdown of server and two rendered clients gave three
exit codes 0. A cold restart of the *same* server world and both clients
gave authoritative Depot NBT with one flour, type `create:splashing`, timer
111 and motor speed 0. The actor still held the first dough and no flour.

**Failing observation:** the peer's pre-close screenshot showed flour on the
Depot, but its same-pose cold-reopened screenshot showed an empty top while
the authoritative query still held flour and timer 111. Resuming power made
the second dough; the peer's same-pose product screenshot showed it. The peer
ordinarily collected that one dough and its same-pose empty screenshot showed
the clear Depot. Final actor and peer each held one dough; authoritative
Depot slots and dropped entities contained no flour or dough. The final
server and two clients again exited 0. These item counts do not repair or
override the failed cold peer presentation. The accepted Transport 001
empty-clear correction was not integrated into this `.8` fan candidate; its
effect on this initial nonempty reopen has not been established.

## Evidence and preserved failures

Ignored evidence index:
`.local/machinery-001/completion-evidence.json`, SHA-256
`08cf3d18cbef32f0bf96168404bf95e40c6ad515cd6082659a08ddabecf0bd9c`.
It records the fresh lane source map, held binary pins, 26 measured control
receipts (23 successful, three failed), seven zero process exits, five selected
images and hashes of 13 mirrored artifacts. Raw commands, logs, images and
worlds remain in the isolated Linux runs
`/home/arbiter/.cache/create-horizon/machinery-001-runs/integrated-2tg5gdsg`
and `.../dedicated-2w3jyso_`. Source and runtime inputs stayed frozen through
gameplay; adaptive control batches have their own hashes and receipts.

The four peer images in the index use the same position
`[10.5,66,13.5]` and look-at `[8.5,66.15,11.5]`:

| Image | What it shows |
| --- | --- |
| `dedicated-peer-flour-paused.png` | Visible held flour before normal close |
| `dedicated-peer-flour-reopened.png` | **FALSE:** empty-looking Depot top after cold reopen, with server flour/timer 111 |
| `dedicated-peer-dough-product.png` | Visible dough after power resumes |
| `dedicated-peer-empty-after.png` | Empty top after peer collection, with peer dough 1 |

The first integrated batch failed before insertion because the Depot top face
was not visible from the original camera height; a platform and continuation
used the same supplied flour. A later global no-dropped-items assertion was
FALSE: two `minecraft:leaf_litter` entities lay outside the cleared plot at
z about 16. Follow-up item-entity queries found no flour or dough drops.
The first dedicated batch's `container` assertion returned empty client slots
after the actor's click; server NBT already held the resulting dough. No
client-container view is treated as authoritative. All failed receipts and
their original outputs remain indexed rather than rewritten as passes.

The reopened peer log has 6,024 lines and 5,004 fan-propeller pose lines but
no `create:depot` or `FanProcessing` lines. No existing packet or client
inventory-NBT trace was available from that log. The screenshot is actual
client presentation; the query is server authority. This evidence establishes
the discrepancy, not its network or renderer cause.

## Handoff

The central host-wide game slot is free; no process is running. A narrow
read-only shared Depot replica/update investigation belongs to the project
lead for allocation with Transport. Machinery must not edit shared Depot
callbacks, host hooks, pins or acceptance scope. Any named fan-owned repair
would receive focused checks and fresh independent read-only Sol High review.
The one-item fan observations do not qualify belts, loose-item processing,
other fan media, stack/chanced recipes, arbitrary worlds, crash atomicity or
release. Independent completion/evidence review is pending on this HOLD record.
