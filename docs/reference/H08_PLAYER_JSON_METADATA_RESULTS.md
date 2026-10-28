# H08a11 checked borrowed player JSON metadata results

Local engine commit `ecb93cc57f749037dfdb8e26ccad0123e1ba5240`, tree
`a872296669da48fe499f8abb6534d094cb14f663`, checks local and integrated-owner
statistics/advancement writes. JSON results are deferred until the existing Level
and clock metadata attempts complete. Integrated shutdown attempts LAN cleanup and
writer flush even when saving fails, preserving the first error. The explicit
session-lock field is released on success; on failure it remains owned until the
server Game drops. No retired-player snapshot is retained by this change.

Fresh independent read-only Sol High design review found an initial shutdown-order
gap; the revised common/deferred ordering and minimal shutdown cleanup resolved it
before implementation. Stable six-file source review found no issue. Record guards,
UUID selection, advancement-before-statistics order and both independent attempts
remain intact. Network dirty entries, scoreboard events and advancement UI queues
are not disk-completion flags and remain unchanged.

All 172 binary tests pass (three ignored), including real JSON directory/leaf
obstructions, same-state repair/retry, empty/zero/revoked projections, identity and
guard checks. A native content assertion proves the statistics attempt still runs
when the preceding advancement write fails. Feature-on/off Clippy, formatting and
public-tree checks pass. Final source hashes are captured in .local/h08a11-source-hashes.txt.

Frozen optimized game SHA256
`2f2e062a433bcf12a703a5ace904f5294c460548ad16850fbac06eb7a945192d`
passes the unchanged extended counter driver:

| Mode | Run | Checks | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `fee6a990-f32f-483c-b40b-ade0d8a54f51` | 362 | 62 |
| In-frame | `f0f26d53-1a0a-4586-b20e-f4be8c82bd2d` | 347 | 20 |

Normal initial/restart exits are zero with null cleanup errors. Both modes' changed
and missing package refusals exit one, preserve saved bytes and have null cleanup
errors. Driver SHA remains d19c4805... (full CURRENT_WORK pin). The coordinator's
execution record says the older driver was wrapped with unset HORIZON_SESSION_FILE;
its environment metadata does not independently attest that, and no separate
invocation script was archived. These runs establish counter/lifecycle/save regressions;
the counter payload remains transient.

Broad quick checks pass in the native clone at ecb93cc: 369 Python/1,355 client
tests, format, Clippy, zero doc tests and public-tree checks. Cargo.lock is unchanged;
the soft scope warning is six files, +754/-72 including tests. No PR is proposed.
Content/theme/launcher checks are unchanged-scope skips; parity is explicitly
skipped. The CI update log records the actual candidate; the pr_ready footer
prints its merge base. Original composite counter/quick codes are zero.

## Focused integrated-owner fault probe

Initial probe SHAb4dd9dcf... run `597e166d-4a0b-42cb-9e9f-f841f4f11f21`
failed baseline block readback before any JSON obstruction was installed, with no
cleanup error. The raw saved palette correctly contains the gold block as a 26.3
mixed-list string wrapper. The qualified raw NBT reader preserves that wrapper;
the new probe had not unwrapped it. The independently reviewed narrow correction
accepts only a sole-empty-key compound before the existing exact block-name check.
Engine and qualified helpers are unchanged; the original failure stays failed.

Corrected probe SHA256
`06741fc3070107f4c74f8547e2abcd0349f4497ff3b4a4aa4d5c21ec4bdad5d0`
passes separate fresh integrated-owner runs:

| Obstructed record | Run | Checks |
| --- | --- | --- |
| Advancements | `d816b220-cd4e-44a8-8ba8-32c5d5212832` | 78 |
| Statistics | `2c1452a5-c9ae-492c-84c1-615e00d6209b` | 78 |

Each synchronous save returns a contextual JSON error while the game remains alive.
Parsed player inventory, chunk block, Level spawn and clock mutations reach disk
despite that error. Another distinct mutation persists through failed shutdown.
Both shutdowns naturally exit one (forced=false), with the exact path in the report
and console; owned repair/reopen saves and naturally exits zero. Both results have
null failure and empty cleanup_errors. Original aggregate player_json_exit_code=1
is preserved separately from the corrected rerun's zero status. The display-service
runtime connection is inherited; save/data/config/cache homes are fresh, and every
test child is stripped of HORIZON_SESSION_FILE. Raw runs stay outside Git.

Direct JSON writes are not atomic and may partially truncate on other failures.
Pre-open obstructions establish only their specified behavior. Runtime record-shape
checks alone do not prove both independent write attempts; the native content
assertion supplies that coverage. Playerdata writes, retired owner/guest retention,
all-player capture and coherent checkpoints remain open. Final independent read-only
Sol High receipt review finds no issue, including the disclosed older-driver
attribution limit; it did not execute candidates or rehash inaccessible frozen Linux
binary bytes. Matching build/driver/environment/results support attribution.
No G1 or public engine
push follows from this packet.
