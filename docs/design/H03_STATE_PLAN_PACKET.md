# H03a: bounded finite-state content plan

The owner resumed the development path through a genuine blocker or required
input. H02d real-game smoke qualification is in progress; its failures must be
resolved independently. A useful parallel assignment can implement the next
content-validation prerequisite without changing running game sources.

Base: local engine `60050caa3b70b3496f40cf2080b1c3c8354a67a6`. Worker checkout
`.local/worktrees/h03-state-plan`, branch `codex/h03-state-plan`. Coordinator owns
integration and later materialization. No worker commits, stages or publishes.

Implement a small immutable finite-state plan for original diagnostic blocks:
explicit property names, legal values, defaults, canonical ordering and bounded
Cartesian expansion. Reject duplicate names/values, invalid defaults and overflow.
Use a versioned opt-in declaration under the existing experimental feature; stable
API compatibility is not claimed. Keep legacy template declarations unchanged.

This packet may validate and enumerate only. If resource materialization is not
implemented alongside it, admission must explicitly reject that declaration;
never accept new fields and silently copy vanilla template states instead. Static
diagnostic behavior and inherited template shapes are the initial boundary.
Custom shape overrides, placement rules and machine behavior bindings need their
own follow-up; no machine lifecycle, record, inventory or persistence is implied.

Focused evidence: equivalent declarations reordered produce equal canonical
plans and defaults; malformed declarations and bounded expansion reject; legacy
content tests remain unchanged. Before accepting live custom states, verify every
JSON/NBT state table and vanilla IDs, then actual game state lookup in both startup
modes. Obtain fresh independent Sol High review of substantive changes. These
checks advance H03; they do not close full H03 or G1.
