# Development environment

Initial validation targets Linux x86_64. This Windows workspace uses the existing
Ubuntu WSL distribution for Rust work. WSL is a development environment, not evidence
that a native Linux game release or other platform has passed acceptance.

The required references are pinned in `workspace.toml`. The active local engine is
`../horizon`; the Create reference is `../references/create`. Earlier audit checkouts
under `../sources` are preserved independently. These are code/source locations,
not runtime dependencies for orchestration instructions.

## Rust scaffold

Rust 1.99.1 is pinned from the official stable channel resolved on October 27, 2026.
The project-local installation lives outside this repository in
`../.local/toolchains`, with no changes to shell startup files. A normal installation
of the same toolchain also works. Run from this repository in Bash:

```sh
./tools/dev.sh cargo fmt --all -- --check
./tools/dev.sh cargo check --workspace --locked
./tools/dev.sh cargo clippy --workspace --all-targets --locked -- -D warnings
```

The current library has no gameplay implementation or conformance tests. Compile
and lint success is limited to the repository scaffold.

## Engine and reference prerequisites

At initial inspection, Ubuntu WSL has no C/C++ compiler, pkg-config or Java on PATH.
An administrator must provision the Linux build dependencies before full Horizon
builds: its documented prerequisites include build tools, ALSA and udev headers.
Prepare Java 25 for Horizon tools and Java 21 for the pinned Create reference.
The Windows Java installation is not a verified Linux reference environment.

Licensed Minecraft 26.3 files and the Minecraft 1.21.1 reference environment remain
to be prepared in S0. Keep all JARs, prepared data, disposable worlds and run outputs
outside Git. Follow the engine's own CONTRIBUTING.md and AGENTS.md for the exact
preparation and applicable checks. Use isolated data paths for every test.

The engine checkout contains a local ignored instruction overlay copied from
`orchestration/horizon/AGENTS.override.md`. Its upstream AGENTS.md remains intact.
The overlay and reviewer profile are excluded locally so private governance is not
published to the public fork. A fresh fork clone needs deliberate manual adoption
from the private source before internal work; the upstream instructions remain
self-contained in the public repository.

## Git boundaries

The private repository's initial setup is on main. Subsequent work uses focused
`codex/` branches. The engine integration branch starts at the pinned reference;
its origin is the public Cha0sCollective fork and upstream is fetch-only by local
push configuration. Reference-source pushes are also disabled locally.

No new release, upstream pull request, public engine change or wider permission is
implied by this setup. The public fork initially contains upstream material only.
The private repository has no release automation or game-resource downloads.

## Reference provenance

The source DOCX is preserved byte-for-byte under `docs/source`. The Markdown version
is a readable transcription of its paragraphs, tables and hyperlinks. Its proposed
decisions and schedules are retained as proposals. The specification selects Create
6.0.11 at `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`, superseding the earlier audit's
6.0.10 baseline for future development.

The owner explicitly authorized reference engagement for a personal port and plans
no contributions to Create. Preserve that intent without claiming a clean-room
implementation. Horizon upstream contributions and any redistribution of restricted
assets need their own explicit scope and provenance review.
