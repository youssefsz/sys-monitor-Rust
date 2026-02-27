# ── sys-monitor installer for Windows ────────────────────────────────────
# Usage: powershell -Command "irm https://raw.githubusercontent.com/youssefsz/sys-monitor-Rust/master/install.ps1 | iex"
# ─────────────────────────────────────────────────────────────────────────

$ErrorActionPreference = "Stop"

# Force TLS 1.2 (required by GitHub, older PowerShell defaults to TLS 1.0)
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$repo = "youssefsz/sys-monitor-Rust"
$binary = "sys-monitor.exe"
$installDir = "$HOME\AppData\Local\Programs\sys-monitor"

Write-Host ""
Write-Host "  sys-monitor installer" -ForegroundColor Cyan
Write-Host "  ---------------------"
Write-Host ""

# ── Detect architecture ──────────────────────────────────────────────────
$rawArch = $env:PROCESSOR_ARCHITECTURE
switch ($rawArch) {
    "AMD64"  { $arch = "x86_64"  }
    "x86"    { $arch = "x86_64"  }  # 32-bit PS on 64-bit OS still reports x86
    "ARM64"  { $arch = "aarch64" }
    default  {
        Write-Error "Unsupported architecture: $rawArch"
        exit 1
    }
}

$platform = "windows"
$assetName = "sys-monitor-$platform-$arch.exe"

Write-Host "-> Detected: $platform-$arch"

# ── Fetch latest release ─────────────────────────────────────────────────
Write-Host "-> Fetching latest release..."

$latestUrl = "https://api.github.com/repos/$repo/releases/latest"

try {
    $release = Invoke-RestMethod -Uri $latestUrl -Headers @{ "User-Agent" = "sys-monitor-installer" }
} catch {
    Write-Error "Failed to fetch release info from GitHub. Check your internet connection."
    exit 1
}

$asset = $release.assets | Where-Object { $_.name -eq $assetName }

if (-not $asset) {
    Write-Error "Could not find a release binary for $assetName.`nCheck https://github.com/$repo/releases for available downloads."
    exit 1
}

$downloadUrl = $asset.browser_download_url
$version = $release.tag_name
Write-Host "-> Latest version: $version"

# ── Download binary ──────────────────────────────────────────────────────
if (-not (Test-Path $installDir)) {
    New-Item -Path $installDir -ItemType Directory | Out-Null
}

$tmpFile = Join-Path $env:TEMP "sys-monitor-download-$([guid]::NewGuid().ToString('N').Substring(0,8)).exe"

Write-Host "-> Downloading $assetName..."

try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tmpFile -UseBasicParsing
} catch {
    Write-Error "Failed to download binary from $downloadUrl"
    exit 1
}

Write-Host "-> Downloaded successfully" -ForegroundColor Green

# ── Install ──────────────────────────────────────────────────────────────
Write-Host "-> Installing to $installDir..."

$destPath = Join-Path $installDir $binary
$oldPath  = "$destPath.old"

try {
    # Windows locks running executables – rename the old binary first
    if (Test-Path $destPath) {
        if (Test-Path $oldPath) { Remove-Item $oldPath -Force }
        Rename-Item -Path $destPath -NewName "$binary.old" -Force
    }

    Move-Item -Path $tmpFile -Destination $destPath -Force

    # Clean up the old binary (best-effort; may still be locked)
    if (Test-Path $oldPath) {
        try { Remove-Item $oldPath -Force } catch { }
    }
} catch {
    # Clean up temp file on failure
    if (Test-Path $tmpFile) { Remove-Item $tmpFile -Force }
    Write-Error "Failed to install binary to $installDir"
    exit 1
}

# ── Add to PATH ──────────────────────────────────────────────────────────
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    Write-Host "-> Adding $installDir to PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    $env:Path += ";$installDir"
}

# ── Verify ───────────────────────────────────────────────────────────────
$installedPath = Join-Path $installDir $binary
if (Test-Path $installedPath) {
    Write-Host ""
    Write-Host "  Installed! Run it with: sys-monitor" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "  Installation may have failed. Binary not found at $installedPath" -ForegroundColor Yellow
}

Write-Host ""
