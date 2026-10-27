# Orchestration adoption

The owner requested workspace setup using Cha0sCollective/Ch4oS-Orchestration on
October 27, 2026. The inspected source revision is
`0bcf16d896a0bd79039453586c3e69487d804b0a`.

The reusable standards and agent catalog in this directory are manually retained
source copies. Effective project rules are in the root AGENTS.md. The only installed
custom role is the fresh independent reviewer; optional specialists are not a team
that must run for every packet. Source instructions are consulted only as needed.

The effective `.codex/agents/independent-reviewer.toml` is adapted from the canonical
profile with one deliberate model change: `gpt-5.6-sol` becomes the currently available
`gpt-6.1-sol`, retaining high effort. The source copy is unchanged. The coordinator
model/effort is not overridden. Current official Codex documentation supports these
custom agent fields and warns that a custom profile's sandbox setting does not
override the parent permission boundary; read-only behavior and actual permissions
must be verified at launch.

The private port repository holds internal planning, original inputs and effective
configuration. The public Horizon fork holds upstream source and deliberately
published engine changes. Its local ignored AGENTS.override.md is copied from
`orchestration/horizon/AGENTS.override.md`; it explicitly preserves and requires the
upstream guidance. The effective reviewer profile is copied locally as well. The
overlay can operate without reading a sibling checkout. These local copies are
not public source changes or an automatic synchronization mechanism.

`orchestration/workspace/AGENTS.md` retains the short workspace navigation guide
deployed at the parent workspace root. Effective repository instructions remain
self-contained; the parent guide supplies navigation and reference boundaries.

The canonical orchestration repository is unchanged. A future canonical project
registration or policy update is a separate task; local differences are not silently
reconciled back into it. No new merge protections, approval system, standing workers,
experiment DSL or distribution service was introduced.

Source: https://github.com/Cha0sCollective/Ch4oS-Orchestration/tree/0bcf16d896a0bd79039453586c3e69487d804b0a

Codex reference: https://learn.chatgpt.com/docs/agent-configuration/subagents
