# Native S3 — PASS within the declared scope

Native S3 is COMPLETE. Fresh independent read-only Sol High completion review
found no blocking gaps or material misclaims. S1/S2 stay accepted. Its scope is
`docs/design/NATIVE_S3_MVP_PACKET.md`, independently reviewed before changes.
This is the native five-type Creative MVP, not a historical G3 decision or full
Create/modpack qualification.

## Reuse and changes

Existing native placement, motor board/network controls, millstone intake and
collection already supplied the gameplay. Existing Horizon UI/input, inventory,
save/content/native build and mod-list comparison supplied the host services.
No game/engine gameplay algorithm or protocol was added in S3.

The private revision declares `0.5.0-ch4os.1` consistently in existing native,
mixin, content and bundled-content metadata, retaining `create:` IDs, upstream
provenance and exact compatible pins. The existing configuration exchange uses
that declared version to distinguish the private revision from upstream0.5.0.
It does not authenticate arbitrary changed native code that declares the same
version; folder content also has no archive hash.

The existing developer play helper now accepts a separate owned instance root,
fixed test terrain, offscreen rendering and a local control socket. It refuses
an unowned nonempty destination, a foreign marker or redirected direct managed
paths. Its default route remains available. The existing stage build also builds
the matching control tool. `native/PLAY_PRIVATE.md` documents this private Linux
source-build/play route, including explicit mob-free setup in a new empty world.
No public launcher registry, release distribution or general installer was added.

## Actual player workflow

Integrated `integrated-z79z7hpy` and dedicated `dedicated-1106jy8n` PASS. The
integrated run actually launches through `tools/play.py` with a fresh owned root;
it reopens that same isolated instance and preserves player gravel and motor
setting. The dedicated run has a server and two matching rendered clients.

Both modes perform all six connected Create placements through held-item/right-
click interaction: motor, shaft, small cog, large cog, second small cog, millstone.
The motor is ordinarily oriented upward; full states are recorded. The actor
uses the actual motor board to select64,−32,64, and authoritative observations
confirm motor setting and millstone speeds−64,+32,−64. Root inspected both board
screenshots, including the drawn64 setting and Done button.

The actor selects its two cobblestone and presses Q twice over the millstone.
Its actual inventory reaches zero cobblestone. The machine produces two gravel;
the empty-handed actor right-clicks to collect both, leaving its output empty.
The dedicated peer retains zero gravel and sees the same full block states.
Machine NBT comes from correlated authoritative queries; player inventories and
dedicated peer states are client observations. No inventory-NBT insertion,
summoned item or speed-NBT write substitutes for these player actions.

World preparation, a vanilla stone scaffold, stock provisioning, time, mob rule
and initial actor placement are explicit fixture setup. This does not qualify
survival material acquisition. Subsequent movement uses the normal walking path;
S1's unchanged custody/output-loss evidence and S2's unchanged arbitration,
topology/reopen evidence are reused, not rerun. The scaffold occludes lower
machinery in the final S3 chain screenshots; full states/actions and S2's visible
assembly evidence supply those distinct observations. Root inspected all images.

## Existing compatibility refusals

After the matching peer leaves, two fresh isolated rendered clients are refused
before play by the existing host exchange. Both show the disconnected menu and
`world=null`; root inspected their actual screenshots:

- Disabled Create: `Missing: create 0.5.0-ch4os.1`.
- Preserved S1 binary/upstream0.5.0: `Different version or file: create
  (server 0.5.0-ch4os.1, yours 0.5.0)`.

The positive clients are the exact matching build. The server keeps its whitelist
enabled and listens only on localhost. All seven orchestrated runs exit0: two
integrated helper/game runs, dedicated actor/peer, two refused clients and server.
The helper's checked child execution also requires the game to return0.

## Checks and provenance

Fresh read-only independent Sol High reviews cleared the S3 packet, source patch
and driver, including bounded setup corrections. Python compilation, unowned-
instance refusal and all four version/embedded-manifest consistency checks pass.
The locked build takes133.864seconds total with41.13seconds in the compiler.
No Rust gameplay source changed, so S2's focused core checks are reused.
S2 binaries were verified and copied to Linux cache/native-s2-baseline-bin before
the build cache was reused. S1/legacy work, original failures and worlds remain.

First integrated attempt `integrated-ebtj8e8j` stopped before placement because
the scaffold blocked the floor face. Second `integrated-pgep8nar` correctly placed
an upward motor, then stopped on a driver's wrong response-field lookup (`block`
ID instead of full `state`). Both remain FALSE/incomplete with clean game returns;
exact drivers/maps remain in local `attempt1`/`attempt2`. Only setup/movement and
the field check changed; the later peer comparison became stricter. An initial
build-checker's wrong TOML table lookup also remains recorded before compilation.

Pins remain port16c5d454, official Horizonfc021038, Mixinsa1e13326/API359,
Rust1.99.1, Minecraft26.3; Java reference6.0.11/7ff2760e/NeoForge21.1.219.
Original art/base stays6.0.10. Port main10cb93e6 and experimental64b35ad3 were
refreshed; official main6319cb70 adds hit outlines and was not needed for S3.

Exact main input map:
`c880261c7e8603464a2c963c8122f3c194618832c58fba43fa25560af35a5e10`.
Driver `4164b877aa7ea8c147a51cd5a532ee4b46d326fa1142aebd2b9c4aee684134a0`.
Game `f12a7cdcf1fb1d434a75e1079357a0b159cf82541dafe9c42f94569e16e0e4b8`;
control `a1d6054a746623085a88134d01f261fff86e2eed2cef25f413e93047f8f71970`.
Integrated raw result76effb54; dedicatedb0ee087b. Source/build and28260 declared
inputs are rehashed before launch. This is observed build binding, not compiler
reproducibility proof. Raw receipts/transcripts/images remain local under
`.local/native-s3-01/` and the named Linux scratch runs; they do not enter Git.

Supported scope stays fixed mob-free Creative test terrain, the five types and
plain cobblestone-to-gravel milling, integrated and matching dedicated clients.
Open to LAN, arbitrary worlds/modpacks, other imported machines, survival,
Java/world import, performance and custom crash/replay guarantees remain outside
S3. Stage4 introduces its own already-authorized survival/ordinary-terrain packet.
