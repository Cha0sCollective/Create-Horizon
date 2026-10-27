# Current work

Active work: close H02b evidence and repeat the isolated unmodified-engine B0
baseline without overlapping builds. H02b is locally committed at 2f10419;
see docs/reference/H02_TYPED_RESULTS.md. H02a is locally committed at 0654559.
P02b sandbox comparisons, lints and review
passed; see docs/reference/P02_WASM_RESULTS.md. Reviewed P02a is committed
at 9bc54c0; the complete P02 and all live-host gates remain open. Fifteen real reference
fixtures passed twice with 1,730 samples comparing equal, including the strengthened
overload assertion. G0's bounded experimental direction is in docs/design/G0_DECISION.md.
The owner authorizes further stages in this run until
7 p.m. Central on October 27, 2026 if their preceding gate is actually met;
public publication remains separate. Stop this run at that cutoff.

- Mod notes: `codex/h02-typed-preflight-notes`, based on reviewed P02b `cf4a830`.
- Engine: local `codex/h02-paired-types` at reviewed H02b `2f10419`; unpublished.
- Reference: detached at the specification's Create revision.
- Effective orchestration: see `orchestration/ADOPTION.md` and AGENTS.md.
- Linux build tools, ALSA/udev headers, Vulkan software drivers and JDKs 21/25 are
  installed in Ubuntu 24.04; exact versions are in `docs/reference/ENVIRONMENT.md`.
- The owner installed and ran 26.3 in Prism. Its client matches Horizon's pin;
  resources are prepared and an isolated integrated-server client smoke passed.
- Two fresh expanded Create captures agree: 15 fixtures, 1,730 samples each, 211
  classpath records and 15 config files. Original six-case captures still compare.
  See `docs/reference/S0_RESULTS.md` for scope, controls and human-assisted motor UI.
- The owner selected reference-compatible full-output result loss; see R01 in
  `docs/reference/COMPATIBILITY_DECISIONS.md`. Do not invent backpressure in P04.
- Retained Linux executables are in `../.local/bin`; build cache is persistent in
  WSL. Raw resources, worlds and evidence remain outside Git.
- The private kernel passes ten test groups and matches 16 settled checkpoints /
  62 machine observations from each accepted E/F reference. See P02_RESULTS.md for
  the precise limits. P02b now matches those states through the actual server-Wasm
  sandbox too. Native world machinery and full trace timing remain absent.

Next implementation proposal: freeze paired package content and identity before
activation, scoped in docs/design/H02_NEXT_PACKET.md. Discovery, side activation
and writable-world admission remain subsequent work. The initial five five-minute
B0 software-rendered windows include one compilation overlap; a fresh repeat is
running. Neither run ratifies B2 or hardware-GPU targets. Do not compile during the
repeat's measurement windows. Public engine publication remains separate.
