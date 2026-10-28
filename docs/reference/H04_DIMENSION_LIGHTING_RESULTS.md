# H04b actual dimension round trip and lighting correction

The strict round trip now passes in both startup modes. The failed in-frame run
exposed a bounded diagnostic/superflat generator omission; readiness was not
weakened. Full H04 and G1 remain open.

## Failure and correction

On engine 4a3e3a1, integrated run `24c6ec1e-822f-48ea-b197-b80766e3be1e` passed
153 assertions, while in-frame `32759998-39ab-4249-8ac6-37206d00a7be` failed after
returning from the Nether. After 90 seconds it still had player:null, 38 chunks,
36 protos and 24 dirty lighting entries, with no pending worker jobs. This remains
failed evidence; it is not converted to a pass by later analysis.

Travel requires a lit 5x5 arrival square. At radius 2, its corner lighting needs
an outer decorated proto neighbour outside the resident range. TestTerrain and
FlatTerrain already generate complete Chunk protos, but omitted light_view, whose
trait default is None. Streaming therefore could not schedule the corner's light
job. VanillaTerrain supplies this view, explaining the successful outward Nether
trip. The persistence wrapper already forwards it.

Local fix commit `dfcd7e65a7c7e94692ada9941669f8791a533f9b`, tree
`cf522eb66ae8adc1dd687d2901218aaca19e0df1`, supplies an immutable cloned Chunk view
for these two generators. The proto remains available for normal finishing and
section copy-on-write preserves its data. No arrival conditions, timeouts, finishing
ranges or player reconstruction changed. Parent 22ef8c5 separately adds reviewed
machine metadata; its files do not overlap this fix.

## Final game evidence

Pinned Rust 1.99.1, fast profile, no defaults, mods+paired-packages. Frozen game:
`74f0dc6265c21f91a3d9d8b17b811c4de27eccd617e20d8f344f5ca61637d031`.
Build receipt: `.local/proto-lighting-build.json`. Both runs archive driver
`74166cea0104294e114c9e345d7dc52573075d640224fa221eebd752defd6ef8`
and unchanged base `9cb17515a22cdcb74a4e550370b2ecc51c94810b4aeb32094d31b34c837f00fd`.

| Mode | Accepted final run | Result |
| --- | --- | --- |
| Integrated | `31210bf1-b926-47b6-ab72-f0cd0219db6a` | 153 assertions pass |
| In-frame | `3b12ec1e-7321-4e8a-9d16-374711479674` | 140 assertions pass |

All homes/worlds are fresh and isolated; raw evidence remains under
`.local/h02d/game-runs/<UUID>`. Owned resource manifest stays
`f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`.
Rendering uses llvmpipe and establishes no performance qualification.

The actual owner types execute-in teleport commands and waits for both destination
dimension and authoritative placement/readiness. A pending Overworld transition at
`[2,110,2]` remains scoped to its parked World while a different tuple at the same
Nether coordinates is observed under a different scope. Integrated return retains
the original scope/history and restores the player. In-frame return also restores
the actual player; its replacement World is observer-disabled and retains the
saved block. The probe cleans its cells and restores player location.

Both runs repeat all H04b placement/order/no-op/overflow/piston/SDK-write checks and
original activation/captured-reload/static-state/restart/refusal controls. Initial
and restart exits are zero; refusals exit one; final cleanup errors are all null.
Full saved-world byte equality across each refusal is asserted by the inspected
driver; separate raw hash manifests are not retained.

## Focused checks and review

- Two new snapshot independence/coordinate/biome/finish tests pass with the paired
  feature both on and off. All four tests in the two changed generator modules pass.
- Explicit owned-resource radius-2 streaming regression passes: the full arrival
  square lights while the outer corner stays nonresident. Missing resources fail.
- Applicable library/integration-test Clippy and full formatting check pass.
- Normal public-tree commit hook passes. Raw logs: `.local/proto-lighting-*.log`.

Fresh independent read-only Sol High review covers the fix and actual final
receipts; no remaining findings. It verifies unchanged reviewed hashes, driver/
guest identities, raw cross-dimension events, restored players, scope retention,
test logs and phase exits. Executable identity uses recorded build receipts;
independent reviewer binary rehash is not claimed. Review performs no imports,
builds, game launches or writes.

This qualifies one bounded round trip, not Nether retention beyond idle closure,
complete residency/activity events, persistent placement generations, inventories,
coherent machine recovery, superflat gameplay, external clients or performance.
Next is `docs/design/H04_RESIDENCY_PACKET.md`. No public engine push occurred.
