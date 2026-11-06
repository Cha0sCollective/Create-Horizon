# Transport-005 prebuilt-belt custody trial result

Status: **all ten dedicated command phases completed; gameplay acceptance
pending Operations review**. One newly supplied gold ingot traveled from the
source chest through the prebuilt source and receiver belts to the Depot and
final sink under the script's exact custody assertions. This is one fresh
normal005 dedicated world, not a replay or repair of the custody001 HOLD or
the earlier missing-item HOLD. Those failures remain preserved. The source
code, host, schema and acceptance rules did not change.

## Frozen identity, lease and shutdown

- Transport lane base `76f374ad2ef9773e889467bab14f9af539891ad2`, branch
  `codex/team-transport`; this report is the only planned Git change.
  Packet-local reviewed adapter `custody_retry_002.py` SHA-256
  `0c9b0281bd66c5e3183cb4361ea49980fe1218089d9f56c62dc1ac67428d7eb5`,
  plan SHA-256
  `abb3fb033aede85e758270bf58c16405be17542d27e519ab0b39872f4a903d31`,
  unchanged frozen driver SHA-256
  `0d9ecfe95bc04fc97cf708d6f00dbc3d2e29921cdd10f8c1d7c34b98df183c8e`.
- Private compiled source HEAD `9651dba12d163896d45a8cc4ba1b0b3b0da1feff`,
  native tree `90c2156ed5ffb38131950581f05cbdc76beba9ba` (.13/cache16),
  frozen source-map SHA-256
  `c912836f0ec11311340a4dc0e7f8748abf25ee1a46fba17df4c2c5a33c790647`,
  binary-map SHA-256
  `6783a1002c422e7a829a7b65ccfd0459ac0344ffb8c13477c04b11e0beee37f5`.
  Game binary SHA-256
  `b862982154b5ad4cde22e95618920fbec3f6d61f11ed8984357254576c139e6e`,
  control binary SHA-256
  `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`,
  pinned Java reference JAR SHA-256
  `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
- Project-lead lease `transport-funnel005-prebuilt002`; fresh disposable
  dedicated session `dedicated-5h3x4ef0`, index00, port51047. The launcher
  session map SHA-256 is
  `1cddf04d974cc5a3b6db9b723352f4912566a4076fa3015c793f1d9abac65886`.
  Actor and actual peer quit normally; the server stopped normally. Exit
  receipt SHA-256
  `a4f750654f58cb65e1d425339124d3bf3fa4ed6e324c7fd7c0b1d3024d54e241`
  records all three code0. Post-run verification SHA-256
  `88c8dba18e169ce52f7d6a44e2333952339609907478761ec58d7062379f69dc`
  reconfirmed all 711 frozen source file hashes, both binaries, JAR, adapter,
  plan, ten phase statuses and exits. Pre-run verification SHA-256
  `7bde101b8d24a82c117100d487752759a459bcf10b14717ce718514062ed4509`.
  The lease was released at `2026-11-06T05:33:08Z`; lease-release receipt
  SHA-256 `330ca8ee507e339aa944c32bd3d5069e939a4bee9169eb4d695671ddc12792e0`.

## Bounded custody observations

`setup` and `receiver` prepared the already-qualified fixture. Before the
trial's only new gold input, `prebuilt-source-belt` proved source shafts and
air, both south-face camera targets, source chest0, powered funnel without
gold, receiver/Depot/hopper/sink0, actor and actual peer client/server gold0,
no gold world item and no fixture drop. It formed the moving source belt
with Length5 and Speed-32, checked source and receiver inventories0, then
repeated the zero census. The new input was supplied only after this receipt.

`supply-release` observed gold1 in the source chest while the funnel remained
powered and the source/receiver/downstream, both players and world remained
gold0. Removing the redstone power then yielded source chest0 and source
belt gold1, with receiver/Depot/hopper/sink0, both players0 and no world gold
or fixture drops. The `peer-occupied` command from the actual peer saw the
source belt gold1. `receiver-release` then required source0/receiver1,
Depot0 and no world/player gold. The actual peer's receiver command saw gold1.
`depot` required source0/receiver0/Depot1/hopper0/sink0 and no world/player
gold. The actual peer then saw receiver inventory empty. `final` required
source0/receiver0/Depot0/hopper0/sink1, actor/peer0 and no world or fixture
drops. Each phase completed its command assertions; none recorded a HOLD.

The ten exact phase receipts, command logs, adapter sidecars and five peer
captures are enumerated with checked hashes in packet-local
`G:\Create Rusty\.local\transport-005\custody-retry-002\evidence-index.json`
SHA-256 `046089ec3c09753626eac36c667519a628e06d2b0f3f2707ebd1a2975f48b841`.
Root directly inspected all five actual peer images. They show the belt,
receiver and final funnel contexts with occupied/empty chat overlays. The
ingot sprite alone is not a reliable count witness; server and actual peer
command receipts support the counts. Root's narrow image-review receipt
SHA-256 `6e1267978d7f440cbe232c45270882b1f83802150e211f45ac31d99031cb9d42`.

This pipeline did **not** directly capture the brief world entity between
funnel spawn and belt capture. The prior powered obstruction and natural
world-item observations remain separate, attributed evidence, and the 001
source-belt HOLD is not rewritten. This run establishes one ordinary
chest-to-belt-to-Depot-to-sink **command-checked** custody chain in dedicated
mode through sampled observations, not an atomic every-tick trace. It does
not establish arbitrary native spawn-veto behavior, broad version parity,
cold-reopen durability, or public release readiness.

Resource record: one granted heavy dedicated runtime session with server,
actor and peer. The Transport phase ledger records the `test` interval from
`2026-11-06T05:27:37Z` through release at `05:33:08Z`, including verification
and normal shutdown. No additional gold or game run occurred. Token counters
for this chat/reviewer are unavailable here; central accounting is
Operations-owned. Independent read-only Sol High completion review: **CLEAR
for this bounded dedicated command-checked result**. Operations retains the
acceptance decision.
