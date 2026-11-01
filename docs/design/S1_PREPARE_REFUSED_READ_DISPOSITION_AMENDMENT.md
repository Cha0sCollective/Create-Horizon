# C0a refused read-only input disposition

Status: ACCEPTED by fresh s1_composition_api_review at proposed document hash
d62158ab1f86689a386dafb171af47bed1e94774127d492918d97e6ee06e0a0f,
conditional on the private assignment update. No new source path, ordinary
activation or native validation profile is granted.

The retained-load carrier can preserve malformed/proto/unsupported original
input without producing a new column. That read-only result must refuse
generation, adoption, validation and rewrite. Making it permanently uncloseable
would prevent failed preparation cleanup even though the original stored input
was never consumed or changed.

Permit a distinct checked refused-read disposition ONLY for an actual retained
load result produced before generation/finish, never a fresh Generated/Finish
result or a missing/unexplained work payload. Preserve the full carrier, original
error and work lease until the original submitted read is accounted as a refused
read. Keep original raw/pending evidence in the retained owner through final
same-allocation writer completion and checked terminal acknowledgement.

This disposition claims only that a failed read produced no new domain output,
performed no generation/adoption/activation/rewrite, and its original input
remains stored or owned by the original accepted pending writer. It does not
claim the input is valid or that preparation/game startup succeeded. The terminal
PreparationClosed result must retain the refusal/error distinctly from a healthy
or canceled-ready result. Preserve the first preparation error separately.

Relevant storage sequence/current-state checks and actual writer closure remain
mandatory. Newer conflicting writes require separate explicit reconciliation;
poison, unexplained missing output, worker panic with held work, unknown provider
ownership or failed writer closure remain Blocked. Never apply this disposition
to unique unsaved generated chunks, returned proto owners, callbacks or arbitrary
item payloads. A decoder-produced loaded Chunk remains read-only and unactivated;
its exact original raw/pending source must still be retained and correlated.

Positive loaded/full columns continue to use strict column/item validation and
AlreadyStored checks. Refused malformed/proto/unknown-item fixtures can instead
show refusal/no regeneration/no rewrite, release/reap actual controls, close the
original producers/writer and check terminal failed-cleanup plus original bytes
and receipts before assertions. No destructor cleanup or automatic fault waiver.
