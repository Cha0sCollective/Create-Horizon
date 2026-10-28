# H04b dimension round-trip qualification

Extend the existing explicit-observer smoke with optional `--dimensions`, without
engine changes. Use fresh worlds and typed owner chat for actual `execute in ...
run tp @s` travel; console @s is not the owner. Wait for both the window dimension
and authoritative owner-level status before probing.

Leave one observed Overworld write pending. Travel to the Nether, verify the new
World starts disabled, explicitly enable a new scope and observe a different
tuple at the same coordinates. Drain and clean the Nether probe, then return.
The integrated Overworld retains its original scope and pending history in its
parked World. In-frame dimension replacement intentionally discards the transient
observer; returning must still preserve the saved block. Clean the probe and
restore the player's location before original reload/restart/refusal checks.

This does not qualify Nether retention beyond idle closure, chunk lifecycle,
persistent machine generations or machine saves. Preserve earlier archived
drivers and accepted run identities. Fresh independent review covers the driver
delta and actual receipts; no new gameplay API or publication is needed.
