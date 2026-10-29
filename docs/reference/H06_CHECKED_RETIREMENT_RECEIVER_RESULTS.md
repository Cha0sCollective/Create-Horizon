# H06c checked resident retirement receiver results

The dormant paired-packages Game helper moves one World-owned pending retirement
into actual Game.dropped storage. It adds no enabling caller, command or timer.
The acceptance boundary is docs/design/H06_CHECKED_RETIREMENT_RECEIVER_PACKET.md.

Local engine commit: 51e2882cd582e862e5e709e7d6d8b7e7978d852e, based on
dc12bd60aeaee33f09b18d9e8ffee4d763a52355. The normal public-tree commit hook passes;
the engine change has not been pushed publicly. The five-file change adds 708 lines,
including the 543-line actual-Game test module, above the upstream soft size target.

Preflight checks the native expected opaque key/current pending allocation,
captured scope and actual Game.dim, plus original-column residency. level.dimension
is intentionally excluded because parked-level swaps do not swap player metadata.
Positive roots must be registered/non-air. Counts must be positive and fit signed
NBT integers at the root and in the four modeled native child collections, using
a borrowed checked worklist. Opaque tags are uninterpreted. Oversized representable
counts move intact; this is no universal stack-size cap or complete codec proof.

Checked entity ID, temporary/vector reservations, cloned particle RNG and scalar
results precede consuming the retirement. Original SlotProjection::into_stack moves
the payload without normalization. Only invariant-backed extraction, struct fields
and reserved publication follow. None acknowledges once without ID/RNG/entity
changes. Refusals retain pending contents and context; no auto-reconciliation.
Existing snapshot inspection clones stack data before take. Successful publication
moves the original owned payload; no payload clone follows that successful take.

Reviewed source SHA256:

| File | SHA256 |
| --- | --- |
| client/src/game.rs | `2d780b6eec6b348c95103b50c23707fbf6f2cb0a32fed82f8271038247049f03` |
| client/src/game/slot_retirement.rs | `7b8b81848473fb2f75fc5672e09aa1e7e3a97e825517da4f40d3e79f8850a858` |
| client/src/slot_transfer.rs | `9ae98c925cd74d28079ffcd91b25fe409c70482f49e1a4eb7ddccd826b577bc8` |
| client/src/game/integrated/transition_ownership_tests.rs | `1d18f54cace20bc3146d5ed0bbe1255c3899e2b63fde8ae19b619ebae9243f87` |
| client/src/game/integrated/transition_ownership_tests/slot_retirement_tests.rs | `132b16dcf970bdbd0dce0324d3e7f4ef023e48bf4a66298aebe3a6d17a6c83da` |

Fresh independent read-only Sol High design and implementation review find no
blocking issues. The original resource fixture and four transition-test bodies are
unchanged; only a gated child declaration is added. The new fixture uses captured
machine metadata/stub component files without instantiating those Wasm bytes.

Frozen native test artifact SHA256
`8392e9838fbca472045fd230ab1fa812eaa0f8475218ecfac0549a09f13c1870`
pins all five source hashes. Its build precedes the final source commit; it does
not claim a clean-commit binary build. The isolated runner checks source/binary/
resource hashes and the exact six names, then runs with fresh cwd/homes/temp,
session removed and inherited /run/user/1000. All six pass, zero ignored, in 20.24s:

- Original metadata/recursive BundleSelected values move once; existing entities
  remain exact. Counts 5/99/i32::MAX are delivered intact, and empty ownership
  acknowledges without creating an entity.
- Foreign allocation, dimension mismatch, exhausted ID and nonresident source
  retain actual pending key/revision/raw contents, entity entries, ID and RNG.
  Public Streaming.update drives real World removal/admission for successful retry.
  This fixture exercises streaming hooks, not all Game entity-unload persistence.
- Unknown/air roots, oversized signed count, zero/overflow counts in all four
  modeled collections and a deeper child refuse without metadata loss. Negative
  fields inside opaque tags remain ordinary retained data.
- Real enter_level swaps preserve parked World/drop ownership. Delivery follows
  Game.dim even when level.dimension still names Overworld, and never duplicates.
- An inert integrated frontend handle and actual local memory-Pipe client both
  refuse without consuming ownership. The peer is shut down and Session dropped;
  auth reads only process memory and the route uses no external network/account.

Seven library transfer/extraction tests pass in runtime-on and paired-only modes;
the extraction test covers None, zero-count representation, raw selection and the
original metadata allocation. Ordinary binary tests pass 173 with 14 ignored; six
are the explicitly executed receiver cases, four are previously qualified producer
cases, and four are earlier captured-fixture tests. All three feature lint matrices,
formatting and public-tree checks pass. The broad native-checkout run also passes:
369 Python tests, 1,357 client tests, zero doc tests, formatting and Clippy. Locks
are unchanged; content-root/launcher/theme checks are skipped as untouched, and
vanilla parity is explicitly skipped by --quick. Its soft warning reports 708 lines.

The wrapper exits 1 after those completed checks: its final CRLF sends printf's
successful status JSON to a CR-suffixed filename, then a CR-only blank line fails.
The original script, malformed filename and status remain preserved. The status
was copied byte-for-byte to its intended filename; a separate wrapper receipt
records both pr_ready exit 0 and wrapper exit 1. No checks were rerun or failed
run relabelled. Broad log SHA256:
4934785672e1101da07fdc8d937ff97d8066c4cccccebfe6f70a25bf7939bc42.
Final independent read-only receipt follow-up finds no blocking issues and accepts
this bounded dormant receiver. It independently verifies unchanged source hashes,
the broad log hash and identical original/copied status bytes. Raw evidence stays at workspace-root
.local/h06c-*. No engine fixes have been needed after source review/execution.

Recoverable allocation refusal is source-order reviewed, not forced by memory
exhaustion or a mock allocator. Current Game counter-mode proofs are unchanged
coverage from H08a13; no new ordinary-game execution is claimed for an unconnected
helper. All World-loss/save/shutdown receivers, player authority/borrowing/replay,
guest transactions, coherent recovery, complete H06/H07 and G1 remain open.
