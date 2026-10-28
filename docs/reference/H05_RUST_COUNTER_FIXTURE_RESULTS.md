# H05d Rust component prerequisite

The private standalone server/quiet-client sources in tools/fixtures/guest-counter
pass fresh independent read-only Sol High source and native-example delta review.
They build with pinned Rust1.99.1/wit-bindgen0.60.0 through the existing standard
componentizer. The server exports canonical server-events, inert canonical players
and literal counter-trial; the client uses the ordinary Rust SDK. Neither init
registers commands/hooks. Canonical SDK WIT is copied byte-identically into ignored
dependencies, SHA256
`ecc6869191b51fc878f99c50845cf4b16fc285e3f34bab8eae03c7cc9de77438`.

Standalone locks contain only registry versions already pinned by the engine:

| Lock | SHA256 |
| --- | --- |
| Server | `5f779e00d79d60fd640a3aeb86e1b181a416e2e99a223276d9724331ce0a732e` |
| Client | `96dd18ffab93743c6bb05e75f3ef8abf1760f4788ac84d41339aca91dd83e07e` |

The standard checker successfully starts each component, exercises its ordinary
sandbox flow and reports imports=[], commands=[], output=[] for both. Captured
component SHA256:

| Half | SHA256 |
| --- | --- |
| Server | `b7ac226e5ee8355c71d6c6079a7e7c5e15b04a529d3e8525063499f88ac79fcb` |
| Client | `07d80e001cf7191e4973abf42d940df270a0c9feb697e10c0a1c209e24791b6e` |

The reviewed native example independently captures a minimal paired folder, checks
both ordinary side preflights and both actual ComponentType import counts before
initialization, then constructs CounterEvaluator::from_pair from the server half.
It activates that same captured Arc in Runs::Both, checks one Running entry and an
actual server run with no output, and evaluates the whole known 9-to-10 proposal.
The actual result passes with captured identity
`d6069e5b8857aeb7b7c1a0b7687aa4ee8060afb70962e5f232da00a27d8779f3`.
This empty-content prerequisite identity will differ from a later Game package
that adds its own block/machine/assets; component hashes remain independent inputs.

Successful native proof executable is frozen outside Git, SHA256
`dcb47de157c8a1dce1d6a13d00721a7609543c3c8fdcbbc448f72c1d8fab37e1`.
Its build uses clean engine83649b8. .local/h05d-fixture-* records source/lock/WIT,
components/checks, scratch path and probe output; final status is zero. Build/probe
homes/config/cache/temp paths are fresh and HORIZON_SESSION_FILE is removed. The
reviewed prototype ran in native validation-nu12uQ, not a playable world.

Preserve two infrastructure failures separately: unfiltered offline Cargo metadata
requested uncached macOS mach2 despite this Linux build; Linux-filtered metadata
then resolves successfully. The first receipt assembler expected imports at the
JSON root rather than report, after both builds/checks had already passed. Its
failure occurred before the native probe. The corrected finalizer reuses those
same successfully checked components, then runs the probe; it does not relabel the
failed invocation as successful. Both initial scripts/statuses remain archived.

These results meet the Game packet's componentizer/preflight/activation/evaluator
prerequisites. They do not prove World commit, in-game guest dispatch, fault
isolation in both game modes, inventory, persistence, performance or G1. Final
independent read-only Sol High receipt review finds no issue, including dependency
versions and reused checked artifacts. It did not independently rehash Linux bytes;
native recorded hashes/build/run receipts agree. No public push or binary commit.
After proof, only trailing empty lines were normalized in fixture source/manifests;
the built input copies remain .local/h05d-fixture-built-source and receipts retain
their original hashes. That evidence is reused for this mechanical cleanup.
