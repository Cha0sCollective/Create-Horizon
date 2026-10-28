# Bounded SDK deferred-write fixture stabilization

The unchanged full-counter driver failed integrated run
23ce9294-d959-4e3d-ae3d-01e7daea8059 before reaching its counter phase.
Captured block query initial-146 reports grass_block[snowy=false] at 10/72/0;
the actual SDK mutation initial-160 reports dirt -> glowstone at the same cell.
Event count/scope/sequence are intact. The sampled value differs from the actual
authoritative previous value, consistent with grass decay and/or replica delay;
the captures do not establish the exact timing of decay. The old fixture's
exact-previous-state assumption is unsound for that changing target. This does
not establish a regression in the pending dimension-transition fix.

Keep all qualified parent helpers byte-identical. Add only
tools/run_stable_counter_game_smoke.py: a subclass temporarily prepares the lower
SDK target as stone before the existing parent samples it, runs every inherited
assertion unchanged, then restores the original named state by coordinate-specific
en_us setblock success feedback in the current first/newest chat line. Dispatch
`ok` or a matching older history entry is insufficient; a no-op is
accepted only after resident exact-state verification.
It does not disable random ticks, freeze the world or filter/reorder observations.
The parent's upper/default marker restoration remains unchanged. Record setup/
restore metadata and wrapper source/hash alongside the original parent archives.
Preserve the parent's original exception if restoration also fails; restoration
failure after a successful case must fail qualification.

Require fresh independent read-only Sol High source/isolation review before
executing the new entrypoint. Root runs it in both modes against the SAME frozen
dc12bd6 game, with short fresh homes and copies of the two approved component
inputs, session unset and inherited display runtime retained. Keep original failed
integrated, passing in-frame and composite statuses unchanged; report stabilized
runs separately. No engine, acceptance, API, source pin or publication expansion.

Completed: independent source and final receipt review pass after both feedback
verification findings were corrected before execution. Wrapper SHA256
b09390209a9f03f3597ddcd25f9a412d40cc93344ad1858b78994a76ce31c35e
passes integrated368/final64 and in-frame353/final19. Detailed runs and limits
are in H08_DIMENSION_TRANSITION_OWNERSHIP_RESULTS.md. Keep this entrypoint and
the qualified parent helpers immutable after qualification.
