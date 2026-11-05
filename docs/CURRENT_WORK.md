# Current work

Native S1 through S5 are COMPLETE under the owner-adopted November4 native
route. S6 is OPEN. The owner authorizes progress through S6 until the next
requested break, genuine blocker, required input or scope boundary. Read
`docs/design/STAGE_PROGRESSION_PLAN.md` and `NATIVE_PLAYABILITY_PLAN.md`.

## Active dry-mixing packet

Branch `codex/native-s6-basin-mixer` starts from accepted/private-synced `6defc66`.
Fresh upstream236c87a8 and official main85d8e283 were read before work. The
upstream Basin/Mixer/heating code is reusable, but full0.7 import is HOLD for
host/type/schema/shared-behavior differences. Independent Sol High reuse and
audit/packet review CLEAR for `docs/design/NATIVE_S6_BASIN_REUSE_AUDIT.md` and
`NATIVE_S6_DRY_MIXING_PACKET.md`. Implementation is starting; no new gameplay
profile is accepted. Retain tested host/Mixins pins. Only dry plain andesite +
iron nugget -> alloy is admitted; heat/full fluids are subsequent decisions.
Source/effects review is required before live use. Root alone writes/builds/runs.

Next: narrow Basin/Mixer/codec/cache import, private entity-identity comparator,
selected content and Mixer-only kinetic cases; then focused checks and review.
Stop before a broad host/API/schema/lifecycle framework or changed acceptance.

## Accepted portable-interface packet

Private branch `codex/native-s6-portable-interface`; reviewed source checkpoint
`6376c3a`, based on accepted mounted-depot `a77af67`. Fresh independent read-only
Sol High completion review CLEAR. Result:
`docs/reference/NATIVE_S6_PORTABLE_ITEM_RESULTS.md`. Audit/packet:
`docs/design/NATIVE_S6_PORTABLE_ITEM_AUDIT.md` and
`NATIVE_S6_PORTABLE_ITEM_PACKET.md`. Native/content version `0.5.0-ch4os.4`.

One one-pole/one-cell piston carries a depot/plain iron ingot and one portable
interface. Its sole canonical item slot is exposed to one vanilla hopper/chest
through a live checked private view; no stationary buffer or new host API.
Integrated and dedicated/two rendered clients passed motion, connection/stall,
stock transfer, timeout/resume, empty settlement/nonsticky return, refusal,
ordinary collection and client occupied-to-empty presentation. A normal cold
connected reopen preserved exactly one saved ingot. Both actual ticker orders
passed. Focused constructed partial-capture/restoration retry and stone regression
passed. Four core/four host tests, manifests, formatting, Clippy27warnings and
locked build passed. The accepted sessions total17 normal process exits0.
No game is running.

Input map `74a642c6ec94818e1c3b6d6003472884d1c9f3a947571d5e2ccca686db58a847`;
game45e08336/ctl507b23e1. Raw packet `.local/native-s6-portable-01`, final
native-read mirror `evidence-complete/`. Preserve pre-cap map4b4cb3fc and FALSE
motion, insufficient319/320 initial fill, unqualified socket restart and camera
attempts. The result distinguishes failed recipes from passed game observations.
Do not rerun old drivers against newer sources or relabel their evidence.

Accepted result and source are privately synced at `6defc66`. Fresh upstream
source at236c87a8 contains Basin/Mixer, heating and water/lava Basin handling;
the bounded dry packet above selects reuse without a host upgrade.
Ask before broad host/API/schema or general contraption architecture changes,
changed acceptance outcomes, or other scope expansion. This packet does not
complete S6, broad Create, the modpack, crafting progression or crash atomicity.

## Accepted stages and pins

S1: `docs/reference/NATIVE_S1_PLAYABILITY_RESULTS.md`, branch
`codex/native-create-playability`. S2 `c0905ca6`/`NATIVE_S2_KINETICS_RESULTS.md`;
S3 `67566136`/`NATIVE_S3_MVP_RESULTS.md`; private guide `native/PLAY_PRIVATE.md`.
S4 `b88eb47b`/`NATIVE_S4_FACTORY_RESULTS.md`;
S5 `9b806837`/`NATIVE_S5_ACTUATOR_RESULTS.md`.
Earlier S6 families: pressing `f6e3d74`/`NATIVE_S6_PRESSING_RESULTS.md`, mounted
depot `a77af67`/`NATIVE_S6_MOUNTED_DEPOT_RESULTS.md`. These checkpoints are
privately synced; their exact claims/failed attempts remain in their results.

Pins: port16c5d454, official Horizonfc021038, Mixinsa1e13326, native API359,
Rust1.99.1, Minecraft26.3. Java reference6.0.11/7ff2760e/NeoForge21.1.219;
base/local art6.0.10 with selected differences explicitly qualified. IDs are
`create:`. `native/create/` has its own Cargo workspace; root Cargo and old
Wasm/native-host runners remain historical. S1-S3 qualify five types in fixed
mob-free Creative TestTerrain. S4 adds one Survival water-wheel/hopper mill in
normal terrain; S5 and S6 add only their recorded finite profiles. Open to LAN,
custom crash/replay framework, broad compatibility and Java/world import remain
deferred/outside scope. Ordinary native save/reopen is a distinct claim.

## Preserved state and publication

Coordinator implements directly; root alone performs serial writes, compiler,
runtime, WSL and Git. Required Sol High reviewers are read-only, no standing team.
Private pushes need fresh PRIVATE metadata, exact origin and normal hooks.
Public engine publishing needs a separate bounded assignment and FINAL/privacy
review. No merge, release, upstream message, cleanup or playable/Prism/Java edit.

Old engine `.local/worktrees/s1-native-close-composition` stays frozen on
`codex/s1-configured-operation` at9f7d5ec with five dirty files, original map8c89696b
and invalid TransferOutcome-import HOLD. Preserve references, worlds, caches,
prior binaries and every historical FALSE/HOLD; do not fix/format/build that tree.
