# Native development

The active source is `create/`, imported from the tested MIT Create port. It is a
separate Cargo workspace; repository-root Cargo commands exercise historical work.
Read [source provenance](UPSTREAM.md), the [adopted plan](../docs/design/NATIVE_PLAYABILITY_PLAN.md)
and [current work](../docs/CURRENT_WORK.md) for support and acceptance status.

The compatible host is official Horizon `fc02103818c212920fd24aa34a2b2f91ba49e56f`,
Mixins `a1e13326306c6bc074b11e6a4926deb551c4083e`, Rust1.99.1 and native API359.
The current Linux workspace reuses the already installed compiler, Cargo cache and
warm target through ignored local links. Original evaluation binaries are preserved
separately. A fresh setup can use upstream `create/tools/bootstrap.py`; do not
replace global Java, Rust or launcher settings.

From this directory's `create/` subdirectory:

```sh
# Use a clean checkout at the exact compatible host revision.
HORIZON_SOURCE="/path/to/pinned/horizon" python3 tools/stage.py build

# Optional owned Linux output avoids the slower Windows-mounted staging path.
# Use a new/empty directory; other nonempty external folders are refused.
HORIZON_SOURCE="/path/to/pinned/horizon" ../../tools/dev.sh python3 tools/stage.py build --work-dir "$HOME/.cache/create-horizon/native-stage"

# Focused changed behavior, using this package's own Rust workspace.
python3 tools/cargo.py test --locked --lib milling::tests
CREATE_SOURCE="/path/to/pinned/Create-reference" python3 tools/milling_oracle.py --focused
../../tools/dev.sh python3 tools/cargo.py clippy --locked --lib --example milling_probe -- -D warnings
```

Select JDK21 on the command's PATH for the reference comparison. The focused oracle
extracts the selected 6.0.11 Java methods and tests 36 cycle/output-limit cases in
declared development contexts; it is not a full Minecraft Java game or chance-roll
qualification. The old full oracle battery is available upstream but is not the
default validation for this patch.

Prepared Minecraft26.3 resources still come from the owner's installation. Original
Create art is imported locally from the pinned6.0.10 JAR; see `create/ASSETS.md`.
Do not commit that JAR, imported art or prepared resources. The package's upstream
README/validation/workflow files describe its original base; local patches and
results are separate. The nested upstream workflow is not a repository-root CI job.

Use only fresh disposable game/launcher/config folders when validating. Native S1
uses `--creative --test-terrain`, mob spawning disabled, integrated singleplayer
and a localhost dedicated server with two matching rendered clients. It does not
qualify normal generated terrain, Open to LAN, old world conversion, arbitrary
modpacks or interrupted multi-owner crash recovery. Source build/activation is the
tested route; launcher registry/release distribution is not qualified here.
