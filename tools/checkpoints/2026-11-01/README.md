# Source-only checkpoint of local development tools

These are authored tool/input backups, not a supported general test framework.
They retain exact local paths and prerequisite guards. Never run them blindly.
No test outputs, game files, generated worlds or raw run logs are included.

- s1pg3-qualify.py and expected input: historical325/9/21/6qualification. ALL modes
  are RETIRED after clean Guest74cafef; staged-parent guards deliberately refuse
  that committed checkout. The completed execution is documented separately.
- s1pg3-compatibility.py: seven transient Guest and two projection/receipt cases
  from pinned existing artifacts; completed successfully without a rebuild.
- s1pg3-default-check.py: completed default audio/controllers plus paired mods
  all-targets Clippy; no runtime test replay. Clean Guest74cafef received FINAL.
- s1ra1-qualify.py and expected input: unexecuted draft110/25plan, now SUPERSEDED
  by reviewed24/4selection. Its source pin is intentionally PENDING. Revise and
  independently review it against the actual formatted candidate before use.
- s1-storage-cleanup.py: exact reviewed17-target cleanup tool. It refuses to run
  without actual verified remote checkpoints, complete preservation metadata and
  project inactivity. Its locally created sync gate is not part of this backup.

The active copies and all raw-source/evidence snapshots stay under G:/.local.
For raw-byte restoration from Git, extract canonical blobs with git show and
respect the recorded line-ending distinction rather than inferring byte identity
from a Windows checkout.
