# Create for Horizon

A work-in-progress Rust port of Create for the [Horizon](https://github.com/horizon-rust/horizon)
Minecraft client and server. This project reuses the MIT-licensed
[Create for Horizon port](https://github.com/Delete228/create-horizon) and native
Horizon services, with selected behavior checked against Java Create **6.0.11**.

**November 6, 2026:** the bounded S1–S5 milestones are complete and S6 feature-family
work is underway. Playable factory workflows have been checked in integrated
singleplayer and dedicated multiplayer. Full Create parity remains in development.

## Current gameplay

The checked profiles include:

- Motor, shafts, cogwheels and selected kinetic/stress behavior; a Survival
  water-wheel and milling factory.
- Milling, pressing, dry mixing, coal-heated brass, water dough, fan washing,
  upward Saw processing, stationary Drill and a stationary Crushing Wheel pair.
- Selected belts, Depot handoff and Funnel transport, plus Engineer's Goggles.
- A water-only Tank and a bounded Tank → Pump → Pipe → Basin line. Normal Tank
  mining and Shift-wrench removal now pass in both game modes.
- A finite mechanical-piston assembly with selected mounted/portable item transfer.
- Straight Track placement and the Schematic and Quill selection HUD.

These are specific tested workflows. General fluid networks, arbitrary moving
assemblies, trains, advanced logistics, complete recipes/progression and full
presentation are still incomplete. See [current progress and limits](docs/PROGRESS.md).

## Current work

We are reusing upstream implementations wherever practical. The next approved
integration is ordinary dry vertical **Chutes**, including chest transfer, stacked
Chutes, item components and blocked save/reopen. Its source and checks are in
development; Chutes are not part of this checked source snapshot yet.

The source snapshot here is candidate008, package **0.5.0-ch4os.15**. Active code
lives in [`native/create/`](native/create/). The root Cargo scaffold and original
planning documents are historical; use the native project for current mod work.

## Build status and compatibility

This is a development source checkout, with no packaged release or supported
one-click installation. The checked build uses Minecraft **26.3**, Horizon
`9c7f9b22`, native hooks API **359**, Rust **1.99.1**, and the reviewed Mixins
`ac5d9e7` source-workspace profile. A generic official Mixins archive does not
provide that local compiler profile. Build portability is unfinished.

Inherited native README/PLAN/launcher documents describe earlier upstream
baselines; this README, the progress report and exact pins govern this checkpoint.

- [Exact source compatibility pins](native/create/compatibility.json)
- [Native build tools](native/create/tools/)
- [Progress, validation and remaining work](docs/PROGRESS.md)

The imported Rust base and original-art input target Create **6.0.10**; selected
6.0.11 behavior corrections are recorded separately. Full version equivalence,
Java addon execution, Java-client compatibility and existing Java-world import
are not established. Other mods will be addressed individually.

## Credits and resources

Create is by [the Creators of Create](https://github.com/Creators-of-Create/Create).
The existing Rust port is maintained by [Delete228](https://github.com/Delete228).
This project preserves upstream MIT notices and source provenance; see the
[native license](native/create/LICENSE) and
[third-party notices](native/create/THIRD_PARTY_NOTICES.md).

Original Create art/audio and Minecraft resources are supplied locally and are
not redistributed in this repository. Reusable changes are being organized into
focused patches with their dependencies, validation and limits.
