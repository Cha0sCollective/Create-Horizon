# P02b selected checkpoints in the real Wasm sandbox

The development component executes the private kinetic library through Horizon's
existing API 0.3 server world and real Wasmtime sandbox. Both accepted Java E/F
references compare equal at 16 settled checkpoints / 62 machine observations.
The deliberately wrong large-to-small mapping fails with the expected −32 versus
−16 theoretical-RPM difference. Native results remain equal after extracting the
shared JSON bridge. Host and Wasm-target guest Clippy checks pass with warnings
denied; formatting passes for both workspaces.

Accepted raw run, relative to the parent workspace:
`.local/p02/wasm-runs/c493dee9-eec4-4691-8ede-aa28a4b819fc/`.

| Identity | Value |
| --- | --- |
| Compiler | rustc 1.99.1 (58cf8f9c3 2026-10-13) |
| Engine base | cf62fe270b243780f9aa86c49b456d9dd9078b42 |
| Exact engine patch SHA-256 | 07d1bcb44b54c50b5e709ebd9d6e5c2505306dca398a81cf1d540130cd2ee851 |
| Core Wasm SHA-256 | 498e3931295439dccf2ebade26e0f724a17671bb90783043e53ea8db0980b43f |
| Component package SHA-256 | 26bf5023b47bcca0154b91be544683cecf8246520a89da633b6f344565870865 |

The engine patch is the separate H02a prototype, not a changed kinetic rule or
active paired loader. Its feature is disabled for this probe. The run stores the
actual patch/status and private port patch, command/exit/log receipts, reference
paths and hashes of participating code, inherited manifests, dependency lock,
comparators, build helper and compiler identity. Before its completion marker it
checks source hashes, engine HEAD/diff/status and core/package artifact hashes
again. It refuses untracked engine inputs. Raw package/wasm files remain local.

The separate `tools/wasm-probe` workspace keeps SDK/Wasmtime dependencies out of the
dependency-free player kernel. Its guest uses the same JSON conversion as the
native development example and calls the actual library. The expected results
still come from validated Java captures, not from another Rust implementation.

The server command's exported player type imports the SDK `players` interface, so
that is the probe's only declared capability. The guest calls no player operation
and has no world/storage/item capability. The host uses an isolated `TestGame`
with one virtual fixture actor, never the owner's server/world. Existing sandbox
memory/fuel limits are unchanged. Each checkpoint returns exactly one identity-
checked reply; stops, errors, missing replies or other side effects fail the run.

## Calibration and review

The first attempt failed because a capability-free manifest omitted the SDK's
required players interface; it is not accepted evidence. The corrected second
attempt matched, but independent review found its source closure/engine drift
receipts incomplete. The accepted third run includes the strengthened collector.
The review follow-up found no remaining issues in source candidate
`823eb7d889739e426850efb98d6895a807bffe2d` and remained strictly read-only. Execution
was verified by the coordinator, separately from static independent review.

Run from this repo inside WSL, after the separate dev-tool lock is available:

```sh
python3 tools/run_wasm_probe.py <accepted-e> <accepted-f>
```

This is selected settled-state execution only. It does not establish live block
registration, world-derived connections, propagation tick timing, source arbitration,
items, saves, peers or retained rendering. Full P02, H02, G1/G2/G3 and B2 performance
qualification remain open. The component is a development probe, not a playable mod.
