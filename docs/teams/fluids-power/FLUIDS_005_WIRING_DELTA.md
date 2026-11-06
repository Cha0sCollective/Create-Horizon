# FLUIDS-005 — isolated wiring and content delta

**Status:** source-only integration proposal on `codex/fluids-005-wiring`,
based on independently reviewed source `3f4e6cd16503c1e98ab22aaca82315165c6e3191`
and accepted native baseline `55339ce3347b2ce667281e8a3e6318ec856144e4`.
No private version, cache schema, host or workspace pin is changed here.
Operations owns serial composition and the heavy build/runtime lease.

## Shared source and generated files

- `native/create/mixins/create/horizon-mixin.toml` declares only the existing
  private endpoint adapter as `create_port_fluid_pump_transfer`. Existing
  `create_port` hooks remain the entrypoints; no public Horizon API changes.
- `native/create/mixins/create/runtime.rs` owns Pump and Pipe, gives only
  Pump an ordinary kinetic block entity and tick, maps its `facing` axis to
  `Kind::Pump`, and calls the bounded adapter after signed network RPM is
  calculated. Pipe has no block entity, kinetic node, tick or fluid owner.
- `native/create/mixins/create/placement_runtime.rs` places Pump with the
  directional look-facing fallback and selects the straight Pipe axis from
  the clicked face. `interactions.rs` flips Pump facing on ordinary wrench;
  its kinetic axis stays the same. The previously reviewed Funnel/Pipe/Pump
  exclusions in `placement_runtime::level_ports` are retained.
- `native/create/tools/catalog.py` registers both IDs as selected fluid
  blocks. `tools/content.py` calls the standalone authored generator,
  supplies English/Russian labels and updates the creative tab. Generated
  outputs are `native/create/mixins/create/assets/src/catalog.rs`,
  `native/create/mixins/create/data/content.json`, two language files and
  the creative-tab JSON. The existing twelve pump/pipe JSON files are the
  authored inputs. No new model `PARTS` import is requested: the selected
  MODEL blockstates reference bundled authored model files directly.
- `native/create/mixins/create/assets/src/lib.rs` explicitly excludes only
  Pump and Pipe blockstate/item-model files from the local original-art
  overlay. The original 6.0.10 Pipe state is multipart over six connection
  booleans and cannot match the axis-only native Pipe. The bundled authored
  `axis=x|y|z` Pipe and six-facing Pump models therefore remain in the
  resource root. Other original asset imports remain untouched. One focused
  fixture test in `assets/src/tests.rs` supplies a synthetic multipart Pipe
  state and asserts that neither selected original state/item model is
  imported while an existing Shaft state still is. It has not run.

The Python content generator was unavailable in this worker environment.
The bundled index was regenerated from the checked-in content tree with
the generator's path-component sort order and identical JSON formatting.
Static comparison with its previous index shows **12 added paths, no
removed paths and only three changed prior values** (English, Russian and
creative tab). A later normal generator run must verify byte-identical
outputs before staging. Root retains the actual version/cache pin choice;
`assets/src/lib.rs` still has the accepted cache schema value.

## Fail-stop path and ordinary refusal

`fluid_pump_transfer.rs::tick` now refuses client, stopped, nonfinite or
out-of-range RPM before any endpoint callback. Invalid/unloaded/branched
line, unsupported Tank/Basin profile, other adjacent fluid owners, a Mixer
above Basin, empty source, or a full first Basin input also return ordinary
`Refused` before execute. Simulated fill/drain disagreement returns
`Refused` if both endpoint snapshots remain unchanged.

Only two paths enter `restore_or_fault`: (1) a supposedly simulated native
callback changed an endpoint owner, or (2) an execute receipt, canonical
delta, permitted field, view or total-water check failed after execute.
That function attempts both full NBT/item restorations and resyncs, then
re-reads endpoint identity, exact snapshots and the four-block line. If
everything is restored, tick returns `Restored`; if either restoration or
readback fails it panics and may stop the process. Ordinary capacity,
topology and 0-RPM refusals cannot reach that panic. The transient Basin
`contents_changed` flag is not rolled back; the adapter refuses an active
Mixer above the empty-item Basin, so its later `mixer::schedule` returns.
This design does not guarantee cross-owner crash atomicity or replay.
Actual callback failure/restoration still needs a focused disposable test
under Operations' lease before gameplay qualification.

## Next integration checks

Independent read-only review should inspect the shared wiring, exact
content/index outputs, authored fallback pack priority, and fail-stop
reachability. Under a later granted heavy slot, run focused Pump kinetics,
`pump_transfer` and selected asset-import tests, then compile/lint and test
the four-block integrated/dedicated normal cold recipe from
`FLUIDS_005_TRANSFER_HANDOFF.md`. A focused injected endpoint failure must
verify full NBT restoration and census in a disposable fixture, never a
playable world. No broader fluid network, flow persistence, public host API,
new transaction layer or active-tick crash guarantee is included.
