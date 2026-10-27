# Current work

S0 is active; its initial P01/H01 environment, first reference fixtures and host
proposal are complete. The owner authorized the dependency/JDK installation and S0
work. G0 remains open and later stages are not active.

- Mod: `codex/s0-reference-contract`, based on initial setup `5d7593b`.
- Engine: local `codex/s0-feasibility` at the Horizon revision in `workspace.toml`.
- Reference: detached at the specification's Create revision.
- Effective orchestration: see `orchestration/ADOPTION.md` and AGENTS.md.
- Linux build tools, ALSA/udev headers, Vulkan software drivers and JDKs 21/25 are
  installed in Ubuntu 24.04; exact versions are in `docs/reference/ENVIRONMENT.md`.
- The owner installed and ran 26.3 in Prism. Its client matches Horizon's pin;
  resources are prepared and an isolated integrated-server client smoke passed.
- Two fresh Create captures agree: six fixtures, 730 samples each, 211 classpath
  records and 15 config files. See `docs/reference/S0_RESULTS.md` for scope/controls.
- Retained Linux executables are in `../.local/bin`; build cache is persistent in
  WSL. Raw resources, worlds and evidence remain outside Git.
- Rust 1.99.1 scaffold checks pass; its domain library remains empty. No native
  machinery, native conformance or product performance result is claimed.

Next bounded work: extend P01 with finite-capacity stress, interruption, storage
and control cases; review the H01 proposal at `docs/design/HOST_CONTRACT.md` before
accepting its gate decisions. Public engine publication and S1 remain separate.
