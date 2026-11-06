# TRAINS-003 world-local Quill Save design handoff

Status: bounded read-only design CLEAR after a fresh independent Sol High
review of the repair delta. No Save source was implemented or game qualified.
The project lead's QUILL-001/002 runtime milestone can proceed independently.

The team lane `codex/team-trains-schematics` started this packet at
`09217f6` and committed the proposal at `2d5a6b9`, then its reviewed repair
at `dc0d3cbb05dd281ef5abde6bc49c2a5a0747b014`. The only design file is
`TRAINS_003_WORLD_LOCAL_SAVE_BRIDGE_PROPOSAL.md` in this directory. It reads
the pinned Horizon source `9c7f9b2215dfed497aba5522c44f38926628648d`
and preserved Java Create reference `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.

**Owner decision:** Approve or reject a first Save that produces only a
world-local Horizon native structure template at
`<world>/generated/create/structures/quill/<slug>.nbt`. Proposed authority
is a living Creative player, operator level at least 2, with a selected
main-hand Quill. Keep the existing inclusive 16-block per-axis limit; save
blocks and air, omit entities, reject block entities and structure void, and
refuse an existing name. The server accepts only a 1..35-character lower-case
ASCII slug from `[a-z0-9_-]` and builds `create:quill/<slug>` itself. This
does not produce a Create item, Java-compatible export or portable import.

The bounded path is a third-click name prompt followed by a registered
operator-level-2 native command with slug and six coordinates, an acting
player permission/item check, a transient per-player queue, and capture in
the matching `level_tick_post` after a fresh authority and bounds check.
Pinned native player/entity/living handles provide a live selected-main-hand
read without direct guest inventory access. Only the template writer needs
internal current-level `Game` access.

**Writer gate:** Pinned `save_structure_area` uses a direct filesystem write
and caches the template even after a failed write. Preflight alone cannot
guarantee no overwrite or symlink-safe world containment. Require a narrow,
separately reviewed private writer repair for exclusive safe creation and
failed-write cache policy before implementing Save. This is part of the
owner's authority decision, not an approved source change.

The first strict read-only Sol High review found one source-reuse error: the
proposal had claimed private guest selected-slot access was needed. The
repair switched to the native player/entity/living handle path. Fresh delta
review returned CLEAR with no remaining finding. Review durations were
179.5 and 13.1 seconds. `git diff --check` passed; source search and review
were read-only. No Cargo, staging, linking, Java run, game session, host-wide
lease, network use, runtime test, or public release occurred for this packet.
UTC phase intervals are recorded under
`.local/team-operations/trains-schematics/phases/`; per-session token counts
are unknown.

After the owner decision and separate writer review, qualification should
cover privilege/Survival/held-item refusals, malformed names and collisions,
dimension/world changes, loaded and height boundaries, 16/17-axis cases,
block-entity and structure-void refusal, air inclusion, exact file/readback,
disk failure, reopen and two-client peer isolation. Train assembly remains
dependent on the contraption contract; Java/world import, upload, placement,
Schematicannon and public release are outside this packet.
