# Team operations and accounting

The main chat owns this registry, heavy resource queue and integration. The six
charters live under `orchestration/teams/`; PROJECT_TEAMS defines the reviewed
workflow. Other mods are deferred. Team chats use their explicit lane paths.

Record phase boundaries with PowerShell:

```powershell
& tools/operations/phase.ps1 -Team machinery -Agent /root -Phase coordination
& tools/operations/phase.ps1 -Team machinery -Agent /root -Phase implementation
& tools/operations/phase.ps1 -Team machinery -Agent /root -Phase complete
```

Use the actual nested task path for workers. Each agent has its own hashed file
under `G:\Create Rusty\.local\team-operations\<team>\phases`. This permission is
only operational metadata for otherwise read-only discovery; strict independent
reviewers report timing instead of writing. Close phases before idle/final.
Build/test phases wrap actual validation commands; attach their receipt path.
Command receipts retain precise execution durations separately from phase elapsed.

Only operations runs `tools/operations/collect-usage.ps1`. It reads session/parent
metadata and token_count usage records from accessible local Codex logs with a
shared read handle, selects registered teams and descendants, subtracts counters
at enrollment and deduplicates sessions. The first session header owns a log;
embedded parent headers in full-context forks do not replace child identity.
Cached input and reasoning output remain
subsets. No raw messages/transcripts, credentials or billing API are copied.

The local output contains token-usage.csv, time-events.csv and usage-summary.json.
Unknown usage is null. These are reported counters, not final billing. Team totals
cover lead plus descendants once. Parallel agent-seconds can exceed project
elapsed seconds; open/unclassified intervals are visible, not silently counted
as tests. Historical time before enrollment is not reconstructed. Ops starts at
the first retained usage capture; team enrollment begins at chat dispatch.

Do not edit the central registry, steal a lease or start heavy builds/games from
a team without project-lead allocation. New thread prompts preserve existing model
settings and actual permissions. No global concurrency limits are changed.
