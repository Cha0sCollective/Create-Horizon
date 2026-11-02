# Game streaming owner representation

FINAL ACCEPT at engine `7d443a1bf41a83db6ff5b8426460a224a4bf612a`, tree
`d37a5faec2740407c945ace5352c2e7d5caa1cea`, parent `e724758`, branch
`codex/s1-game-streaming-owner`. Different independent FINAL receipt `cd4090f3`
also clears public privacy. The normal-hook public push was verified exact.
This accepts the factory-invariant representation change, not a remote Game.

## Result

The 39-file diff has 552 additions and 181 deletions: one small private owner
module and 38 mechanical Game access/adoption changes. Game now explicitly stores
an original Existing stream or an opaque retained remote owner. Boxing moves the
same owner; it does not construct a substitute pool. The enum is non-Clone and
has no Deref, generic trait facade or remote inner-streamer escape.

Checked borrow, consuming extraction, replacement and borrowed active/parked swap
refuse the remote variant without losing its original owner or the caller's
incoming stream. Successful replacement returns the old owner for caller
disposition. Current production replacements retain their original disposal
boundaries. The active/parked swap checks the variant before clearing native
provenance or moving World, then preserves the original successful swap order.

Every production Game initializer and replacement still installs Existing,
including today's legacy remote NoTerrain pool. Existing is a representation
label, not native provenance. Native authority, SaveDir and session-allocation
checks remain exact. PreparedWorld, preparation owners and parked levels keep
their raw Streaming values. Library streaming.rs and world_loading.rs are unchanged.

Explicit field-local assertions document the current factory invariant while
preserving original signatures and split field borrows. No production command,
environment option or initializer installs the retained remote variant. A later
actual remote recipe must replace every reachable assertion with honest dispatch
or refusal before side effects and owned-input loss. This packet supplies no
default-value fallback, fake Game/storage/writer or supported remote dispatch.

## Qualification

All checks use the same post-format 39-path manifest `ea74c758`, qualifier
`569d5c10`, expected definitions `71892baf` and exact Game-bin artifact. The build
uses opt-level 0, line-table debug information, debug assertions, overflow checks
and exact `mods`, `native-writer-test-control`, `paired-packages` features.

| Check | Result |
| --- | --- |
| Explicit-file format, paired all-target Clippy, feature-off compile, public tree | Four passed, 60.70 seconds |
| Game-bin test build | Passed, 90.16 seconds |
| Three resource-backed owner groups and actual cold-native authority regression | Four passed, 46.70 seconds |

Everything passed on the first attempt. Initial source review found no blocking
issues; bounded pinned formatting with LF conversion received renewed exact
review before compilation. There were no source corrections or failed executions.

The new groups use the actual existing Game-bin Fixture and pinned WorkerContext,
production TestTerrain/Streaming construction, original LOD-source Arc identity
and native observations. They exercise original extraction/replacement/swap,
intact incoming/slot refusals, and genuine checked closure of an inert opaque
remote owner. These observations do not prove every private allocation's identity;
the inspected move/swap bodies establish that distinction. The existing cold
factory test checks actual original native authority and all three real gates.
Unchanged successful swap/disposal behavior did not justify the optional parked-
save test. Historical full window, network, save, Java and fatal batteries were
not replayed.

The complete 28,176-entry resource inventory was checked before and after the
four-case batch. Missing resources, timeouts, missing cases and nonzero exits
would fail qualification. The actual binary is
`1c44a69a1d199582b01722d536c2a82be84b2077c1ba36ba37933735343ef6ff`.

## Integrity and remaining work

All 53 mutable input pairs are archived and mapped (`587f7096`): 39 changed sources
are raw-exact committed Git content, ten unchanged baseline files have explicit
CRLF canonical comparisons, and four private recipe/instruction inputs match raw
bytes. Supplemental workspace/current-work context was copied after qualification.
Different FINAL rehashed all 196 frozen inputs initially/finally and all 3,262
historical indexed entries, with zero mismatches. Its index is `36a4925c`.

The reviewer inspected source, recipe, guarded receipts and matching logs using
native read-only operations. It did not reopen the Linux-only binary/resources
or execute tests. Qualification ran guarded dirty-parent code subsequently
committed raw-exact; no archive execution or fresh clean-commit execution is
claimed. The qualifier is retired by its original-parent guard and stays immutable.

The original remote Game still needs honest no-local-storage representation,
retained Session/decoder adoption, immutable or revision-aware mesh configuration,
checked client admission/scheduling, original construction and close composition.
Server startup/Hub ownership, shared LOD/storage ordering and scoped checked joining
remain separate. S1/G1 stay open. No remote Game/default activation, live S2,
whole-client completion, performance qualification, PR, merge, tag or release follows.
