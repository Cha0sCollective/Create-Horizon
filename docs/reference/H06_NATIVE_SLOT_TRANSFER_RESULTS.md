# H06a detached native slot-transfer results

Local engine commit `0fd5e1ee2ef43cb8fd52835f0f282f24920bb77a`, tree
`523a3c6805c9e3fa74d480af9e5994e44c2e29f3`, adds the experimental
paired-packages-gated client library module modeling two detached
owned slot projections. It uses native ItemStack values and merge compatibility,
private allocation identities, checked revisions and consumed prepared plans.
The accepted boundary is docs/design/H06_NATIVE_SLOT_TRANSFER_PACKET.md.

Native-equal replacement retains the original stack entirely: native equality
ignores bundle selection, including nested selections. The direct metadata tests
therefore compare those integer fields separately. Positive transfer clones retain
the source remainder and the existing destination's component data. Depleted
sources become None; no-move paths retain unusual zero-count representations.

Commit checks both captured identities, revisions, native values and fresh limits,
then computes both next revisions and constructs its result before publication.
Only assignments follow the first update. Dropping a plan changes nothing;
duplicate plans and A/B/A replacements become stale through revisions. Allocation
identity retained by snapshots/plans prevents reconstructed projections matching.

Source after pinned formatting:

| File | SHA256 |
| --- | --- |
| client/src/slot_transfer.rs | `d25272c304844af6c18f41a3f48b02262f406a21af9e78693ed25bcd60b0c9fb` |
| client/src/lib.rs | `9f11c8e278677396f66a4e1b0a0d11a636177e16ef15ad9d3402cc9786a53dc0` |

Fresh independent read-only Sol High design and implementation review find no
issues within this scope. The six focused test matrices cover native-identity
conservation with widened totals, both directions, exact/partial/custom limits,
u32 maximum counts, oversized inputs, zero representations, incompatible item/
damage/components, nested selection retention, abort, stale/duplicate plans,
changed limits, A/B/A, new allocation, alias refusal and exhausted revisions.

All six focused matrices pass with runtime+paired features and with paired alone;
no tests are ignored in either execution. Runtime-on, paired-without-runtime and
feature-off client library/binary/test Clippy pass with warnings denied. Pinned
formatting, public-tree and the normal local commit hook pass. The two-file +551
scope exceeds the preferred roughly 500-line size and is disclosed, including the
focused native tests. No dependency or lockfile changes occur.

The broad quick pipeline passes 369 Python and 1357 client tests, zero doc tests,
format/lint/public-tree and unchanged lock checks at the exact local commit.
Content-root and launcher checks are explicitly skipped because those areas are
untouched. Parity is explicitly skipped in quick mode; no vanilla parity claim is
made for this disconnected experimental module.

Final independent read-only receipt review finds no issues, reconciling the source
pins, native logs and exact broad checkout/status. It does not repeat execution or
independently rerun the normal commit hook. Raw validation stays under the
workspace-root .local/h06a-* paths. No source fixes were needed after independent
review or focused execution.

This is detached in-memory accounting, not a live inventory adapter. Native input
replacement is an explicit model-update seam, not an item-conserving transfer.
No Game hook, player authority, replay protocol, slot ownership at world teardown,
guest transaction, persistence, performance or broader H06/H07/G1 acceptance is
established. Existing H05d both-mode game results are unchanged coverage, not new
game execution of this unconnected module. No public push or stable SDK is implied.
