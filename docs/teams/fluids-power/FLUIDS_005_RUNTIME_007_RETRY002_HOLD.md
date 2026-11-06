# FLUIDS-005 007 retry002 — dedicated result HOLD

**Overall status: HOLD.** The clean reviewed driver was
`712e67e1398585f72282ec8d037ab12e8289e675`, source
`f732bd481cfc3d9c4720bc51e4a31e7385e18020`, native tree
`471136a9f27049a58bac2707c75b18466e94e17c` (742 tracked files).
The previous `integrated-sxsxi_r_` setup HOLD remains unchanged (raw result
SHA-256 `2fcadf8602a0b70739733bc0db5b4549543a7548674fce0478b001a08c153bb3`).
No driver or production source was edited during retry002. The exclusive
game slot is released; no process for either retry002 run remains.

## Integrated run: finite script passed

Owned run `/home/arbiter/.cache/create-horizon/team-fluids-005/integrated-0k0jzn83`;
raw mirror `G:\Create Rusty\.local\native-fluids-005-runtime\integrated-0k0jzn83`.
Both `before-cold-result-00.json` and `after-cold-result-01.json` report
`PASS`; both `exit-00.json` and `exit-01.json` report actor code 0. Fresh
server replies confirmed `spawn_mobs=false` and authoritative completed
`tick step` counts. The script observed zero-RPM stability at Tank/Basin
1,000/0, the 16-RPM 8 mB/tick deltas, branch refusal/recovery, signed-speed
reversal without flow reversal, and normal stop at **728/272 mB** (1,000
total). The normal cold reopen returned **728/272**, resumed to **0/1,000**,
then the explicitly refilled/full-input refusal held the separate
**1,000/1,000** (2,000 total) census. This is actual integrated script
evidence, not dedicated peer qualification or final acceptance.

## Dedicated run: body completed, result receipt failed

Owned run `/home/arbiter/.cache/create-horizon/team-fluids-005/dedicated-d1m1sey4`;
raw mirror `G:\Create Rusty\.local\native-fluids-005-runtime\dedicated-d1m1sey4`.
The `before-cold` body reached `close()`: it wrote
`pre-cold-amounts.json` with Tank/Basin **728/272 mB** (1,000 total),
captured `before-cold-basin-empty-peer-00.png`, saved normally, and issued
peer/actor quits. `exit-00.json` reports server, actor and peer all code 0.
The initial peer PNG is raw evidence only; canonical visual inspection
belongs to Operations. No dedicated cold reopen or occupied/emptied peer
images were attempted.

After the body returned, `case.py:311` tried to serialize the full `steps`
list for `before-cold-result-00.json` and raised
`TypeError: Object of type PosixPath is not JSON serializable`. `shot()` passes
a `Path` to `call('screenshot', path)`, and `call()` records its original
`words` tuple in `steps`, even though it converts the subprocess arguments
to strings. The error therefore occurs during result recording **after**
normal closure; the dedicated run has no sealed `PASS` result and remains
HOLD. Do not infer a dedicated pass from its partial NBT or PNG.

Raw dedicated receipts:

- `calls-00.jsonl` SHA-256 `59e7bba5f395d1f9aab292a71a31a66510fe65c97e72a4925e20db6fc45e05db`.
- `pre-cold-amounts.json` SHA-256 `98c0856a5510369358fa76114697b4a3ce71d6fe21fa955c3a8d0e8a430d7fc3`.
- `before-cold-basin-empty-peer-00.png` SHA-256 `8f4a2222ff1532ce84351d4ead638c95a1f603aee26e8b08d17fdd21f5ed28da`.
- `exit-00.json` SHA-256 `a4f750654f58cb65e1d425339124d3bf3fa4ed6e324c7fd7c0b1d3024d54e241`.

## Post-run identity and gate

Physical 001 remained clean at the HEAD/tree above. Source-map SHA-256
`0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a`,
binary receipt SHA-256
`2ec5726527df08817663fa847df0b2dfdbb783deecc31d961c92a4ebae5074f5`,
and prerequisites SHA-256
`43de03a356d6deb4d76465730f1c75098c4083e6120d865a518b349c95715396`
were unchanged. Linux rehashes matched both held binaries, the Create JAR,
all four prepared base manifests and all three composed resource tables to
their frozen receipt values. Both retry002 worlds and all prior HOLDs remain
preserved. Review a narrow result-serialization driver correction before a
new root-granted game retry. No fixture replacement or acceptance relaxation
was performed.
