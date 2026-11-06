# Machinery 004 — isolated water + wheel source composition

**CLEAR as a source-only proposal. Not adopted or runtime-qualified.** This
packet follows the owner's approval of the bounded stationary Crushing Wheels
controller. It does not change the original design decision's historical
DESIGN/HOLD provenance, authorize public action, or extend the selected recipe
and orientation boundary.

## Stable inputs and output

- Base: reviewed water-line006
  `22087014723ac08c53e0c1f44d72866a91a90248`.
- Isolated worktree:
  `G:\Create Rusty\.local\workers\machinery-004-water-wheel-wiring`, branch
  `codex/machinery-004-water-wheel-wiring`.
- Clean HEAD: `f05d8ccfb1e164d865eb923f7156c68546780fd2`.
- Source composition: the reviewed current005 wheel commits equivalent to
  `9651dba12d163896d45a8cc4ba1b0b3b0da1feff..5ed810133b2c8974472d95ec6146f6ac47bbca11`
  were cherry-picked in order. The one kinetics conflict combined Pump's
  existing shaftless small-cog behavior and 4 SU/RPM with Crushing Wheel's
  axial ports and 8 SU/RPM. Neither S2 destructive eligibility nor Pump
  behavior was broadened.
- Shared-wiring commit: `f05d8cc` adds only source/content integration in this
  isolated candidate. It does not touch the primary/root006/active005 trees,
  host, version, permissions, profile or cache schema.

## Isolated shared delta

The shared commit exports the wheel cycle, registers the adapter in
`horizon-mixin.toml`, connects block-change and pre-block-entity level
reconciliation, registers the wheel/controller as owned block entities, maps
only the wheel to the kinetic Node, and dispatches the controller before the
kinetic Node guard. It passes the current solved wheel RPM while its block
entity is detached. It registers the 32-slot no-sided-access container and
the wheel/controller render path. The internal controller is excluded from
placement shaft-port inference; Pump and Pipe exclusions remain.

Two HEAD helpers bind existing pinned host API 146's
`native_entity_tick_listened` and `native_entity_tick_pre` to the selected
one-item contact adapter. The pinned `native_block_entity_tick` callback is
server-only. No host API or persisted in-contact ID/UUID was added.

`tools/catalog.py` and generated `assets/src/catalog.rs` include the wheel
block and its `crushing_wheel/block` local OBJ import part. The controller
remains an internal, no-item block. The selected `tools/crushing_content.py`
generator retains the nine authored wheel/controller JSON files after shared
content generation, adds English/Russian wheel labels and the creative-tab
entry, and regenerates the packaged content bundle. No crushing recipe was
added; the one-item cobblestone path uses the existing milling fallback.

## Verification and required root gate

The source generator ran under the supplied native Python with `-X utf8`;
generated text was normalized to LF. Python AST parsing of the three changed
tools, TOML parsing with 83 unique injection IDs, all nine wheel JSON parses,
and `git diff --check` passed. The bundle has **499 entries** (limit 512), with
all nine wheel file strings matching its entries. The candidate worktree is
clean. No Cargo, WSL, Java, stage, game or control process ran here.

Fresh independent read-only Sol High review of the complete
`2208701..f05d8cc` composition returned **CLEAR for the source-only
proposal**. It checked all 28 changed files and effects on Pump/Pipe/Funnel,
goggles, Quill, inert contraption admission, the five-type destructive
profile, and the unchanged native fluid restoration test. Review ran
2026-11-06 04:34:26–04:37:48 UTC (202 s); token telemetry is unknown.
The team lead's implementation/coordination phase ran 04:28:35–04:38:55 UTC
(620.126 s in the phase ledger). Build and game-test time here are zero;
verified token counters remain unknown.

**Before runtime adoption, the project lead must assign the new combined
profile and invalidate the existing asset import cache.** The import catalog
now requires wheel resources, but `native/create/mixins/create/assets/src/lib.rs`
still sets `SCHEMA = 17` (lines 17–19 explicitly require a bump when the
catalog changes). Its cache validation checks the old receipt's file set;
a valid water-line v17 cache can omit the new wheel OBJ/model indefinitely.
Root owns the schema bump or equivalent required-resource invalidation under
this allocation. This isolated candidate intentionally leaves it untouched.

Compilation, real controller intake/spill, integrated and dedicated/two-client
views, output pickup and cold reopen remain separate root/Transport lease
work. Preserve failed receipts and distinguish server state from observed
client replication. No runtime acceptance follows from the source review.
