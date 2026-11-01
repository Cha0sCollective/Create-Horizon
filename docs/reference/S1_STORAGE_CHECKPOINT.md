# November 1 checkpoint and storage cleanup

Completed the owner's request to finish active work, sync GitHub, reclaim disk
space and examine the cost of the testing strategy. Development is held at this
checkpoint. S1/G1 remain open.

## Storage result

- Removed exactly17reviewed obsolete Cargo targets:251.07GiB reclaimed in Linux.
- Used Ubuntu-only built-in WSL compaction, after inactivity and sync/trim.
- Ubuntu VHD shrank342.24→93.73GiB.
- C:free space rose13.3→261.8GiB;260.79GiB after the final compile check.
- Verified175preservedsource/evidence hashes and4currentbinary hashes after restart,
  before authored report updates. Linux restarted normally as UID1000.

Preserved source repositories, current target-s1pg3, all copied test binaries,
resources, fixtures, runs, checkouts, G:snapshots/logs, Prism/playable data, references
and Java. No distribution relocation/reinstall/unregister, diskpart or broad WSL
shutdown. The exact fixed-target tool was independently reviewed; complete private
remote checkpoint and preservation manifest preceded deletion.

## GitHub and component result

The following FINAL-reviewed engine branches are publicly synced, with exact
remote SHAs verified after independent source/privacy screening and normal
pre-push history checks:

- codex/s1-native-writer-close:3fe2a2e8a85bc23e60ea53ebf74953a843cecc0f
- codex/s1-registered-writer-adoption:e369cac6da8f6e0b604a928fcb2e5233be3cf17c
- codex/s1-persistent-guest-counter-adopted:74cafefcdebf4e7adc013c76de7b49886ff06064

No tags, force push, PR, merge or release. Only explicitly authorized, fully
reviewed code was published. Private386ae82 was pushed before cleanup, preserving
source-only Guest/acquisition patches, authored tools and workflow documents.
The acquisition draft remains private, preformat SOURCE CLEAR, uncompiled.

Guest74cafef received different FINAL after325normal,9evaluator,21fatal,
sixstatic,9compatibility and one default-feature all-targets compile passed.
See S1_PERSISTENT_GUEST_RESULTS.md for scope and exact provenance. This accepts
persistent Guest host/storage/recovery primitives; ordinary activation is not done.

## Testing strategy correction

Fresh independent Sol High review confirmed repeated cold builds, duplicate
inherited suites and excessive per-case Git/hash wrappers. Adopted workflow8631:

- Cheap checks, then defining changed-behavior tests and dependency regressions.
- Reuse valid unchanged results explicitly at their original source/profile.
- Broad affected suites at meaningful composition/stage gates, not after every
  component and again after composition.
- Reuse each physical checkout's own target; never share across physical worktrees.
- Batch-boundary identity validation and bounded artifact retention.
- Keep complete S1/G1 connected-game and conservation/fault/auth/recovery coverage.

The redundant Guest A832broad requirement was SUPERSEDED, NOTPASSED. Instead, the
review identified seven actual transient-Guest compatibility cases and the missing
default audio/controller compile configuration. Seven-plus-two cheap cases passed
from existing binaries; the scoped compile passed in105.04seconds. No broad replay.

The next acquisition qualification shrinks110normal/25fatal to24focusednormal/
fournewfatal while preserving defining new loader/lock/custody cases, sixstatic
checks and appropriate existing regressions. Its tool must be revised/reviewed
after formatting before execution. This checkpoint starts no new development queue.