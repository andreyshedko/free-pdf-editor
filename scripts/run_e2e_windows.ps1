param(
    [string]$AppExe = "",
    [switch]$SkipBuild,
    [ValidateSet("smoke", "regression")]
    [string]$Suite = "smoke"
)

$ErrorActionPreference = "Stop"

if (-not $SkipBuild) {
    Write-Host "Building app-desktop (debug)..."
    cargo build -p app-desktop --bin pdf-editor --features mupdf
}

if ([string]::IsNullOrWhiteSpace($AppExe)) {
    $AppExe = Join-Path $PSScriptRoot "..\\target\\debug\\pdf-editor.exe"
}

$resolvedApp = Resolve-Path $AppExe
Write-Host "Using APP_EXE=$resolvedApp"

$sandboxAppData = Join-Path $PSScriptRoot "..\\tests\\e2e\\.tmp\\appdata"
New-Item -ItemType Directory -Path $sandboxAppData -Force | Out-Null
Write-Host "Using APPDATA sandbox=$sandboxAppData"

Push-Location (Join-Path $PSScriptRoot "..\\tests\\e2e")
try {
    if (-not (Test-Path "node_modules")) {
        Write-Host "Installing e2e npm dependencies..."
        npm install
    }

    $env:APP_EXE = $resolvedApp
    $env:APPDATA = (Resolve-Path $sandboxAppData)
    if ($Suite -eq "regression") {
        npm run test:regression
    }
    else {
        npm run test:smoke
    }
}
finally {
    Pop-Location
}
