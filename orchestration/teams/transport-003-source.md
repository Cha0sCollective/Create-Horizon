# Transport-003 source handoff

Status: source candidate committed on November 5, 2026; independent read-only
Sol High source review CLEAR after a one-line cache repair. No shared build or
game lease was assigned to Transport.

## Revision and scope

- Lane: `G:\Create Rusty\.local\teams\transport`, branch
  `codex/team-transport`; base `d499585ee8dccba62205cc0303d3f174aa6d7ac4`,
  source `80198967b0729583f78b95c1583147aef860b2f8` plus cache repair
  `1c14bc924145f044b4b3e2e0df0e934f44d3c944`.
- Accepted native tree before this packet: `fc71cb5f0fb0c038853902a7d2f775085b34a0f1`.
  Pinned inspected host: `9c7f9b2215dfed497aba5522c44f38926628648d`.
- Only private Create source, generated native content and provenance changed.
  No host source, active pin, general framework, shared fixture or public
  release changed. The funnel uses the accepted belt and Depot custody path.

## Candidate behavior

The private `use_hand` mixin treats an unsneaked, main-hand Andesite Funnel
click on a chest as secondary use for that one chest block-use decision.
Actual Shift remains unchanged in the later BlockItem placement context.
The matching remote client hook passes chest reaction so normal placement
prediction runs. The placement adapter accepts only the selected standing
down-facing extractor below a chest when the player looks up. It cleanly
refuses Shift, other facing choices and non-chest backing; Belt Funnel is
separate.

The entity adapter checks redstone before extraction, previews the chest's
downward-accessible slot, stops for an existing outlet item, extracts at most
one item, spawns one world item at the down outlet, and starts an eight-tick
cooldown. Existing belt world-item capture then owns belt custody. New content
includes the block, item, states, shape, provisional vanilla-textured model,
loot, creative entry and language. Both native notices cite the Create MIT
funnel classes and logistics config used as references.
The asset importer schema advances from 12 to 13 so an existing valid cache
cannot omit newly cataloged funnel resources. The project lead should reconcile
this shared version against any other team's catalog additions at integration.

## Checks and limits

- Regenerated committed content with `tools/content.py`; TOML parsed, both
  changed Python sources parsed, and the bundled `mixins/create/data/content.json`
  exactly matched the content tree. Seven selected new/generated JSON files
  parsed. `git diff --check` passed before commit.
- The existing `tools/check_distribution.py` stops at its recipe-count
  assertion: the content tree has 127 recipe JSON files and the reference list
  has 123. This packet changes no recipe file. The two NOTICE files also
  differed at the base revision because only the root notice contains the
  private S5 feasibility paragraph; the funnel provenance was added to both.
  These baseline differences remain for the project lead to resolve.
- No Cargo/staging/game process was run. The source candidate still needs
  compile validation and the allocated one-mod dedicated/peer fixture under
  the project lead's heavy lease. Verify ordinary chest-click placement,
  powered pause, source-to-world-to-belt-to-Depot-to-sink exact item count,
  obstruction, cooldown and peer occupied-to-empty presentation.
- `native_extract_item` commits before `Game::spawn_item`, which returns no
  acceptance result. The fixture can prove observed custody for its case;
  arbitrary spawn veto conservation remains an explicit host API gap.

Independent `/root/funnel_source_review` on Sol High found the stale importer
schema in the first source commit (P2). Its focused review confirmed the
schema-13 repair at `1c14bc9` and returned CLEAR for the repaired source. No
further source finding was reported. Compilation, mixin expansion and gameplay
remain unverified, and review excludes this handoff document.
