# H02d real-game activation qualification

The owner resumed after the break and authorized continued development to a
genuine blocker or required input. The local paired activation seam now passes
real-game smoke in both startup modes. Full H02/G1 and external networking remain
open; these results do not establish Create machinery or machine persistence.

Frozen engine: `60050caa3b70b3496f40cf2080b1c3c8354a67a6`, tree
`e13629cc51c0e95d026a9e2db1b71a1ca7c4ce49`. Rust 1.99.1, fast profile,
`--no-default-features --features mods,paired-packages`. Executable SHA256:
`c220b90cb8757628e9e08d1420b690c72b4a48485c08f6b1d3ecb24d2e10451f`.
The original baseline binary was preserved. A build receipt binds copied binaries
to that frozen source even while later development changes the checkout HEAD.

Driver: `tools/run_paired_game_smoke.py`, SHA256
`d69927060302c477e6e8d9dbea39115b453f765d9c2dd073f5780b56a490e07a`.
Each run archives its actual driver, source/build identities, package file hashes,
commands, control replies, phase logs, exit codes and result under the raw local
path `.local/h02d/game-runs/<UUID>`. Raw game files are not in Git.

| Mode | Accepted run | Result |
| --- | --- | --- |
| Default integrated server | `0cafe3ba-1d3c-4566-ba89-f21af219733f` | Extended pass, 34 assertions |
| In-frame fallback | `f20bd80e-5b61-4fec-b745-1675ed735b43` | Extended pass, 34 assertions |

Each test uses a fresh UUID game/launcher/XDG home under the Linux development
cache and its own new `PairedSmoke` world. Only that test world is reused for the
explicit restart/refusal phases. No Prism instance, playable save or live server
was opened. Owned prepared 26.3 resources were copied into a Linux cache for
repeated testing; their manifest SHA256 is
`f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`.
Rendering used Vulkan llvmpipe. This is functional evidence, not a performance run.

## Observed behavior

- One running installed package row appears. A custom marker block registers and
  is observed by actual client-world block queries. The test does not establish
  rendered appearance or count internal registration calls.
- Actual typed chat reaches the client guest, which responds with a positive tick
  count. The server guest answers the player's online command with a new reply.
  In-frame retains the engine's existing local owner label `Player`; the integrated
  client uses the requested `SmokeTester` name.
- The client writes its note under `client/`; the server creates its `seen` key
  under `server/`. The corresponding opposite-side files do not appear.
- Replacing client code with invalid bytes, deleting server code and corrupting
  the source block declaration and language asset cannot replace the running
  components. F3+T after deleting the content stamp forces an actual resource
  rebuild, recreates the stamp and emits the original captured language bytes.
  Fresh client and server replies still arrive afterward.
- After restoring source package bytes, normal shutdown exits 0. Same-build
  restart loads the client's saved note, retains the same exact package/build
  pins and reads the saved custom block. Restart shutdown also exits 0.
- Changing only the valid package version, then disabling the required package,
  each produces the explicit package/build admission refusal and exit 1. All
  saved-world file bytes compare unchanged after each refusal. Expected refusal
  exits/crash reports are controls, not successful startup or unexplained failures.

These settings checks do not establish server settings persistence across restart,
host-owned inventories, coherent machine saves or H06 transaction rollback.
External paired peers/LAN/dedicated networking still refuse pending H09.

## Review and excluded attempts

Fresh independent Sol High review covered the driver and follow-up changes, then
inspected actual receipts from both accepted runs. No remaining actionable finding
was reported. It found and prompted a bounded terminate/kill/reap cleanup fix.
The reviewer used read-only behavior under a workspace-write runtime profile;
it performed no launches, imports, builds or source/index writes.

Five earlier attempts are retained as failed calibrations, not passes:
`051f2940-d0a3-4caf-9482-f741b4515c65`,
`03bd4a3c-1d23-4d14-9d12-b42abf18c2e5`,
`d31fab0c-5d08-4741-9719-c0725a2fd23d`,
`46e5375c-9638-4fda-b625-069aea8f3c09`, and
`7691c2d5-bfd3-462e-bc3e-8682e377a81c`.
The driver originally left the main Options page open, then initially tried to
read a declaration file which the materializer consumes rather than copying into
the overlay. Explicit screen/input checks and the captured-asset assertion fix
those test-procedure errors. No engine fix was needed for these observations.

Probe rule: `horizon-ctl chat` on a remote client goes directly to the server.
Client commands must use the actual chat UI (T, type, Enter); `messages.styled`
includes client-local drawn output. Leave each Options subpage before resuming.
Server-reply checks require a newly observed reply, not an older matching line.

H03 finite-state planning and static materialization can proceed. Broad final
regression/parity checks follow the next integrated candidate; no public engine
push, PR, merge, release or performance promotion is authorized.
