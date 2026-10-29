# H08 retained lazy raid lifetime results

Local candidate63a7d62fae27dcd9ce8bfbe1209616f4e4988607, tree
571ad7e2cc20d8178898c6fdd9a0244252f8dcad, is under qualification on
codex/h08-retained-raid-lifetime, based on accepted3c4e7e2. Source boundary is
H08_RAID_LIFETIME_PACKET.md. Accepted after final independent source/receipt review
with no material findings. Generic engine commits remain local.
Exactly game/raids.rs and new raids/lifetime_tests.rs change; qualified metadata
tests, Fixture bodies and all other modules remain unchanged. Two files add773/
remove12lines, including a720-line resource child, above the upstream soft size
target. Fresh independent
read-only Sol High design/source reviews find no blocking issues.

load_raids now returns the old save error before replacing its loaded identity,
native payload, helper maps or target RNG/read state. Four callers log and return
their existing refusal values. Eight direct access roots first require current
dimension identity, preventing retained raid IDs from affecting a destination
with colliding IDs. Existing successful algorithms and native save format remain.
The prior in-memory omen projection on attempted save remains intentionally
unchanged, so this is not whole-Game rollback. Continue still permits duration-1
Raid Omen expiry; skipped effects/deaths/commands are not replayed.

Frozen precommit resource test binary SHA256:
6385190e7c4d5629b79618baae7ee38087288b7d8da521762dc91e2cbd117435.
Build receipts bind both formatted source hashes. It predates the candidate commit;
no clean-commit test-binary build is claimed. Actual isolated execution checks
source/binary/resource hashes and exactly12names with fresh cwd/HOME/game/launcher/
data/config/cache/temp, session unset and inherited display runtime preserved.
All12cases pass, zero ignored, exit0/no timeout,44.88s:

- Seven new cases cover actual lazy transitions with old-path obstruction,
  runtime/helper allocation retention, colliding-ID direct guards, all four caller
  refusals and retries, repaired original-file write, return to retained source,
  native effect expiry, ordinary defaults/seed draws and parked-level round trips.
- Five unchanged H08a14 metadata cases preserve strict error/dirty/repair behavior,
  queued chunk/entity completion, earlier raid-error precedence, sticky writer
  errors, incremental pending-save retention and native no-write guards.

Ordinary native bin tests pass176/45ignored in34.23s; the12explicitly executed
resource cases remain separate. Three feature lints pass43.22/40.46/49.86s;
formatting/public-tree/normal commit hook pass. Independent read-only resource and
parity receipt follow-up finds no blocking issues.

New clean-commit fast Game SHA256:
7738d0c23944068344a07d673e6054ef9bda863d37aee22d81c8205b0543fc39.
Both unchanged full stable-counter regressions pass:

| Mode | Run | Checks | Final committed counter |
| --- | --- | --- | --- |
| Integrated | d1b12272-6486-4d4d-8c32-6c96416ea471 | 368 | 61 |
| In-frame | 3c7301a6-d3df-4feb-a3a6-bcf2dcaa4e34 | 353 | 27 |

Original environments/results bind the same engine commit/tree, Game/resource
identities and unchanged stable wrapper. Four normal initial/restart exits are0;
four intended package-refusal exits are1; all eight cleanup fields are null.
Current-feedback restoration succeeds in both modes. Counter values depend on
ordinary execution timing; no cross-mode trace-equality claim follows. Original
receipts live in .local/h02d/game-runs/<run>. h08a15-game-run-index.json is a
summary only.

Broad --quick passes at the exact candidate:369Python tests,1358client tests,
zero doc tests, formatting/Clippy/public-tree and unchanged locks. Untouched
content-root/theme/launcher and blanket parity are explicitly skipped; the
separate1900-case executed raid-library check is documented below. The broad and
composite wrappers exit0, with counter/quick statuses both0. Broad log SHA256:
5900f17191ca9bf8efc7ffb27e215b4eb8e207b1c0a0db2562feda379349491b.
Final independent read-only receipt review verifies the unchanged clean source/
commit/tree, native/broad results, original Game/refusal/exit/restoration evidence
and separate resource/parity receipts. No material findings remain. Linux binary/
resource identity is supported by coordinator runner checks and bound receipts;
the reviewer independently reads/hashes the accessible original evidence without
executing candidate code.

## Separate unchanged raid-library parity

The existing unmodified vanilla harness executes on frozen clean engine3c4e7e2,
using the owner's pinned Minecraft26.3 artifacts, checked against the upstream
research lock and bundled dependency checksums. JDK25.0.4.1 is selected only for
this process; no Java/Prism settings change. Only lab output/input directory
constants are redirected, preserving the harness, oracle and comparison algorithms.
Fresh owned cwd/HOME/data/config/cache/temp, session unset, offline Cargo, jobs2;
generated output remains outside Git under workspace-root .local.

All1900cases match vanilla:1000patrol,400raid,500codec, including474patrol
pillagers,1364raid waves,11399raiders,149victories and21losses. Process exit0,
all checked source hashes unchanged. This qualifies unchanged raid-library
success/format behavior; the harness does not call Game's new failed handoff or
identity guards and cannot substitute for their actual-Game fault tests.

Original run is .local/h08a15-raid-parity/80c74c1e-6038-4607-af76-f7bb9c663cff,
with rust-raid-parity/report.json plus original Java/Rust inputs, outputs and logs.
Root receipt is .local/h08a15-raid-parity-result.json; console log SHA256 is
690f925a0c0508524e8e6d4defbfa016d76078bca2bfec12df5c79bbc411d05c.
The complete original Java/Rust output files have the identical SHA256:
3aed51b86b48feee4cbb194d2ccdc18a0c19ba21fdd9c4e6ecd2f4bfea9c6801.

No global checkpoint, cross-world instance identity, prior-loss reconstruction,
shutdown-owner lifetime, immutable publication/recovery, external paired peers,
Create conformance or G1 acceptance follows. Generic engine commits remain local;
public publication is separate.
