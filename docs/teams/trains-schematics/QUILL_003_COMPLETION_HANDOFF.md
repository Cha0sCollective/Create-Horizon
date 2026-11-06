# QUILL-003 selected HUD milestone — completion handoff

Status: **runtime and project-lead visual evidence complete; final independent
read-only completion review pending**. This is a bounded Quill selection UI
milestone on frozen005. It is not a completed schematic Save workflow.

## Frozen inputs and change scope

- Native source revision `9651dba12d163896d45a8cc4ba1b0b3b0da1feff`, native
  tree `90c2156ed5ffb38131950581f05cbdc76beba9ba`, source map
  `source-inputs-r3.json` (`c912836f0ec11311340a4dc0e7f8748abf25ee1a46fba17df4c2c5a33c790647`),
  711 unchanged source files.
- Frozen game `b862982154b5ad4cde22e95618920fbec3f6d61f11ed8984357254576c139e6e`,
  ctl `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`,
  Create 6.0.10 JAR `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
- No production source, content pin, package, schema or game binary changed for
  this repair. The only post-review adapter correction was one mistyped
  character in the JAR hash constant. Its first invocation stopped at that
  preflight assertion before any ctl command or gameplay receipt; the corrected
  adapter then ran the focused case successfully. The original r2 visual HOLD
  images and receipts remain preserved.

## Focused integrated retest

The project lead granted the exclusive `trains-quill005-focused-cancel` heavy
lease and launched saved integrated run `integrated-9vkqdlrp`, session index02.
The adapter reused the original fixed four-stone fixture and compared its
entire region and actor item counts with the saved pre-unload receipt. It
established a new completed 16×1×1 selection, held Shift through three ticks
after the cancel click, then sent two right-click edges at `(56,65,20)`.

Raw command receipt:
`G:\Create Rusty\.local\native-team-runtime-005\integrated-9vkqdlrp\quill\integrated-02-cancel-quick.json`.
It records 38 successful ctl steps, unchanged region and Quill1/stick1 counts,
and before/after integrity gates with 711/711 source files, no mismatches,
matching game/ctl/JAR hashes. Status is
`AUTOMATED_CHECKS_PASS_HUD_INSPECTION_PENDING` by design; the receipt itself
does not claim visual inspection. The actor was saved and normally quit;
`exit-02.json` records actor code 0. A separate post-shutdown read-only check
again matched source map/revision/tree, all 711 files, both binaries and JAR.

The project lead directly inspected the three new PNGs and saw completed
16×1×1, “Selection canceled” plus the first-corner prompt, then completed
1×1×1 at `(56,65,20)`. The project lead also directly inspected all 38
canonical images across integrated full/reopen, dedicated full/reopen and the
focused replacement. Its raw `quill-operations-canonical-visual.json` lists
every image and SHA-256, five observations, and the two excluded old integrated
cancel/quick images. A separate read-only file check confirmed all 38 listed
PNG hashes. The [image index](QUILL_003_IMAGE_INDEX.md) links each raw PNG.

Earlier r2 receipts are integrated `integrated-00-full.json` and
`integrated-01-reopen.json`, and dedicated `dedicated-00-full.json` and
`dedicated-01-reopen.json`, under their run-specific `quill/` directories.
Those four sessions had eight code-0 process exits; index02 adds one code-0
actor exit, making **nine** exits across the qualification composition. The
two original integrated cancel/quick images are **excluded** from the visual
claim because they still showed the completed 16×1×1 selection. Dedicated
cancel/quick and the index02 integrated replacements carry that behavior.

## Review and limits

The pre-run adapter candidate `8ead75f` received strict independent read-only
Sol High source review CLEAR (120.7 seconds). Python AST and diff checks
passed. The one-character JAR constant correction and this composed runtime
evidence need a fresh independent read-only completion review before final
team closure. No more game run is requested; root transferred the heavy slot
after normal shutdown.

This milestone supports Quill selection preview, 16-block limit/refusal,
reversed coordinates, explicit no-Save message, cancel/quick edges, item and
screen reset, same-process leave/rejoin, cold reopen, and separate dedicated
peer selections in the selected HUD path. It does not exercise Save,
export/upload, placement, Schematicannon, Java/client-file interoperability,
general world import or train behavior.
