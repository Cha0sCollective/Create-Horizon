param([string]$RegistryPath = (Join-Path $PSScriptRoot '../../operations/team-registry.json'))
$ErrorActionPreference = 'Stop'
function Read-Utc($Value) {
    if ($Value -is [DateTimeOffset]) { return $Value }
    if ($Value -is [datetime]) { return [DateTimeOffset]$Value }
    return [DateTimeOffset]::Parse([string]$Value)
}
$registry = Get-Content -LiteralPath $RegistryPath -Raw | ConvertFrom-Json
$logRoot = Join-Path $env:USERPROFILE '.codex/sessions'
$outputRoot = 'G:\Create Rusty\.local\team-operations'
New-Item -ItemType Directory -Path $outputRoot -Force | Out-Null
$files = @{}
$rootFile = Join-Path $logRoot $registry.root_rollout
if (Test-Path -LiteralPath $rootFile) { $files[$rootFile] = Get-Item -LiteralPath $rootFile }
foreach ($offset in -1,0,1) {
    $folder = Join-Path $logRoot ([DateTimeOffset]::UtcNow.AddDays($offset).ToString('yyyy/MM/dd'))
    if (Test-Path -LiteralPath $folder) {
        foreach ($file in Get-ChildItem -LiteralPath $folder -Filter '*.jsonl' -File) { $files[$file.FullName] = $file }
    }
}
$indexPath = Join-Path $outputRoot 'session-index.json'
if (Test-Path -LiteralPath $indexPath) {
    foreach ($known in (Get-Content -LiteralPath $indexPath -Raw | ConvertFrom-Json)) {
        if (Test-Path -LiteralPath $known) { $files[$known] = Get-Item -LiteralPath $known }
    }
}
$sessions = @{}
foreach ($file in $files.Values) {
    try {
        $fs = [IO.FileStream]::new($file.FullName,[IO.FileMode]::Open,[IO.FileAccess]::Read,([IO.FileShare]::ReadWrite -bor [IO.FileShare]::Delete))
        $reader = [IO.StreamReader]::new($fs)
        $meta = $null; $metaAt = $null; $samples = [Collections.Generic.List[object]]::new()
        try {
            while (-not $reader.EndOfStream) {
                $line = $reader.ReadLine()
                if ($line -notmatch '"type"\s*:\s*"(session_meta|token_count)"') { continue }
                try { $value = $line | ConvertFrom-Json } catch { continue }
                if ($value.type -eq 'session_meta') { $meta=$value.payload; $metaAt=(Read-Utc $value.timestamp); continue }
                if ($value.type -eq 'event_msg' -and $value.payload.type -eq 'token_count' -and $null -ne $value.payload.info.total_token_usage) {
                    $samples.Add([pscustomobject]@{at=(Read-Utc $value.timestamp);usage=$value.payload.info.total_token_usage})
                }
            }
        } finally { $reader.Dispose() }
        if ($null -ne $meta) { $sessions[$meta.id]=[pscustomobject]@{id=$meta.id;created_at=$metaAt;parent=$meta.source.subagent.thread_spawn.parent_thread_id;agent=$meta.source.subagent.thread_spawn.agent_path;samples=$samples;path=$file.FullName} }
    } catch {
        # An inaccessible/partial live log is unknown, not zero. Missing root rows
        # are emitted below; other logs are discovered again at the next snapshot.
    }
}
$mapping = @{}; $starts = @{}
$mapping[$registry.root_thread_id]='operations';$starts['operations']=(Read-Utc $registry.operations_enrolled_at)
foreach ($team in $registry.teams) {
    if ($team.thread_id -and $team.enrolled_at) { $mapping[$team.thread_id]=$team.key;$starts[$team.key]=(Read-Utc $team.enrolled_at) }
}
do {
    $added=$false
    foreach ($session in $sessions.Values) {
        if (-not $mapping.ContainsKey($session.id) -and $session.parent -and $mapping.ContainsKey($session.parent)) { $mapping[$session.id]=$mapping[$session.parent];$added=$true }
    }
} while ($added)
$rows=[Collections.Generic.List[object]]::new();$knownPaths=[Collections.Generic.List[string]]::new()
foreach ($id in $mapping.Keys) {
    $team=$mapping[$id];$session=$sessions[$id];$cutoff=$starts[$team]
    if ($null -eq $session) {
        $rows.Add([pscustomobject]@{team=$team;session_id=$id;parent_id=$null;agent=$null;status='unknown-log';observed_at=$null;input_tokens=$null;cached_input_tokens=$null;output_tokens=$null;reasoning_output_tokens=$null;total_tokens=$null});continue
    }
    $knownPaths.Add($session.path)
    $latest=$session.samples | Select-Object -Last 1
    $baseline=$session.samples | Where-Object at -LE $cutoff | Select-Object -Last 1
    $isLead=($id -eq $registry.root_thread_id -or $registry.teams.thread_id -contains $id)
    if (-not $isLead -and $null -ne $latest -and $null -ne $session.created_at -and $session.created_at -lt $cutoff -and $latest.at -le $cutoff) { continue }
    $row=[ordered]@{team=$team;session_id=$id;parent_id=$session.parent;agent=$session.agent;status='reported';observed_at=if($latest){$latest.at.ToString('o')}else{$null}}
    foreach ($field in 'input_tokens','cached_input_tokens','output_tokens','reasoning_output_tokens','total_tokens') {
        $value=if($latest -and $null -ne $latest.usage.$field){[long]$latest.usage.$field}else{$null}
        $baseValue=if($baseline -and $null -ne $baseline.usage.$field){[long]$baseline.usage.$field}elseif(-not $baseline -and $null -ne $session.created_at -and $session.created_at -ge $cutoff){0L}else{$null}
        if ($null -eq $value -or $null -eq $baseValue -or $value -lt $baseValue) { $row[$field]=$null;$row.status='unknown-or-reset' } else { $row[$field]=$value-$baseValue }
    }
    $rows.Add([pscustomobject]$row)
}
@($knownPaths | Sort-Object -Unique) | ConvertTo-Json -AsArray | Set-Content -LiteralPath $indexPath -Encoding utf8
$rows | Export-Csv -LiteralPath (Join-Path $outputRoot 'token-usage.csv') -NoTypeInformation -Encoding utf8
$timeRows=[Collections.Generic.List[object]]::new()
foreach ($file in Get-ChildItem -LiteralPath $outputRoot -Filter '*-events.jsonl' -File -Recurse) {
    foreach ($line in Get-Content -LiteralPath $file.FullName) {
        try { $event=$line|ConvertFrom-Json;$timeRows.Add($event) } catch { }
    }
}
$timeRows | Export-Csv -LiteralPath (Join-Path $outputRoot 'time-events.csv') -NoTypeInformation -Encoding utf8
$summary=[ordered]@{observed_at=[DateTimeOffset]::UtcNow.ToString('o');project_started_at=$registry.operations_enrolled_at;elapsed_seconds=[Math]::Round(([DateTimeOffset]::UtcNow-(Read-Utc $registry.operations_enrolled_at)).TotalSeconds,3);teams=@()}
if ($summary.elapsed_seconds -lt 0) { throw 'Enrollment is in the future; do not publish incorrect elapsed time.' }
foreach ($key in @('operations')+@($registry.teams.key)) {
    $own=@($rows | Where-Object team -eq $key);$phases=@{}
    foreach($phase in 'implementation','build','test','review','coordination','waiting') { $phases[$phase]=[Math]::Round((($timeRows|Where-Object { $_.team -eq $key -and $_.phase -eq $phase }|Measure-Object phase_seconds -Sum).Sum),3) }
    $totals=@{}
    foreach($field in 'input_tokens','cached_input_tokens','output_tokens','reasoning_output_tokens','total_tokens') { $totals[$field]=if($own.Count -gt 0 -and -not ($own|Where-Object status -ne 'reported')){[long](($own|Measure-Object $field -Sum).Sum)}else{$null} }
    $summary.teams+= [ordered]@{team=$key;tokens=$totals;reported_sessions=$own.Count;phase_agent_seconds=$phases;open_phases=@(Get-ChildItem -Path (Join-Path $outputRoot "$key/phases/*-state.json") -File -ErrorAction SilentlyContinue | ForEach-Object {Get-Content -LiteralPath $_.FullName -Raw|ConvertFrom-Json}|Where-Object phase -ne 'complete')}
}
$summary|ConvertTo-Json -Depth 8|Set-Content -LiteralPath (Join-Path $outputRoot 'usage-summary.json') -Encoding utf8
$summary|ConvertTo-Json -Compress -Depth 8
