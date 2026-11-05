# S6 Experience packet 001 — active millstone audio

Status: isolated team candidate; no gameplay acceptance yet. Base is accepted
water `.8` at `02d029f3`, with team operations adoption at `76dfaf7`.
The active pins remain Horizon `9c7f9b22`, Mixins `e8cd0aa3`, Rust `1.99.1`,
Java reference `7ff2760e` / Create 6.0.11, and local source/art 6.0.10.

## Source and reuse

Pinned Java `MillstoneBlockEntity.tickAudio` calls `SoundScapes.MILLING` when
speed is nonzero and input is present. `SoundScapes` uses `crushing_1` every
client tick and `crushing_2` every second tick, with pitch clamped from RPM,
a 16-block listener bound, one-source volume and Ambient category. Its local
sound playback delays arrivals beyond ten blocks. The generated Create sound
declarations map both events to existing vanilla sounds. The native local-JAR
import already carries those declarations; original art stays local.

The current upstream `Delete228/create-horizon` main `mixins/create/millstone.rs`
ends its client tick after particles (blob `ed7bbd25fe0e42d1fb108f53748e19e46925d586`);
it offers no sound implementation to reuse. The selected native client tick,
Catnip clock and Horizon audio engine already exist. The candidate changes only
`native/create/mixins/create/millstone.rs`.

## Bounded behavior

Select the nearest loaded, powered millstone with a nonempty input within
16 blocks of the local player. Emit the two reference events at their selected
one-/two-tick cadence, RPM pitch, one-source volume, Ambient category and
distance delay. Sound is client-local; new requests stop when power or input is
absent, the player leaves range, or play is paused. Already delayed requests may
still arrive. The existing particle eligibility
remains distinct: Java audio may play when a mill has input and speed even if
its timer is idle or output is full.

This packet does not reproduce Java's grouping/mean position for several
millstones, its Create client ambient settings, or camera-versus-player position
at the boundary. It changes no processing, item ownership, save schema, host
pin, sound framework, or acceptance rule.

## Exit checks

1. Focused helper checks for zero speed, 16-block refusal, Java pitch,
   one-source volume, event cadence and arrival delay; formatting and diff check.
2. Under a project-lead host-wide reservation, run the affected native test,
   compile/Clippy and only necessary sound-resource checks.
3. Under the same reservation, use a disposable integrated and dedicated/client
   millstone milestone. Record actual client sound events/category where the
   control surface exposes them, plus power/input removal and restart behavior.
   A command trace or server query alone does not prove audible output.
4. Obtain fresh independent read-only Sol High review of the stable source and
   final claims. Preserve all failed attempts and raw evidence outside Git.

No public release, upstream message, playable-world or Prism change follows.
