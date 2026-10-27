# H02b non-executing paired-code validation

Local engine commit `2f1041966d0be580bcecb96ee040d8941503f1fa` on
`codex/h02-paired-types` follows reviewed H02a `0654559`. Neither engine commit
has been pushed or proposed upstream. The experimental feature remains off by
default and absent from the game loader.

`Mods::preflight_pair` compiles both captured code halves with the existing
Wasmtime engine, checks each side's declared imports and supported API functions,
and validates its required callback interface. It returns immutable compiled
components borrowing the prepared declarations. No Store, instance, init or guest
callback is created or run. Failure produces no installed/running entry or output.

An extra negative test exposed an important distinction in pinned Wasmtime 48.0.7:
generated interface `GuestIndices::new` checks names, with function type checks
deferred until instance loading. Generated top-level function checks are stronger.
The final code therefore explicitly checks parameters/results of all 13 required
client/server callbacks against the existing generated Rust types, and rejects
asynchronous callbacks. This is a local preflight correction, not a change to the
legacy loader or a new guest API.

| Check | Result |
| --- | --- |
| Focused paired runtime tests | Four groups pass: valid unreachable dummy pair without execution; each of 13 callback signatures independently corrupted while retaining its name; malformed/core/exportless halves; wrong worlds, missing capabilities and unsupported newer imports |
| Post-fix full feature suite | 22 unit, 4 API-reference, 10 paired declarations, 4 paired runtime and 32 real sandbox tests pass; one existing sandbox test ignored; doc tests pass |
| Post-fix feature Clippy | All targets, `paired-packages,runtime,pack`, warnings denied: pass |
| Broad quick self-check | Pass: public tree, 369 Python tests, formatting, workspace Clippy, 72 selected mod tests, 5 content-root tests, launcher Clippy and 197 launcher tests; full vanilla parity intentionally omitted by `--quick` |
| Independent review | Fresh read-only Sol High review, followed by exact-delta review of final tree `65f6f4b706593f396d63112bbd130dccd4351a56`: no remaining actionable findings |
| Commit hook | Required public-tree check passes; hook remains enabled |

The broad quick self-check completed before the explicit ABI correction. After
that substantive correction, focused tests, the full feature suite and its Clippy
were rerun. Unaffected launcher/Python checks were not repeated. This distinction
is deliberate; there is no claim of a second complete self-check or full parity.

Raw logs remain outside Git in parent-workspace `.local/h02/`:
`typed-pr-ready.log`, `typed-signature-tests.log`, `typed-final-regressions.log`
and `typed-final-clippy.log`. Tests use scratch package files and isolated runtime
data. No launcher UI or owner world was opened by these checks.

This does not freeze content files: `Content` still retains a folder source and
reads it later. It does not negotiate package identity, register content, discover
paired folders, activate either side, isolate their storage, or prove connected
client/server behavior. Optional UI/browser callback interfaces also need their
activation-path checks before being supported by a paired loader. Full H02, G1,
transactions, machine persistence and retained rendering remain open.
