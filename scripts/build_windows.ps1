# scripts/build_windows.ps1
# Idempotent Windows release build script.
# Produces a signed MSIX package at dist/windows/FreePDFEditor_<VERSION>.msix
#
# Version/channel/build_number are read exclusively from release/release.json.
# CI pre-populates that file via scripts/generate_release_json.py before
# invoking this script.
#
# Required environment variables:
#   WINDOWS_CERT_BASE64   - base64-encoded PFX certificate
#   WINDOWS_CERT_PASSWORD - certificate password
#   PUBLISHER             - Publisher identity string (CN=...)
#
# Optional:
#   SKIP_SIGNING          - set to "1" to skip code-signing (debug builds)

param()
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$AppName    = "FreePDFEditor"
$BinaryName = "pdf-editor"
$Target     = "x86_64-pc-windows-msvc"
$DistDir    = "dist\windows"
$StageDir   = "$DistDir\stage"
$AssetsDir  = "assets"
$Publisher  = if ($Env:PUBLISHER) { $Env:PUBLISHER } else { "CN=FreePDFEditor" }
$CertPath   = "$Env:TEMP\win_cert.pfx"

# ── Read version/build_number from release/release.json ──────────────────────
$ReleaseJson = Get-Content "release\release.json" | ConvertFrom-Json
$Version     = $ReleaseJson.version
$BuildNumber = $ReleaseJson.build_number
$Channel     = $ReleaseJson.channel
$MsixPath    = "$DistDir\${AppName}_${Version}.msix"

Write-Host "==> Building $AppName $Version (build $BuildNumber, channel $Channel) for $Target"

# ── 1. Build binary (version injected via build.rs) ───────────────────────────
$Env:APP_VERSION      = $Version
$Env:APP_CHANNEL      = $Channel
$Env:APP_BUILD_NUMBER = $BuildNumber
$Env:STORE_BUILD      = "1"
cargo build --release --target $Target

# ── 2. Prepare staging layout ─────────────────────────────────────────────────
if (Test-Path $StageDir) { Remove-Item -Recurse -Force $StageDir }
New-Item -ItemType Directory -Force -Path "$StageDir\Assets" | Out-Null

Copy-Item "target\$Target\release\$BinaryName.exe" "$StageDir\$BinaryName.exe"
Copy-Item "$AssetsDir\icon-44.png"  "$StageDir\Assets\icon-44.png"
Copy-Item "$AssetsDir\icon-150.png" "$StageDir\Assets\icon-150.png"
Copy-Item "$AssetsDir\icon-310.png" "$StageDir\Assets\icon-310.png"
Copy-Item "$AssetsDir\splash.png"   "$StageDir\Assets\splash.png"

# ── 3. Generate AppxManifest.xml ─────────────────────────────────────────────
$MetaRaw = Get-Content "store\metadata.json" | ConvertFrom-Json
$ManifestTemplate = Get-Content "platform\windows\AppxManifest.xml.template" -Raw

# Convert semver to four-part Windows version (MAJOR.MINOR.PATCH.0)
$Parts   = $Version -split '\.'
$WinVer  = "{0}.{1}.{2}.0" -f $Parts[0], ($Parts[1] -replace '[^0-9]','0'), ($Parts[2] -replace '[^0-9]','0')

$Manifest = $ManifestTemplate `
    -replace '__PACKAGE_NAME__',           $MetaRaw.windows_package_name `
    -replace '__PUBLISHER__',              $Publisher `
    -replace '__VERSION__',                $WinVer `
    -replace '__PHONE_PRODUCT_ID__',       ([System.Guid]::NewGuid().ToString()) `
    -replace '__DISPLAY_NAME__',           $MetaRaw.display_name `
    -replace '__PUBLISHER_DISPLAY_NAME__', $MetaRaw.publisher_display_name `
    -replace '__DESCRIPTION__',            $MetaRaw.description

$Manifest | Set-Content "$StageDir\AppxManifest.xml" -Encoding UTF8

Write-Host "==> AppxManifest.xml generated (version=$WinVer)"

# ── 4. Sign binary (optional) ────────────────────────────────────────────────
if ($Env:SKIP_SIGNING -ne "1") {
    if (-not $Env:WINDOWS_CERT_BASE64) { throw "WINDOWS_CERT_BASE64 not set" }
    [System.IO.File]::WriteAllBytes($CertPath, [Convert]::FromBase64String($Env:WINDOWS_CERT_BASE64))
    try {
        # Locate signtool.exe dynamically across SDK versions
        $KitsBase = "${Env:ProgramFiles(x86)}\Windows Kits\10\bin"
        $SignTool = Get-ChildItem "$KitsBase\*\x64\signtool.exe" -ErrorAction SilentlyContinue |
            Sort-Object FullName -Descending |
            Select-Object -First 1 -ExpandProperty FullName
        if (-not $SignTool) { throw "signtool.exe not found under $KitsBase" }
        & $SignTool sign /fd SHA256 /f $CertPath /p $Env:WINDOWS_CERT_PASSWORD /v "$StageDir\$BinaryName.exe" 2>&1 |
            Write-Host
    } finally {
        Remove-Item -Force $CertPath -ErrorAction SilentlyContinue
    }
    Write-Host "==> Binary signed"
}

# ── 5. Package MSIX ──────────────────────────────────────────────────────────
New-Item -ItemType Directory -Force -Path $DistDir | Out-Null
& MakeAppx.exe pack /d $StageDir /p $MsixPath /l
Write-Host "==> MSIX created: $MsixPath"

# ── 6. Validate manifest ─────────────────────────────────────────────────────
Write-Host "==> Validating AppxManifest..."
[xml]$xml = Get-Content "$StageDir\AppxManifest.xml"
$identity = $xml.Package.Identity
if (-not $identity.Name -or -not $identity.Publisher -or -not $identity.Version) {
    throw "AppxManifest validation failed: missing Identity fields"
}
Write-Host "==> Manifest valid (Name=$($identity.Name), Version=$($identity.Version))"

Write-Host "==> Windows build complete: $MsixPath"
