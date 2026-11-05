# S6 — dry Basin/Mixer integration

Fresh independent read-only Sol High audit/packet review is CLEAR for bounded
implementation. Stable source/effects review is still required before game use.
Base `6defc66746509cebd2213a41f917bd5e1a051a93`; source reuse is pinned upstream
`236c87a889cd76f7e663d1d182c28dc1145f7b14`. See `NATIVE_S6_BASIN_REUSE_AUDIT.md`.

## Playable outcome

Normal generated mob-free Survival Overworld; integrated singleplayer and one
dedicated server/two matching rendered clients. One downward, unfiltered Basin
is operated by a fixed-Y Mechanical Mixer two blocks above. Power uses an existing
motor/shaft/cog connection. Basin/Mixer ordinary placement, hopper-menu input,
visible head/item motion and empty-hand product collection must work. Supplied
machine/material items and motor/terrain fixtures are recorded separately from
ordinary player actions. This first packet does not qualify their crafting chain.

Support exactly `create:mixing/andesite_alloy`: one plain `minecraft:andesite`
and one plain `minecraft:iron_nugget` become one `create:andesite_alloy`.
No fluids, heat, filters, side spouts, random outputs, item components, container
remainders, other mixing/vanilla recipes or Press/Basin processing enter this
profile. Unsupported recipe definitions and nonempty/unknown tank/filter/spout
state refuse processing without being cleared or consumed. No fluid capability
is registered with the old host. Core dry simulation retains its modeled internal
capability precondition; do not disable all matches by setting it false.

## Implementation boundary

1. Reuse exact MIT `engine/src/core/basin.rs`, the NBT-only `fluid_codec.rs`,
   and selected Basin/Mixer/recipe adapter code with explicit provenance. Extract
   only upstream's small session/dimension cache utility from transport; no belt
   subsystem import. Keep `processing_ticks` and active recipe transient, and
   `Running`, `Ticks`, deferral/initialization behavior source-aligned.
2. Add Basin's18 canonical item slots and original `InputItems`/`OutputItems`
   mirrors through a Basin-only codec/load/save/update path. Preserve unknown
   sibling NBT and unsupported sizes opaquely. Validate before normalization;
   never truncate an unsupported payload. Register input0..8 and extraction0..17
   with the source's unique-item/first-empty-slot insertion rule. Empty-hand
   collection uses ordinary authoritative inventory movement, then updates peers.
3. Add the private generated-only `ModRevision::same_entity` born comparison
   in the defining renderer crate, using pinned Mixins. Do not use full revision
   equality, object addresses as persistent identity, or a saved owner token.
   No official engine checkout, public fork, ABI pin or broad API is changed.
   If this finite route fails, stop before a general lifecycle service.
4. Add only Mixer's fixed-Y small-cog/no-shaft and4SU/RPM cases to the existing
   kinetic model, plus effective-speed and contents-change scheduling. Retain
   accepted solver/source behavior. Do not import the newer kinetic runtime or
   add a timing offset to make a trace fit. Preserve source minimum30RPM,
   abort/retraction, repeat checks and recipe application output simulation.
5. Backport ordinary placement and native render/model/item paths. Omit unsupported
   context-shape fields and document hopper-only intake; preserve supported fixed
   player/block geometry. Use existing tint policy. Keep local original-art import
   and fallback provenance; original art bytes/resources never enter Git.
6. Keep the existing piston `portable_changed` callback, other machine serializers,
   recipes and versions readable. Introduce matching native/content version
   `0.5.0-ch4os.5`; preserve accepted `.4` binaries/maps. Add only selected content,
   recipe/tag and asset entries. Do not run upstream's whole content generator
   over private accepted content without proving every resulting file change.

Heat/burner code is assessed in the audit but is a following packet, not an
implicit prerequisite. Fluids, contextual falling-item intake, GUI/filters,
crafting progression, arbitrary recipe support, custom crash recovery, public
engine/schema work and general machine/contraption frameworks remain outside
this packet. Ask before broader architecture, schema or acceptance changes.

## Focused verification

- Reuse upstream core tests relevant to item matching/output admission, Mixer
  state/timing and power abort. Check the narrow adapter's definition/plain-input
  gate, unsupported-state retention,18-slot codec and stable entity comparison.
  One focused invariant check per concrete failure risk; no duplicated framework.
- Check formatting, four manifest versions, affected lint/compilation, source
  hashes and the selected resource-generation delta. Review stable source/effects
  before real-game use. No historical S1-S5 or full upstream battery replay.
- In one integrated milestone, ordinarily place Basin/Mixer and supply the recipe
  through a vanilla hopper menu. Verify below30RPM refusal, powered processing,
  no consumption while stopped/blocked, exact output and empty-hand collection.
  Include output-capacity refusal and recovery; no extra product or dropped item.
- In one dedicated/two-client milestone, verify ordinary input/collection, actual
  peer head/item rendering, power stop/resume and normal cold save/reopen with
  unprocessed inputs. Check saved inputs/output/counters and actual reconnect;
  source transient recipe/timer reset is distinct from loss or duped input.
  Finish with actor/peer/container/drop counts and normal process exits.
- Reuse unchanged stage evidence with attribution. One small existing Depot/press
  or portable-slot regression covers shared callback/runtime/serialization edits;
  broaden only for an actual changed path or failure. Preserve every failed setup
  and distinguish fixture mutations, server queries and actual client observation.

Record exact source/resource/binary maps and a concise result. Fresh independent
read-only Sol High completion review must clear the implementation/evidence
before accepting this profile. Checkpoint privately under the existing metadata,
origin and hooks rules. This completes one S6 family increment, not S6 overall.
