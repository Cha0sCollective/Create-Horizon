# H09a BlockRegistry inspection clarification

PROPOSED bounded amendment to accepted d17bb651 design; no source/API expansion.
Base engine ea96036ce48c269225169e4865e02e4860182b47. Exactly the existing three source paths.

The complete both-direction/cardinality requirement continues literally for public
Builtin maps and Dynamic maps. BlockRegistry's four maps are private and immutable
through its public API. Every public constructor funnels through private from_states,
which rejects duplicate states, builds only indexed reverse entries (including hash
collisions), validates every default target/name, and requires exactly one default
for each distinct state name. The snapshot may rely on these construction-enforced
invariants, pinned to the inspected registry.rs source, rather than claim independent
enumeration of inaccessible reverse/default maps.

Snapshot still walks all state IDs without gaps, validates ID->state->ID, validates
all supported names/properties and existing bounds before allocation, checks each
name's default resolves to a valid same-name state, and hashes the complete numeric
state sequence and one default mapping per distinct name. Charge state/property/default
entries and framing under the existing budgets. No unsafe construction, hidden-map
fabrication or assertion that inaccessible extra entries were independently inspected.
Changing constructor/private-map invariants requires revisiting this dependency.
Tests exercise public-constructor duplicate/invalid/missing-default refusal and actual
state/default fingerprint changes. Builtin extra reverse-entry controls remain required.

No fourth path, runtime mutation, admission, wire-format, world/save or startup change.
All other accepted resource, ordering, provenance and qualification rules remain.

ACCEPTED: fresh independent Sol High native read-only review accepted the exact proposalSHA dd1dd7d0a531e6acbdfda4d2b966e9060b09dd901ff4982f18689dd366f86ac0; constructor/child-module inspection supports the stated invariants at registry.rs SHA714bc53cc9a811beacdf7aaa1ab55705b65f59d8f7fe8de5242cb62a00bb0027. No blocking findings or fourth-path dependency. Implementation remains unreleased until completed and exact-source reviewed.
