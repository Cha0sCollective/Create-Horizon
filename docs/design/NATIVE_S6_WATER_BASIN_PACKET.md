# S6 — bounded plain-water Basin reuse

Next separate increment under the owner's continuing S6 authority, based on
accepted coal/brass074122b7/source5ac874f. Branch
`codex/native-s6-water-basin`; preserve accepted `.7` maps/binaries/saves.
Candidate `.8` retains host9c7f9b22/Mixinse8cd0aa3, Rust1.99.1/API359. No host,
save-schema, general plumbing or crash-framework work is proposed.

## Read-only reuse findings

Current upstream default-branch horizon-native.json still declares0.7.0-alpha.2
and this exact host pair; its dough recipe was fetched and agrees with retained
source236c87a8. Use that exact source snapshot, not a wholesale upgrade. The
private Basin core, canonical tank codec, tank replacement/save fields and client
fluid-level chasers already exist. The newer accepted host already implements
the four custom-fluid ownership/view/fill/drain hooks and native fluid rendering
that were unavailable on the earlier fc021 host. No engine gap was identified.

Select upstream Basin fluid-state/commit/capability adapters, water-bucket
interaction and draw_fluids/native_draw_fluid_box. Keep private item guards,
save-slot snapshot, kinetics, burner and two existing recipes. Existing water
textures and wheat_flour/dough item resources need no catalog/schema change.
Use the same strict full supported-state guard for capability reads. Require
ordinary exact bucket stacks, and propagate successful Execute/commit results
before changing a bucket/remainder; upstream's unchecked void commit is not a
reason to report a failed commit as successful. Claim/refuse lava at the Basin
to suppress vanilla spill and second-hand exchange. Review found the host's
older reaction callback covers only main hand. The candidate reuses the existing
per-hand native UseItemOn prefix for Basin buckets, preventing offhand vanilla
UseItem fall-through without predicting Basin inventories on the client.

Retained Create6.0.11 BasinBlockEntity constructs two1000mB input segments and
two1000mB output segments. Its dough JSON requires flour-tag1 + water1000mB ->
dough1. Upstream uses equivalent string-tag encoding for the same ingredient.
Both APIs use1000 units per bucket here; preserve canonical source NBT fields.
No claim of complete6.0.10/6.0.11 or NeoForge patch-version equivalence follows.

The core also contains known mixed-fluid maximum-drain discard and multi-result
overbooking behavior. Neither prior millstone nor coal decisions adopt those
cases. This packet keeps them unreachable: only plain water in supported input
segments, empty output-fluid segments, one fluid ingredient and one deterministic
item result with no remainders. Refuse lava, component-bearing/foreign fluid,
nonempty output-fluid payloads, arbitrary recipes, filters/spouts and mixed fluid
state without consuming or flattening it. Do not install sponge actions.

## Supported outcome

Supplied ordinary water bucket inserts1000mB into a downward Basin and returns
one empty bucket in Survival. A full source matching input refuses another bucket
without consumption; retain the source variety/capacity rule. An empty bucket
can recover1000mB plain water through the existing exact-fluid drain path.
Unsupported water/lava/component state and insufficient water remain unmodified.

Strict third recipe: create:mixing/dough_by_mixing, plain flour1 + water1000mB ->
plain dough1 at the existing Mixer speed requirement. Keep exact dry alloy and
heated brass definitions unchanged. No fluids are produced by these recipes.
Canonical saves preserve stored water and item ownership during ordinary cold
reopen; actual clients render the water/chase surface and dough-to-empty states.
Only selected supplied-materials processing is qualified, not acquisition,
pipes/pumps/tanks outside Basin, fluids in world, lava, automation network or
general fluid recipes.

## Bounded changes and validation

Add four upstream host-hook helpers, selected fluid adapters/bucket branch and
render call; extend existing supported-state and exact-recipe predicates. Keep
the engine algorithm and tank codec unchanged. Snapshot/version edits and one
recipe are the only content additions; original-art importer remains schema11.
If actual closure needs broader source/API/schema work, stop before expanding.

Fresh read-only Sol High packet/source/effects review; formatting, exact versions/
content delta, stage64 injections/29 modules expected, focused newly reached
capability/codec/refusal/recipe tests, Clippy and locked build. Reuse unchanged
coal/dry/host tests instead of replaying their batteries.

One composed milestone in integrated and dedicated/two rendered clients: ordinary
bucket input/output/remainders, full/insufficient-water refusal and recovery,
ordinary hopper flour delivery, exact dough/collection, normal cold stored-water/
input reopen, actual peer water/product/empty views. Use the already verified
unobstructed Basin camera before processing. Include one dry/brass regression as
affected composition, with fixtures and normal product counts explicit. Decode
all saved owners and distinguish stock local-owner mirrors; preserve all failed
attempts. No acceptance from command success or screenshot filename alone.
An explicit500mB input-water fixture supplies the insufficient case; remove only
that fixture before ordinary whole-bucket refill/recovery. Do not claim partial
bucket top-up or automatic recovery of the supplied500mB.
