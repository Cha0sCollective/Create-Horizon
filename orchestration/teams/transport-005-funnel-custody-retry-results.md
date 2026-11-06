# Transport-005 fresh gold custody retry result

Status: **dedicated source-belt HOLD; full gold route unqualified**. This was
the one fresh, disposable, normal005 trial granted under
`transport-funnel005-custody001`. It did not resume or reinterpret the older
missing-item run. The lease was released at `2026-11-06T04:41:53Z`; all three
owned game processes exited code0. No second input or run occurred.

## Exact identity and shutdown

- Transport lane base `1512196905b7b95cddde90e89dfc615d985db5bd`, branch
  `codex/team-transport`. This result document is the only Git change. Private
  compiled source HEAD `9651dba12d163896d45a8cc4ba1b0b3b0da1feff` and
  native tree `90c2156ed5ffb38131950581f05cbdc76beba9ba` (.13/cache16)
  remained fixed. Frozen source map SHA-256
  `c912836f0ec11311340a4dc0e7f8748abf25ee1a46fba17df4c2c5a33c790647`;
  binary map SHA-256
  `6783a1002c422e7a829a7b65ccfd0459ac0344ffb8c13477c04b11e0beee37f5`.
  Game binary SHA-256
  `b862982154b5ad4cde22e95618920fbec3f6d61f11ed8984357254576c139e6e`;
  control binary SHA-256
  `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`.
- Frozen driver `G:\Create Rusty\.local\transport-005\driver.py` SHA-256
  `0d9ecfe95bc04fc97cf708d6f00dbc3d2e29921cdd10f8c1d7c34b98df183c8e`;
  reviewed one-shot adapter `custody_retry_001.py` SHA-256
  `a93ef9bf0c9a077b8dbe8fba08ff5de900268594a9f3422f2264cbcde4c7630b`.
  Fresh launcher session `dedicated-lwe852qn`, index00, port58029, session-map
  SHA-256 `2f7db6e02ef0149e5f27371d144ccd703d7f9f00975f221dca4c869a593b086d`.
- Packet-local pre-run verification SHA-256
  `eba1b7069848dd904bf4f683dfa898bfa27960ee9f3462d8c9cee03aa3df5c33`;
  post-run verification SHA-256
  `9a68801abba8ac21faf49e444deebd0a32b4e6a29aedb0f91bc0d8294ea9fda1`.
  They verified all 711 frozen source files, source HEAD, and both binary
  hashes. The post-run receipt includes server, actor and actual peer exit0
  from `exit-00.json` SHA-256
  `a4f750654f58cb65e1d425339124d3bf3fa4ed6e324c7fd7c0b1d3024d54e241`.
  Lease-release receipt SHA-256
  `d328b8589897687fc679e887b23e68932c05bcfb9a892c4e97f520a4202779bc`.

Resource record: one dedicated disposable server, actor and peer session used
the granted heavy runtime lease. The Transport phase ledger records 408.72
seconds in `test` from `2026-11-06T04:35:05Z` through lease release at
`04:41:53Z`, including pre/post checks and normal shutdown. No build or
additional game session ran. Token counters for this chat/reviewer are not
available in this packet; central accounting remains Operations-owned.

## Trial observations

`setup`, `receiver`, `pause-obstruct` and `world-item` command checks completed
in order. Before the only new gold1 supply, the retry asserted both south
shaft faces visible from `(6.5,64,9.5)`, shaft/air geometry, source chest0,
receiver belt0, Depot/hopper/sink gold0, actor and actual peer client/server
gold0, no gold world item and no fixture drops. It then supplied the labeled
gold1 to the chest, observed chest1, placed the obstruction sentinel and
observed the naturally extracted gold world item. The world-item log recorded
gold count1, UUID `[I; 184620956, -1499184729, -2014354467, -1397531403]`,
at `(5.5,65.0,7.5)` on the ground before belt formation. Root viewed the
actual peer screenshot (SHA-256
`f55fdc81b2fa11fe72f80c7cc737aa72f07e950020c15b9bb64faaa4fd1b1ca6`);
it shows the peer at the fixture and an item near the funnel. Entity identity
and custody depend on command receipts, not the image alone.

The `source-belt` phase then made the successful south-face connector clicks.
The controller at `(4,65,7)` reported `Length: 5`; after 45 requested ticks it
reported `Speed: -32.0f`, but its `Inventory` was
`{Items: [], PositiveOrder: 1b}` when the
unchanged custody assertion required gold1. The driver stopped immediately:
`HOLD`, with no peer-occupied, receiver-release, peer-receiver, Depot,
peer-empty or final phase. The source-belt command log SHA-256 is
`a2a8462ec2ee5fd794b004114a54128e8283ecc6abe77d4d3f6a1ccd426ac4f7`;
the phase receipt retains the assertion. No belt-to-Depot-to-sink or final
single-item custody result can be claimed.

A bounded read-only live diagnosis after HOLD found one gold world entity
with the **same UUID**, count1, at `(5.5,64.0,5.908296256323984)`, `Age: 3713s`.
The captured diagnostic SHA-256 is
`6ac2e6922aa98c53a9662dfde51ade2502aec85ad065d8f44005166a14424051`.
The final scoped commands also reported chest Items empty, no Inventory
compound at the queried middle segment and no Depot Items path; they do not
constitute a complete post-HOLD custody census. A request for that census
arrived after normal session closure, so no further live reads are possible.
The preserved raw stdout/stderr SHA-256 values are
`5fc652d42146873bb7fa6ce72879972df9355ab14f7501905e60fe76947a0b6e`
and `c18ee0ca7117593a0fc5c91f5d6c67584fa5f6370803bba6fb1c6f3b2189bd48`.

## Bounded diagnosis and next proposal

Frozen `native/create/mixins/create/belt.rs` `capture` builds an item search
box whose minimum Y is each segment's `Y + 0.5`; for this belt at Y65 that is
65.5. It skips a dropped item below the minimum. Frozen
`native/create/mixins/create/funnel.rs` emits the extracted item at the
funnel block's Y, 66.0. The observed gold was already grounded at Y65.0 before
the belt existed. Thus the already settled item was below the new belt's
capture plane. Its later movement below the belt is consistent with an order
or geometry mismatch, but the post-HOLD sample was delayed and does not prove
every intermediate item step or exclude other causes.

For **a separately granted and reviewed future trial only**, propose the
minimal fixture order change: keep the source funnel powered by the existing
redstone block; after all baseline-zero, south-camera and shaft/air checks,
form the empty source belt and verify Length5, Speed-32, Inventory0 and no
gold. Then supply the single new gold1 to the chest, prove chest1 with every
other custody location still gold0, and remove the redstone power so the
funnel emits into the existing moving belt. Omit the old stick sentinel and
pre-belt world-item observation **from this new custody trial**: a moving
belt could capture the stick and defeat its obstruction role, while the
completed 005/001 observations already retain that separate qualification.
Preserve the one-item, actor/peer, receiver, Depot, hopper, sink and
world-drop census and ordinary peer occupied/empty views; do not move/redrop
the gold, extend its lifetime, patch the source/host, or weaken any ownership
assertion. Review the exact ordering and focused zero-census before any
runtime lease. This proposal is not a game result.

No public release, upstream contact, source patch or acceptance change is
included. Root directly inspected the peer image and scoped receipts.
Independent read-only Sol High diagnosis and draft review: **CLEAR for the
bounded report and future order proposal only**; custody remains HOLD.
