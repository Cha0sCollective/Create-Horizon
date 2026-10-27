# Reference compatibility decisions

## R01: full millstone output follows the pinned reference

Accepted by the owner on October 27, 2026 after the concrete reference result was
reported. The owner selected “Match the pinned reference, including this item-loss
behavior.” This supersedes any proposal to pause completion until output fits.

For Create `7ff2760e4924d35d4bc2e22c4b19923f0178cd16` / NeoForge 21.1.219, nine
output slots each holding 64 gravel do not block this cobblestone recipe. At 256
RPM it consumes the one cobblestone; output remains 576 gravel. The output handler
reports a slot limit of 99, while insertion respects gravel's ordinary stack limit
of 64. The reference does not retain the uninserted recipe result.

The port must reproduce this case and identify it as a known reference behavior.
It is an explained recipe-result loss, not an item-conservation pass or a native
divergence. Do not silently introduce a backpressure rule. Player/entity transfers,
retrieval, aborts and save recovery still have their own conservation obligations;
this decision does not authorize arbitrary inventory deletion.

Evidence: `milling_full_output` in the edge corpus and its real GameTest assertions.
`milling_one_full_slot` separately proves a single 64-item stack leaves other slots
available, yielding 65 total gravel after one input. Later P04 must test both cases
against the Rust implementation, along with interruption and changed/removed input.
