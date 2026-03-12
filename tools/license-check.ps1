#!/usr/bin/env powershell
<#
.SYNOPSIS
Scans the project for license compliance issues and generates a report.

.DESCRIPTION
This script checks:
- License headers in source files
- Missing LICENSE file
- Dependencies and their licenses (if applicable)
- Source file copyright notices

.PARAMETER WorkspacePath
The root path of the project (default: parent of tools directory)

.PARAMETER ReportPath
Output path for the license report (default: ./license-report.txt)

.PARAMETER Extensions
File extensions to check for license headers (default: @('*.cpp', '*.h', '*.py', '*.rs'))

.EXAMPLE
./license-check.ps1 -WorkspacePath "C:\projects\free-pdf-editor"
#>

param(
    [string]$WorkspacePath = (Join-Path (Split-Path $PSScriptRoot) -ChildPath "."),
    [string]$ReportPath = (Join-Path $WorkspacePath -ChildPath "license-report.txt"),
    [string[]]$Extensions = @('*.cpp', '*.h', '*.py', '*.rs', '*.tsx', '*.ts')
)

$script:foundIssues = 0
$script:checkedFiles = 0
$report = @()

function Write-Logo {
    $report += "╔════════════════════════════════════════════════════════╗"
    $report += "║           Free PDF Editor - License Checker            ║"
    $report += "╚════════════════════════════════════════════════════════╝"
    $report += ""
}

function Write-SectionHeader {
    param([string]$Title)
    $report += ""
    $report += "═══ $Title ═══"
    $report += ""
}

function Check-LicenseFile {
    Write-SectionHeader "1. License File Check"
    
    $licenseFiles = @("LICENSE", "LICENSE.md", "LICENSE.txt")
    $found = $false
    
    foreach ($file in $licenseFiles) {
        $path = Join-Path $WorkspacePath $file
        if (Test-Path $path) {
            $report += "✓ Found license file: $file"
            $found = $true
            break
        }
    }
    
    if (-not $found) {
        $report += "✗ ISSUE: No LICENSE file found in root directory"
        $script:foundIssues++
    }
}

function Check-GitIgnore {
    Write-SectionHeader "2. .gitignore Check"
    
    $gitignorePath = Join-Path $WorkspacePath ".gitignore"
    if (Test-Path $gitignorePath) {
        $report += "✓ .gitignore file found"
        $content = Get-Content $gitignorePath -Raw
        if ($content -match "node_modules|\.env|target") {
            $report += "✓ Common ignore patterns present"
        }
    } else {
        $report += "⚠ Warning: No .gitignore file found"
    }
}

function Check-SourceFiles {
    Write-SectionHeader "3. Source Code License Headers"
    
    $report += "Scanning for license headers in source files..."
    $report += ""
    
    $srcPath = Join-Path $WorkspacePath "src"
    if (-not (Test-Path $srcPath)) {
        $report += "⚠ No src/ directory found"
        return
    }
    
    $files = Get-ChildItem -Path $srcPath -Recurse -Include $Extensions -File
    $report += "Checked $($files.Count) source files`n"
    
    $missingHeaders = @()
    $withHeaders = 0
    
    foreach ($file in $files) {
        $script:checkedFiles++
        $content = Get-Content $file -Raw -ErrorAction SilentlyContinue
        
        # Check for common license header patterns
        $patterns = @(
            'Copyright|©|\(c\)',
            'SPDX-License-Identifier',
            'License:',
            'Permission is hereby granted'
        )
        
        $hasHeader = $false
        foreach ($pattern in $patterns) {
            if ($content -match $pattern) {
                $hasHeader = $true
                $withHeaders++
                break
            }
        }
        
        if (-not $hasHeader -and $file.Name -notlike '*test*') {
            $missingHeaders += $file.FullName.Replace($WorkspacePath, ".")
        }
    }
    
    if ($missingHeaders.Count -gt 0) {
        $report += "✗ Files missing license headers ($($missingHeaders.Count)):"
        foreach ($file in $missingHeaders | Select-Object -First 10) {
            $report += "  - $file"
        }
        if ($missingHeaders.Count -gt 10) {
            $report += "  ... and $($missingHeaders.Count - 10) more files"
        }
        $script:foundIssues += $missingHeaders.Count
    } else {
        $report += "✓ All source files have license headers"
    }
}

function Check-CMakeLists {
    Write-SectionHeader "4. CMakeLists.txt License Metadata"
    
    $cmakePath = Join-Path $WorkspacePath "CMakeLists.txt"
    if (Test-Path $cmakePath) {
        $report += "✓ CMakeLists.txt found"
        $content = Get-Content $cmakePath
        
        if ($content -match 'project\(' -and $content -match 'VERSION') {
            $report += "✓ Version information present"
        } else {
            $report += "⚠ Missing version information in CMakeLists.txt"
        }
    } else {
        $report += "⚠ CMakeLists.txt not found"
    }
}

function Check-Dependencies {
    Write-SectionHeader "5. Dependency License Tracking"
    
    $report += "Checking for dependency declaration files..."
    $report += ""
    
    $depFiles = @(
        @{ Name = "CMakeLists.txt"; Type = "C++ CMake" },
        @{ Name = "Cargo.toml"; Type = "Rust Cargo" },
        @{ Name = "package.json"; Type = "Node.js npm" },
        @{ Name = "requirements.txt"; Type = "Python pip" }
    )
    
    foreach ($depFile in $depFiles) {
        $path = Join-Path $WorkspacePath $depFile.Name
        if (Test-Path $path) {
            $report += "✓ Found $($depFile.Type) dependency file: $($depFile.Name)"
        }
    }
}

function Check-Documentation {
    Write-SectionHeader "6. Documentation & Legal Files"
    
    $docFiles = @(
        @{ Name = "CONTRIBUTING.md"; Type = "Contributing Guidelines" },
        @{ Name = "CODE_OF_CONDUCT.md"; Type = "Code of Conduct" },
        @{ Name = "SECURITY.md"; Type = "Security Policy" },
        @{ Name = "README.md"; Type = "Readme" }
    )
    
    foreach ($docFile in $docFiles) {
        $path = Join-Path $WorkspacePath $docFile.Name
        if (Test-Path $path) {
            $report += "✓ Found $($docFile.Type): $($docFile.Name)"
        } elseif ($docFile.Type -eq "Readme") {
            $report += "✗ ISSUE: Missing README.md"
            $script:foundIssues++
        } else {
            $report += "⚠ Missing $($docFile.Type): $($docFile.Name)"
        }
    }
}

function Generate-Summary {
    Write-SectionHeader "Summary"
    
    $report += "Files Checked: $script:checkedFiles"
    $report += "Issues Found: $script:foundIssues"
    $report += ""
    
    if ($script:foundIssues -eq 0) {
        $report += "✓ No license compliance issues detected!"
        $report += ""
        $report += "Project appears to be in good standing for license compliance."
    } else {
        $report += "✗ $script:foundIssues license compliance issues found."
        $report += ""
        $report += "Please review the issues above and take corrective action."
    }
    
    $report += ""
    $report += "═══════════════════════════════════════════════════════"
    $report += "Report generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
    $report += ""
}

# Main execution
Write-Logo
Check-LicenseFile
Check-GitIgnore
Check-SourceFiles
Check-CMakeLists
Check-Dependencies
Check-Documentation
Generate-Summary

# Output report
$report | Out-String | Write-Host
$report | Out-File -FilePath $ReportPath -Encoding UTF8

Write-Host ""
Write-Host "Report saved to: $ReportPath" -ForegroundColor Cyan

# Exit with appropriate code
if ($script:foundIssues -gt 0) {
    exit 1
} else {
    exit 0
}
