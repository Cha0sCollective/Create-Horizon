# H08a4 strict writer-side old POI resolution

Local engine commit `1adef09f4d48e1826d126d80690acb1f18d7eb8d`, tree
`ef297078f3e391632acbd494633d036780d3c036`, changes only save.rs on
`codex/h08-strict-poi-writer`. Reviewed file SHA256 is
`f14f2b846029fe73c025cc579f0f73a816290133786dfc93a7810077fea56350`.
The normal public-tree hook passes; no engine push occurred.

Writer-side old-POI reads now reprobe cached negatives, accept only genuine
region-open NotFound/absent slots as absence, and propagate reported open/read/NBT
errors. Every selected record resolves before any batch write. The common
pending/error/outstanding-count notification tail remains intact, including newer
sequence replacement. Ordinary tolerant gameplay loading and pending reads retain
their existing behavior; successful claim merging and formats are unchanged.

Fresh independent read-only Sol High source review finds no issue. Three explicit
owned-resource cases pass for full batch retention/unchanged old bytes, successful
claims/missing/deletion and older-failure/newer-sequence progression. All 37 regular
disk-save tests pass, including real open/read/decode and negative-cache cases.
Three finished-handoff, three normal generation-save and 137 binary tests pass
(three binary tests ignored), with feature-on/off Clippy and formatting.

Frozen game SHA256
`f83f449607ba6df59f1fe73379f66f576cbd1b94c765a2f8ad084725148e80c7` passes the
unchanged full counter regression in both fresh modes:

| Mode | Run | Assertions |
| --- | --- | --- |
| Integrated | `dc4d7411-897d-4aab-b914-820eee24a36c` | 362 pass |
| In-frame | `d94885cd-f3be-41d7-8853-9b4ee60fc0ef` | 347 pass |

Independent receipt review confirms identities, raw normal save/dimension/restart/
close controls and final counter retirements 60/19. In-frame dimension retirement
is 16 before a fresh zero record. Initial/restart exits are 0 with null cleanup;
admission refusals exit 1 and preserve saved bytes. Integrated missing-package
quit finds its already removed socket; other cleanup fields are null.

`tools/pr_ready.py --quick --base a75184e1c0e0d34bcb99414f8f0b1550cc57650a`
passes in the resource-equipped native clone: 369 Python and 1,311 client tests,
formatting, Clippy, doc tests (zero), public tree, unchanged Cargo.lock. Untouched
content/launcher and vanilla parity are skipped. Raw logs: `.local/h08a4-*.log`.

This propagates reported errors, not comprehensive corruption detection or repair.
Failures after resolution begins writing are not atomic. Waiter wakeup uses the
unchanged completion tail; new tests poll completion before flushing. Actual-game
POI failure injection, coherent checkpoints, machine persistence and G1 remain open.
