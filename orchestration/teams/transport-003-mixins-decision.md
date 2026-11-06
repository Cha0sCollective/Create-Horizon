# Transport-003 chest-click planning boundary

Status: owner decision requested only after independent read-only review. No
source, host, Mixins, pin, fixture or acceptance repair is made by this note.
Funnel stays held at integration candidate `1bcf4f6f1093c42d0e6c3d25c0c0cb87c20240b1`;
the other teams' source can build separately. The packet-local runtime driver
remains unexecuted.

## Observed block and current upstream

`G:\Create Rusty\.local\native-team-integration-002\stage.log:1-8` records
`E_UNSUPPORTED_CONTROL_FLOW` for `funnel-chest-use-priority` at
`Game::use_hand`: "nested items in target functions are unsupported". This is
a **planner** rejection, before generated Rust or gameplay. The private hook
selects `let sneaking` in pinned Horizon `game.rs:6551` with
`MODIFY_VARIABLE` at `STORE`. `use_hand` later contains the ordinary local
`use horizon_client::first_person::UseResult;` at `game.rs:6813`. That import
does not execute; the selected Boolean value itself is simple.

The declared pins are Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`
and Mixins `e8cd0aa38cffef49cb2e088ef716baa107148c02`. Read-only GitHub
inspection on November 5 found current upstream Mixins `main` at
[`5a64af9`](https://github.com/horizon-rust/horizon-mixins/commit/5a64af938ec86b00d53d16a408aabc0038fb3e57),
two commits after the pin: crate-name target resolution
[`68ede14`](https://github.com/horizon-rust/horizon-mixins/commit/68ede14d220db4b1ef7586d7c886cf765bf6a3f4)
and support-module glob resolution. Current
[`plan.rs:1177-1185`](https://github.com/horizon-rust/horizon-mixins/blob/5a64af938ec86b00d53d16a408aabc0038fb3e57/crates/horizon-mixins-core/src/plan.rs#L1177-L1185)
still rejects every nested item for this injection kind. There is no current
upstream fix or pin-compatible configuration for this failed target. The
[`compare`](https://github.com/horizon-rust/horizon-mixins/compare/e8cd0aa38cffef49cb2e088ef716baa107148c02...5a64af938ec86b00d53d16a408aabc0038fb3e57)
contains only those two commits. An upstream-version switch alone would not
unblock the funnel.

## Owner choices

| Choice | Exact scope | Cost and boundary |
| --- | --- | --- |
| A. Use current upstream as-is | No supported fix for this target. Hold the funnel until upstream implements one; do not infer support from the two unrelated commits. | No immediate source work or fixture. A later upstream revision still needs review, a new tool pin and stage/compile proof. No maintainer contact is proposed. |
| B. Narrow Mixins planner support **(recommended for the selected behavior)** | In pinned `horizon-mixins-core/src/plan.rs`, permit only an ordinary `syn::Item::Use` statement while validating a `MODIFY_VARIABLE` target, still visit its attributes and reject cfg/macro imports. Within `MODIFY_VARIABLE`, continue rejecting other nested item kinds; leave all other injection kinds' existing behavior unchanged. The current `Sites` and `Scopes` traversals do not assign an expression to this import; verify that the transformer preserves it unchanged. Add positive/negative planner and compile fixtures, then stage this exact Create hook. | Small tool/parser and tests, with no host API or selected interaction change. It **cannot retain the e8cd source pin** if tool code changes: owner must approve a separately identified private Mixins revision and accurate pin/provenance (or wait for an equivalent upstream revision). No local unpinned patch or upstream contribution is authorized. |
| C. New host pre-block item decision | Add a narrowly specified hook before chest `useWithoutItem` that can skip only block use while letting ordinary `native_use_item_on`/BlockItem run. Retain the private main-hand client prediction pass. | Host API/source/pin change and broader regression review for other block interactions; larger surface than B. Current `native_right_click_block` cancellation stops both block and item (`native_hooks.rs:6668-6673`), while `native_use_item_on` is reached after block use (`game.rs:6765`), so existing host hooks cannot express this. |
| C-alt. Change acceptance to Shift plus toggle | Explicitly replace ordinary unsneaked chest-click placement with Shift placement followed by a selected wrench toggle into extraction. | Avoids host and Mixins changes but changes the owner-selected interaction and fixture. Pinned Java makes Shift placement `extracting=false`, and current private placement refuses that unsupported mode; the toggle and its own checks would have to be implemented. A backing-block shortcut does not satisfy ordinary chest-click placement. |

The earlier recursive `use_hand`/temporary Shift idea is excluded by the lead:
it introduces reentrant input and changes what early hooks observe. The
selected direct chest-click contract stays held until the owner chooses a
boundary. For B, minimum proof is planner acceptance, unchanged local import
in staged output, compilation, the ordinary main-hand chest-click fixture,
unrelated held-item chest use, physical Shift/offhand refusal, and the packet's
exact-item custody run. A completed command sequence alone is not gameplay
acceptance.
