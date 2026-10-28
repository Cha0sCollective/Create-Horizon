# Focused vanilla 26.3 readback of Rust saved data

Run `ad33ad7f-f997-4f2c-832c-f220e6d7733b` passes the private driver
`tools/run_vanilla_save_readback.py`, SHA256
`2fb9639a0f67ec989821b915b36fb5970129ee035fbcf219d50d598c3bac51f7`.
It uses frozen H08a3 commit `a75184e`, tree `81541f5`, executable SHA256
`325e1f53f0860e3614229f514b4cfd1fd8f8254a8d30a49b567a52fc6a7bc25b`.
The later strict-POI change leaves these encoders unchanged.

Rust runs offscreen without selected mods in a fresh isolated home. Its container
reply verifies exactly 23 stone in slot zero, other slots empty. Its entity data
predicate verifies the authored pig. After successful save and exit, the completed
world is physically copied to a second fresh home and its hashes match.

The owner's official Minecraft 26.3 server loads that copy under explicit Java 25,
loopback only. Actual Java commands verify one chest entry matching slot zero,
stone/count 23; exactly one tagged pig; UUID `[28672,0,0,99]`, NoAI, NoGravity and
a one-entry tag list. The original completed Rust world remains hash-identical.
Rust and Java exit 0; cleanup/log error lists are empty under the existing helper
filter. This is direct Minecraft-reader coverage of this chest/entity fixture.

Preflight verifies the accepted launcher manifest, official client/server SHA1s,
the bundled server-game SHA256, prepared-resource identities, build receipt and
JDK major version. Fresh independent read-only Sol High source review found and
cleared an inherited-session-file risk: the driver now removes HORIZON_SESSION_FILE
from test children. The restrictive launch note received explicit review; older
qualified drivers stay immutable and use a sanitized launch environment. Current
Windows/WSL variable-presence checks were false, without reading account paths.

Independent receipt review rehashes archived driver/helpers/JARs and checks the
raw Java predicates, process configuration and cleanup. Linux executable/JDK/world
bytes were not independently rehashed by that reviewer; the inspected driver
performs those checks. Artifacts remain outside Git in
`.local/h08a3/vanilla-readback/<run>/`; no JARs, assets or worlds are committed.

This is not the full upstream differential parity suite, proof that an out-of-range
fresh callback executed in this game, coherent durability, recovery, performance,
machine payload persistence or G1. Those claims remain separate.
