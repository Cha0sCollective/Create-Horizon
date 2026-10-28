# H04a: committed resident mutation observations

Owner continuation authorization covers this bounded next S1 prerequisite while
the H03b frozen candidate completes broader checks. Base engine is
`0264c18fd268ff369af64dcaa3dd8d94ddfdecd6`. Implementation remains isolated until
H03b qualification is recorded. The original S1 diagnostic cube does not require
custom shapes, but full H03 and behavior/type bindings remain open.

Add a small opt-in observer at `World::set`, the common successful resident-write
seam. Keep the existing LAN block log and sculk watched-write queue independent.
Observe only a bounded host-supplied watched set, recording every relevant old/new
transition, position, actual world game-time, immutable authority-session/dimension
scope and checked sequence. Do not coalesce multiple replacements at one position.
Initially unclassified causes must remain explicit; no guessed player/cause metadata.

The observer is passive. Same-state writes and failed height/unloaded writes emit
nothing. Queue overflow/exhaustion must make the observation batch explicitly
faulted; never return a truncated batch as complete, never silently reset sequence,
and never add a new World::set failure that would change already-authorized vanilla
writes or item effects. It is not a transaction journal or authoritative record.

Feature-off/default worlds remain unchanged. Scope/queue travel with World across
dimension swaps; no global event queue or later dimension lookup. A transient
world-instance/session ID is not persistent world identity or object generation.
No automatic client observer, guest imports, machine callbacks, inventories,
type binding, chunk lifecycle, record store or persistence is authorized by this
subpacket. Those need explicit follow-up before full H04/G1 claims.

Tests: actual successful/no-op/failed World::set paths; ordered replacements;
filtering; explicit sticky overflow/sequence exhaustion; independent existing
logs; distinct equal-coordinate dimensions and scope surviving World swaps;
default disabled behavior. Root later owns bounded authoritative game setup and
additional real command/player/piston/deferred-write coverage before claiming those
paths qualified. Fresh independent Sol High review covers substantive code.

Coordinator owns integration and commits. One isolated worker owns the new module,
World seam and focused tests; no staging/commits/pushes. Keep all worker Cargo
commands stopped until the coordinator releases the shared target after H03b
checks, to avoid replacing running test executables. No standing team or general
event framework is required.
