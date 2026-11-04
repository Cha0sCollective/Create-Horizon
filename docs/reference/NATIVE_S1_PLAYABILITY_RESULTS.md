# Native S1 playability results

Status: native S1 complete after independent checklist review. This is the
owner-adopted November 4 native playability milestone, not a pass of historical
Wasm G1 or full Create/modpack compatibility. See
[the adopted plan](../design/NATIVE_PLAYABILITY_PLAN.md).

## Implementation and provenance

Private branch: `codex/native-create-playability`. Adoption commit `d8c7274` followed
one fresh read-only Sol High operating-rule review. Source import commit
`e401338aa1bc1a921139a33145a70c730f6011d7` preserves all500 upstream tracked files;
its `native/create` tree equals upstream tree
`7ce954394753d1a6c490157660ec6eb7a6c0cb8e` at
`16c5d454b1b9860da01b78474ea5206f0664d384`. Original MIT notices remain intact.

The local patch changes two gameplay Rust files: separate NeoForge's processing
guard99 from normal insertion64, including the matching client particle guard.
The existing insertion path already discarded uninsertable results. The remaining
changes repair the nine-slot probe/reference context, affected checks and explicit
host checkout selection. No reusable engine service, host API or new save format
was implemented. Compatible official Horizon remains
`fc02103818c212920fd24aa34a2b2f91ba49e56f`, Mixins
`a1e13326306c6bc074b11e6a4926deb551c4083e`, native API359 and Rust1.99.1.

Create behavior reference is6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`,
NeoForge21.1.219. Upstream source/art bootstrap remains6.0.10; only the bounded
selected milling reference is reconciled here. Original art is imported from the
private pinned JAR, never distributed. Source build/activation is qualified;
launcher registry/release packaging is not.

Stable nine-file source map SHA-256:
`8ebf7d274fb56be9cda6539038751171bde7cedd3e5ac2d289a4003dd5d47a0e`.
The reviewed patch is committed at `76189de63a6c31332d9e8dbc95fefca700497551`.
Independent `native_milling_source_review` found no actionable source/false-pass
findings, verified the import tree, limit separation and final focused oracle.
Separate reviewed local drivers identify actual player actions versus diagnostic
layout/item injection and authoritative server queries versus client observations.

## Focused checks

- Six milling Rust unit tests passed; unchanged tests were filtered, not replayed.
- Thirty-six selected comparisons passed against AST-extracted6.0.11 Java
  tick/process/speed bodies using declared inventory/recipe contexts. These are
  cycle/output-limit controls, not chance-roll or a full Java game qualification.
- Focused Clippy for the native library/milling probe passed with warnings denied.
  The two changed crate Rust files pass pinned formatting; changed Python tools
  pass syntax compilation. No full historical oracle or Minecraft battery ran.
- The private native export applied with34 injections,14 modules and two crates.
  Game plus matching control client built successfully, fast profile4m22s.

Game SHA-256: `8acfe51f818c07e228d1926ae8a468a7daf20d922d61390a662ac67275ba9cc0`.
Control SHA-256: `ee31bde46244aa8d686e5818a7450ea83c4095dde415b2d57b2d6fd69ddee06f`.
The prior evaluated binaries were copied to a separate private cache before reuse.
Clippy's rustup helper reported automatically installing the same pinned1.99.1
toolchain. No uninstall or default-selection command was run. Future Clippy
commands should use `tools/dev.sh` to supply the existing workspace rustup home
and avoid redundant installation. No Java/Prism settings were changed.

## Actual game results

Both fresh disposable runs use fixed Creative TestTerrain Overworld, mob spawning
disabled, imported original resources and software Vulkan rendering. Dedicated
binds localhost and retains an enabled whitelist for only the two test identities.
Game/launcher/config paths are isolated. No playable world was used.

| Check | Integrated `integrated-w9yly2cq` | Dedicated `dedicated-uakyr03b` |
| --- | --- | --- |
| Overall component result | PASS | PASS |
| Ordinary player placement | All five types | Millstone observed on both clients |
| Powered chain | Shaft64, small64, large-32, small64, millstone-64; reversal+64 | Same unchanged chain; server-query millstone-64 |
| Two cobble captured/processed/collected | Player receives2 gravel | Actor receives2; both connections query empty machine afterward |
| Paused cold reopen | Timer208, input3, motor32, player2 retained | Timer212, input3, motor32, actor2/peer0 retained |
| Resumed completion | Exactly3 gravel | Exactly3 gravel through both server-query connections |
| Peer reconnect | Not applicable | Passed before cold restart |
| One ordinary full output slot | Input consumed;65 gravel total | Input consumed;65 gravel total |
| Nine ordinary full output slots | Input consumed;576 gravel retained | Input consumed;576 gravel retained |
| Ordinary filled-machine break | World377 + player201 =578 gravel | World347 + actor231 + peer0 =578 gravel |
| Clean process exits | Two exits0 | Two servers + four clients, all six exits0 |

The two full-output cases match [R01](COMPATIBILITY_DECISIONS.md), including the
owner-selected intentional recipe-result loss. Removal totals account for the
earlier two player-held gravel plus576 stored gravel; they do not excuse arbitrary
transfer loss. Real mining removes the machine; dropped items and player holdings
are counted. These are ordinary observations, not an atomic snapshot/crash proof.

Ordinary placement, collection and mining use actual player control paths. The
oriented chain and full-output fixtures use explicit diagnostic commands. Falling
input stacks are summoned diagnostically; player Q/drop ingress was not tested.
Cold reopen actually closes and launches new processes on the same disposable
world. No in-process reload is substituted. All eight game/server processes exit0.

All three screenshots were visually inspected. Original machine models render in
the world; both dedicated images show the same construction, and one also shows
the other player. Chat overlays obscure part of the dedicated views. Submitted
millstone-inner matrices independently change on both clients: actor875 poses/
688 distinct matrices, peer308/294. Recent normalized Y-axis deltas at the final
256RPM setting give medians about-256.00 and-256.01 RPM respectively. This is actual
render motion, not Java phase-oracle, anchored-phase or GPU-buffer qualification.

Dedicated machine inventories are correlated authoritative server NBT queries
through each connection. They are not independent reads of client machine-inventory
replicas. Own player inventory, received block presence and submitted rendered
matrices are distinct local client evidence.

## Checklist and limits

All five native S1 outcome groups have supporting evidence: exact native build/
activation; placement and server-authoritative processing; selected chain and
recipe controls; ordinary custody/removal plus normal cold restart/reconnect; and
actual client rendering/motion. Fresh read-only Sol High reviewer
`native_s1_completion_review` found no checklist or claim-accuracy gaps and verified
the source/driver/result hashes, logs, assertions, provenance and all three images.
The coordinator therefore accepts native S1 under the adopted plan. Review of the
candidate report used SHA-256
`ba3f8177bbb23f4a73fb50a4087c82692ff1a78e65257255d043a7a07320b54e`;
this final text records that acceptance and the committed patch without new tests.

Open to LAN, custom cross-owner interrupted crash checkpoints, extra hardened
generation/replay guarantees, unqualified client inventory resync/chunk retracking,
large graphs, Java overspeed destruction, survival/contraptions/fluids, world import,
full modpack compatibility and release packaging are outside this acceptance.
Native code is trusted; native saves are not a composed multi-owner transaction.
No legacy G1/S2/S3 pass follows automatically.

The original failed evaluation runs, source HOLD and all legacy accepted branches
remain intact. The old configured-operation worktree still has its five frozen
dirty files, unchanged sixth path and invalid-import HOLD. No old runner was used
against the new parent. Public Horizon source and reference snapshots are unchanged.

## Local evidence and handoff

Raw receipts/scripts and copied small results/images are under workspace
`.local/native-adoption-01/`; complete worlds/logs remain in the named private Linux
run directories. They stay outside Git. Relevant local inputs are source-inputs.json,
runtime-inputs.json, focused-checks.log, focused-oracle-final.log, lint.log,
integrated-console.log, dedicated-console.log and evidence/*. Source/driver reviews
are recorded separately. No game/compiler jobs remain after these checks.

For source build commands and the supported developer environment, see
[native development](../../native/README.md). Existing native kinetic/milling code
and retained reference observations are the basis for subsequent S2/S3 work; no
new host framework is the next prerequisite.
