# H08a4 strict old-POI reads on the writer path

After H08a3, close one remaining writer false-success path in save.rs. Current
resolve_poi logs an old-record read failure and proceeds as if no previous POI
existed, potentially losing claims while reporting successful completion.
cached_region(create=false) also masks all open errors as cached absence.

Give only writer-side old-POI resolution a strict read path. Genuine NotFound or
an absent record remains absence; propagate other open/read/decode errors. Reprobe
cached negative lookups because ordinary tolerant reads cannot distinguish an
absent file from a suppressed error. Preserve ordinary gameplay load/pending-read
semantics and successful POI claim merging. Do not change world formats, claim
rules, region corruption tolerance or introduce repair/retry policy.

Make resolution return a result and resolve the whole filtered batch before any
record in that batch is written. On failure, use existing writer error retention:
keep matching pending snapshots, decrement outstanding work and notify waiters.
The writer must not exit early and leave flush waiting forever. Preserve newer
sequence replacements when a preceding batch fails.

Focused tests must establish actual reported open/read/decode errors, a previously
cached negative lookup, unchanged old bytes on failed resolution, retained pending
snapshots and repeated flush/writes_finished failure. Verify legitimate missing
files/records, unchanged successful claim preservation and newer-sequence handling.
Some malformed region headers are tolerated by the existing RegionFile reader;
do not call this comprehensive corruption detection. Require prepared data
explicitly where needed. Keep the implementation and tests confined to save.rs.

This is a prerequisite to a coherent checkpoint experiment, not one. Stop/rescope
before broad load-error changes, corruption repair, POI behavior changes, player/
entity ownership or checkpoint publication. Fresh independent read-only Sol High
review is required. Coordinator owns serialized Cargo, integration and local
commits; no public engine push or gate promotion.
