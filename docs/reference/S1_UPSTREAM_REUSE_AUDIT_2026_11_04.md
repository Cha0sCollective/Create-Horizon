# Upstream reuse audit, November 4, 2026

Read-only assessment; no engine upgrade, migration or S1 gate promotion. Owner
selected **Evaluate the existing Create port** after reviewing the initial findings.
This authorizes an isolated assessment, not adopting its behavior or changing G1.

## What was missed

Development used Horizon cf62fe270b243780f9aa86c49b456d9dd9078b42 from October27;
current-upstream overlap was not checked consistently before subsequent packets.
Official upstream now has major native-mod facilities that need evaluation before
more bespoke integration. Root independently fetched history for reference only;
no merge/rebase/checkout/build of that source occurred.

Audited upstream pin63eccfc3559807d338e93ca2fec1e1fdc077a721. GitHub compare reported:

- Fork default main27c3559421fc26354bc8dee232a263d8ab361ccd:13213 behind upstream.
- Accepted development9f7d5ecb7abee68b0f3af6363fbdb3e6b0547219:103 custom commits
  ahead of common baseline and13239 upstream commits behind.
- Local repository is shallow. Local rev-list produced a different behind count;
  it is not a reliable substitute for GitHub's canonical compare counts here.

[Default branch comparison](https://github.com/Cha0sCollective/Horizon/compare/main...horizon-rust:main)
[Development comparison](https://github.com/horizon-rust/horizon/compare/main...Cha0sCollective:codex/s1-ordinary-empty-counter)

## Official source overlap

Independent native-read-only Sol High audit by s1_op_source_review inspected official
source/docs at the exact upstream pin. No runtime proof or exhaustive commit audit.

| Need | Official upstream implementation | Implication |
|---|---|---|
| Mod execution | Compiled Rust native hooks/mod_api; Wasm game removal7ebf7d26c on Oct29 and crates2a8c60b6d on Oct30. | Old Wasm bridge is a custom architecture choice, rather than current upstream's mod route. |
| Persistent machine lifecycle | native_hooks new_block_entity/tick_mod_block_entity/put_back_mod_block_entity; block_entity_ticks; save.rs mod NBT/items codecs. | Creation, server tick, dirty marking and chunk persistence already exist. |
| Inventories and menus | machines/native_items insert/extract/move; native_menus expected-offer checked exchange; native_inventories live/file operations. | Strong overlap with planned slot plumbing and input/menu work. |
| Client state | Native update tags/explicit sync, lan/chunks encoding, remote client entity creation/tag updates. | Evaluate ordinary block-entity replication before a bespoke transport. |
| Moving visuals | native_block_entity_draw frame/tick Spin and retained geometry cache; native hook submission. | Broad H11 overlap; not proof of equivalence to our fixed GPU compact-update diagnostic. |
| Compatibility/server builds | Content identity/mod-list configuration; native dedicated server export. | Reusable foundation, not our exact allocation-bound captured admission contract. |

Confirmed same-problem overlap: mod_content.rs:1666 auto-adds declared mod Creative
items/tabs/search; game.rs:16629..16651 routes nonsearch/rebound hotbar keys through
inventory_hotkey; creative.rs:890+ performs display swaps. Parts of9f7d5ec solve
these same catalog/key problems on the old architecture. Exact equivalence of our
client item-model fallback fix was NOT established.

Native ticks mutate host state directly. These sources do NOT establish our Wasm
trap/fuel/invalid-output rollback, generation/replay contract, complete cross-owner
checkpoint selection or interrupted recovery. Preserve current evidence and these
requirements until the owner approves a concrete revision. Do not label103commits
obsolete or G1 complete from available APIs.

[Official native mod API](https://github.com/horizon-rust/horizon/blob/63eccfc3559807d338e93ca2fec1e1fdc077a721/docs/native-mods.md)
[Official trait API](https://github.com/horizon-rust/horizon/blob/63eccfc3559807d338e93ca2fec1e1fdc077a721/crates/horizon-client/src/mod_api.rs)

## Existing personal-port candidate

[Delete228/create-horizon](https://github.com/Delete228/create-horizon) publishes an
unofficial experimental Rust Create port. At main0.5, its README lists motor,
shaft, small/large cogs and millstone, plus press/depot/wheels/fan/drill and other
content. Thus its documented implemented set includes our initial S3 five blocks.
These are the maintainer's claims, not locally verified results.

Main's compatibility.json pins Horizonfc02103818c212920fd24aa34a2b2f91ba49e56f,
Mixinsa1e13326306c6bc074b11e6a4926deb551c4083e, native API359, Rust1.99.1,
Create6.0.10/ac0c444d9828da3453ae8cc65338e8de063286fb, sourceMC1.21.1/targetMC26.3.
Root verified the Horizon pin is an ancestor of official upstream/main. Our Create
reference is6.0.11, so compatibility/parity cannot be transferred silently.
Main documentation names validated source16c5d454b1b9860da01b78474ea5206f0664d384.
Its MIT license and asset instructions keep original art local to the player's JAR.

stage/parity-06 documents0.6.0-alpha.2,27blocks/20items/137recipes and estimates
8..10percent of the full port. It requires a separate pinned Horizon fork9c7f9b2
and pending support, and reports a real one-tick pressing mismatch. It is not a
complete Create replacement or a verified match for our acceptance profile.
The0.5 main is the smaller initial evaluation target; stage06 is context only.

[Main README](https://github.com/Delete228/create-horizon/blob/main/README.md)
[Main pins](https://github.com/Delete228/create-horizon/blob/main/compatibility.json)
[Reported validation](https://github.com/Delete228/create-horizon/blob/main/VALIDATION.md)
[Experimental06 limits](https://github.com/Delete228/create-horizon/blob/stage/parity-06/REPORT_06.md)

## Authorized evaluation and decision points

1. Pin and inspect the published0.5 source, build/package instructions, licenses,
   dependency/resource needs and source changes; retain current project unchanged.
2. Map existing implementation to our five-block gameplay objective and verify
   saved machine/inventory ownership, actual server authority and synchronization.
3. If its existing build route is usable here, run a focused disposable-world
   trial of motor/shaft/cogs/milling/save/reopen and dedicated/two real clients.
   Reuse its existing tools where suitable; do not replay its entire reported
   oracle/test battery or run the retired S1 wrappers against another engine.
4. Explicitly compare6.0.10 vs our6.0.11 reference and the pinned full-output item
   loss decision. Verify chunk/reconnect/multiplayer limitations and in-frame fit.
5. Report adoptable behavior, verified gaps, environmental blockers and a revised
   development plan before adopting an architecture or changing acceptance.

No third-party port implementation was read during the initial upstream audit;
only public README/pins/license/validation were consulted. Owner's subsequent
Evaluate selection authorizes source inspection and an isolated assessment now.
Do not contact maintainers, publish our private work or import third-party mod code
into the public engine candidate. No playable saves/Prism/Java settings or source
reference snapshots change. Keep assets/resources/raw runs outside Git.

## Preserved work

Accepted/public9f7d5ec and private accepted records remain intact. Current
codex/s1-configured-operation has exactly five dirty files within its six-path
assignment; raw source1 map8c89696b08028d9077aa4645008b6f7a4d3096d3dc07b6c3b20d1f3130b6c108
is frozen. SOURCE HOLD invalid TransferOutcome import is recorded separately;
no formatting/compiler/runtime has run for s1op1. No new s1op1 execution tools exist.
Do not fix/compile that candidate while assessing the upstream route. G1/S1 OPEN.
