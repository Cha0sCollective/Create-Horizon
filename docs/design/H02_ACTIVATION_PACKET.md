# H02d: discovery and local side activation

The owner authorized this packet after H02c and explicitly asked for useful parallel
delegation and continued work until a legitimate blocker or required input. The
earlier October 27 time cutoff does not govern this resumed assignment. This is
development authorization, with no public push, release or upstream submission.

Base: local engine `58e49d4d0717b7d2382a6e6e02b58ee3bd531cfd` and private
`84e476186f8d9c442545347dc5af849fd3c26390`. Verify actual dirty files before
integration. Engine integration branch: `codex/h02-activation-integration`.
Private notes branch: `codex/h02-activation-notes`.

Implement deterministic mixed legacy/paired discovery, explicit ambiguity and
ownership rejection, one captured paired content owner, and one installed row with
accurate permissions for both executable declarations. Capture selected packages
once before content registration. Pass the same immutable snapshots to the game
runtime and its integrated server. Validate both halves, including optional
interfaces, before guest initialization. Select client/server hooks by execution
role; the in-frame fallback needs two separate Stores. Preserve existing per-side
fuel, memory and I/O limits and isolate side storage.

Package selection/content changes require restart for paired packages. The local
prototype must refuse unsupported network admission before registry traffic and
refuse writable saved-world mismatches before world loading. Exact package pins
are admission metadata, not authoritative machine storage or a completed H09
protocol. Do not enable external paired clients/servers without negotiation.

Coordinator owns startup, resource reload and admission integration. Separate
bounded workers own catalog and runtime changes in native isolated worktrees.
Neither worker commits, stages or publishes. A fresh independent read-only Sol
High reviewer checks the exact integrated candidate and follow-up fixes.

Validation: focused discovery negatives; actual Wasm role and storage isolation;
optional ABI rejection without initialization; init failure cleanup; snapshot
reuse after source edits; feature-on and feature-off compile/lints; existing mod
sandbox/content regressions; isolated real-game integrated and in-frame startup
where supported. Record unsupported paths explicitly. Stop at a reviewed local
activation seam or a demonstrated missing prerequisite. Live Create machine
lifecycle, transactions, persistence, retained animation and full H02/G1 remain
open; do not substitute guest inventory sidecars for later host work.
