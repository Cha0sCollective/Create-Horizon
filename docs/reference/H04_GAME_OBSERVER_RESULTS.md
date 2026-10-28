# H04b authoritative passive observation

Local engine commit `4a3e3a110a397e9ca478cf207ed12c68c1528013`, tree
`ec9a644ee63ab54b62f1dffe9427b6970f1c8ef1`, adds an explicit local control fixture
on top of reviewed H04a. Normal startup does not enable observation. It is not
machine lifecycle, callback delivery, persistent generations or a transaction log.

## Frozen build and initial qualification

Pinned Rust 1.99.1, fast profile, no default features, `mods,paired-packages`.
Frozen executable SHA256:
`5e808e0dfa6a9f994d378285eda13229b18b57d2b8e53372415c9715298dfd51`.
Build receipt and subsequent coordinator rehash are copied outside Git as
`.local/h04b-build.json` and `.local/h04b-frozen-binary-sha256.txt`.

| Mode | Accepted extended run | Assertions |
| --- | --- | --- |
| Integrated server | `77f61c41-27f2-407a-9820-d357806291f1` | 131 pass |
| In-frame | `19123aef-0715-4a2d-a268-19d22b8fc221` | 125 pass |

Raw evidence is `.local/h02d/game-runs/<UUID>`. Each uses a fresh world and all
isolated game/launcher/XDG homes, owned native cached resources with manifest
`f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`, and Vulkan
llvmpipe. This is functional evidence, not performance evidence.

Archived wrapper SHA256 is
`955a2c5c7611953d13bca33067ec3b193ed8baa57781983a452b5dc9ee42317e`;
base driver SHA256 is
`9cb17515a22cdcb74a4e550370b2ecc51c94810b4aeb32094d31b34c837f00fd`.
Both drivers, paired guest hashes and the selected code-only beacons fixture and
component hashes are archived. Beacons is an existing SDK sandbox artifact;
its identity is verified without inferring build provenance.

Observed in both startup modes:

- Default-disabled observers; explicit authority enable, with the integrated
  window's remote replica remaining disabled. Strict placed-owner/dimension
  validation prevents a failed level switch from silently observing another level.
- Seven exact ordered events for six static tuple commands plus actual block-item
  placement. Canonical previous/current properties, position, captured scope,
  sequence and game time match. Causes remain explicitly unclassified.
- Five same-position replacement/removal events, with no-op, unwatched and rejected
  height-command exclusion. Repeated drain is empty; duplicate enable refuses.
- Capacity-one overflow leaves the second world write committed. Repeated drains
  return sticky faults without a partial successful batch or a sequence reset.
- Actual piston displacement produces the expected two watched transitions:
  source custom tuple to moving piston, then target moving piston to that tuple.
- Actual typed SDK beacon command produces two deferred glowstone writes; repeating
  it gives an empty batch because both resident writes are no-ops.
- Observation deliberately enabled before save/exit is disabled after restart.
  Original package activation, six static states, actual placement, captured reload,
  settings and same-build restart controls remain passing. Initial/restart exits
  are zero with no cleanup errors.
- Changed/missing package controls explicitly refuse with exit one and assert all
  saved-world bytes unchanged. Integrated refusal cleanup receipts include a
  socket-already-gone quit race; these expected refusal phases are not clean zero
  exits. Raw before/after world hash manifests are not retained separately.

Fresh independent read-only Sol High review checked source and actual receipts,
archived identities and raw event batches; no remaining correctness or false-pass
findings. Reviewer WSL reads were unavailable, so its executable identity check
uses recorded receipts plus the coordinator's copied fresh checksum, not an
independent Linux binary rehash. Review performed no imports/builds/launches/writes.
The coordinator viewed the in-frame screenshot: the diagnostic cube is visible;
no new pixel, orientation or animation claim follows.

## Checks and corrections

Feature-on binary tests: 137 pass, three ignored. Feature-on and feature-off
mods-only binary Clippy with warnings denied pass. H04a's eight new library tests,
one World regression and feature-on/off library checks remain valid at unchanged
hashes. Normal public-tree hook passed at the local commit. Raw logs:
`.local/h04b-{clippy,bin-tests,feature-off-clippy,game-build,game-smoke}.log`.

Initial compilation exposed a private helper call and the renamed atomic update
API; the world guard moved to its existing dispatch module. Clippy then caught
the renamed API's incompatibility with the declared Rust minimum. A checked CAS
loop and exhaustion regression resolve it without raising that minimum. Execution
on an older compiler is not claimed. Earlier failure logs remain excluded.

Driver review found optional-artifact setup could leave ambiguous evidence before
run() handled failure. Preflight now validates the exact archive/component bytes
before creating any run. Three targeted missing/invalid ZIP/component tests pass
and prove base run setup was never called (`h04b-driver-preflight.log`).

The actual dimension round-trip probe and its bounded generator lighting fix now
pass in `docs/reference/H04_DIMENSION_LIGHTING_RESULTS.md`. External clients,
chunk load/unload/activity, persistent generations,
machine records/inventories and coherent checkpoints remain unqualified. Full
H04/G1 remain open. No public engine push or gate promotion occurred.
