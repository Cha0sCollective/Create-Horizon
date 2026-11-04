# S1 ambient mob scope decision

Status: PROPOSED, awaiting owner decision. This file does not authorize source
changes, amend the supported profile or close S1/G1. November 3, 2026.

## Concrete failure

Actual non-test game source35 (source map a23cf48e; ELF2c1874ca) supports normal
integrated rendering/movement, creative stone pickup, native saving and checked
Leave back to the named-world list. The first title-screen own-world reopen does
not reach playable state. Live status from isolated run6 identifies:

    session checkpoint NotPublished (0 item identities assigned):
    unknown raw native entity field/wrapper; Endpoint Released | Server 1 None

Read-only inspection of the preserved run4 native entity regions found a donkey,
seven pigs and a chicken with native living-entity fields. The current checked
entity projection intentionally accepts native ItemEntity records only; it rejects
these living records. No save record has been deleted, normalized into an item or
accepted by weakening validation. Raw runs and native worlds remain ignored/local.

## Recommended bounded option

Keep the selected paired fixed-TestTerrain S1 profile free of ambient living mobs.
Prevent both initial chunk-generation creature seeding and later natural/custom
spawning before they create live/save owners. Continue the ordinary simulation,
native dropped-item path, stone interaction, counter/slot work and existing checked
save/recovery. Leave ordinary unpaired Horizon behavior unchanged. Keep existing
living-entity records refused; no import or removal of prior records is authorized.

The existing native `spawn_mobs=false` rule already blocks both producers.
`tick_creatures` still performs its fresh-column bookkeeping, then generation's
`spawn_mobs_for_chunk_generation` returns through NaturalLevel::spawn_mobs; normal
and custom spawn passes read the same rule. No mob gameplay or codec rewrite is
needed, and no additional source path is currently indicated.

After owner acceptance, prepare the smallest independently reviewed amendment
within the existing twenty paths: apply this explicit fixed-profile rule to fresh
ordinary worlds before native rule-file publication and before load_world_state
or any simulation tick. Updating only Level.game_rules after construction would
be insufficient because load_world_state reloads the effective rules from disk.
Persist and verify the actual false rule. Own reopen and recovery preserve the
captured rule; verification must not rewrite it or erase existing living records.
Reject conflicting existing profile data rather than silently converting worlds.
Legacy/unpaired constructors keep their current rules and behavior.

Independent read-only source investigation confirmed both producer gates and the
need for owner acceptance: the adopted fixed-TestTerrain proposal does not specify
this rule or authorize silently changing saved gameplay rules.

Validation: one focused proof that both seeding routes stay absent in this paired
profile, unchanged ordinary behavior outside it, followed by the existing integrated
stone/Leave/own-reopen smoke. Reuse passing compilation/recovery evidence where
unchanged; do not add a general mob or save framework. Then continue the dedicated
two-rendered-client and in-frame checks and remaining three ordinary error groups.

## Alternative

Retain normal living-mob simulation in S1. This expands the checked projection and
custody boundary beyond ItemEntity: native living state, UUIDs, equipment and other
item-bearing owners, unload/save/recovery and interruption all need an explicit
bounded design. Horizon already owns mob gameplay and native codecs; this would
integrate their ownership with checked recovery, not reimplement their AI. It has
additional review and validation cost. Do not start that expansion without an
owner decision and revised bounded assignment.