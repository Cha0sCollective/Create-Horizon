# Current work

Active packet: finish review/commit of the S0 edge corpus and G0 decision, then
start P02's settled single-source kernel in the private repo. Fifteen real reference
fixtures passed twice with 1,730 samples comparing equal, including the strengthened
overload assertion. G0's bounded experimental direction is in docs/design/G0_DECISION.md.
The owner authorizes further stages in this run until
7 p.m. Central on October 27, 2026 if their preceding gate is actually met;
public publication remains separate. Stop this run at that cutoff.

- Mod: `codex/s0-edge-fixtures`, based on reviewed initial packet `2d0f0ea`.
- Engine: local `codex/s0-feasibility` at the Horizon revision in `workspace.toml`.
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
- Rust 1.99.1 scaffold checks pass; its domain library remains empty. No native
  machinery, native conformance or product performance result is claimed.

Next bounded work: the small settled single-source kernel in
`docs/design/P02_FIRST_PACKET.md`, independent of host APIs. Five five-minute B0
software-rendered unmodified-engine windows are collecting locally; these do not
ratify B2 or hardware-GPU targets. Public engine publication remains separate.
