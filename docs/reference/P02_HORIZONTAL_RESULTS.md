# P02 horizontal reference observations

Accepted bounded Java experiment at private source candidate
4dbc0923cbfbc8b1f6f70f1c32cb1c79f91db337. Fresh independent read-only Sol High
design, source and final evidence reviews find no blocking issues. This extends
the reference corpus, not the native kinetic kernel or live S2 activation.
The accepted scope and source-derived layouts are in P02_NEXT_KERNEL_AUDIT.md.

Pinned Create 6.0.11 remains clean and detached at
7ff2760e4924d35d4bc2e22c4b19923f0178cd16, Minecraft 1.21.1,
NeoForge 21.1.219, JDK 21.0.12.1+1-1-24.04.4-Ubuntu, Gradle 8.14.3.
The five reviewed Java/runner/loader source hashes were frozen before compilation
and both captures, and verified unchanged afterward. No reference source or
playable world/Prism configuration was changed. This is personal source-informed
reference work, with no contributions to Create planned.

## Four authored cases and observed boundaries

Set p02-horizontal-1, namespace create_horizon_horizontal, contains X and Z
motor/shaft/large-to-small-cog layouts, each with a connected small cog and an
axis-mismatch alternative. Every fixture records 60 completed samples, each
BEFORE that callback's action. Annotation rotation is zero; observed absolute
coordinates independently establish the unrotated basis. Placement survival,
exact identities/properties and complete live item-drop maps are checked.

At tick 10 the requested GENERATED speed becomes +16 RPM; tick 30 requests -16
after sampling. EAST motor settings are respectively +16/-16; NORTH settings
are -16/+16. At settled ticks 20, 50 and 60, motor/shaft/large cog share the
requested speed; connected small cogs run at -2 times that speed. Mismatched
small cogs remain at zero without a source or network. Connected parent links
are motor-root, shaft<-motor, large<-shaft, small<-large, with zero stress and
262144 capacity at magnitude 16. Every intervening sample is retained, but
general transition timing and arbitrary geometry are not qualified.

Fresh isolated capture runs:

| Run | Result | Samples | Machine observations |
| --- | --- | --- | --- |
| b2362bee-0d81-42f6-a67a-ca24b665dd12 | Four required GameTests pass, process exit 0 | 240 | 960 |
| 0a48e0f9-ee92-47d1-b6c2-20e7b4e36f15 | Four required GameTests pass, process exit 0 | 240 | 960 |

Each run verifies 211 dependency records and 15 configuration files. Authored
template SHA256 is a7d6e45afbbfb50e312b5f776f5610018cd96f3d070fcf42e78308678e7923f1.
Strict pair comparison exits 0: all complete rows, source/JDK/adapter inputs,
dependencies and configurations match. Only the generated Properties date
comment is excluded. Axes, facing, coordinates, source links and speeds are not
normalized away. The independent reviewer also compared every complete row and
all 24 settled rows across the two original captures using native read-only tools.

Compilation and both captures succeed on their first attempts. Compile run
8dbc229a-0c48-431f-941f-7897a7ca210f takes 11m25s, mostly resource work;
capture Gradle durations are 4m22s and 2m33s. These are execution receipts, not
performance qualification. Initial missing server.properties messages precede
normal default creation; subsequent required tests pass in each fresh directory.

## Validator controls and compatibility

Fourteen derived software controls use copies of declared loader inputs only,
not new game captures or copies of the original worlds. Baseline copy compares
equal (exit 0). Twelve malformed inputs refuse (exit 2): wrong valid axis/facing,
missing/duplicate sample, unknown parent, Boolean numeric zero, nonfinite/string
RPM, wrong valid rotation basis, unknown fixture, duplicate JSON key and changed
template bytes. A valid changed RPM compares unequal (exit 1), identifying
horizontal_x_connected/021.machines[3].effective_rpm, -32 versus -31.
Original captures remain untouched. All fourteen expected outcomes pass, and the
independent reviewer verifies each mutation and receipt without executing code.

Existing six-case/730-sample, fifteen-case/1730-sample and arbitration/800-sample
reference pairs still compare equal under the updated strict loader. This is
compatibility with retained evidence, not a new execution of those old GameTests.
The private Rust core, kinetic tests and Wasm probe are unchanged.

Raw evidence stays outside Git at workspace-root .local/s0/reference-runs/<run>,
.local/p02h1-compile.log, p02h1-capture-{a,b}.log, p02h1-final-pair.json,
p02h1-evidence-index.json, p02h1-legacy-regressions.json and
p02h1-controls-result.json. Derived controls live under
.local/p02h1-controls/0419ec5e-eac6-4587-8b96-6fa707876dec.
Console SHA256 values for the two captures are respectively
5260828b604f4ec81a93eebc8291626fc4a997ac5d11ad8d5d59ae790b968c85 and
601e1a837925ad374938a47cff1aaf7771cae7ba273f46c3aba599edf8acdf1e.

No native geometry inference, arbitrary-axis proof, multi-source arbitration,
live graph integration, retained rendering, processing conformance or G1/G2
acceptance follows. P02a/b remain the existing stateless declared-topology,
single-source kernels. Live S2 integration still requires G1.
