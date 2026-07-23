[CmdletBinding()]
param(
    [switch]$PrerequisitesPublished
)

$ErrorActionPreference = 'Stop'
$packages = @(
    'slatec-bundled-x86_64-pc-windows-gnu',
    'slatec-bundled-x86_64-unknown-linux-gnu',
    'slatec-sys',
    'slatec-core',
    'slatec-src',
    'slatec'
)

foreach ($package in $packages) {
    if (-not $PrerequisitesPublished -and $package -in @('slatec-core', 'slatec-src', 'slatec')) {
        Write-Host "STOP: $package needs published registry prerequisites. Re-run with -PrerequisitesPublished only after verifying them."
        break
    }

    Write-Host "Dry-running $package"
    cargo publish -p $package --dry-run --locked
    if ($LASTEXITCODE -ne 0) {
        throw "cargo publish --dry-run failed for $package; do not continue."
    }

    Write-Host "Verify package metadata and prerequisites before the next package: cargo info $package@0.1.0"
}
