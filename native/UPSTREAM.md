# Native Create source provenance

The owner adopted this native base on November 4, 2026. `create/` imports the 500
tracked files of [Delete228/create-horizon](https://github.com/Delete228/create-horizon)
at `16c5d454b1b9860da01b78474ea5206f0664d384`, Git tree
`7ce954394753d1a6c490157660ec6eb7a6c0cb8e`. The initial import was byte-for-byte
from Git archive; no untracked files, Git metadata, builds or local resource inputs
were copied. Local modifications are subsequent private Git commits.

Original MIT license, copyright headers and `create/THIRD_PARTY_NOTICES.md` remain
intact. Upstream attributes the original Create team and the Rust port contributors.
This is source-informed reuse; no clean-room or full Java compatibility is claimed.

The imported package is version 0.5.0, originally referencing Create 6.0.10
`ac0c444d9828da3453ae8cc65338e8de063286fb`. Its compatible host is official Horizon
`fc02103818c212920fd24aa34a2b2f91ba49e56f` and Mixins
`a1e13326306c6bc074b11e6a4926deb551c4083e`, native API359, Rust1.99.1, Minecraft26.3.
Our selected behavior reference remains Create6.0.11/NeoForge21.1.219; bounded local
differences are tracked explicitly, without claiming the whole package was upgraded.

The package includes more content than our supported five-block target. Keeping
those files intact does not qualify or assign development of additional machinery.
Original art is imported only at runtime from the privately held pinned6.0.10 JAR;
that JAR, imported art, Mojang resources, worlds, caches and binaries are not in Git.
Source-authored provisional resources already tracked upstream remain part of its
source package.

`create/Cargo.toml` is a separate workspace. The repository-root Cargo workspace
and old host tools are preserved historical work. The active scope, reference pins
and acceptance are in `../docs/design/NATIVE_PLAYABILITY_PLAN.md` (relative to the
repository root: `docs/design/NATIVE_PLAYABILITY_PLAN.md`). No public engine import,
upstream contribution, merge or release is part of this adoption.
