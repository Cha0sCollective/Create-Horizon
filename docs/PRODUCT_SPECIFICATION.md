# Native Create Mod for Horizon

Product specification and staged development plan

Version 0.1  |  27 October 2026  |  Proposed product and engineering requirements

Build an independent native Horizon mod that reproduces an explicitly declared subset of Create mechanics. The first reliable release contains a creative motor, shafts, small and large cogwheels, and a millstone that converts cobblestone to gravel. It must work through normal player interactions, preserve machine and item state, and synchronize correctly in integrated singleplayer and with two network clients.

The implementation has two coordinated workstreams: generic Horizon host capabilities and a separate Rust mod. The critical path is persistent machinery, authoritative item transactions, client synchronization and retained world rendering. Kinetic rules can be developed and compared with a pinned Create reference in parallel.

The first funding and delivery commitment is a bounded feasibility stage. Later stages proceed when their acceptance evidence is complete. A rotating demonstration alone does not meet the processing MVP release gate.

| Stage | Player or engineering outcome | Release boundary |
| --- | --- | --- |
| S0 | Pinned reference, reproducible fixtures and agreed host contract | Feasibility decision |
| S1 | A persistent server-controlled test machine with client animation | Engine foundation |
| S2 | Verified motor, shafts and cogs with kinetic diagnostics | Engineering prototype |
| S3 | Playable milling chain, persistence and two-client validation | First reliable MVP |
| S4 | Obtainable survival power, transport and a small production line | Factory beta |
| S5 | One controlled moving assembly with collision and recovery | Contraption alpha |
| S6 | Additional machines, moving actors, trains and logistics | Separate feature releases |

Sections 1 to 5 define the product and its requirements. Sections 6 to 8 define the technical contracts. Sections 9 to 13 define delivery, verification and operating decisions. Section 14 records the primary sources.

## 1 Product definition

### Purpose and users

Players should be able to build visible mechanical systems in Horizon and understand how power, gearing and processing affect them. The product should preserve familiar Create behavior for its supported features while using Horizon for simulation, rendering, world storage and networking. Every release must state what is implemented and what remains outside its compatibility promise.

| User | Job to accomplish | First supported stage |
| --- | --- | --- |
| Creative builder | Place a connected drivetrain, change its speed and process material | S3 |
| World owner | Save, resume and recover machines without losing their contents | S3 |
| Multiplayer group | Operate a shared machine without conflicting inventories or stale state | S3 for exact matching peers |
| Survival player | Construct a useful factory from materials obtainable in a new world | S4 |
| Mod maintainer | Add behavior through documented APIs and regression fixtures | S1 onward |

### Product principles

Publish a supported-feature matrix for every release. Passing fixtures for five blocks is evidence for those five blocks, not all Create mechanics.

Run gameplay on the authoritative Horizon server. The client presents committed state and sends interaction requests.

Preserve world and inventory integrity before increasing the feature list. Unsupported save schemas and missing required content must stop an editable load.

Keep Create-specific rules in the mod repository. Add reusable engine services to Horizon through documented, versioned capabilities.

Use observed reference behavior and explicit conformance tests to decide correctness. Any intentional divergence is named, explained and versioned.

Ship a coherent installable package. Players should not assemble mismatched client and server components themselves.

The working product name is descriptive and does not establish affiliation with either upstream project. Naming, attribution, implementation provenance and distributable assets are resolved before public release. [S24](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/LICENSE.md) [S25](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/CONTRIBUTING.md) [S26](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/AGENTS.md)

## 2 Scope and compatibility

### First reliable release

S3 is a creative-mode alpha intended for new test worlds on the declared Horizon build. Its functional component set is exactly the creative motor, shaft, small cogwheel, large cogwheel and millstone. The only required processing recipe is one cobblestone to one gravel. The reference recipe declares processing_time 250; observed completion must match the speed-dependent reference machine trace rather than treating that value as an unconditional number of game ticks. [S16](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlockEntity.java) [S18](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/generated/resources/data/create/recipe/milling/cobblestone.json)

| Area | Required for S3 | Later or excluded |
| --- | --- | --- |
| Acquisition | Creative inventory access to all five components | Survival crafting and resource progression at S4 |
| Gameplay | Placement, connections, signed speed, stress fixtures and milling | Belts, fluids, presses and additional recipes in later stages |
| Interaction | Motor setting, dropped-item millstone intake and empty-hand retrieval | Exact Ponder experience and complete Create UI recreation |
| Worlds | Fresh Horizon worlds and tested native save schemas | Importing existing Java Create worlds |
| Networking | Integrated singleplayer; server with two exact-match Horizon clients | Java Create servers, NeoForge handshake and cross-version protocols |
| Compatibility | Declared native components and recipes | Unmodified Java jars, arbitrary Create addons or complete modpacks |
| Platforms | Linux development and initial alpha validation | Windows and macOS release claims only after platform verification |
| Presentation | Original or authorized assets, functional animation and diagnostics | Pixel-identical artwork or sound without an authorized asset plan |

### Release metadata

Every release must publish its port version, exact supported Horizon build or tested API range, reference Create revision/configuration, capability requirements, content digest, network schema, save schema, supported platforms and known divergences. The supported-feature matrix distinguishes implemented, experimental and unavailable features. Unsupported components must not appear as fully functional content.

The package identifier is provisionally create_horizon. The proposed native namespace is create_horizon, with an explicit mapping to create identifiers in reference fixtures. This avoids implying existing save or addon compatibility. A future importer may map identifiers only after a separate migration specification and tests.

### Support boundaries

The initial network contract requires exact package, content, registry and schema agreement. Relaxing this into semantic-version compatibility is later work. A supported engine build must be explicit even if it is a temporary integration branch; such a build must be described as required until the generic capabilities are available upstream.

## 3 Verified engineering baseline

The reference baseline is Create commit 7ff2760e4924d35d4bc2e22c4b19923f0178cd16 on mc1.21.1/dev and Horizon commit cf62fe270b243780f9aa86c49b456d9dd9078b42 on main. Create source declares version 6.0.11 for Minecraft 1.21.1 and NeoForge 21.1.219, with Java 21. Horizon targets Minecraft 26.3 and protocol 777. These are distinct platform contracts. [S01](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/gradle.properties) [S02](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-core/src/version.rs) [S34](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/build.gradle)

| Observed capability | Current baseline | Planning consequence |
| --- | --- | --- |
| Wasm runtime | Sandboxed components and API 0.3 server execution exist | Extend the established runtime rather than inventing a parallel loader |
| Content | API 0.2 blocks copy vanilla templates and shapes | Add explicit custom properties, shapes and behavior bindings |
| Server access | Tick, selected player events, loaded-block reads and deferred writes exist | Add complete lifecycle, dimension identity and transactional mutations |
| Persistence | Installation-level mod storage exists; generic mod block entities are not integrated | World/chunk records are a prerequisite for machinery |
| Rendering | Public render API exposes HUD and selected engine features | Add retained world model instances |
| Networking | Vanilla transport exists; general mod replication and registry negotiation are incomplete | Build strict negotiation and machine snapshots before claiming compatibility |
| Packaging | API 0.3 requires server side; both does not run paired executable halves | Define paired-module packaging and execution semantics |

Baseline evidence: WIT, manifest, runtime, content schema and server integration. [S03](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mod-sdk/wit/horizon.wit) [S04](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/manifest.rs) [S05](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/runtime.rs) [S06](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/content.rs) [S07](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client/src/game/server_mods.rs)

Normal singleplayer runs an integrated server, with the window acting as a client. Shared content resources help registration but do not eliminate machinery-state replication. The first playable milestone must use this default path; the in-frame fallback remains a regression target. [S08](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client/src/game/integrated.rs)

PR 1189 is an open draft with a machine API and reference harness, but explicitly lacks actual in-world machine creation, ticking and saving. PR 2888 is also an open draft and proposes effects and queries rather than generic animated machinery meshes. Reuse or reconcile their designs after checking the current branches; neither draft is an implemented prerequisite at the pinned baseline. [S10](https://github.com/ludovic111/horizon/pull/1189) [S11](https://github.com/ludovic111/horizon/pull/2888)

The pack carry pipeline may transfer resources and retain custom recipe JSON. It does not implement the execution semantics of Create machinery. Horizon already describes Create-style contraptions as a future native mod. [S27](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/ROADMAP.md) [S28](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-port-pack/src/carry.rs)

## 4 Player workflows

### Build and operate the first machine

The player installs the compatible package, opens a new creative world, and finds the five supported components. They place an upward-facing motor, a shaft and a millstone above it, then set a supported nonzero signed speed. The millstone receives shaft power from below and rotates about its Y axis. A separate fixture demonstrates the small and large cog transmission path. [S17](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlock.java)

The player drops cobblestone onto the millstone. The authoritative server accepts the item once, advances the selected recipe while the machine has effective power, and produces gravel. Empty-hand use retrieves output; when there is no output it retrieves remaining input, following the reference overflow behavior. A dedicated processing GUI is unnecessary for this interaction. [S16](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlockEntity.java) [S17](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlock.java)

### Diagnose an idle or stopped machine

An inspection overlay reports signed RPM, effective operating status and the relevant reason for inactivity: no power, overstress, waiting for input, or the reference output-storage condition. The overlay is read-only and uses replicated state. The player can change the motor setting or repair a connection and see the state recover. Stress behavior is tested with a matching finite-capacity reference fixture; the creative motor is not the principal survival-power demonstration.

### Resume a factory

The player saves and closes the world during processing. On reopening, the machine retains its configuration, input, output and canonical progress, and the network restores the supported behavior. If a required mod or schema is unavailable, the game explains the incompatibility before opening the world for editing. The original files remain recoverable.

### Share the same machine

A second player joins while the first player is operating the millstone. The server verifies compatibility, sends the current state for tracked chunks, and then supplies ordered changes. Simultaneous retrieval requests cannot both obtain the same output. Leaving and re-entering tracking distance, reconnecting, or replacing the machine at the same position must not resurrect stale state.

## 5 Functional requirements for the MVP

MVP requirements are release-blocking unless a requirement explicitly identifies a development-only allowance. Acceptance identifiers refer to the test matrix in Section 11.

M01 Install and compatibility checks

The package must declare and validate its engine/API requirements, components, capabilities, content owner and schemas before gameplay. Missing executable halves, invalid resources and registry mismatches must produce an actionable error without partially admitting a world.

Acceptance evidence: A01, A18 and A20; delivered through S1 and S3.

M02 Acquire place and remove content

All five components must be obtainable in creative mode, identifiable and placeable with supported orientation, connection, selection and collision behavior. Breaking and replacement must use host world operations and normal item accounting. Template cubes are acceptable only for an S1 test object.

Acceptance evidence: A02, A03 and A12; S2 and S3.

M03 Control the creative motor

Players must be able to select signed speed magnitudes 1 to 256 RPM, default +16, matching the normal reference control. Negative speed reverses direction. Although the internal property bounds include zero, the reference value-setting UI clamps magnitude to at least one. Test stopped machinery by disconnecting power; do not expose zero as a reference-faithful motor setting. In-world control is required by S3. A different native gesture may be documented if its gameplay effect matches. [S19](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/motor/CreativeMotorBlockEntity.java) [S35](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/motor/KineticScrollValueBehaviour.java)

Acceptance evidence: A03 and A04; debug configuration is allowed at S2 only.

M04 Transmit rotation

The supported drivetrain must reproduce connection rules, signed shaft speed, small-cog reversal, large/small ratios, source replacement, removal, reconnect and applicable conflict/overspeed behavior. Graph updates must be bounded and must not scan every loaded block each tick. [S12](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/RotationPropagator.java)

Acceptance evidence: A04, A05 and A06; S2.

M05 Model stress and effective speed

The implementation must distinguish theoretical from effective speed, calculate stress and capacity for supported components, stop overloaded machines and recover when the load allows. With the reference stress system enabled, stress exceeding capacity causes overstress; equality remains operational. Unloaded network contributions required by the reference must not disappear during reconstruction. [S13](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/KineticNetwork.java) [S14](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/KineticBlockEntity.java)

Acceptance evidence: A06 and A11, including stress equal to capacity and matched source/load fixtures; S2 and S3.

M06 Accept and return items

The millstone must accept eligible dropped item entities through a server-controlled transaction. Empty-hand use returns output, or input if output is empty, following the pinned reference. Stack remainders, player inventory overflow and simultaneous requests must conserve items. [S17](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlock.java)

Acceptance evidence: A07, A08, A09 and A16; S3.

M07 Process the milling recipe

The millstone must implement one input slot, nine output slots, the selected deterministic milling recipe, speed-dependent progress and the reference completion/consumption order. Zero effective speed pauses processing. Full-output and input-change behavior must match the reference trace, rather than a newly invented backpressure rule. [S16](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlockEntity.java) [S18](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/generated/resources/data/create/recipe/milling/cobblestone.json)

Acceptance evidence: A07, A09 and A10; S3.

M08 Preserve machines and world state

Completed saves must preserve machine settings, canonical progress, inventories and required kinetic accounting. Chunk unloading, non-ticking chunks, removal, destruction and world close must be distinct. Loaded-but-inactive or unloaded machines must not keep processing due to a stale guest cache. [S15](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/foundation/blockEntity/SmartBlockEntity.java)

Acceptance evidence: A10, A11, A12 and A21; S1 through S3.

M09 Render working machinery

The client must display correct axes, rotation direction, speed changes, stopping and gear alignment. Static and dynamic parts must not be double-drawn. Geometry is retained on the host and animation uses batched parameters; frame rate must not change simulation. [S20](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/SingleAxisRotatingVisual.java) [S21](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/KineticBlockEntityRenderer.java)

Acceptance evidence: A13 and N03; S1 through S3.

M10 Maintain server authority

The server alone may accept item transfers, advance recipes, modify topology or change motor settings. Client messages express intents and target object generations. Validate authoritative held-item/empty-hand state, reach, loaded target, allowed action and current dimension/session; deduplicate replayed intents. Invalid requests leave gameplay state unchanged.

Acceptance evidence: A16, A17 and A19; S1 and S3.

M11 Synchronize tracked state

Integrated singleplayer and exact-match multiplayer must use the same authoritative machine-state semantics. Initial snapshots, ordered deltas and resynchronization must support join-in-progress, reconnect, tracking-distance changes and same-position replacement.

Acceptance evidence: A14 through A19; S1 and S3.

M12 Explain failures and protect saves

Players must receive distinguishable diagnostics for supported idle/failure states, incompatible packages, unsupported save schemas and runtime faults. A fault must not leave a visually active machine producing uncommitted results. Missing required content must block destructive load.

Acceptance evidence: A18, A20 and A21; S3.

## 6 Quality and operating requirements

The following are proposed release requirements and benchmark targets. They are not measurements of current performance. The performance targets are ratified on named hardware at G0 and may be revised only with a recorded product/engineering decision.

| ID | Requirement | Initial acceptance target |
| --- | --- | --- |
| N01 | Declared behavior conformance | All mandatory release fixtures pass; every intentional divergence is listed and tested |
| N02 | State and item integrity | Zero unexplained loss/duplication across A07 to A21, relative to the recovered checkpoint after interruption; 100 save/reopen and 100 chunk-cycle repetitions |
| N03 | Target workload performance | B2 sustains 20 TPS without a growing work backlog. Provisional p95 added server work <= 5 ms/tick and GPU work <= 2 ms/frame at 1080p on G0 hardware |
| N04 | Synchronization | 50 late-join/reconnect cycles; committed state converges, with loopback status changes visible within a provisional 100 ms budget |
| N05 | Bounded execution | Stay within negotiated memory, fuel, message, instance and transaction quotas; demonstrate isolation on exhaustion |
| N06 | Recovery and migration | Recovery selects a complete checkpoint; supported migrations are tested on copies; unsupported schemas refuse writable load |
| N07 | Vanilla regression | Applicable Horizon checks pass with and without the mod; disabled-mod performance stays within the agreed baseline noise envelope |
| N08 | Usability | A tester completes the build, process, retrieve and reopen workflow without a console, save editing or debug inventory injection |

### Benchmark fixtures

B1 is the player workflow: one motor, a direct shaft drive, a cog-ratio fixture and one millstone. B2 contains 1,024 supported kinetic blocks across 16 networks, including 16 active millstones, with a server and two clients; a separate run applies one topology change per second. B3 contains 10,000 supported nodes and is an exploratory stress test, not an S3 performance promise.

Record CPU, GPU, operating system, renderer backend, Horizon/mod commits, world seed, view/simulation distance, resolution and asset set. Run an identical baseline without active mod work. Measure at least five five-minute warm runs and report p50/p95/p99 tick cost, frame/GPU cost, peak guest/host memory, fuel, rebuild time and bandwidth. Measure graph rebuilds separately from steady rotation. If G0 hardware cannot support B2 even before the mod, choose and document a meaningful benchmark before setting targets.

Horizon currently gives each guest a 64 MiB linear-memory cap and fuel limits, while storage and write quotas have separate bounds. Fuel is not a guaranteed wall-clock deadline. Preserve the sandbox and measure costs rather than silently increasing limits. [S05](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/runtime.rs)

### Durability boundary

S3 guarantees coherent normal save/reload and recovery to a completed consistent checkpoint. It does not promise that every tick acknowledged in memory survives immediate power loss. For interrupted-save recovery, conservation is evaluated against the selected complete checkpoint: later transfers may roll back only if player inventory, dropped entities and machine records roll back consistently. A coordinated checkpoint or scoped journal strategy must be proven at G1. If the existing save path cannot support this within a bounded change, revise the scope and estimate before proceeding.

## 7 Architecture and ownership

The port lives in a separate repository. Generic host changes are proposed to Horizon in small reviewable units. All capability names and data structures below describe the intended contract; they do not imply that the pinned Horizon API already exposes them.

| Component | Owns | Must not own |
| --- | --- | --- |
| create core | Kinetic rules, processing state transitions, typed content definitions and testable domain state | GPU objects, filesystem access, network sockets or Java class emulation |
| Server component | Authoritative adaptation, machine inputs, validated commands, persistence payloads and state publication | Direct GPU access or trusting client inventory claims |
| Client component | Input intents, read-only inspection, interpolation and model-instance requests | Authoritative recipe outputs, stress decisions or world mutations |
| Horizon host | Registries, lifecycle, transactions, native inventories, saves, transport, collision, renderer and quotas | Hard-coded Create recipes, gear rules or machine-specific branches |
| Reference tools | Pinned Java fixtures, normalized traces, comparison reports and regression corpus | Runtime Java dependency for players |

### Package contract

The supported distribution is one package with one content owner and separately sandboxed server and client components. It declares package/version, required host API, executable entries, capabilities by side, content digest, network schema and save schema. Content registers once before world admission. A dedicated server never instantiates the client renderer. The existing side=both field is insufficient for this; a manifest/runtime extension is required. [S04](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/manifest.rs) [S05](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/runtime.rs)

Temporary paired development packages may be used during the feasibility spike, provided exactly one owns the content namespace and their versions are coupled. They do not satisfy the final installation requirement. Do not assign a final new Horizon API version until its compatibility policy is agreed.

### Boundary performance

The host supplies bounded batches of relevant events and machine views. The mod returns validated state changes and effects. Cache graph connectivity and update affected components after structural changes; avoid a host call for each vertex or a world scan for every tick. Keep meshes, lighting and culling in Horizon. Guest graph caches may be discarded and reconstructed after a fault; they are not the sole authority for durable inventory or machine records.

## 8 Required host and data contracts

### Custom content and identity

Register explicit block property names, allowed values and defaults; per-state collision, selection and occlusion shapes; static/dynamic model bindings; and a behavior type. Validate all references before loading a world. State ordering must be deterministic and existing vanilla IDs must remain stable. Visual animation does not alter collision unless an authoritative gameplay transaction does so. This extends the current vanilla-template schema. [S06](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/content.rs)

### Lifecycle and tick order

Events carry world, dimension, authoritative tick, deterministic sequence, position, object generation, old/new state and cause where applicable. Capture dimension and actor context at event creation. Supply world open/close, chunk load/unload, chunk eligibility to tick, machine creation/loading/removal, neighbor/topology changes and relevant block changes caused by every gameplay path. Unloaded, inactive and air are distinct query results.

At the documented mod phase, the host delivers ordered events and validated player intents, then ticks active machines against a stable view. It commits successful changes in deterministic order and publishes replication afterward. The exact phase relative to vanilla item motion, hopper transfers and block updates must be validated with fixtures at S0/S1. No machine may tick after its removal notification or continue through an unloaded chunk because its graph node remains cached. [S07](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client/src/game/server_mods.rs) [S15](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/foundation/blockEntity/SmartBlockEntity.java)

### Machine records and persistence

| Field | Meaning |
| --- | --- |
| World and dimension | Scope the record and prevent data leaking between saves |
| Position and object generation | Distinguish a replacement at the same coordinates |
| Namespaced type and schema version | Select behavior and an explicit migration path |
| Record revision and committed tick | Identify a consistent saved/replicated state |
| Bounded machine payload | Settings, progress and canonical domain state |
| Host inventory handles | Make slots participate in host transactions and saves |
| Unknown compatible fields | Preserve data across supported round trips |

Persist speed/configuration, progress, inventories and the kinetic accounting needed to reproduce unloading behavior. Derived connectivity may be rebuilt, but reference unloaded stress/capacity and source relationships must not be discarded merely because only loaded nodes can be scanned. Never persist process-local pointers or ephemeral registry indices as durable identity. [S13](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/KineticNetwork.java) [S14](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/KineticBlockEntity.java)

Dirty records join normal chunk/world saves. A missing mod, unsupported newer schema or failed migration blocks writable load and identifies the required version. Migrations run on a copy with a retained backup. A save-generation design or bounded journal must prevent a recovered machine from disagreeing with the player/entity inventory involved in the same transfer. Extend the existing persistence path rather than writing machines into installation-level settings storage. [S05](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/runtime.rs) [S30](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client-core/src/block_entity_nbt.rs)

### Transactions and faults

A gameplay callback operates on a stable view and a staged write set. It may read its own staged changes. The host validates revisions, authority, quotas and item identities and commits only after successful return. Input consumption, output production, machine progress, entity intake, player retrieval and their replication notifications commit together within the defined operation.

Trap, fuel exhaustion or validation failure discards the staged gameplay effects. Diagnostic output may still report the failure. Because guest memory may already have changed, a faulted module is disabled or re-instantiated from committed host state; execution must not resume with a mutated cache against rolled-back world state. The existing runtime failure path needs review before it can underwrite this guarantee. [S05](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/runtime.rs)

This is in-memory atomicity. Crash durability is governed separately by the checkpoint policy. Avoid an unbounded transaction spanning an entire factory: use bounded, deterministic operation batches and resumable structural work without exposing half-completed inventory operations.

### Inventory and recipe services

Provide bounded slot views, sided access rules, simulated insert/extract and transactional commit through adapters for both native and mod inventories. For the first millstone, add dropped-item intake and empty-hand block-use transactions, including correct remainder/overflow handling. General custom entities and a custom machine menu are not prerequisites for this narrow interaction.

The mod owns machine recipe execution. The host supplies namespaced resource loading, tag/item facts and optional viewer metadata. The first recipe schema supports deterministic ingredients/results and processing work. Later schemas add probabilistic output, fluids and heat only when their semantics and RNG draw order have tests. For the millstone, retain input in the accessible input slot until the reference completion event, and revalidate it when processing completes. Empty-hand retrieval and input changes must remain possible. Persist canonical progress and inventory; internal recovery structures must not reserve or consume input early or repeat completion after restart. [S16](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlockEntity.java) [S18](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/generated/resources/data/create/recipe/milling/cobblestone.json) [S28](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-port-pack/src/carry.rs)

### Replication protocol

| Message family | Required content and rule |
| --- | --- |
| Compatibility negotiation | Host/mod protocol, package/content/schema digests and registry digest; reject mismatch before world admission |
| Tracking snapshot | Dimension, chunk/session generation, object identities, current revisions and complete state for tracked objects |
| State delta | Base revision, new revision, authoritative tick and changed committed fields |
| Interaction intent | Unique intent ID, target generation/revision and requested action; validate server-side context |
| Result or rejection | Authoritative accepted revision/result or actionable failure; repeated intent cannot reapply |
| Resynchronization | Request/replace snapshot for an unknown baseline, gap or changed subscription |

Position alone is not a network identity. Include world/session, dimension, chunk subscription generation and object generation so delayed messages cannot affect a replacement machine. Initially use the existing reliable ordered transport; application-level IDs still protect replay, reconnect and obsolete subscriptions. Bound message size, rate and object count, and version message schemas independently of save schemas.

For animation, replicate signed speed, phase at an authoritative tick and stopped/overloaded state. The client interpolates presentation without advancing recipes. Snapshots must reconstruct a correct current view without replaying the machine from the beginning. Internal snapshot handling and external multiplayer must share the same protocol semantics. [S08](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client/src/game/integrated.rs)

### Retained world rendering

A mod references validated host-loaded meshes/materials and creates bounded retained instances. The host owns GPU resources, light, culling and drawing. The mod submits compact batched transforms or animation parameters. Remove instances when objects or tracking subscriptions end, invalidate handles on resource reload, and ensure dynamic parts are excluded from the static chunk mesh. [S20](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/SingleAxisRotatingVisual.java) [S21](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/KineticBlockEntityRenderer.java)

### Future moving assemblies

A later assembly service provides stable identity, local block coordinates, bounded membership, atomic capture/removal, authoritative pose, host collision and persistence. Disassembly validates the destination before restoring blocks and inventories. It must define obstruction/conflict behavior, chunk activity and ownership before adding riders, actors or trains. Repeatedly replacing static blocks each frame is not the intended movement mechanism. [S22](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/contraptions/Contraption.java) [S23](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/contraptions/ContraptionCollider.java)

## 9 Staged development plan

### S0 Reference and feasibility

Establish a runnable baseline, a precise behavior target and a bounded host integration route before committing to the complete MVP.

Pin toolchains, engine/port/reference revisions and source configurations; prepare isolated development worlds.

Capture normalized traces for direct drive, gearing, stop/reverse, network removal, stress and milling; verify the motor control gesture and item overflow behavior.

Write an RFC for package identity, lifecycle, tick phase, transactions, persistence and rendering; reconcile PR 1189 and other relevant work.

Choose initial hardware, source-provenance process, namespace and placeholder/authorized asset set; establish baseline timings.

Exit gate G0: Reference fixtures run repeatably; an agreed contract and backlog exist; the first host experiment is bounded. Unresolved upstream policy or engine dependence has an explicit owner and fallback decision.

Dependencies: None. Do not require upstream merge to perform read-only research; do require a credible integration strategy before forecasting distribution.  Accountable role: technical lead with product owner.

### S1 Persistent synchronized machine foundation

Prove one generic host-owned test machine through the default integrated server and a separate client.

Implement paired execution, custom content/schema validation and dimension-scoped lifecycle notifications.

Add generic machine records, staged transactions, save/recovery integration and a narrow inventory adapter.

Implement exact-match handshake, initial state and deltas, object generations and resynchronization.

Draw a retained rotating instance from server-owned state; fault-inject callbacks and save boundaries.

Exit gate G1: The test machine survives completed saves and chunk cycles, keeps one authoritative state, rejects mismatched peers, renders correctly and aborts failed mutations. Recovery reaches a coherent checkpoint.

Dependencies: G0. Rendering and handshake work can proceed alongside lifecycle/persistence once schemas are stable.  Accountable role: engine engineer.

### S2 Kinetic engineering prototype

Implement the declared kinetic rules and connect them to the new host APIs.

Develop the pure Rust kernel with supported connections, signed ratios, source arbitration, graph split/merge and finite-capacity stress fixtures.

Implement motor, shaft and both cog sizes with correct state/shape/connection data and functional models.

Add topology invalidation, chunk-boundary accounting and read-only kinetic diagnostics.

Run the same fixtures natively and through Wasm; profile steady-state and topology-change workloads.

Exit gate G2: A03 to A06 and relevant lifecycle/render tests pass against the reference. Unsupported connection cases are explicitly rejected or outside the feature matrix. The prototype has measured host-call and render costs.

Dependencies: G0 for standalone core work; G1 for the integrated prototype.  Accountable role: gameplay engineer.

### S3 Playable processing MVP

Complete the five-block product and prove it is usable, persistent and shared correctly.

Implement millstone slots, deterministic milling, dropped-item intake, empty-hand retrieval and motor controls usable without debug tools.

Complete inspection feedback, errors, compatible-package installation and missing-mod protection.

Run all MVP conformance, save/chunk, transaction-fault, join/reconnect and two-client interaction scenarios.

Package a pinned Linux alpha with an example world/recipe guide, supported-feature manifest and reproducible evidence report.

Exit gate G3: M01 to M12, N01 to N08 and A01 to A21 pass within their declared scopes. There are no unresolved critical save/duplication defects. A tester completes the product workflow unaided by development controls.

Dependencies: G1 and G2; recipe/interaction work can start on a mock host while foundations stabilize.  Accountable role: product owner and both engineers.

### S4 Useful stationary factories

Deliver a survival-obtainable production line with a deliberately bounded feature set.

Add water-wheel power and the minimum drivetrain controls needed for a reliable factory.

Add mechanical press/depot, belts, basic funnels/chutes and container transfer as separate verified increments.

Implement the complete crafting/material dependency chain for advertised survival features; avoid recipes that depend on unavailable machines.

Expand recipe/tag behavior, diagnostics and platform verification. Treat mixer/basin, fluids and chance-based recipes as separately accepted increments.

Exit gate G4: A player builds the declared water-powered line from obtainable materials, runs a fixed input batch, saves/unloads/rejoins and recovers exactly the expected state and outputs.

Dependencies: G3. Water-flow and press-mode fixtures must be defined before implementation. [S32](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/waterwheel/WaterWheelBlockEntity.java) [S33](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/press/PressingBehaviour.java)  Accountable role: gameplay engineer with product owner.

### S5 Controlled moving contraptions

Add one constrained rotating or translating assembly before general moving-machine behavior.

Introduce a generic assembly record, local coordinates and atomic block/payload capture and restoration.

Bind authoritative pose to rendering, selection and collision; start with a bounded size and one actuator type.

Prove save/reload during motion and obstruction-safe disassembly before adding riders and interactions.

Add supported player carrying and two-client observation, then verify block/item conservation at every transition.

Exit gate G5: The declared assembly can capture, move, save, reload and restore its blocks and inventories exactly once, with accepted collision/rider behavior and no stale client copies.

Dependencies: G3 plus the relevant S4 transfer foundations. The assembly spike receives a new estimate and separate review.  Accountable role: engine engineer and gameplay engineer.

### S6 Expanded Create feature families

Expand only through individually specified releases with maintained conformance evidence.

Add mounted inventories, moving actors and portable transfer after assembly lifecycle is stable.

Specify fluids, heat and additional processing separately if not completed in S4.

Treat trains, track graphs, signaling, schedules, schematics, logistics and tutorial systems as distinct products within the roadmap.

Define addon-facing APIs or world import only when there is a concrete supported compatibility contract and migration test corpus.

Exit gate G6: Each advertised feature family has its own requirements, reference fixtures, performance envelope and save/network compatibility declaration. Full Create parity is a feature-matrix decision, not a date label.

Dependencies: Relevant earlier gates and a fresh scope/estimate for each feature family.  Accountable role: product owner.

## 10 Prioritized engineering backlog

The backlog identifies review units and their evidence. An item may require several small pull requests; it is not a claim that a complete persistence or network subsystem fits in one patch. Owner E is the engine engineer, G the gameplay engineer, and Q the verification/release owner. One person may hold multiple roles, but ownership remains explicit.

### Host foundations

| ID | Deliverable and owner | Depends on | Required proof |
| --- | --- | --- | --- |
| H01 | Host RFC and compatibility policy  E | S0 baseline | Ownership, lifecycle, tick, fault and package rules recorded |
| H02 | Paired-package validation and execution  E | H01 | Only the correct side runs; missing half and namespace collision rejected |
| H03 | Custom properties and shapes  E | H01 | Deterministic states; correct selection/placement; vanilla IDs unchanged |
| H04 | All-causes lifecycle events  E | H03 | Dimensions and generations correct for placement, commands, pistons, unload and removal |
| H05 | Generic persistent machine records  E | H04 | Create/load/save/unload/replacement and missing-schema fixtures |
| H06 | Staged mutation and fault rollback  E | H05 | Trap at each staged operation leaves no partial gameplay effects |
| H07 | Inventory and dropped-item adapters  E | H06 | Intake/retrieval/overflow and simultaneous requests conserve items |
| H08 | Coherent save and recovery path  E | H05 to H07 | Faulted writes and interrupted saves recover a complete checkpoint |
| H09 | Peer and registry negotiation  E | H02 H03 | Mismatch rejected before editable world admission |
| H10 | Snapshots deltas and resync  E | H05 H06 H09 | Join, reconnect, replacement and dimension/tracking transitions |
| H11 | Retained world-model instances  E | H03 H05 | No duplicate draw; reload/cleanup; no renderer on dedicated server |
| H12 | Timestamped animation presentation  E | H10 H11 | Stop/reverse and phase at varied frame rates |
| H13 | Typed recipe resource service  E | H03 H07 | Valid tags/data; malformed resources rejected; no invented execution semantics |
| H14 | Instrumentation and quotas  E Q | H06 H10 H11 | Per-mod timing, fuel, memory, bandwidth and bounded-work reports |

### Mod and verification work

| ID | Deliverable and owner | Depends on | Required proof |
| --- | --- | --- | --- |
| P01 | Pinned reference and trace schema  G Q | S0 | Reproducible fixtures; canonical IDs and event order |
| P02 | Pure kinetic core  G | P01 | Pure rules underlying A03 to A06; live-game closure occurs in P03 |
| P03 | Motor shaft and cog adapter  G | P02 H03 H04 H05 | Same traces through Wasm and live server |
| P04 | Millstone and recipe state machine  G | P01 H07 H13 | Progress, full storage, interruption and reload conformance |
| P05 | Player controls and inspection  G | H10 P03 P04 | M03 M06 M12 without console commands |
| P06 | Models and animation bindings  G | H11 H12 P03 | Correct axes/speed/phase and lifecycle cleanup |
| Q01 | Differential conformance runner  Q | P01 | Useful failure diffs; negative control detects a deliberately changed rule |
| Q02 | Persistence and fault corpus  Q | H05 H06 H08 | Checkpoint, trap, missing-mod and migration tests |
| Q03 | Network end-to-end fixtures  Q | H09 H10 | Two clients, stale intents, join/reconnect and replacement |
| Q04 | Performance and visual fixtures  Q | H14 P06 | B1/B2 results and varied frame-rate behavior |
| Q05 | MVP package and release evidence  Q G | P01 to P06; Q01 to Q04; H gates | Install smoke test, feature matrix, known issues and rollback guide |

### Implementation locations

Host work begins in horizon-mod-sdk/wit/horizon.wit; horizon-mods manifest, runtime and content modules; horizon-client mod_content and game/server_mods; block-entity ticks/serialization and saving; protocol and client/LAN handling; and renderer/model-library modules. Existing sandbox, content, wide-state and LAN tests should be extended. New machinery comparison belongs in a dedicated harness; static resource tests do not prove processing behavior. [S29](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/docs/architecture.md) [S30](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client-core/src/block_entity_nbt.rs) [S31](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/tests/sandbox.rs)

## 11 Acceptance and release evidence

All acceptance scenarios use recorded setup, versions, configuration, seed, event/tick ordering and expected output. Numeric tolerances are explicit and limited to justified presentation/representation differences. Inventory counts and ownership changes are exact. No test may normalize away a gameplay mismatch merely to become green.

| ID | Scenario | Pass condition |
| --- | --- | --- |
| A01 | Install compatible package | All required resources/components resolve; one content owner; clean errors for malformed package |
| A02 | Place and remove five components | Orientation, selection, collision, supported connections and drops match the declared reference cases |
| A03 | Direct motor and shaft drive | Supported motor/shaft orientations, connection, signed transmission and source disconnection match |
| A04 | Speeds and cog ratios | +16 and -16 reverse correctly; disconnecting power stops the load. Small/small is -1; large to small is -2; small to large is -0.5 for valid reference geometry |
| A05 | Topology changes | Break, replace, split, merge and source replacement leave no stale powered network |
| A06 | Stress and invalid networks | Equality of stress/capacity stays operational; excess stops effective speed. Recovery and supported opposing-source, loop and overspeed cases match without unbounded work |
| A07 | Full milling chain | Millstone receives power through its bottom connection. Dropped cobblestone is accepted once; exactly one gravel appears at the reference completion event |
| A08 | Empty-hand retrieval | Output, or input when output is absent, returns once with correct remainder/overflow |
| A09 | Storage and input edge cases | Full output, changed input, multiple stacks and competing intake follow reference item accounting |
| A10 | Power loss and reopen | Processing pauses and resumes correctly; save/reload retains the canonical recipe state |
| A11 | Chunk lifecycle | Unload/reload and loaded-but-inactive boundaries preserve correct accounting and eligibility |
| A12 | Destruction and replacement | Stored items are dropped/transferred once; replacement gets a new generation |
| A13 | Animation and resource lifecycle | Correct axis/phase/stop/reversal at varied frame rates; no stale instances after reload/unload |
| A14 | Late join | Second client receives current state without restarting processing or replaying outputs |
| A15 | Reconnect and tracking | 50 reconnect/tracking cycles converge without stale object state |
| A16 | Concurrent interaction | Two clients cannot retrieve or consume the same stack twice |
| A17 | Invalid or repeated intents | Replay, obsolete targets, wrong hand/item, excessive reach and invalid context leave gameplay state unchanged |
| A18 | Compatibility mismatch | Wrong package, content, registry or schema is rejected before admission |
| A19 | Dimension and session change | Queued actions stay attached to their original authoritative dimension/generation |
| A20 | Trap and quota exhaustion | Staged writes are discarded; committed data remains consistent; module fault is visible |
| A21 | Save recovery and missing content | Recovery conserves the selected checkpoint consistently across player/entity/machine state; missing/newer schema refuses destructive load |

### Reference comparison rules

Run the selected Create build in a development-only Java environment. Compare named states, signed speed, stress/capacity, progress, inventory changes and saved payload observations with the native core and then the Wasm/live-world implementation. Pin Java-side dependencies and config. Record every adapter needed between Minecraft 1.21.1 and Horizon 26.3; raw registry numbers are never compared as if globally stable. [S01](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/gradle.properties) [S02](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-core/src/version.rs) [S12](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/RotationPropagator.java) [S16](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlockEntity.java)

Use negative controls: deliberately change a ratio, completion decrement or item result and confirm the harness fails at the expected event. Preserve failing traces as regression cases. Existing Create tests and Horizon reference machinery are sources of scenarios, not proof that the declared subset has passed.

### Release checklist

Every M requirement has linked passing acceptance evidence and no unresolved critical state-integrity defect.

Applicable Horizon formatting, lint, test and parity checks pass for the actual changed code; default integrated-server and dedicated/LAN paths are exercised.

The package installs from a clean compatible profile, and uninstall/missing-mod handling cannot destroy an existing modded world.

The release includes exact build identifiers, feature/compatibility manifest, asset provenance, known divergences, save policy and recovery instructions.

B1 and B2 measurements are attached with hardware and methodology. B3 results are diagnostic until a later release promises that workload.

An authorized tester completes the player workflow without debug controls. Additional platforms are listed only after their own smoke, visual and recovery checks.

## 12 Staffing schedule and first iteration

### Roles and capacity assumptions

The initial estimate assumes two experienced full-time engineers: one responsible for Horizon engine integration and one for gameplay/reference tooling. Product and release verification need explicit attention from that pair or a part-time third contributor. Access to the required game/reference files and a working Horizon development environment is assumed. Upstream review and merge time is outside the team's control.

| Stage | Initial effort envelope | Parallel work and uncertainty |
| --- | --- | --- |
| S0 | 1 to 2 elapsed weeks | Both engineers establish fixtures and contract; stop/replan if the integration route is not bounded |
| S1 | 3 to 6 elapsed weeks after usable contract | Lifecycle/save work on critical path; rendering/handshake can overlap |
| S2 | 2 to 4 elapsed weeks, partly overlapping S1 | Pure kinetics can begin early; live integration waits for host foundation |
| S3 | 2 to 4 elapsed weeks after usable host/core | Interaction, persistence, multiplayer and release hardening converge |
| S4 to S6 | Estimate at the preceding gate | Feature-family campaigns; assembly and train work require new spikes |

The earlier 8 to 16 week MVP range remains an optimistic planning envelope rather than a delivery commitment. For budget decisions, allow approximately 12 to 20 weeks until G1 measurements reduce uncertainty; an engine save/network redesign or unresolved upstream route can exceed that range. These are engineering estimates, not repository benchmarks. Do not add stage durations mechanically because some work overlaps. Reset the forecast at G0 and G1.

### First two weeks

| Period | Engine work | Gameplay and verification work |
| --- | --- | --- |
| Days 1 to 2 | Build pinned Horizon, map current machine/render/network hooks, inspect overlapping drafts | Run pinned Create, establish exact fixtures and trace schema |
| Days 3 to 5 | Draft package/lifecycle/transaction/save RFC; define test object and render contract | Capture direct drive, cog, stop/reverse and milling traces; verify native control expectations |
| Days 6 to 8 | Prototype the smallest persisted server-state path and client state handoff | Build the pure core test scaffold and deterministic recipe fixture; implement comparison/negative control |
| Days 9 to 10 | Demonstrate or clearly isolate the remaining save/render/network obstacles | Review trace results, original assets and workload baseline; publish G0 decision and revised backlog |

The two-week outcome is a feasibility decision and evidence, not a promised complete machine platform. A test shaft that persists and drives client animation is the preferred technical demonstration if the host changes fit within the spike. Failure to complete it must identify the exact missing capability and change the estimate, rather than being hidden behind a visual mockup.

### Critical path and work allocation

Contract and content identity lead into lifecycle, persistent records, transactions and coherent saves. These enable item operations and replication. Retained rendering proceeds in parallel after model/state semantics stabilize. The kinetic core and reference runner proceed in parallel from G0. S3 converges only when host foundations, gameplay, client presentation and end-to-end evidence are all ready.

Keep the engine integration branch narrow and rebase it at defined checkpoints. Prefer upstream generic capabilities when accepted, while documenting any temporary required engine build. A persistent fork is a product/support decision, not an incidental implementation detail.

## 13 Risks decisions and future release policy

### Priority risks

| Risk | Effect | Mitigation and decision point |
| --- | --- | --- |
| Incomplete host lifecycle or persistence | Ghost networks, lost progress or machine state | Prove G1 before broad content work; all-causes events and coherent checkpoints |
| Cross-version behavior drift | Apparently working machines diverge from Create | Pinned traces and named adapters; compare gameplay, not raw IDs |
| Partial inventory or save commits | Item duplication/loss on failure or recovery | Bounded transactions, fault injection and explicit durability boundary |
| Networking deferred too long | Singleplayer demo cannot support normal architecture or peers | Default integrated server and late-join tests from S1 |
| Render or Wasm overhead | Large networks miss tick/frame budgets | Retained meshes, batched calls and B2 profiling before content expansion |
| Upstream acceptance and provenance | Engine changes cannot be maintained as planned | Agree generic original-code boundaries; do not call source-derived work clean-room |
| Asset distribution restrictions | The package cannot ship its intended visuals | Original/authorized assets first; explicit provenance inventory and permissions decision |
| Scope expands to full Create | MVP deadline and correctness become unmanageable | Five-block scope and feature gates; separate estimates for factories, assemblies and trains |
| Engine/API churn | Repeated rework and incompatible saves | Pinned release profiles, versioned contracts, migration fixtures and planned rebases |

### Decisions to close

| Decision | Proposed default | Owner and deadline |
| --- | --- | --- |
| Implementation provenance | Use a behavior specification and independently implemented generic engine services; keep accurate provenance for the separate port | Technical lead before G0 |
| Upstream or integration build | Seek agreement on reusable host APIs; declare a temporary required build if needed | Product owner and engine lead at G0 |
| Namespace and package format | create_horizon namespace, one bundle and separate sandboxed components | Engine lead at G0 |
| Save recovery mechanism | Reuse a coherent engine checkpoint if demonstrably sufficient; otherwise design a bounded coordinated mechanism | Engine lead before G1 |
| Interaction controls | Native in-world motor setting; reference millstone dropped-item/empty-hand flow | Gameplay owner at G0 |
| Performance hardware and targets | B1/B2 profile with provisional N03/N04 targets, ratified after baseline | Verification owner at G0 |
| Assets and public branding | Original placeholder art and independent working name until authorization is explicit | Product owner before external alpha |
| Initial platform scope | Linux alpha; Windows/macOS only with measured validation | Release owner at G3 |

### Provenance and distribution

Horizon's policy is stricter than general code-license permission: it prohibits contributions translated from other projects and describes behavior/documentation-based porting. This specification is informed by a source feasibility analysis and must not be represented as an independently established clean-room artifact. A separate repository alone does not settle upstream acceptance. Agree the implementation process and keep source attribution explicit before proposing core contributions. [S09](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/docs/modpacks.md) [S25](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/CONTRIBUTING.md)

Create's license reserves files under src/main/resources/assets/ while licensing other files under MIT. Record code and asset provenance separately, retain applicable notices, and resolve the distribution of each external asset set before publishing. Local extraction is not itself a redistribution grant. Create's contribution policy also prevents assuming AI-authored upstream work will be accepted; the roadmap does not depend on Create maintainers taking patches. [S24](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/LICENSE.md) [S26](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/AGENTS.md)

### Go or no go decisions

At G0, continue only with a runnable reference, a bounded host plan and an explicit provenance/engine-distribution route. Otherwise complete the research outputs and re-scope the prototype.

At G1, stop expansion if persistent state, fault rollback, synchronization or rendering cannot work through maintainable bounded host services. Re-estimate before adding more blocks.

At G3, hold release for any unresolved corruption, duplication, missing-mod destruction or authoritative-state defect. Cosmetic differences may be documented only when they do not obscure function.

Before S5, require a separate assembly spike and budget. Stationary machine success does not establish collision, moving storage or train feasibility.

### Ongoing compatibility maintenance

Freeze a supported engine/port pair for each release. An upgrade must pass the declared conformance, save, network and platform suite before broadening support. Publish behavioral changes and migrations together; retain a rollback path based on backups rather than promising arbitrary downgrade compatibility. Addon APIs, imported worlds and Java-server interoperability remain separate proposals until they have owned requirements and validation.

## 14 Primary sources

Repository source links below are pinned to the analysis revisions. Pull-request status was rechecked on 27 October 2026 and must be rechecked before implementation. Requirements, stage durations, budgets and proposed interfaces elsewhere in this document are design proposals; source references establish the observed baseline and reference mechanics.

S01 Create build and version

Development source declares Create 6.0.11, Minecraft 1.21.1 and NeoForge 21.1.219. Java 21 is specified in build.gradle at the same revision.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/gradle.properties)

S02 Horizon version constants

Minecraft 26.3 and protocol 777 at the pinned Horizon revision.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-core/src/version.rs)

S03 Horizon mod API contract

Implemented host imports and client/server exports; the main contract for assessing available capabilities.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mod-sdk/wit/horizon.wit)

S04 Horizon manifest validation

Accepted API versions, capabilities, namespaces and side restrictions.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/manifest.rs)

S05 Horizon runtime

Execution-side selection, resource limits, storage and callback failure behavior.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/runtime.rs)

S06 Horizon content declarations

Template-based block and item registration; existing content schema.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/src/content.rs)

S07 Horizon server mod integration

Current events, tick sequencing, world views and deferred writes.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client/src/game/server_mods.rs)

S08 Horizon integrated server

The default singleplayer server thread and shared content resources.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client/src/game/integrated.rs)

S09 Horizon modpack and porting guidance

Content carry limits, registry synchronization scope, reference testing and port provenance policy.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/docs/modpacks.md)

S10 Machine API draft PR 1189

Open draft when rechecked on 27 October 2026. Head 847210b1e764638c97fce84489b2e9c368668388. Proposes a machine API and oracle; actual world creation, ticking and saving are explicitly absent.

[Open primary source](https://github.com/ludovic111/horizon/pull/1189)

S11 Client effects draft PR 2888

Open draft when rechecked on 27 October 2026. Head b4a411a394bd346918843a94d8424bf1acd0e652. Proposed queries, particles, audio and events do not establish moving-mesh support.

[Open primary source](https://github.com/ludovic111/horizon/pull/2888)

S12 Create rotation propagation

Connections, signed ratios, source arbitration and invalid-network behavior.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/RotationPropagator.java)

S13 Create kinetic network

Stress, capacity and unloaded contributions.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/KineticNetwork.java)

S14 Create kinetic block entity

Theoretical versus effective speed, network state and persistence.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/KineticBlockEntity.java)

S15 Create block entity lifecycle

Initialization, lazy ticking, removal, unload and destruction distinctions.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/foundation/blockEntity/SmartBlockEntity.java)

S16 Create millstone processing

Inventory, processing progress, speed scaling, persistence and item capability behavior.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlockEntity.java)

S17 Create millstone interactions

Dropped-item intake, empty-hand retrieval, Y-axis rotation and bottom shaft connection.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/millstone/MillstoneBlock.java)

S18 Cobblestone milling recipe

One cobblestone becomes one gravel; processing_time is 250 units of recipe work.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/generated/resources/data/create/recipe/milling/cobblestone.json)

S19 Create creative motor

Configurable signed speed from -256 to 256, default 16.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/motor/CreativeMotorBlockEntity.java)

S20 Create rotating model instances

Reusable model instances with orientation and kinetic data.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/SingleAxisRotatingVisual.java)

S21 Create kinetic renderer

Rendering of rotating geometry, including the non-Flywheel visualization path.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/base/KineticBlockEntityRenderer.java)

S22 Create contraption state

Block capture/restoration, mounted state, actors, seats and serialization.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/contraptions/Contraption.java)

S23 Create contraption collision

Transformed collision and movement considerations.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/contraptions/ContraptionCollider.java)

S24 Create license

Files under src/main/resources/assets/ are All Rights Reserved; other files are MIT licensed.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/LICENSE.md)

S25 Horizon contribution policy

Original-code policy and restrictions on source-derived contributions from other projects.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/CONTRIBUTING.md)

S26 Create agent contribution policy

Policy governing LLM engagement and contributions; upstream acceptance is not assumed.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/AGENTS.md)

S27 Horizon roadmap

Create-style contraptions are described as a future native mod.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/ROADMAP.md)

S28 Horizon resource carry code

Resource conversion, vanilla recipe upgrading and retention of custom recipe types.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-port-pack/src/carry.rs)

S29 Horizon architecture map

Relevant rendering, world, persistence, network and test locations.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/docs/architecture.md)

S30 Horizon block entity serialization

Existing vanilla block entity serialization that a generic persistence path must extend.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-client-core/src/block_entity_nbt.rs)

S31 Horizon runtime sandbox tests

Existing real-component tests for permissions, limits and runtime behavior.

[Open primary source](https://github.com/ludovic111/horizon/blob/cf62fe270b243780f9aa86c49b456d9dd9078b42/crates/horizon-mods/tests/sandbox.rs)

S32 Create water wheel

Flow sampling and update behavior for the later survival-power stage.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/waterwheel/WaterWheelBlockEntity.java)

S33 Create pressing behavior

Multiple processing modes and speed-dependent press behavior.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/press/PressingBehaviour.java)

S34 Create Java build configuration

Java 21 toolchain and required/optional dependency distinctions.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/build.gradle)

S35 Create motor control behavior

Normal value-setting interaction clamps magnitude to at least one and selects a direction; internal bounds do not imply a selectable zero speed.

[Open primary source](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/kinetics/motor/KineticScrollValueBehaviour.java)
