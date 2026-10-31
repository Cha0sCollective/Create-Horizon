# Persistent lifecycle seam audit

Read-only research against the a56e013 foundation and the frozen-window candidate.
This is input for a subsequent bounded design review, not source authority or G1
acceptance. Complete the frozen-window packet first and re-resolve line numbers.

Production World mutation/residency hooks already exist. World::set routes changed
writes through counter and slot hooks; insert_streamed_chunk distinguishes absent
arrival from replacement, and remove_streamed_chunk marks retained records
nonresident. Ordinary streaming calls these hooks. Temporary SDK map lending and
remote replica writes are different operations and must not become unload events.

The persistent counter changed hook advances checked machine generations, but its
inserted hook faults on resident replacement, reload mismatch and matching arrival
into a dormant domain. Persistent slot removal retains Pending custody. The
existing take_world_slot_retirement unconditionally detaches persistent mode, so
the checked Game retirement sink cannot yet preserve a healthy durable domain
through remove, resolve and replace. Pending/Faulted/Detached/nonresident capture
refusals are correct and must remain until explicit successful resolution.

The next proposed packet should define checked persistent retirement resolution
and readmission for the existing single target. Preserve independent generation
high-water marks and the original pending owner across repeated writes. Admit no
replacement slot before custody resolves. Reuse the item-preserving checked
receiver, with fallible preflight before consuming owners. Successful readmission
allocates fresh empty slot authority correlated with the current machine
generation; stale keys and prepared transfers still refuse. Matching unload and
reload preserve contents/runtime identity and never catch up while unloaded.
Resident replacement and mismatched reload need explicit semantics, not fault
clearing or generation resets.

Likely source seams are world/counter_restore.rs, world/slot_restore.rs,
world_slot_trial.rs, their focused test children and a narrow Game retirement
bridge. Determine exact API/path budget in independent design review. Real Game
qualification should cover changed/no-op writes, repeated blocked retirement,
successful checked delivery/retry, stale keys, actual streaming unload/reload and
genuine checkpoint/restore/recapture. Synthetic chunk replacement is a focused
control only.

Two later adapters remain distinct. NativeBinding::from_prepared physically binds
the accepted prototype to vanilla iron; ordinary custom-machine binding needs an
explicit registered-block contract. Diagnostic counter retirement and taking a
trial detach persistent ownership; ordinary close/world replacement need a real
Game lifetime/custody adapter before filled-slot activation. Running, authorized
player transfers, Creative reconstruction, admission/replication and retained
animation remain separate S1 outcomes.

Research performed by a bounded native read-only agent. No source changes or
runtime checks established these proposed behaviors.
