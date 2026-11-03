# Ordinary startup terrain decision

Pending owner decision, November 3, 2026. S1/G1 remain OPEN. This note accepts
neither option and releases no new source, provider authority or acceptance claim.

The current source packet is s1ord1 on codex/s1-ordinary-paired-startup, parent
ca9b3f49c97db6cd981246de99be35f7b53420ec. It has unfinished, unqualified work in
eleven of its sixteen assigned paths. The last accepted/public engine remains
ca9b3f49. No s1ord1 formatter/compiler/runtime has run. The complete sixteen-path
working copy and prefix inputs were preserved under .local/s1ord1/draft6 before
this hold. Existing historical results and frozen evidence are unchanged.

## Confirmed gap

Horizon already provides ordinary FlatTerrain and VanillaTerrain generation.
The specific retained save/close/coherent recovery service being connected here
has a narrower supported provider. This is an integration guarantee gap, not a
missing terrain generator.

Coordinator source inspection and an independent read-only Sol High follow-up
by paired_config_handoff_design_review confirmed:

- PrepareOwner::prepare_streaming in game/world_loading.rs issues the actual
  NativeCheckpointOrigin only in its identified Overworld TestTerrain branch.
  The preceding flat branch receives no origin. VanillaTerrain receives no origin
  and retains a locator.
- OrdinaryPreparation::into_prepared uses the existing consuming prepared-owner
  transfer, which requires the original provider witness and validates its actual
  SaveDir, root and dimension. Ordinary options default test_terrain=false.
- NativeCloseAuthority, coherent session_context_cpu and late native-column
  disposition require that actual constructor-issued provider origin.
- prepare_session_checkpoint_profile_steps in game/levels/session_checkpoint.rs
  requires the recovered origin and absence of a locator. Its column validation
  accepts full chunks with bounded structures, ticks and postprocessing; saved
  proto status and unsupported owner shapes are explicitly refused.

Removing those checks, minting the current witness for another provider, or
silently forcing test_terrain would misrepresent requested terrain or supported
save/recovery behavior. General legacy Game generation does not establish these
retained guarantees. No such changes have been made.

S1_COMPLETION_PLAN.md defers unrestricted generators/subsystems and requires
consultation before supported-profile or acceptance expansion. The ordinary
startup proposal promises normal CLI/title entry and a fresh bounded Overworld,
but did not resolve its no-flag provider semantics. The reviewer confirmed that
this needs an explicit owner decision; adding another source file alone would
not resolve the scope question.

## Choices for the owner

1. **Keep S1 on the existing fixed TestTerrain provider (recommended).** Require
   the existing --test-terrain option for selected-pair entry, fence flat and
   unsupported creation options before acquisition, and continue ordinary menu,
   integrated Memory, dedicated two-client and in-frame work. Normal creative
   controls, placement, Wasm state and save/recovery remain the S1 goal. Record
   fixed terrain honestly; make no normal Vanilla/flat terrain claim. This keeps
   S1 focused on the mod integration and defers broader terrain support.
2. **Expand this packet to a specified ordinary terrain provider.** First agree
   the exact provider/profile, then review its construction provenance, producer
   completion, native-owner capture and coherent recovery behavior. Reuse Horizon's
   generator and codecs; do not invent a new generator or relax conservation.
   Supporting default Vanilla generation is broader than a startup caller change
   and adds implementation and validation work. No duration estimate is established.

After a choice, amend the bounded proposal/assignment accurately and obtain the
required independent delta review before further dependent implementation. Preserve
the fixed Cold constructors and old qualified evidence in either case. No new
general framework, Open to LAN, arbitrary imports or public release is implied.
