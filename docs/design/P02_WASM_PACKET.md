# P02b: execute selected checkpoints in Wasm

Use the same private kinetic library and development JSON bridge in a small
Horizon server component. The SDK command's player type requires the `players`
interface capability; the probe calls no player operation. Drive it through the existing real
`horizon-mods` sandbox and `TestGame` helper, with fresh temporary directories.
Do not add a world adapter, storage, fake block entities or new host capabilities.

The existing Python comparator supplies the same 16 settled checkpoints / 62
machine observations from each accepted E/F Java reference. A separate development
workspace under `tools/wasm-probe` keeps Wasmtime/SDK build dependencies out of the
player core. The native and Wasm entry points share only JSON conversion; their
kinetic calculations still call the real library, and expected values come from
the validated Java trace. The wrong large-cog mapping must fail in the sandbox too.

Stop after builds, both comparisons, the negative control, source/artifact receipts,
appropriate lints and independent review. A Wasm compile alone does not pass this
packet. Commands return data through an existing server hook; this is not proof of
live-world server phase, placement, inventory, saves, networking or rendering.

Record the engine revision and exact local diff used by the tool. H02a is a separate
generic engine packet under validation; this probe uses the existing API 0.3 server
world and does not activate its experimental paired-package feature. Neither full
P02 nor G1/G2/G3 closes from selected settled-state execution.
