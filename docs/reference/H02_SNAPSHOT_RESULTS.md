# H02c immutable paired-folder snapshots

Completed local engine commit `58e49d4d0717b7d2382a6e6e02b58ee3bd531cfd` on
`codex/h02-immutable-snapshot`, based on reviewed H02b
`2f1041966d0be580bcecb96ee040d8941503f1fa`. The owner explicitly resumed development
after the earlier cutoff. The commit remains unpublished.

The experimental paired-folder reader now captures descriptor, both executables
and accepted content bytes using pinned directory handles. Parsed declarations,
later resource reads and typed code preflight use captured bytes. Source edits or
removal cannot change an existing snapshot. Legacy folder/archive reading remains
available with its previous behavior.

Identity uses the documented, framed SHA-256 scheme `paired-folder-sha256-1`, with
the descriptor, both code files and every sorted resource path/byte sequence.
Tests mutate each input class, rename resources, reverse file creation order and
check an independently computed Python hashlib/struct vector. Descriptor bytes,
including whitespace, count. The resource-cache stamp uses this identity, avoiding
the original folder's metadata once content is captured.

Raw content payload is capped at 512 MiB, each file at 16 MiB, each code half at
32 MiB and the descriptor at 64 KiB. Captured content clones share byte storage
through Arc; parsed metadata and allocations add overhead, and an ordinary reader
may copy one bounded file. This is an explicit prototype tradeoff rather than a
multi-package memory budget. The loader does not discover/admit these snapshots.
Traversal additionally limits depth and directory entries, with bounded nonblocking
regular-file reads and nofollow opens. Aggregate admission quotas remain future work.

Independent Sol High review found a real child-directory replacement race in the
first candidate: inspecting a directory then using cap-std's ordinary entry open
could follow a replacement relative symlink. The corrected production helper opens
through its pinned parent with nofollow. A deterministic test replaces an already
observed directory with a sibling symlink and requires that open to fail.
Follow-up review reports no remaining findings in tree
`bf1eb97e8b83425f03e5a91d10ebffb6cf2473b7`. Review did not run tests. Initial plain
Git status may have refreshed index stat metadata; subsequent review commands
disabled optional locks. No review source changes or manual Git mutations occurred.

After that correction, the full feature suite passes: 24 unit tests, 4 API-reference
tests, 15 paired-package tests, 4 typed-preflight tests and 32 real sandbox tests;
one existing sandbox test remains ignored. Feature-specific all-target Clippy
passes with warnings denied. The real prepared-resource cache regression also
passes after the correction (one selected test, 15.45 seconds).
The cache regression requires actual prepared data, removes the source directory,
builds from captured bytes, reuses an unchanged identity and rebuilds for a changed
identity. It does not start a game or use owner saves. Broad
`pr_ready --quick --base 2f1041` completed with exit 0: 369 Python tests, formatting,
workspace Clippy, 2,327 workspace tests, 2 doc tests, 5 existing content-root tests,
launcher formatting/Clippy and 197 launcher tests pass. The quick invocation began
before the nofollow correction; the affected feature suite, feature Clippy and
captured-cache regression were explicitly rerun afterward. The self-check lists
348 affected parity harnesses for CI rather than running them. Full vanilla parity
and native Windows qualification are not claimed. The required public-tree commit
hook passed and remains enabled; no public push or PR was made.

Raw logs are parent-workspace `.local/h02c/`. Snapshot capture is not an atomic
filesystem epoch: concurrent writes may yield a mixed version, whose exact captured
bytes are hashed and validated. Identity is neither authenticity nor guest validity.
Discovery, side activation, peer admission and persistent machinery remain absent;
full H02/G1/G2/G3 remain open.
