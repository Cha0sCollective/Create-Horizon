# Deliverable 1: startup, save and close route map

Source inspection at c2d098c plus the bounded s1srv2 admission correction.
Planning input within the adopted S1 completion plan; no extra source paths or
new supported profile are released by this map. Paired Open to LAN stays deferred.

| Actual route | Current source | Reuse and remaining connection |
| --- | --- | --- |
| Default integrated launch | game/integrated.rs open_integrated_with_identity -> serve -> server_game -> run_integrated | Already creates an original server Game and Memory Pipe, activates server mods, owns the native integrated player mapping and authoritative tick loop. Reuse its role/owner/pause rules. |
| Default integrated close | same file stop_integrated; game/saving.rs begin_closing_save; game.rs save_for_exit | Existing stop saves/kicks players, saves world, closes LAN and flushes. serve retires counters after return. This is not failed-save retention or coherent checkpoint routing; do not just insert an archive call after owner release. |
| Accepted retained server | game/retained_server.rs Supervisor::prepare/worker/run_game; game/lan.rs native admission; paired_close.rs | Original Game, retained Hub, actual Pipe offers/TCP, native save/metadata close and joined worker exist. Fixed fresh offline unpaired DedicatedServer role, all players external; it is not the integrated-owner mapping. |
| Accepted rendered remote client | game.rs new_retained_remote/InitialGameWorld::RetainedRemote; game/retained_remote.rs; window_close.rs run_remote | Original received-world Game, decoder/streamer/window close already work for explicit TCP. Constructor currently accepts an address string, although library RetainedTarget already supports Memory(Pipe). |
| In-frame native world | game/world_loading.rs ColdPreparation InFrameClient; window_close.rs; paired_close.rs | Retained cold-world/window close exists. Ordinary in-frame gameplay uses a real local player; it must not be represented by a fabricated network guest. Need ordinary lifecycle/save/reopen connection for this profile. |
| Package agreement | lan/login_negotiation.rs; game/mod_packages.rs; lan/conn.rs | Existing composed login agreement intentionally Disconnects before paired configuration; external paired guard still refuses. Complete actual configured admission on selected modes with the retained final content/registry inputs. |
| Coherent reopen | game/levels/session_checkpoint.rs and connected recovery/managed owner components | Reuse complete images and actual participant recovery. Native files newer than the selected complete epoch must not bypass recovery. Bind actual integrated/local/external owners; do not substitute a zero-player roster. |

## Smallest next integration outcome

After the current correction receives different FINAL acceptance, connect an
actual original client Game through the already implemented Memory Pipe to the
original owned server, including checked close of both sides. Reuse the TCP
client decoder/streamer and existing server supervisor; no second transport or
renderer. Couple preparation, endpoint handoff and failure custody so no fallible
preflight silently abandons an offered Pipe or a running server worker.

The next bounded design must include a real Game/Memory integration test and
the route into the supported integrated application. A factory-only success does
not complete it. Preserve existing TCP behavior. Keep dedicated external-player
mapping distinct from native integrated-owner metadata mapping and in-frame local
ownership; a Boolean relaxation of the cold authority gate is insufficient.

Then connect the same selected startup/save/close paths to actual configured paired
admission and coherent archive routing, before enabling filled machine ingress.
Fresh login, departure/rejoin, successful stop and failed-save retry are observable
requirements, not reasons to expand every native subsystem or invent a new host.

The exact subsequent source budget needs independent design/API review at the
accepted correction revision. If the map exposes a broad engine redesign or a
supported-profile expansion, return concrete options/cost to the owner BEFORE
implementation. No paired Open to LAN, general executor, new loader or journal.

Independent native route review confirms the map. Before the next source release,
specify genuine integrated Owner(saved Level)/Serving.owner admission and native
save projection as well as joint application/server lifetime. The existing cold
server authority and population profile deliberately reject integrated ownership;
do not widen them with a Boolean. Memory attached to unchanged Dedicated/external
mapping is only component evidence. The actual integrated outcome requires owner
readback and rendered original Game/application execution, not protocol clients.
Likely existing seams are game.rs, integrated.rs, window_close.rs,
retained_server.rs, world_loading.rs and lan.rs; check paired_close.rs before fixing
the exact budget. No transport or renderer source change is presently justified.
