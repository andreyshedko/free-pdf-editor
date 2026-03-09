param(
    [string]$QtRoot = "C:\Qt\6.10.2\mingw_64",
    [string]$MingwRoot = "C:\Qt\Tools\mingw1310_64"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$qmake = Join-Path $QtRoot "bin\qmake.exe"
$mingwBin = Join-Path $MingwRoot "bin"
$mingwMake = Join-Path $mingwBin "mingw32-make.exe"
$gitSh = "C:\Program Files\Git\usr\bin\sh.exe"
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"

if (-not (Test-Path $qmake)) {
    throw "qmake not found: $qmake"
}
if (-not (Test-Path $mingwMake)) {
    throw "mingw32-make.exe not found: $mingwMake"
}
if (-not (Test-Path $gitSh)) {
    throw "Git Bash sh.exe not found: $gitSh"
}

New-Item -ItemType Directory -Force -Path $cargoBin | Out-Null
Copy-Item -Force $mingwMake (Join-Path $cargoBin "make.exe")

$env:QMAKE = $qmake.Replace("\", "/")
$env:CC = "gcc"
$env:CXX = "g++"
$env:AR = "ar"
$env:SHELL = $gitSh.Replace("\", "/")
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-w64-windows-gnu -IC:/Qt/Tools/mingw1310_64/lib/gcc/x86_64-w64-mingw32/13.1.0/include"

$prepend = @(
    $cargoBin,
    $mingwBin,
    (Join-Path $QtRoot "bin"),
    "C:\Program Files\Git\usr\bin"
) -join ";"
$env:PATH = "$prepend;$env:PATH"

Write-Host "Qt/MinGW environment is set for this PowerShell session."
Write-Host "You can now run:"
Write-Host "  cargo build -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf"
