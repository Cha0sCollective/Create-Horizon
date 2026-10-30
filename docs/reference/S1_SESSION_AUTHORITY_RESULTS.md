# Trusted session authority results

IN PROGRESS. Fresh independent design accepted exact6f98cc8d; source delta is
independently CLEAR before execution. Formatting, all three feature-lint configurations and public-tree checks PASS;
debug test executable build PASS. Ordinary199 PASS and resource0..10 PASS as last observed;
remaining resource/legacy groups are running, NOT a complete-suite pass.
Local worktree .local/worktrees/s1-session-authority, branch codex/s1-session-authority,
base872dac2da765f0b290bfcfe13502f7d97724be1f; five scoped paths, uncommitted.

## Intended behavior and limit

A dormant explicit managed entry holds one OS authority lock per trusted WorldUUID
in one fixed configured namespace. New identified creation acquires it before
identity/package publication and streaming. Guarded recovery reads only the selected
owner's bounded identity under its real native lock, derives expected UUID internally,
retains both locks through unchanged strict archive recovery, and transfers authority
into the recovered WorldSession before returning Game. Native-only managed reopen
refuses. No change to checkpoint schema or the existing unmanaged research primitive.

Complete owner/namespace/archive/destination separation and authority UUID-target
checks precede lock writes; native and authority locks are rechecked. A malformed
identity or linked/overlapping route must refuse without repairing it. Authority
files persist after release; locks, not marker existence, establish exclusion.
The exclusion claim covers cooperating opted-in callers using the SAME configured
namespace. Legacy/unmanaged opens, different namespaces and hostile filesystem
replacement are outside it. Linux validation is intended; no Windows runtime claim.

No ordinary startup/close caller, failed-close worker retention, connected-player
staging, in-frame recovery or filled-item ingress. This does not close S1/G1.

## Source and review

Exact five-source raw manifest retained at .local/s1sa1-source-sha256.json.
Authority module SHA25620333e3cc938dac333cc8e57ad5122a7517b264bfdd4030f2fd3cea94bdde795;
other four source hashes remain unchanged from initial formatted source review.
Initial hash snapshot preceded formatter completion and was refreshed, not executed.
Source review found one P2: linked authority child/lock was checked after owner lock
write. Correction moves that preflight before native acquisition, retaining rechecks;
fresh-owner assertions verify unchanged identity and no added lock. All five sources
and packet artifacts preserved before correction. Independent delta review CLEAR.

## Validation pending

Frozen expected suite:199 ordinary (193 retained +6 new),18 checkpoint resource
(16 retained +2 new),12 legacy resource. Positive-owner native Game objects are
component participants, not real connected/admitted network clients. Retained tests
use pinned resources and prebuilt package fixture, never compile guests internally.
No claimed passing counts until actual execution and evidence verification.
Three feature lint configurations PASS; optimized TYPECHECK, committed clean broad quick
and a different final independent review remain required for bounded acceptance.
Raw logs/receipts/worlds stay ignored under .local or isolated WSL cache; only this
small authored summary enters the private project. Engine local only, normal hooks.
