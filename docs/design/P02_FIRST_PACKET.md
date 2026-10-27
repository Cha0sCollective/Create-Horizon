# P02 first packet: settled single-source kinetics

Active after G0; implementation evidence and remaining limits are recorded in
`../reference/P02_RESULTS.md`. This is not a G2 pass. The first private core
packet uses the measured P01 rules and has no Horizon or world dependency.
Do not implement a general ECS, machine framework or loader here.

The boundary is an already-declared topology: stable node IDs, explicit oriented
transmission edges and at most one source per connected component. The later
adapter owns block geometry, loaded-state checks and object generations. It must
never treat an unloaded neighbor as a removed node merely to fit this kernel.

Required calculations:

- A motor setting represents signed integer magnitudes 1..256, default +16. Its
  value-board conversion clamps magnitude zero to one and maps row 0 to negative,
  row 1 to positive. Internal zero is not a player setting.
- Direct shaft ratio is +1, equal cogs −1, large-to-small −2 and small-to-large
  −1/2 for the observed Y-axis layouts. Ratios are explicit inputs here; this is
  not proof that arbitrary world positions are connected.
- A connected source propagates theoretical signed RPM. Stress sums impact per
  RPM times absolute local RPM. Capacity sums source capacity per RPM times its
  absolute RPM. Strictly greater stress stops effective speed; equality runs.
- A disconnected component has no source and stops. Rebuilding from supplied
  topology must not retain power from a removed edge. Reversal changes signs,
  while stress uses magnitudes.

Use deterministic node/edge iteration and reference-compatible f32 arithmetic.
Validate non-finite/negative stress facts, duplicate/unknown nodes, self-edges and
zero/non-finite ratios. Conflicting ratio cycles and multiple sources are explicit
unsupported results until their actual reference arbitration/removal rules have
dedicated observations. Do not call a guessed conflict policy Create conformance.

Stop after meaningful pure tests and a small probe compare the declared settled
P01 states. The probe reads normalized fixture identities rather than raw registry
IDs. Deliberately changing a cog ratio must fail the comparison. Initial warm-up
and network-rebuild tick timing remain reference observations for a later adapter;
settled algebra must not be reported as a full 1,730-sample timing match.

Out of scope: multiple-source arbitration, overload block destruction/overspeed,
orientation-to-edge discovery, unloaded capacity retention, persistence, milling,
animation, Wasm host calls and live clients. P02 as a whole remains incomplete
until these declared requirements receive their own bounded packets and evidence.
