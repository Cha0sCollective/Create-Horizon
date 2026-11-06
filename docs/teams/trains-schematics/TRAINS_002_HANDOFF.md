# TRAINS-002 design handoff

Status: initial source/design CLEAR, follow-on decision delta review pending;
no runtime packet executed. Team branch
`codex/team-trains-schematics`, baseline
`99d97fb187c3e1b129d5d8ae8908220b5acd2622`, reviewed design commit
`b405d372a6ca25a43a21f10e428311e33555ba84`. Primary accepted source
read at `55339ce3347b2ce667281e8a3e6318ec856144e4` (`.11/cache14`),
primary documentation checkpoint `8470fc515d460a95a0cacabc644160f9e7aba4b5`.

- `TRAINS_002_ENTITY_OWNER_REQUIREMENTS.md` gives Contraptions a source-backed
  stable-ID, portable-payload, transform/contact/custody and normal-reopen
  contract. It separates first bearing needs from deferred train graph, bogey,
  cargo, passenger, cross-dimension and global persistence work. Java's global
  railway saved data, not its unsaved carriage projection entity, is the
  durable train authority.
- `TRAINS_002_STRUCTURE_SERVICE_PROPOSAL.md` records pinned Horizon's in-game
  vanilla structure-block Save/Load as a reusable host workflow. Its next
  proposed **Create-specific** packet is the schematic-and-quill item's
  client-only two-corner selection, coordinate/size HUD and cancel/reset,
  with no export, save, upload or world mutation. Existing accepted straight
  track evidence is reused. A later world-local Save bridge needs a separate
  authority/permission decision.

Source receipts: pinned Create Java 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`
in `references/create`; pinned Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`
in `.local/evaluations/horizon-heating-refresh/source`; the documents give
exact class/file and line references. The staged 6.0.10 Create source/art
remains distinct. Nested read-only specialists: `/root/train_dependencies`
and `/root/schematic_reuse`; independent strict read-only Sol High reviewer:
`/root/track_source_reviewer`. Reviewer CLEAR on `b405d372` with no findings;
the review took 121.5 seconds and made no writes. A narrow delta review of
the subsequent Create selection decision will be recorded before final handoff.

Validation performed: source reads, exact commit/status checks and
`git diff --cached --check`/`git show --check`. No Cargo, staging, linking,
Java, game, server, UI, template save/load or peer test ran. Any future UI
milestone needs a project-lead host-wide runtime lease and frozen inputs.
No Java/world import, train/graph owner, Create quill, Schematicannon, broad
schematic format, public release or other-mod scope follows. Team phase
records remain under `.local/team-operations/trains-schematics/phases/`;
per-agent token counters are project-lead-owned and unknown here.
