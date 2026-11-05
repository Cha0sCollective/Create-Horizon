param(
    [Parameter(Mandatory)][ValidateSet('operations','machinery','transport','fluids-power','contraptions','trains-schematics','experience')][string]$Team,
    [Parameter(Mandatory)][string]$Agent,
    [Parameter(Mandatory)][ValidateSet('implementation','build','test','review','coordination','waiting','complete')][string]$Phase,
    [string]$Evidence = ''
)
$ErrorActionPreference = 'Stop'
$base = 'G:\Create Rusty\.local\team-operations'
$folder = Join-Path $base "$Team/phases"
New-Item -ItemType Directory -Path $folder -Force | Out-Null
$key = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($Agent))).ToLowerInvariant().Substring(0,20)
$statePath = Join-Path $folder "$key-state.json"
$eventsPath = Join-Path $folder "$key-events.jsonl"
$now = [DateTimeOffset]::UtcNow
if (Test-Path -LiteralPath $statePath) {
    $previous = Get-Content -LiteralPath $statePath -Raw | ConvertFrom-Json
    if ($previous.phase -ne 'complete') {
        $started = if ($previous.started_at -is [datetime]) { [DateTimeOffset]$previous.started_at } else { [DateTimeOffset]::Parse($previous.started_at) }
        $entry = [ordered]@{event_id=[Guid]::NewGuid().ToString();team=$Team;agent=$Agent;phase=$previous.phase;started_at=$started.ToString('o');ended_at=$now.ToString('o');phase_seconds=[Math]::Round(($now-$started).TotalSeconds,3);evidence=$Evidence}
        Add-Content -LiteralPath $eventsPath -Value ($entry | ConvertTo-Json -Compress) -Encoding utf8
    }
}
[ordered]@{team=$Team;agent=$Agent;phase=$Phase;started_at=$now.ToString('o')} | ConvertTo-Json | Set-Content -LiteralPath $statePath -Encoding utf8
[pscustomobject]@{team=$Team;agent=$Agent;phase=$Phase;at=$now.ToString('o');events=$eventsPath} | ConvertTo-Json -Compress
