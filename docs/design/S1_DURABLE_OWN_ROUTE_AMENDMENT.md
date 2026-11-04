# Durable OWN route nesting correction

PROPOSED same-packet amendment after source2 HOLD8277fc5b. Accepted basefbf5ca4f
and archive amendmentb6f8c919 stay frozen. Exact8 existing source paths unchanged.

Actual missing case: after A constructs archive P/<AUUID>, B may select that archive
as its parent and create P/<AUUID>/<BUUID>. B poisons A's epoch inventory. No global
archive registry is needed. Conservatively bound this explicit OWN layout: trusted
namespace root, archive PARENT, original native anchor/new native root and recovery
DESTINATION PARENT/root must contain NO path component that parses as a logical
v4 WorldId (case-insensitively). The constructor-created archive alone adds its
actual logical UUID child; namespace-managed UUID keys are internal, never route
configuration. Check canonical/no-link routes before any acquisition/file effects;
recheck these constraints in strict OWN observations/acquisition. Existing legacy
and process-owned routes retain their current contract. This only constrains the
new explicit fixed-terrain OWN layout; it makes no arbitrary-import/path-registry
compatibility promise. No new user-facing profile, framework, format or ingress.

Use the SAME two actual fixture worlds before first close. Once A exists, invoke
B's real NewOwnedRoute builder using A's newly empty archive as parent: require
refusal with A's native archive tree unchanged and B native root nonexistent. Also
check a proposed native root inside A archive refuses before construction. Then
B's ordinary valid shared-parent constructor proceeds; actual UUID archives remain
distinct. No extra world or runtime battery is added. Existing tool/retention inputs
remain unchanged. Source2 held and no format/compiler/runtime executed yet.

Root adopts only after independent read-only Sol High design acceptance, private
checkpoint and serial correction gate. This repairs the same accepted archive
exclusion invariant within the current packet, without an owner scope expansion.
