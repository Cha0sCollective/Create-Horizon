# Current work

Active packet: H02a paired-package preflight in the local engine. Scope and stop
conditions: docs/design/H02_PREFLIGHT_PACKET.md. Reviewed private P02a is committed
at 9bc54c0; the complete P02 and all live-host gates remain open. Fifteen real reference
fixtures passed twice with 1,730 samples comparing equal, including the strengthened
overload assertion. G0's bounded experimental direction is in docs/design/G0_DECISION.md.
The owner authorizes further stages in this run until
7 p.m. Central on October 27, 2026 if their preceding gate is actually met;
public publication remains separate. Stop this run at that cutoff.

- Mod notes: `codex/h02-paired-preflight-notes`, based on reviewed P02a `9bc54c0`.
- Engine: local `codex/h02-paired-preflight`, based on the Horizon pin in `workspace.toml`.
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
  the precise limits. Native world machinery and full trace timing remain absent.

Next bounded work: implement/review H02a without claiming runtime execution;
actual side instantiation/admission is a subsequent packet. Five five-minute B0
software-rendered unmodified-engine windows are collecting locally; these do not
ratify B2 or hardware-GPU targets. Public engine publication remains separate.
