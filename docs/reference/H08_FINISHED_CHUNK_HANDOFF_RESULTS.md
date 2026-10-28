# H08a3 immutable finished-chunk handoff

Local engine commit is `a75184e1c0e0d34bcb99414f8f0b1550cc57650a`, tree
`81541f5eef162cd3c4a8437fed6026470802f10f`, branch
`codex/h08-finished-chunk-handoff`. Exactly streaming.rs, save.rs and the existing
generation-save test generator change. Finished callbacks borrow one immutable
Arc<Chunk>; out-of-range fresh snapshots are retained intact outside World on
failure or a prior sticky generation-save fault. No fake residency or entity
activation is introduced. Region/POI/entity enqueue shares the same Arc, with
entity NBT built on encode/pending read, preserving empty deletion. No normal
Chunk clone or entity-vector take remains in this handoff.

Fresh independent read-only Sol High review found one P2 false-pass in a new
test: inspecting a decoded Chunk could not prove the raw region omitted legacy
entity data, because decoding filters that field. A stronger direct RegionFile
NBT assertion failed against the unchanged encoder, confirming duplicated raw
payload. The full-chunk encoder now removes that legacy field while retaining
unrelated extras; proto encoding explicitly adds its own entities afterward.
Follow-up review clears the exact delta and finds no outstanding issue. The red
test remains `.local/h08a3-raw-entity-red-test.log`, excluded from passing results.
An earlier test-only JavaString lookup compile failure is separately retained.

Final reviewed source SHA256:

| File | SHA256 |
| --- | --- |
| streaming.rs | `8e524b6358c335718a2baeef86008c4cef6091e650992990402f7f014d890175` |
| save.rs | `1d94d53aad14eb71effec328d2be4f2ea50c207e00d62ffdc0a01c103631f502` |
| generation_saves.rs | `19b8155c61905fddc7cd9d3cd46ced43b4c0f7ee74f08d140b905347b44632a7` |

Formatting and feature-on/off library/binary/test Clippy pass. All nine explicit
controlled resource groups, three normal generation-save tests, 36 disk-save tests
and 137 binary tests pass (three binary tests ignored). The normal public-tree
commit hook passes. Frozen fast executable SHA256
`325e1f53f0860e3614229f514b4cfd1fd8f8254a8d30a49b567a52fc6a7bc25b` passes the
unchanged full counter driver and prepared resources in both fresh modes:

| Mode | Run | Assertions |
| --- | --- | --- |
| Integrated | `ba5fbef7-8858-4ed4-b416-c232ba59236b` | 362 pass |
| In-frame | `50546c62-9c02-4cf6-8779-e37af5634aba` | 347 pass |

Independent receipt review confirms normal dimension/save/restart/close behavior,
exact archived drivers and recorded build identity. Final retirements match
counter values 61/18; the old in-frame dimension scope retires at 16. Initial and
restart processes exit 0 with no cleanup error. Admission refusals exit 1 and
preserve saved bytes. Both integrated refusal cleanups record the already-exited
game/socket quit races; in-frame cleanup errors are null. The reviewer checked the
driver's binary hash verification, rather than independently rehashing Linux bytes.

Broad quick verification passes at this commit:
`tools/pr_ready.py --quick --base 928f43c2db203049c6266baf3682b16a806f028a`
reports 369 Python and 1,310 client tests, formatting, Clippy, doc checks (zero),
public tree and unchanged Cargo.lock. Untouched content/launcher checks and vanilla
parity are not run by this quick invocation. Raw records are `.local/h08a3-*.log`,
with raw game archives under `.local/h02d/game-runs/`. No engine push occurred.

The partial-failure probe performs real region/POI enqueue then injects an error
before entities; it is not a fault hook inside Persistent. Separate tests exercise
actual pending entity ownership, disk-writer and disconnected-queue failures.
Success means accepted enqueue, not disk durability or cross-folder atomicity.
Retained data lasts for this Streaming lifetime; there is no retry/reset,
cancellation or absolute quota. Native entity-unload/player ownership and coherent
checkpoint/recovery remain open. No machine persistence or G1 promotion follows.
