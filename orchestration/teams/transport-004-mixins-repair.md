# Transport-004 private Mixins repair handoff

Status: isolated three-file source repair reviewed CLEAR by independent read-only Sol High reviewer; focused offline `variables::` integration group passed. The granted heavy test lease was released. No stage, link, host, Java, game or broader Cargo command ran. Funnel candidate `1bcf4f6f1093c42d0e6c3d25c0c0cb87c20240b1` remains held. Accepted private native baseline at delegation was `55339ce3347b2ce667281e8a3e6318ec856144e4` (.11/cache14); primary documents checkpoint was `8470fc515d460a95a0cacabc644160f9e7aba4b5`.

## Source identity and provenance

- Official immutable source: `G:\Create Rusty\create-horizon\native\create\.tools\horizon-mixins-e8cd0aa38cff`, with `.revision` = `e8cd0aa38cffef49cb2e088ef716baa107148c02`. Its 236 files match the base manifest.
- Isolated edited copy: `G:\Create Rusty\.local\teams\transport-mixins-repair\source`. This copy is not a Git repository or an upstream revision. Operations owns its PRIVATE local Git snapshot, materialized-source pin and bootstrap.
- Base manifest: `G:\Create Rusty\.local\teams\transport-mixins-repair\base-manifest.json`, SHA-256 `cc45e74b4f8f1b79340c022945e3b67ffb873034178627fdf27aa04c26c3d8fb`.
- Corrected overlay manifest: `G:\Create Rusty\.local\teams\transport-mixins-repair\overlay-manifest.json`, SHA-256 `2226f099207eebcb98e6aa3454b9382c23533e5b810d35f24f49108ddfa42a90`. It lists all 236 paths and hashes. Canonical source tree SHA-256 is `4bacf26f3d51fea99537ccae109486e000a815162602238f98f2a73960606207` over ordinal sorted UTF-8 `path<TAB>lowercase SHA-256<LF>` records. The official base tree under that same rule is `43271b866d4e972369654c083f23244cac1a3b7ffa0457db1adbe8fb4a4d7e42`.
- Reproducible overlay patch: `G:\Create Rusty\.local\teams\transport-mixins-repair\repair.patch`, SHA-256 `c1cd633bb6cc22812002ed7fa263bfe0422aa6a92d31756099f3e0575a8efe68`. Read-only `git apply --check` against the untouched official source passed. No files were added or removed.

| Path under `source/` | Official SHA-256 | Edited SHA-256 |
| --- | --- | --- |
| `crates/horizon-mixins-core/src/plan.rs` | `4e81bcaa3bb65dbdd5c88ad87af1471e1d11cadc24bcb32ee62bcff2f09df26f` | `d01480d039ddaceef51e425eba93af16d6de7106c2fc541cda160c89b701a3d0` |
| `crates/horizon-mixins-core/src/standard_macros.rs` | `4e9f82472985260afddefabd11e2937f57d6f44d62e429ab7a28384fd1e628aa` | `240a907c8f6b8eb260ddd3f33298b617813818095e69e2b1817d4e8b7bfeacde` |
| `crates/horizon-mixins-cli/tests/cases/variables.rs` | `97a48bf2ed357021e35cfec134be1b4bae717c4e5f9f382e7ca9c8760b7a2ca5` | `ed27a0328a9b3cc544a5b77de03e315da7c9f0e773dedb018cc56e099a8e9811` |

## Behavior and review record

`MODIFY_VARIABLE` alone now permits ordinary local `syn::Item::Use` outside parsed macro inputs. Import attributes still pass through existing validation; decorated imports, other nested item kinds and other injection kinds retain rejection. The import statement is preserved by transformation and adds no expression-site coordinate.

The first actual-delta review held on local aliases/globs that could shadow preserved standard macros. A later independent pass held on grouped `self` imports and imports inside parsed macro inputs. Its final pass held on raw identifier aliases such as `r#vec` and `r#std`. All findings were fixed in the local-only scan or fail-closed guard, with planner regressions for each. Module-level macro policy was not changed. The same reviewer gave CLEAR on the final patch and exact three-file delta, then independently verified the corrected canonical tree and manifest digests. This is source-review clearance, not stage or gameplay evidence.

Focused fixtures cover STORE/LOAD coordinate stability, a sneaking STORE with a later ordinary import and compiled use, decorated imports, other nested item kinds, alias/glob/grouped-self/raw macro shadowing, macro-input imports, and rejection for RETURN and BEFORE_CALL. The selected actual target needs a plain explicit `UseResult` local import. No recursive Shift workaround was used.

## Focused test receipt

Under the project-lead-granted conditional heavy lease, `run-focused-tests.py` used the pinned native Rust/Cargo paths and Cargo home from `native/create/tools/common.py`, the shared target `/home/arbiter/.cache/create-horizon/evaluation-create05-target`, `CARGO_INCREMENTAL=0`, four Cargo jobs and one test thread. Compile argv was `cargo test --locked --offline --jobs 4 --no-run -p horizon-mixins-cli --test integration --message-format=json`. The resulting integration binary ran with `variables:: --nocapture --test-threads=1`. Compilation took 17.534 seconds; runtime took 14.695 seconds. Result: **8 passed, 0 failed, 99 filtered out**. Source pre/post tree SHA-256 matched `4bacf26f3d51fea99537ccae109486e000a815162602238f98f2a73960606207` across 236 files.

Exact argv, exit codes, timings and raw logs are in `G:\Create Rusty\.local\teams\transport-mixins-repair\test-receipts`. `receipt.json` SHA-256 is `9cacb65e0f272185b34ef64de38657350a869812f851aed694d6ebc1cce53a82`. The receipt references the pre-correction overlay manifest hash `0a98e024f411c8af59e10939df73ea7a1d773e016d97eca5276d1eccf427fca8`; that manifest's only later edit corrected the tree digest sorting metadata. The tested source and patch hashes did not change. The heavy lease was explicitly released to Operations after this focused test.

## Remaining project gates

Operations owns PRIVATE snapshot and pin packaging, overlay bootstrap, exact held Create hook staging, selected native candidate compile and integration. The hook import must survive staging unchanged. Gameplay and Funnel custody fixture remain separate later gates. No public release, generic archive route, upstream contact or pin mutation is claimed by this transport handoff.

Workers: `/root/mixins_implementation` wrote only the isolated copy; `/root/mixins_readonly_review` reviewed it independently and read-only. The primary lane owns integration and the shared heavy queue.
