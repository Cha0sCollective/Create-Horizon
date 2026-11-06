# QUILL-002 shared wiring and content proposal

This isolated candidate starts at the reviewed Quill source
`eb8f0a3610ff3e247a00ba7d0fca8434ebf51b40`. The project lead owns
serial composition into the active private source, host-wide build/runtime
reservation, version and resource-cache key. The native package stays at
`.11`; this proposal does not change any active pin or package version.

## Shared files

- `native/create/mixins/create/engine/src/lib.rs` exports the already reviewed
  pure `quill` module.
- `native/create/mixins/create/horizon-mixin.toml` installs the distinct
  `create_port_quill` Game module and binds five existing named hooks: `key`,
  `interaction_key`, `frame`, `hud` and `client_level_unload`. The Quill key
  press queue feeds the cancellable interaction before ordinary item use;
  held-use repeats remain canceled without advancing a corner. The unload
  helper resets state. Experience's `presentation.rs` is untouched.
- `native/create/tools/catalog.py` adds only `schematic_and_quill` to the
  standalone item catalog. `native/create/tools/content.py` gives it a
  one-item stack, inert vanilla paper template, feather-textured provisional
  model and English fallback name. The generator carries it into the final
  creative tab, not only the bootstrap tab.
- The regenerated public files are
  `native/create/content/data/create/horizon/item/schematic_and_quill.json`,
  `native/create/content/assets/create/models/item/schematic_and_quill.json`,
  `native/create/content/assets/create/items/schematic_and_quill.json`,
  `native/create/content/assets/create/lang/en_us.json`,
  `native/create/content/assets/create/lang/ru_ru.json`,
  `native/create/content/data/create/horizon/creative_tab/kinetics.json`,
  `native/create/mixins/create/assets/src/catalog.rs` and
  `native/create/mixins/create/data/content.json`.

The original Create texture/model are read only from the user's private
6.0.10 local JAR by the existing `ITEMS` importer. Exact JAR/model/texture
SHA-256 values and the distinct Java 6.0.11 source references are in
`QUILL_001_PACKET.md`. No original art, JAR, prepared cache or world data is
committed. The authored vanilla fallback remains valid without the local art.

## Checks and integration plan

In this worker, `tools/content.py` regenerated the content, and the read-only
`tools/check_distribution.py` passed its pin, bundle, catalog, declarations,
creative-tab, no-imported-binary and notice checks. The manifest parses with
five distinct Quill injection IDs and the intended hook names. Pinned Horizon
`native_hooks.rs` provides each named callback with matching game, key,
button, hand, swing, cancel, batch and unload arguments. The source-only
proposal does not qualify a compiled or played UI.

After root composes the packet and selects the next package/cache values,
compile the composed helpers and core, run the five pure Quill tests, then
use one disposable integrated/dedicated two-client milestone. Check an
ordinary first and second click, a first click held through repeated-use
intervals followed by release and a second click, quick separate clicks,
reversed corners, 16/17-axis boundaries, coordinate and size HUD, cancel,
item/screen/world reset, and another held item's ordinary interaction.
Confirm blocks and item counts stay unchanged and that the peer never sees
the actor's selection. Context-sensitive replaceability remains an in-game
check because the reviewed client uses the host's raw replaceable flag.

The host's ordinary ray range replaces Java's 75-block reach. World-space
outline, schematic save/export/upload, placement, recipe/progression, train
graph and durable owner/schema are outside this proposal.
