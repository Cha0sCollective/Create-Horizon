# H02a paired-package preflight evidence

Reviewed local engine commit:
`06545594bad9e67982048b3e70aff40ee26ba53b` on `codex/h02-paired-preflight`.
It remains unpublished. The engine source contains only generic preflight code
and its public-facing description; this private evidence report stays here.

The off-by-default `paired-packages` feature validates an experimental folder:
one content owner, two required fixed executable declarations/files, existing side
capability rules, bounded regular-file reads and namespace collision/fence checks.
Its role plans select code without executing it. The legacy loader still does not
discover or activate the format. This is not full H02 or G1.

| Check | Result |
| --- | --- |
| New preflight tests | 10 pass: owner/roles, declarations, capabilities, fixed paths, reserved IDs, owned/alias/foreign content, collisions, missing/empty/oversized files, symlinks |
| Existing API/runtime checks | 22 unit and 4 API-reference tests pass; 32 real sandbox tests pass, 1 existing ignored |
| Feature-specific Clippy | Pass with warnings denied |
| Game workspace quick check | Public tree, 369 Python tests, workspace Clippy, 72 mod tests, 5 content-root tests and formatting pass |
| Launcher follow-ups | Locked-dependency fetch, Clippy and 197 library tests pass; 2 existing ignored |
| Independent review | Reserved primary-ID gap fixed locally to paired preflight; final strictly read-only review has no remaining findings in tree 4f1834517d50574abbd286108ec3c6cb8d71fdef |

The initial offline sandbox attempt lacked a pinned SDK dependency. The initial
quick self-check also failed its launcher steps because that separate workspace
was not cached. After fetching locked dependencies, only the affected checks were
rerun; they pass. The original quick-check invocation therefore has exit 1, with
explicit successful follow-up receipts, rather than a fabricated green full run.
Full vanilla parity was not run; it remains a publication/release check when needed.

Raw logs are parent-workspace `.local/h02/`: `tests.log`, `sandbox-retry.log`,
`clippy.log`, `pr-ready.log`, `launcher-fetch.log`, `launcher-clippy.log` and
`launcher-tests.log`. Runtime tests used their own scratch data; launcher checks
used a fresh isolated home/data/config/cache/tmp tree. No launcher UI was opened.

The source permits no normal asset ownership of reserved primary IDs `minecraft`,
`c` or `horizon` in the new paired path. Legacy parsing behavior is preserved;
the paired check reuses the reserved list without changing existing formats.

Executable type/import checks are now recorded in H02_TYPED_RESULTS.md. World admission, peer/content
identity, side activation/state/storage, machine records and persistence remain
later work. A parser pass cannot be reported as a running or saved machine.
