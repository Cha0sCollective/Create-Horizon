# H08a12 checked borrowed network-player NBT writer

Local engine commit `f7f8990dbeaad2b4d036f2062d5a6dc0194c2cf4`, tree
`8db0afb2ddf457b3ec529ab96f9e49fdc1b12ae1`, exposes failures from
lan/playerdata.rs::save as crate::Result,
with the canonical player target path in errors. The Guest-to-Level-to-gzip-NBT
projection, UUID formatting, guards and ordinary _new/_old replacement stay intact.
Existing retirement and dedicated shutdown callers log and continue later JSON,
player, kick and world-save attempts. They do not retain failed retired players.

Fresh independent read-only Sol High design and stable three-file source review
find no issue. Final formatted source SHA256:

| File | SHA256 |
| --- | --- |
| lan/playerdata.rs | `acbd0948b778fbbd08d0bf6c3c585052f5cec894925145effc7516a2af0c6dca` |
| game/lan.rs | `f951e629b2b9b5e3440aaa3e1386f716f7afd6ad9b602b7b4b1156c927a4fa96` |
| game/dedicated.rs | `51ef6373d97a6f3d94239bb817372e538859c3703dd791ea52f4aec87380f3b7` |

All three playerdata tests pass, including two new concrete parent/_new obstruction
cases. They verify actual strict gzip/NBT output, canonical signed UUID, inventory,
equipment, ender items, components and opaque data; unchanged borrowed Guest state;
repair/retry; reset over previous contents; and ordinary backup replacement without
test-only backup removal. Unchanged old bytes are asserted only for the specific
pre-open obstruction, not arbitrary write/rename failures. All 172 binary tests
pass (three ignored). Feature-on/off Clippy, formatting, diff/public-tree and the
normal commit hook pass. Composite native validation exits zero.

Normal local/in-frame and integrated-owner player NBT is saved through Level::write,
so inherited counter game success will be regression evidence, not coverage of the
external-player writer's filesystem faults. Native writer tests and caller source
review establish this bounded change. General shutdown continuation, guest lifetime,
cursor/merchant normalization, same-batch reconnect, all-player capture and coherent
checkpoint recovery remain open. No persistence/G1 or public publication is implied.

Frozen optimized game SHA256
`053e8f11d8e2f080cddda3da885c7e600d9a1bf9f48ba8d58b5232eab8e8d5c9`
passes the unchanged full counter driver:

| Mode | Run | Checks | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `78fea2ac-8fca-4674-b33a-6306502e9cf4` | 362 | 63 |
| In-frame | `4a765e9c-99e6-4c78-90e1-d267b2a23a48` | 347 | 27 |

Normal initial/restart exits are zero with null cleanup errors. All expected
package refusals exit one and preserve saved bytes. Changed-package cleanup races
with the already-exited process in integrated mode and the removed socket in
in-frame mode; both missing-package cleanup records are null. These expected
refusals are not clean normal exits. The driver remains SHA256 d19c4805... with
the full pin in CURRENT_WORK. No persistent counter payload is restored.

Local raw validation uses .local/h08a12-*; invocation scripts are archived there
and remove HORIZON_SESSION_FILE. Qualified game drivers stay immutable.
Broad quick checks pass in the native clone at f7f8990: 369 Python/1,357 client
tests, Clippy, formatting, zero doc tests and public-tree. Three files are +351/-6,
Cargo.lock is unchanged, and content/theme/launcher checks are unchanged-scope skips.
Parity is explicitly skipped. CI update records the actual candidate; the footer
prints the merge base. Composite counter/quick exit codes are both zero. Final
independent read-only Sol High receipt review finds no issue. It cross-checks
binary identity through matching build/run receipts; an optional Linux binary
rehash was unavailable in that review. No candidate execution was performed by
the reviewer. Source, tests and attributed game evidence support only this packet.
