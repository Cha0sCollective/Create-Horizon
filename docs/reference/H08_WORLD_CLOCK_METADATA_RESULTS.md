# H08a9 world-clock metadata results

Local engine commit `0028c89d31438d159ee088eafe14827ba6c76488`, tree
`b24c06a7221e56818f04b1c9765bb70dcf7bb2ad`, changes only save.rs, game/chat.rs
and game.rs. Fresh independent read-only Sol High design, code, harness-security,
test-only delta and final receipt reviews find no blocking issue. Final SHA256:

| File | SHA256 |
| --- | --- |
| save.rs | `688ae37142aca312fec253d815fb4af8dfb014aad5d808d4b23b08f6aa6abe04` |
| game/chat.rs | `507ec9fa5ff400ccc62a3c254c80e678fa36dd8c6815a74c191d0e7df3b3a781` |
| game.rs | `614a840ca483ad92991305bd5c28c242518a8ccec94c4f6f18715f7e4685f6a5` |
| Private clock driver | `e29e1b9d823a077be0172cac4b4a296e33c3643059f54778227f3c80125f5f30` |

Both Level::write and Game's later clock save use the same concrete strict
decoder/merge writer. Only NotFound means absence. Rejected compression/NBT,
containers, owned-field types and unsupported newer DataVersion refuse writing
with clock-path context. Raw/gzip decoding keeps 64 MiB/one-million-node/depth512
bounds and checks complete NBT consumption. Unknown root/overworld fields and
other clocks survive ordinary codec round-trips; existing End data stays opaque,
and Level initializes only missing End data. Game applies existing control/default
comparisons. Existing negative-zero read normalization and NaN write canonicalization
remain; no new numerical ranges or migrations. The controls loader resets first
and tolerates bad input. Both write errors propagate before the save-success message.

Eight resource-free groups cover actual Level then Game-style writes, unknown
sentinels/default removal, missing containers/End, rejected files/types/versions,
repair/retry, raw/gzip data above the former 1 MiB Game-loader limit, float codec
semantics and controls reset/live-state retention. All 42 save tests pass (eight
inherited explicit-resource tests ignored). The final test-only delta removes
precautionary backup deletion, so consecutive writes use ordinary rotation; the
42 tests and on/off lint were rerun. The unchanged production candidate's 156 game
tests passed (three ignored); final broad checks rerun the complete final source.

Frozen optimized game SHA256
`00d38a8a24efc2ab7e78568e6c9348b59f01067c14d6d4e5c6c4c08551e0852e`
passes the unchanged extended counter driver:

| Mode | Run | Checks | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `64ec267b-cbc6-418a-833a-6967ea1dddef` | 362 | 67 |
| In-frame | `19c8af4d-c1c4-4b07-8d24-ead875940edf` | 347 | 26 |

Normal initial/restart exits are zero with null cleanup errors. Expected package
refusals exit one and preserve world bytes. Integrated changed-package cleanup
finds a removed socket and missing-package cleanup finds an exited game; in-frame
refusal cleanups are null. Counter driver remains d19c4805... (full pin in CURRENT_WORK).

The separate clock fault harness passes on the same frozen binary:

| Mode | Run | Checks | Outcome |
| --- | --- | --- | --- |
| Integrated | `152e9ad3-6f1d-47b1-bfde-9ef7f2c42212` | 19 | clock error, exact old bytes, same-process repair, clean exit0 |
| In-frame | `96dd95e0-ecb6-4315-b9ac-b1625ebdd5f6` | 21 | clock error, natural exit1, exact old bytes, repair/reopen, clean exit0 |

Both have null failure and empty cleanup-error lists. JSON/report/console errors
name world_clocks.dat. Integrated repair persists the still-live rate0.25 after
baseline0.5; in-frame reopen retains prior saved0.5 without claiming live state
survived process death. The sole fault is an identity-checked owned empty _new
directory, removed with rmdir. This normally reaches the first Level shared writer;
native tests cover both writer modes. Only clock bytes, not the whole world, are
asserted unchanged. Session-file environment is removed and data homes are fresh.

Preserve excluded failed attempt `2de93d5d-6d54-4ff3-94c7-5dc28b9f3158`, driver
071648b1...: its fresh XDG_RUNTIME_DIR hid WSLg's compositor before world readiness
or injection. Reviewed correction preserves the base helper's existing display
service endpoint and records it separately; data/config/cache homes remain fresh.
Original combined status retains clock exit1; reviewed rerun status records exit0.
The failed startup is not clock-error qualification.

Final broad quick checks pass in the immutable clone at 0028c89: 369 Python and
1,339 client tests, formatting, workspace/feature-off lint, zero doc tests and
public-tree checks. Cargo.lock is unchanged. Scope has a soft 747-insert/78-delete
warning, including tests and mechanical moves across three files; no PR is proposed.
Content/theme/launcher are unchanged-scope skips and parity is explicitly skipped.
The tool footer prints merge-base b9b32da, not candidate HEAD; ci-update.log records
the checkout to 0028c89. Raw records remain `.local/h08a9-*` and the named run folders.

Independent receipt review confirms attribution, source/driver hashes, native and
game outcomes, raw retirement records and separated failed/retry evidence; it did
not execute the candidate. Metadata remains nonatomic, write_dat rotation failure
windows are unchanged, and this is not coherent recovery or full parity. Other
metadata/ownership gaps and G1 remain open. Counter payload is transient. No engine
push occurred.
