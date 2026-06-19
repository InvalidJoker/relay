#Requires -Version 5.1
[CmdletBinding()]
param(
  [string]$Version,
  [string]$InstallDir = "$env:LOCALAPPDATA\relay\bin"
)

$ErrorActionPreference = 'Stop'

$Repo  = "InvalidJoker/relay"
$Asset = "relay-windows-x86_64.exe"

if ($Version) {
  $Url = "https://github.com/$Repo/releases/download/$Version/$Asset"
} else {
  $Url = "https://github.com/$Repo/releases/latest/download/$Asset"
}

Write-Host "Downloading relay CLI..."
Write-Host "  -> $Url"

if (-not (Test-Path $InstallDir)) {
  New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$Dest = Join-Path $InstallDir "relay.exe"

try {
  Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
} catch {
  Write-Error "Failed to download relay: $_"
  exit 1
}

# Add to PATH for current user if not already present
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
  [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
  Write-Host "Added $InstallDir to your PATH."
  Write-Host "Restart your terminal or run: `$env:PATH += ';$InstallDir'"
}

Write-Host "relay installed to $Dest"
Write-Host "Run 'relay --help' to get started."
