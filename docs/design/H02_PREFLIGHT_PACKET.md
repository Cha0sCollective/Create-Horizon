# H02a: experimental paired-package preflight

Active local engine packet after reviewed P02a commit `9bc54c0`. Engine base is
the original pinned `cf62fe270b243780f9aa86c49b456d9dd9078b42`; branch
`codex/h02-paired-preflight`. Keep all generic implementation in that repository.
This private note is not an engine/public source file.

Implement a small off-by-default `paired-packages` feature in `horizon-mods`.
Its standalone preflight reads an experimental folder descriptor, validates one
content owner and two executable declarations, applies existing side/capability
rules, checks required files and bounded reads, and produces a side-specific plan.
The content owner registers once. A server plan never selects the client half;
the in-frame fallback plans two separate halves. Planning is not execution.

Keep this format separate from existing `horizon-mod.toml` and the supported API
list. Candidate `horizon-paired.toml` format 1 uses `[content]`, `[client]` and
`[server]`, fixed `client.wasm`/`server.wasm`, and exact experimental contract
`paired-preview-1`. Reuse current API 0.2 content ownership, API 0.1/0.2 client
and API 0.3 server validation. Do not take draft PR 1189's API 0.4 identifier or
claim that its block-entity preview is implemented. New machine imports are later.

The legacy loader must continue rejecting/not discovering this format until H02b
wires full admission and component instantiation. Do not turn a parser test into
a claimed integrated-client pass. Actual Wasm type/import/digest checks and runtime
activation remain mandatory before a writable world can use the bundle.

Stop with focused manifest/content/file-boundary tests, existing `horizon-mods`
regressions, relevant formatting/lint/public-tree checks and a fresh independent
review. A malformed/missing declaration, missing half or namespace collision must
fail before the caller receives a prepared plan. Do not build archive packaging,
registry publishing, version negotiation, machine saves or a general loader here.

## Overlap and source boundary

Checked October 27, 2026 through GitHub CLI and connector. The old
`ludovic111/horizon` URL redirects to `horizon-rust/horizon`; search using the
canonical name, since old-name searches silently returned no results.
[PR 1189](https://github.com/horizon-rust/horizon/pull/1189) remains an open draft
at `847210b1e764638c97fce84489b2e9c368668388` and still excludes actual world
creation/ticking/saving. Searches for paired / client.wasm / server.wasm found no
open paired-package PR. PR 2935 concerns storage quotas/locations; do not conflate
installation settings storage with durable machine records.

The implementation uses Horizon's existing manifest/content behavior and this
original generic contract. It contains no Create domain rules, reference assets,
private specification or orchestration configuration. Public publication remains
separately gated. G1 and H02 as a whole remain open after this preflight slice.
