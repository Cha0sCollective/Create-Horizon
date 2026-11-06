# Transport-004 Funnel runtime HOLD and isolated repair

Status: **HOLD, not ordinary-game acceptance**. The only granted runtime lease,
`transport-004-funnel-runtime`, was explicitly released after all owned
processes exited. The separate repair below has source-review CLEAR but has
not been staged, compiled or rerun in game.

## Frozen run and exits

- Candidate `565bfd35cb37f9cddcb193f68d089a25bd7d785d`, native tree
  `803247026b04bd4abda8d497ab34dd001367bad0` (.12/cache15), private
  Mixins `ac5d9e766380d575838c8250f2a7982281c1f44e`.
- `source-inputs-r2.json` SHA-256
  `61604157828aa246a1e2eccd891e6834cf5373fc4d9a5911c3e9da3ac847cf79`;
  `binaries.json` SHA-256
  `9ec5a68eadde0214a76335f62c0a63b1e87b3ef9b86655c315b1c08299503e7d`.
  Frozen game binary SHA-256
  `6b4c0f7ff4bca2be7b4c2550831b27b744c84d96be54aab0fb02bc09a4341edc`,
  control binary
  `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`.
- Two dedicated launches in the same disposable world each exited server,
  actor and actual peer with code 0. The separate integrated launch exited
  actor with code 0. All were quit normally and launcher stop requests
  completed. `G:\Create Rusty\.local\transport-004\receipts\post-run-verification.json`
  SHA-256 `d314fd9ce5617cd9b6f001ce99dfb48ee8c39aad7f8c74bb1845353c4fa9dcf5`
  verifies all 692 source hashes, HEAD, both binary hashes and all three
  session/exit receipts after shutdown.

## Retained attempt history

1. First dedicated setup stopped before any control command because WSL Git
   could not follow the Windows worktree `.git` pointer. Its HOLD receipt and
   empty command log remain. A read-only HEAD adapter was independently
   reviewed CLEAR; subsequent setup passed.
2. First interaction stopped at a fixture sightline: a setup stone occluded
   the chest north face from the original actor position. The two grounded
   approach coordinates were changed, independently reviewed CLEAR, setup
   rerun, and direct real `look-at block 3 67 7 north` returned `ok:true`.
   The first two HOLD receipts remain intact.
3. Dedicated stone-in-hand north-face click opened the unrelated chest.
   The following ordinary mainhand FunnelItem north-face click returned
   `changed:false`, `screen:playing` and local `inventory_change:-1`. The
   adjacent block stayed air; actor client showed funnel2 from3 even after
   20 more ticks. The driver stopped at its exact-inventory assertion.
   Raw log SHA-256
   `b0d15ff6d739e6b07aaa5c9c14568540a550539891035263e79ae33e7b0f1274`;
   HOLD receipt SHA-256
   `8b67ce6eb82c49f0961777ca266abadade1280da534bb13e8f0d74ca566a403b`.
   `side-click-HOLD.md` preserves the bounded follow-up observations.
4. The same saved dedicated world was reopened without restoring stock.
   Token-delimited `data get entity @a[name=TeamActor,limit=1] Inventory`
   from actor and actual peer both reported authoritative funnel3; the
   reconnected actor client also showed funnel3. Nearby funnel blocks0 and
   world items0. This is a persistent **client prediction loss until
   reconnect**, not authoritative item loss. Census receipt
   `side-click-census.json` SHA-256
   `1b0a9bfe4140bd7992befc2bdb0fe6acc582651f9d172e494f98b9a51829e68c`.
5. With that untouched stock, one supported underside chest click hit face
   down and placed
   `create:andesite_funnel[extracting=true,facing=down,powered=true]` at
   `(5,66,7)` below the chest. Actor and peer server queries and actor client
   all reported funnel2 afterward; actor remained in the playing screen.
   `supported-downward-once.json` SHA-256
   `fa760f04b8ebbbaab82631a3c9ded61fc6724bc34cbb28551832c8d58b16d4a8`,
   with raw log and actual actor screenshot in the same receipt folder.
   The screenshot shows chest/funnel/redstone but chat overlays part of it.
6. Integrated setup stopped before any interaction because a residual item
   remained after `kill @e[type=!minecraft:player]`. Its raw HOLD receipt is
   preserved. No integrated chest-click or both-mode result exists.

The dedicated side refusal, physical Shift, offhand, fan port, full
world-item-to-belt-to-Depot-to-sink custody and actual peer occupied/empty
images remain unqualified. Server `data get` from the peer is server evidence,
not a peer presentation image.

## Isolated source repair

Clean private worktree `G:\Create Rusty\.local\teams\transport-funnel-repair`,
branch `codex/transport-funnel-side-refusal`, base
`565bfd35cb37f9cddcb193f68d089a25bd7d785d`, candidate head
`bce9ed1173590ecc15c8414021e43987b8b5fb40`. Exactly two files differ:

| File | New SHA-256 |
| --- | --- |
| `native/create/mixins/create/funnel_prediction.rs` | `ff7edb7d344f5bd0c40e66fd1ede240aac5d5313ccbfc0737b98638ae4867646` |
| `native/create/mixins/create/horizon-mixin.toml` | `fa3b811b9d5c9afbc6e76402874c103969cde4c9eeec8d647cd47d884a3dfd25` |

Pinned Horizon's remote `DryLevel` skips the native-authoritative placement
state hook. The existing blanket chest `Pass` therefore reaches generic
`remote_can_place`, predicting a side placement and locally decrementing the
stack even though the server rejects it. The new private cancellable HEAD
returns `ControlFlow::Break(None)` from `remote_can_place` only for an enabled
Create Andesite Funnel. The packet still reaches the server; the server's
existing face, Shift, look and support validation decides placement and stock.
This covers both hands without copying placement rules. Existing Java-style
face-independent chest-use suppression remains, as do unrelated items.

Independent read-only Sol High audit and exact-delta review both gave CLEAR.
The reviewer verified the pinned Mixins `ControlFlow` contract and target
signature; `rustfmt --check` on the new helper and `git diff --check` passed.
No host API, schema, pin, public source or acceptance change was made.

## Required continuation

Operations owns integration, exact-hook stage, Clippy/build and the next
runtime reservation. Recheck the side face with actor client and authoritative
server count without reconnect, then the supported underside click, physical
Shift, offhand, stone-held chest open, fan port, exact gold1 custody and actual
peer occupied/empty images. Correct the integrated setup residual-item fixture
without erasing this HOLD before claiming its chest interaction regression.
Only actual completed checks can support a both-mode claim. Obtain independent
completion review before acceptance; preserve every failure and source hash.
