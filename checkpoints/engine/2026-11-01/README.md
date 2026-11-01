# Private engine draft backups

Source-only Git patches against accepted engine base
e369cac6da8f6e0b604a928fcb2e5233be3cf17c, available on the public Horizon fork's
codex/s1-registered-writer-adoption branch. These backups are not production mod
code, public publication, FINAL acceptance or runnable-world archives.

guest-counter.patch reproduces the ten-path child
74cafefcdebf4e7adc013c76de7b49886ff06064, tree d48e644ea999c7241b057626ecc4e2719f513e8a.
Its325normal/9evaluator/21fatal/9compatibility andsixstatic checks passed.
Default-feature all-targets compile and different FINAL remain pending. Source
manifest2035b79e preserves tested raw bytes locally; the patch stores canonical
Git text, allowing only the recorded CRLF-to-LF normalization distinction.

retained-acquisition.patch contains exactly three draft files on the same base:
client save.rs, nested host_identity/session_authority.rs and world generator.rs.
Prefmt SOURCE CLEARcbf2db50; save4c27079c/authoritycaa53775/generator91e55d15.
No formatting, compiler, runtime or FINAL acceptance. The revised future focused
plan is24normal/fourfatal/sixstatic, with explicit reuse of unchanged coverage.

To inspect, use git apply --stat and git apply --check against the exact base in a
fresh checkout. Apply only to an isolated development branch. The two patches
were reviewed separately and need integration review for later composed acceptance;
do not infer a FINAL combined candidate from these backups.

Local originals, all source snapshots, test logs, manifests, prepared resources,
fixtures and active executables remain preserved on this machine. No generated
worlds, Minecraft binaries, credentials or raw run logs are stored in Git here.
