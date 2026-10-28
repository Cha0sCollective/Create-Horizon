# H08a9 strict world-clock merge and save errors

Planned after H08a8 qualifies and releases save.rs. Scope save.rs, game/chat.rs
and the common metadata caller in game.rs. Both Level::write and Game's later
save_world_state merge/write world_clocks.dat. Fixing only the second writer lets
the first replace unreadable data before validation. Use one concrete shared
clock-file writer and decoder, without changing ordinary read_dat for other files.

The writer accepts world path, overworld ticks, optional missing-End ticks and
optional typed rate/partial/paused controls. Level mode updates total_ticks,
preserves current controls and initializes only an absent End entry. Game mode
updates overworld ticks/controls and removes only default-valued controls using
the existing comparisons. Keep normal gzip output and current schema. Public
Rust visibility across library/binary is an internal experimental API, not WIT.

Only genuine NotFound is absence. Other read, decompression and NBT failures must
return contextual errors before writing. Decode valid raw or gzip NBT explicitly;
malformed gzip must not fall back to raw parsing. Reject trailing decompressed
NBT bytes. Use the existing 64 MiB saved-data decode bound and normal NBT limits,
not the Game loader's smaller 1 MiB bound. Do not introduce a general I/O framework.

Preserve unknown root and overworld fields and every other clock entry. Initialize
missing data/overworld compounds, but refuse existing noncompound root/data/
overworld values. Validate present owned fields' NBT types (Long ticks, Float
controls, Byte paused). Missing DataVersion is allowed; require Int when present,
refuse unsupported newer versions and write the current version. No automatic
repair, forced overwrite or migration. Preserve existing numerical/rate policy,
float representation and default-removal comparisons; add no new ranges.
Preserve the existing NBT codec's negative-zero normalization on read and NaN
canonicalization on write; do not promise raw float bit preservation beyond it.

Reuse the concrete decoder for per-world control loading, retaining tolerant
startup and resetting to defaults before recognized fields are applied. Raw NBT
controls should load consistently with Level's existing raw/gzip time support.
Unreadable loading can stay defaulted, but writable merging must fail. Do not add
a general hard-load-refusal or change unrelated command/rule/difficulty behavior.
Return Result from save_world_state and propagate it before the saved message.
Include clock-path context on write failures as well as read/decode errors.

Tests must exercise actual Level-style then Game-style writer use on one file,
retaining root/overworld sentinels, other clocks and existing End data; missing
End initialization; preservation of nondefault controls then exact default-field
removal; gzip/raw codec and a valid file above the former loader's 1 MiB bound.
Use real corrupt gzip/NBT, trailing bytes, wrong container/owned-field/version
types and filesystem obstructions; errors preserve original bytes. Block _new,
repair and retry the same live state. Repair rejected input then retry. Load
nondefault controls followed by missing/default/invalid files without leaking
controls between worlds. Keep normal Level round-trip/rule regressions.

Add one focused private real-game blocked-_new smoke in each execution mode using
fresh isolated session/control helpers, without editing qualified drivers. Set
known nondefault controls with real commands, save successfully, hold the server,
then create only a previously absent empty world_clocks.dat_new directory beneath
the harness-owned canonical world. Require a clock-specific save error and
unchanged existing clock bytes, never successful acknowledgement. Remove only
that harness-created empty directory with rmdir; preserve exact commands/replies,
driver/dependency/build/resource hashes, logs/reports and process outcomes.

Integrated control save errors return synchronously and permit repair/retry in the
same process. In-frame deferred save errors retain pending-save ownership but the
event loop exits naturally with code one; capture that expected outcome before
repair/reopen/save/clean exit. Transport closure or forced termination alone does
not pass. Same-process live-control retention is not claimed after process death.
Clear HORIZON_SESSION_FILE inside the new harness as well as its invocation;
verify fresh home/launcher/XDG paths and never use playable/account directories.
This fault usually reaches the earlier Level shared writer, so native cases still
cover both writer modes. Assert only clock-byte preservation, not whole-world
atomicity. Preserve existing mode-specific error policy.

Metadata remains nonatomic, earlier files may have written before a clock error,
and write_dat rotation/durability behavior remains unchanged. Other metadata,
player/entity/raid/transition ownership and coherent checkpoints remain open.
Require fresh independent read-only Sol High design/source review, targeted
tests, both-mode frozen fullcounter and broad regressions. Coordinator owns Cargo
and commits; no public engine push or gate promotion.

Fresh independent read-only Sol High design review finds no blocking issue and no
unavoidable owner decision. Rejected-input/_new tests must not overstate unchanged
write_dat's later rotation failure guarantees. Source implementation and runtime
qualification are still pending.
