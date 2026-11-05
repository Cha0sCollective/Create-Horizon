# S6 mounted and portable item transfer — scope boundary

This is a read-only feasibility audit after the first S6 press/depot family
passed at private commit `f6e3d74ad7583f2828da6dd213419f195a2facd5`.
It is not an implementation packet. The stage plan already authorizes bounded
mounted/portable work in S6; S5's exclusions describe its completed profile.

## What exists now

The selected S5 normal mechanical piston is one pole, one east-facing inert
stone, one-cell motion. `native/create/mixins/create/s5_piston.rs` explicitly
requires the moved cell to be `minecraft:stone` and refuses a block entity at
that cell or the destination (`capture_extension`, lines 279–296). Its owner
state records a Stone and Pole identity, not a mounted block inventory.
`docs/reference/NATIVE_S5_ACTUATOR_RESULTS.md` explicitly leaves block-entity
payloads, attachments and mounted interactions outside the qualified profile.
The existing native depot has a held item and eight output slots, and its
ordinary save/reopen worked in the first S6 pressing family. That proves a
stationary inventory, not custody while its block is moving.

The pinned Java Create 6.0.11 reference uses separate mounted storage types
and a `MountedStorageManager` to mount/unmount block inventories. A portable
storage interface has both a stationary block entity and a counterpart moving
with a contraption. `PortableStorageInterfaceMovement` finds an opposing
stationary interface, can stall a moving contraption, and keeps the connection
alive while transfer occurs. The stationary block entity manages connection,
timer, redstone disable and sync. A finite portable-transfer demonstration
therefore requires more than putting a new item in the registry: it requires
a moving multi-block assembly with a real item owner and a transfer boundary.

The current public experimental Rust port was checked read-only on its
[`stage/parity-06` README](https://github.com/Delete228/create-horizon/blob/stage/parity-06/README.md)
and [roadmap](https://github.com/Delete228/create-horizon/blob/stage/parity-06/ROADMAP.md)
on November 4, 2026. Its 0.7 developer checkpoint reports contraptions still
pending; the roadmap separately lists moving inventories and portable
interfaces as future work. The pinned imported 0.5 port in this workspace has
no portable interface. This avoids rebuilding an already available implementation,
but is a time-bound upstream observation.

## Incremental profiles

The smallest next packet can move **one depot held item** with the selected
one-pole piston, without a portable interface or active transfer. Pinned Java
`DepotMountedStorage` copies the depot held stack into a one-slot mounted
handler, restores it on unmount and syncs it to the client. It does not mount
the stationary depot's eight-slot processing output buffer. A finite native
profile must therefore refuse a depot with any occupied output slot, preserve
the held stack and relevant item components once through capture, motion,
normal cold reopen and safe restoration, and keep all existing S5 stone cases
working. There is no new Horizon API assumed at this stage. Design the source
representation and ownership checks in a reviewed implementation packet.

The subsequent portable interface packet would use that proven moving item
owner and add the interface pair and a bounded item transfer. It must define
the exact assembly and destination before code.

A candidate portable-interface profile would pair one eastward one-pole piston assembly
carrying one finite item inventory and a moving portable storage interface
with one opposing stationary interface and a destination inventory. It would
need checked ownership when inventory and interface blocks leave/re-enter the
world, a stable moving assembly position, transfer admission and item counts,
stop/reversal/refusal behavior, normal cold reopen with items in motion or
connected, server/client projection and ordinary pickup. This is a new
multi-block, block-entity payload profile in the private port. It may be
possible using existing host services; this audit does not assert that a new
Horizon API is required. The concrete gap is the current S5 carrier's
single-stone admission and lack of a mounted inventory/interface owner.

Define the exact inventory block, interface orientation, direction of
transfer, connection/stall timing and recovery boundary in a reviewed packet
before code. A general contraption manager, portable fluid interface,
arbitrary chests, riders, trains, Java world import and interrupted-crash
atomicity are not implied by this first profile.

## Progression rule

Proceed with the first finite mounted-depot packet under the existing S6
authorization after an independent packet review. Do not infer from S5's
qualified limits that owner approval is required merely to add this bounded
S6 profile. Stop for owner input only if evidence shows a broad Horizon
service, save schema or general contraption architecture is necessary, or if
an unresolved product choice changes the accepted outcomes. No single
mounted-depot milestone completes all S6 families.
