# H08a10 borrowed dirty metadata save errors

After H08a9 qualifies, scope game.rs, game/dragon.rs, game/wandering_trader.rs and
game/nether_portal.rs. Fight, trader and pending portal-arrival saves borrow their
payloads and already clear dirty flags only on successful writing, but their unit
returns suppress errors from common save_game_metadata. Return Result and propagate
all three calls before the save-success message. Include the target path in errors.

Use small private state/path seams. Preserve fight/trader loaded-and-dirty guards;
do not invent remote guards they do not currently have. Preserve arrivals' dirty/
remote guards without adding a loaded guard: dirty arrivals must still save when
loaded is false. Preserve exact schemas, including arrivals' existing root
{arrivals:[...]} without a new data/DataVersion wrapper. On failure preserve every
payload/runtime field and dirty state; success clears only the dirty flag.

Fight has one eager dragon_killed caller after portal/gateway/egg work and killed
flags have committed. Handle its error explicitly with a warning and retained
dirty state; continue that gameplay path and rely on ordinary saves to retry.
Do not propagate a new death-gameplay error, replay committed effects or add a
retry loop. Common metadata saving must still return the error normally.

Use actual blocked _new directories and repair/retry. Fight tests cover committed
killed flags, gateways, exit portal, respawn stage/time/crystals and exact NBT;
unloaded/clean guards write nothing. Trader tests preserve loaded/dirty/tick delay/
spawn delay/chance, serialize only existing persisted fields and test guards.
Arrivals tests preserve multiple destination plans, portal axis/offset/size/End
fields and opaque nested entity/passenger/item/custom NBT, round-trip through the
existing arrival parser, failed last-arrival deletion, remote/clean no-writes,
dirty empty saves and dirty loaded-false inclusion. No renderer fixture needed.

Keep tolerant loading and world-entry reset behavior unchanged. Record the existing
separate fight-lifetime issue: world entry resets trader and Portals but not Fight;
the loaded guard can retain prior-world fight state in the same Game. Portal
send_through_portal removes entities before arrival creation, and resolve_arrivals
removes a plan before exit resolution/publication; error propagation does not
repair these ownership transitions. No cross-file atomicity, lifecycle repair or
coherent checkpoint/G1 claim is accepted by this packet.

Require fresh independent read-only Sol High review, targeted native failure/
retry tests, feature-on/off checks, frozen fullcounter in both execution modes and
broad regressions. Coordinator owns serialized Cargo and commits. No public push,
release, migration or permission expansion.
