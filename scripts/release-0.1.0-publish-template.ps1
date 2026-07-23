[CmdletBinding()]
param(
    [switch]$Execute,
    [string]$ExpectedCommit
)

$ErrorActionPreference = 'Stop'
if (-not $Execute) {
    throw 'Refusing to publish. This is a template: pass -Execute only from the reviewed release commit.'
}
if ([string]::IsNullOrWhiteSpace($ExpectedCommit)) {
    throw 'Pass -ExpectedCommit with the reviewed release commit SHA.'
}
$head = (git rev-parse HEAD).Trim()
if ($head -ne $ExpectedCommit) {
    throw "HEAD is $head, not the approved release commit $ExpectedCommit."
}
if ((git status --porcelain).Length -ne 0) {
    throw 'Working tree is not clean.'
}

$packages = @(
    'slatec-bundled-x86_64-pc-windows-gnu',
    'slatec-bundled-x86_64-unknown-linux-gnu',
    'slatec-sys',
    'slatec-core',
    'slatec-src',
    'slatec'
)

foreach ($package in $packages) {
    cargo publish -p $package --locked
    if ($LASTEXITCODE -ne 0) {
        throw "Publish failed for $package; stop immediately and investigate before retrying."
    }
    Write-Host "WAIT: confirm crates.io index propagation and cargo info $package@0.1.0 before continuing."
    Read-Host 'Press Enter only after the package is visible'
}
