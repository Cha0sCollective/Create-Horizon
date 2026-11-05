# S5 one-cell carrier — provisional feasibility evidence

S5 remains OPEN. This is a bounded private adapter trial on a deliberately
configured creative motor, not an implemented Create mechanical piston. The
approved scope is `docs/design/NATIVE_S5_ADAPTER_SPIKE_PACKET.md`. Fresh
independent Sol High reviews cleared the bounded source correction and final
feasibility result. The corrected source passed its focused game check. This
supports the one-cell adapter route; it is not S5 acceptance or a general
moving-assembly claim.

## Exact build and profile

Private branch `codex/native-s5-feasibility` starts from accepted S4
`b88eb47bbb83138e9fe62684f2fcf2f1969e3702`. The existing package was
versioned `0.5.0-ch4os.2`; `create:` IDs stayed intact. No public Horizon source,
host API, save format or protocol changed. The tested source input map is
SHA256 `55a33ad1e490b5bdfe98bf720028d5d22869465fd5b6b00b66a9dfb67b68f1ee`.
The initial successful pinned build generated game SHA256
`45f94e413dfde20b59e3decc8d71db902d9f3b577f3fe6685898627520abdb2f`
and control SHA256
`ab7e1c9a5f793e1620076845d524815c893ac2aa16e24cb444d70cca4b5e8ebf`.
The launcher checked those binaries, source/resources and the retained 6.0.10
art JAR. Pinned host `fc021038`, Mixins `a1e13326`/API359, Rust1.99.1 and
Minecraft26.3 stayed unchanged. The selected motion reference is Java Create
6.0.11/`7ff2760e`.

Boundary review found that the first source checked only the two travel cells,
not collision shapes protruding from nearby blocks. The corrected source map is
SHA256 `ac29436c602acdbe7bbb07d3f09194654dbccef47d88a4ea8222f6b6354304fb`;
game SHA256 `18ec1fdc2a198ecd6afe5b37cc1686de70e5d643ad267798ff24e426637cdfc6`;
control SHA256 remains `ab7e1c9a5f793e1620076845d524815c893ac2aa16e24cb444d70cca4b5e8ebf`.
The reviewed delta checks nearby native collision boxes across the swept stone
cube before capture, advance and restoration. A failed destination write retains
the captured owner, repairs the proxy where possible, and schedules a later
full-host retry; this defensive branch was source-reviewed but not fault-injected.
Pinned `rustfmt --check` and exact generated-client Clippy both exited0 after
the correction. Clippy emitted 24 warnings in the generated client, including
one collapsible-if suggestion in the injected carrier guard; none was an error.
The corrected game build completed in156.71s.

The isolated dedicated Creative TestTerrain world is mob-free. Actor and peer
are matching rendered native clients. Only one stone payload travels one cell
east in the same chunk; a motor at `[5,65,5]` owns its trial state. Fixture
commands configure that motor and initial stone, while capture reads and removes
the actual world block. The trial does not test assembly attachments, block
entities, inventories, fluids, riders or cross-chunk movement.

## Observed behavior

- One stone at `[7,65,5]` was captured into the motor's namespaced NBT; both
  world cells became nonowning moving proxies. Real `ScrollValue` drove the
  existing motor's signed RPM. At zero RPM, offset/previous held at `0.25` over
  additional ticks. The peer rendered the stone and actual walking collision
  stopped at the corresponding face (`x=8.55`).
- Resuming at `-8 RPM` moved to the east endpoint and restored exactly one
  stone at `[8,65,5]`; the peer was pushed safely to `x=9.31`. An obsidian
  source-cell obstruction refused subsequent reverse capture without consuming
  the endpoint stone. Clearing it allowed reverse capture and a paused `0.25`
  pose.
- A stone wall behind the peer stopped reverse progress at `0.390625`; another
  20 ticks left both progress and peer position unchanged. Zero RPM retained
  that stalled fraction. With a glass-pane escape obstruction, an ordinary
  actor break left the motor and captured payload owned, marked
  `restore-blocked`/`RemovalRequested`, kept the peer at `x=6.875`, and made no
  item. Clearing the pane restored one stone at the source, pushed the peer to
  `x=6.69`, and marked `RemovalReady`. A second ordinary break removed the
  empty motor; the stone remained and no item dropped.
- A fresh configured trial motor recaptured that same world stone. After a
  normal exit of both clients/server and cold reopen of the same world with
  the same binaries, owner NBT retained `Captured=1`, `Offset=Previous=0.5625`
  and the two proxies. The peer client rendered the paused stone. Walking into
  it stopped at `x=8.8625` in both client state and authoritative position.
  A separate peer disconnect/rejoin left the owner unchanged and reconstructed
  the visible stone. Resuming motion restored exactly one stone at the east
  endpoint, source air and no dropped items.
- An ordinary redstone piston in another area extended, pushed its stone and
  retracted normally with these private hooks active. This is one focused
  regression observation, not broad vanilla-piston qualification.
- On the corrected binary, an oak fence one block below the empty destination
  protruded into the moving stone's path. Capture reported `capture-refused`;
  source stone, destination air and zero dropped items remained. Replacing the
  fence with the original stone floor allowed capture at a fractional offset
  and final restoration of exactly one stone at the east endpoint. The focused
  corrected run's server and both clients exited0.

The initial six successful game processes across two dedicated sessions exited
code0, as did all three corrected-run processes.
Raw commands, two session maps, receipts and selected screenshots are copied to
`.local/native-s5-01/evidence/`; the original run remains under Linux
`s5-runs/dedicated-7xh51ljp`. The command transcript SHA256 is
`8a7ba3cc29f985b7b17a89de55269c23b9ade6a81724fde83d85ce1d4a63288c`.
The corrected run's transcript/session/receipt are separately copied to
`.local/native-s5-01/evidence/fence/`, with its original run under Linux
`s5-runs/dedicated-eaf2vofm`. Its command transcript SHA256 is
`6bec8f40c44146c6fcc82877862b286aa52dda318fe41b61e7c11251b456691b`.
Server/NBT queries and client state/screenshot observations are distinguished;
the server command view on a peer is not counted as an independent client
replica. A proxy had no visible pick face, so an attempted right-click command
failed before the server packet and does not qualify that guard.

## Remaining before acceptance

The corrected source has a bounded one-block-halo native collision sweep, not
an arbitrary modded-shape or unloaded-neighbor guarantee. The restore retry
is a defensive source path; no ordinary second-write failure was identified in
the pinned same-chunk, valid-state profile, and interrupted-crash atomicity is
outside the adopted plan. The reviewed next packet is
`docs/design/NATIVE_S5_ACTUATOR_BINDING_PACKET.md`; bind the carrier to one
real Create linear actuator and qualify its reference
controls, save/reopen, collision and disassembly. The actual normal piston
must push stone outward without pulling it back, and its one pole/head must be
accounted for; the trial motor's fractional zero-RPM pause is not that piston
behavior. Do not relabel the configured motor as the actuator.

Initial build failures remain preserved as attempts00–02. An initial placement
click on a moving proxy failed controller visibility before server dispatch; an
initial JSON batch for post-reopen completion failed parsing before game action.
Neither is included in the passing outcomes.
