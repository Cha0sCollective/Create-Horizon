# Transport-004 Funnel ordinary-game preparation

Status: **prepared, independently reviewed CLEAR for preparation only**. No
game, server, staging, Cargo, Java or source mutation was performed by Transport
for this milestone. Operations owns the later runtime reservation and acceptance.

## Exact packet and frozen-input contract

- Retained `G:\Create Rusty\.local\transport-003\driver.py` remains unchanged,
  SHA-256 `5175fafe17a445fbf6b507f97a665a09cd54e5ebdf54ee64ea9161542e96944b`.
- New `G:\Create Rusty\.local\transport-004\driver.py`, SHA-256
  `84bdaf6d9000eaff98fbf6ae66cad545703c5c075bfd518e692e92898fa40451`.
  Packet `PLAN.md` SHA-256
  `32c3060b82abf0fb23ac3e24d12b7a5656facc844c4f0bcc07a8cd32868dfa9b`.
- Root launcher `G:\Create Rusty\.local\native-team-runtime-004\session.py`,
  observed SHA-256 `fda66018a487ea7612464bf999d3bbba29363b473268cc0b8d8ccfd1410fe883`.
  Transport did not edit it.
- Current frozen source map `G:\Create Rusty\.local\native-team-integration-004\source-inputs-r2.json`,
  SHA-256 `61604157828aa246a1e2eccd891e6834cf5373fc4d9a5911c3e9da3ac847cf79`:
  Git HEAD `565bfd35cb37f9cddcb193f68d089a25bd7d785d`, native tree
  `803247026b04bd4abda8d497ab34dd001367bad0`, 692 listed files.
  The independent reviewer rehashed all 692 files and verified HEAD/tree.
  The first `source-inputs.json` and its format-failure receipt remain preserved.
- The pending `binaries.json` must name `source_inputs: source-inputs-r2.json`.
  The driver resolves only the source map declared by that binary map and
  rejects a different explicit map. It checks source, binary, session and
  control hashes before and after each phase, plus live roles, sockets,
  dedicated server readiness and the preceding phase receipt.

## Planned ordinary-game evidence

Dedicated sequence: setup; chest interactions; fan-port; receiver; paused and
obstructed funnel; one naturally released world item; source belt; peer observed
source occupancy; receiver release; peer observed receiver occupancy; Depot;
peer observed empty receiver; final sink. The actor performs a normal unsneaked
mainhand chest underside click that must place the powered extracting funnel.
Separate actual controls check an unrelated stone-in-hand chest opens, side
placement refusal, physical Shift refusal and offhand funnel refusal. Fan
placement at an existing funnel port is checked without replaying Saw, Tank or
audio batteries.

The custody sequence starts with exactly one gold ingot in the source chest.
After natural funnel output, it demands the world item, source and receiver
belts, Depot, then sink hopper/chest in order, with no second injection.
Actor/peer inventories, relevant block entities, belts and world drops are
checked. The actual peer is moved to grounded, aimed viewpoints for
world-item, occupied/empty belts and final images. Peer data queries are
server evidence; screenshots still need visual review before any client
presentation claim.

Integrated sequence is only setup and chest interactions. A later both-mode
claim must be limited to checks actually completed in both modes; integrated
custody is not planned by this packet.

## Preparation check and remaining gate

Plan-only WSL invocations for dedicated `final` and integrated `interactions`
returned the expected phase sequences and `PLAN_ONLY_NO_GAME_RUN`; they read no
binary map and contacted no game. A separate independent read-only Sol High
review compared the entire retained-to-new driver diff and inspected the root
launcher/build-map producer. It gave CLEAR on the exact driver and plan hashes
above. The reviewer did not run game/build commands.

Operations must finish actual compile proof and freeze `binaries.json`, then
grant the sole runtime lease and launch a genuine dedicated and integrated
session. The driver requires that lease ID and session; each phase writes raw
control argv/replies, failure receipts, source/binary checks and uniquely named
screenshots under `.local/transport-004/receipts`. A completed command phase is
review-required, not a gameplay PASS. Keep any failed attempt intact and do not
fabricate source maps, helper calls, item transport or screenshot conclusions.
