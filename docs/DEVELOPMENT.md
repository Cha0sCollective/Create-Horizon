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

The current library implements only settled single-source kinetics over explicit
topology. Run `cargo test` through the helper for its meaningful pure checks.
`docs/reference/P02_RESULTS.md` describes the selected live-reference comparison;
no world adapter or full propagation-timing conformance is claimed.

## Engine and reference prerequisites

Linux build tools, ALSA/udev headers, Vulkan software drivers and OpenJDK 21/25 are
installed in Ubuntu WSL. [The S0 environment record](reference/ENVIRONMENT.md)
lists exact versions, source/resource pins and commands. The Create runner selects
JDK 21 explicitly; Horizon exporters use JDK 25. Windows Java settings are separate.

Horizon 26.3 resources were prepared from the owner's verified Prism client. The
source build and isolated integrated-server smoke passed. Create reference captures
are tracked separately under `docs/reference`; these baseline checks do not close
G0. Keep all JARs, prepared data, disposable worlds and raw runs outside Git. Follow
the engine's own CONTRIBUTING.md and AGENTS.md and isolate every runtime data path.

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

Horizon now resolves to `horizon-rust/horizon`; the original `ludovic111/horizon`
URL redirects. Use the canonical name for pull-request searches: old-name searches
can return an empty result even though direct PR reads redirect successfully.
The engine/reference revision pins are unchanged by this repository-name update.

No new release, upstream pull request, public engine change or wider permission is
implied by this setup. The public fork initially contains upstream material only.
There is no release automation. Development preparation uses official downloads
and the owner's installed game files; all acquired resources remain outside Git.

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
