# H08 restored slot recapture integration audit

READ-ONLY SOURCE ANALYSIS, not a released implementation packet. Candidate input:
clean LOCAL enginebfa5ced912bed16f955e65824317a0b259c2cb6a/tree
d14db6c4a5b5a9eb511cddf5c443832d024b48f5. At audit time slot-epoch qualification
was in progress. It now has different final acceptance; the separate bounded
recapture proposal has fresh design acceptance. This audit itself releases no
edits. Personal source-informed port; no public publication or G1 claim.

The bounded author audited Horizon sources without writes or execution while root
qualified the frozen eight-path candidate. The audit initially observed b17c44d
plus those exact sources; root committed the same source bytes as bfa5ced. Frozen
source manifest is workspace-root .local/h08se1-source-sha256.json, SHA256
769bdb9f1de773426f56f4a505d1ed7616c6d459e7181c098cd5c042711f7f53.

## Native metadata blocks the current second capture

world.rs checkpoint_world_owner refuses any nonempty ChunkExtras.root. This is
correct for arbitrary opaque data. Native save.rs writes InhabitedTime, structures
and PostProcessing; decode_column's GENERATED_KEYS does not remove those three
fields, so native loading retains them. persist.rs attach_save_data also retains
the column's last_update. Source-derived expected values for the authored empty
nine-column fixture, before any ordinary ticking or generation, are:

| Retained field | Expected exact shape |
| --- | --- |
| root.InhabitedTime | Long0 |
| root.structures | Exactly References and starts, each an empty compound |
| root.PostProcessing | Exactly24 empty lists |
| last_update | T |
| Block entities, source/order overlays, rolled loot, saved tick vectors | Empty |

These are SOURCE-DERIVED expectations, not a live enumeration. The next reviewed
test must enumerate all nine actual native loaded roots, including their complete
key sets/types, before relying on that shape. Relevant source anchors at this pin:
save.rs1204/1245 (writer),1432 (decode); game/persist.rs75 (attachment);
world.rs1137 (conservative owner observation).

A fixed borrowed predicate may recognize exactly the empty shapes above. It must
refuse unknown/extra/mistyped keys, nonempty structure starts/references, any
postprocessing entry and every other W1 owner. Structures can carry generation/
loot behavior; postprocessing entries are deferred work. Never widen GENERATED_KEYS,
discard raw tags, clear owners, catch the first opaque error and return success,
or infer that arbitrary values under those three names are safe. Ordinary W1
must remain conservative; a recapture observer must share all remaining checks.

## Inherited checkpoint files block the subsequent overlay

capture_integration.rs copy_selected_epoch copies every world member, including
data/horizon/counter_checkpoint.json and data/horizon/slot_checkpoint.nbt, into
the recovered native destination. Material.copy later copies them into a new
pending image. slot_epoch_integration.rs overlay uses create_new=true at both
paths, so a second capture would then fail with EEXIST even after owner admission.

A bounded recapture profile must validate and account the inherited projections
before native work, then replace ONLY their pending-image copies with the current
validated live projections, following the existing PLAYER overlay pattern. Source
files and epoch1 are immutable inputs; do not delete, overwrite or rename them to
gain admission. Preserve missing/hash-bad versus corrected-hash semantic failure
classification and whole-image recovery. Relevant source: capture_integration.rs
845 (Material copy),1488 (selected-epoch copy); slot_epoch_integration.rs468/476.

## Smallest plausible next cell

Target restored counter3/T BEFORE advancing to4/T+1. correlate_slot currently
requires counter3; widening that scalar/profile is a separate change. Capture a
complete epoch2 through the actual Task; recover it freshly with PLAYER3/slot7.
Preserve epoch1, original source, recovered-source sidecars and live owners across
refusals. Keep existing counter/slot/PLAYER equality and publication checks.

A preliminary six-path scope is world.rs (borrowed fixed recognition plus defining
tests), game/levels.rs (recapture observation sharing all remaining W1 checks),
capture_integration.rs (private copied-sidecar handling), slot_epoch_integration.rs
(fixed entry point and test child declaration), one new recapture test child, and
integrated/capture_completion_tests.rs (fixed actual-Task dispatch). Verify helper
visibility and exact paths in the proposal; this audit does not release edits.
No save.rs/chunk.rs/codec/registry/journal/new Game field or ordinary admission
change appears necessary.

Required proof: exact actual nine-column metadata; repeated borrowed observations
without mutation; refusal of every malformed shape and remaining W1 category;
actual epoch2 publication and direct recovery; complete retained slot/counter/
capture/entity/W1 coverage appropriate to changed paths; feature checks, fresh
ordinary modes and independent final acceptance. After4 capture, general structures,
pending effects, transitions/history, ordinary runtime and G1 remain open.
