# H08a5 map and named-random metadata save results

Close two borrowed-state metadata false-success paths after H08a4. Game's common
metadata save calls save_maps/save_random_sequences, both returning unit, and can
eventually announce success despite their write failures. Scope only game.rs,
game/maps.rs and game/trading.rs, with small private path/state test seams.

Return Result from both helpers and propagate it before the existing success
message. Preserve remote no-op behavior. Failed/unvisited maps retain dirty state;
clear a map's flag only after its own file succeeds. Keep last_id available after
index write failure; retry writes it again. This does not make the map files and
index atomic, or convert autosave HUD text into a disk-writer durability barrier.

Random sequence serialization must preserve the full current borrowed state and
must write an empty collection: skipping empty ids resurrects an older file after
reset/configuration changes. Merchant sessions can hold streams lent out of the
world collection across saves, including LAN sessions. Build an immutable snapshot
from a clone of the world collection plus all live local/LAN session streams;
never drain, return, advance or close the sessions merely to save.

The current loan model can create multiple owners of one id. Accept duplicate ids
only when all state words agree. Conflicting states must return an explicit
ambiguity error before writing the random file; no HashMap iteration winner or
new persistence schema. Live RNG ownership repair and reset-during-loan behavior
remain separate. Tests for empty reset use no live loans. This safe refusal is an
experimental limitation, not a claim that all merchant states are persistable.

Test actual map-file/index errors using blocked _new paths, retained dirty/index
state and repair/retry with exact NBT colors/settings. Test random write failure
without changing world/session state words; repair/retry must write the same
snapshot. Verify nonempty then clear, changed defaults plus clear, local/LAN loan
inclusion, identical duplicate acceptance, conflicting duplicate refusal with old
file unchanged, and remote no-write behavior. Existing format round trips and
normal metadata/Game save regressions remain applicable.

Stop/rescope if representing conflicts requires runtime loan-policy changes,
closing live menus, player/entity retirement or a format redesign. No ownership
winner is accepted by this packet. Other metadata helpers, transactional rollback,
coherent checkpoint publication and G1 remain open. Fresh independent read-only
Sol High review is required. Coordinator owns serialized Cargo and local commits;
no public engine push.
