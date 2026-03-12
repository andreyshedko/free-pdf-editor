#!/bin/bash

##############################################################################
# Free PDF Editor - License Checker
#
# This script scans the project for license compliance issues and generates
# a report. It checks for:
# - License file in root directory
# - License headers in source files
# - Proper documentation
# - Dependency license tracking
#
# Usage: ./license-check.sh [WORKSPACE_PATH] [REPORT_PATH]
##############################################################################

set -e

WORKSPACE_PATH="${1:-.}"
REPORT_PATH="${2:./license-report.txt}"

FOUND_ISSUES=0
CHECKED_FILES=0

# Color codes for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Initialize report file
report_file="${REPORT_PATH}"
: > "$report_file"

write_report() {
    echo "$@" | tee -a "$report_file"
}

write_logo() {
    write_report "╔════════════════════════════════════════════════════════╗"
    write_report "║           Free PDF Editor - License Checker            ║"
    write_report "╚════════════════════════════════════════════════════════╝"
    write_report ""
}

write_section() {
    write_report ""
    write_report "═══ $1 ═══"
    write_report ""
}

check_license_file() {
    write_section "1. License File Check"
    
    local found=false
    for file in LICENSE LICENSE.md LICENSE.txt; do
        if [ -f "$WORKSPACE_PATH/$file" ]; then
            write_report "✓ Found license file: $file"
            found=true
            break
        fi
    done
    
    if [ "$found" = false ]; then
        write_report "✗ ISSUE: No LICENSE file found in root directory"
        ((FOUND_ISSUES++))
    fi
}

check_gitignore() {
    write_section "2. .gitignore Check"
    
    if [ -f "$WORKSPACE_PATH/.gitignore" ]; then
        write_report "✓ .gitignore file found"
        if grep -q "node_modules\|\.env\|target\|build" "$WORKSPACE_PATH/.gitignore" 2>/dev/null; then
            write_report "✓ Common ignore patterns present"
        fi
    else
        write_report "⚠ Warning: No .gitignore file found"
    fi
}

check_source_files() {
    write_section "3. Source Code License Headers"
    
    write_report "Scanning for license headers in source files..."
    write_report ""
    
    if [ ! -d "$WORKSPACE_PATH/src" ]; then
        write_report "⚠ No src/ directory found"
        return
    fi
    
    local files=$(find "$WORKSPACE_PATH/src" -type f \( -name "*.cpp" -o -name "*.h" -o -name "*.py" -o -name "*.rs" \) 2>/dev/null | wc -l)
    write_report "Checked $files source files"
    write_report ""
    
    local missing_headers=()
    local with_headers=0
    
    while IFS= read -r file; do
        ((CHECKED_FILES++))
        
        # Check for license header patterns
        if grep -qi "Copyright\|©\|SPDX-License\|Permission is hereby" "$file" 2>/dev/null; then
            ((with_headers++))
        else
            # Exclude test files
            if [[ ! "$file" =~ test ]]; then
                missing_headers+=("${file#$WORKSPACE_PATH/}")
            fi
        fi
    done < <(find "$WORKSPACE_PATH/src" -type f \( -name "*.cpp" -o -name "*.h" -o -name "*.py" -o -name "*.rs" \) 2>/dev/null)
    
    if [ ${#missing_headers[@]} -gt 0 ]; then
        write_report "✗ Files missing license headers (${#missing_headers[@]}):"
        for file in "${missing_headers[@]:0:10}"; do
            write_report "  - $file"
        done
        if [ ${#missing_headers[@]} -gt 10 ]; then
            write_report "  ... and $((${#missing_headers[@]} - 10)) more files"
        fi
        FOUND_ISSUES=$((FOUND_ISSUES + ${#missing_headers[@]}))
    else
        write_report "✓ All source files have license headers"
    fi
}

check_cmakelists() {
    write_section "4. CMakeLists.txt License Metadata"
    
    if [ -f "$WORKSPACE_PATH/CMakeLists.txt" ]; then
        write_report "✓ CMakeLists.txt found"
        if grep -q "project(" "$WORKSPACE_PATH/CMakeLists.txt" && grep -q "VERSION" "$WORKSPACE_PATH/CMakeLists.txt"; then
            write_report "✓ Version information present"
        else
            write_report "⚠ Missing version information in CMakeLists.txt"
        fi
    else
        write_report "⚠ CMakeLists.txt not found"
    fi
}

check_dependencies() {
    write_section "5. Dependency License Tracking"
    
    write_report "Checking for dependency declaration files..."
    write_report ""
    
    local dep_files=(
        "CMakeLists.txt:C++ CMake"
        "Cargo.toml:Rust Cargo"
        "package.json:Node.js npm"
        "requirements.txt:Python pip"
    )
    
    for dep_entry in "${dep_files[@]}"; do
        IFS=':' read -r filename type <<< "$dep_entry"
        if [ -f "$WORKSPACE_PATH/$filename" ]; then
            write_report "✓ Found $type dependency file: $filename"
        fi
    done
}

check_documentation() {
    write_section "6. Documentation & Legal Files"
    
    local doc_files=(
        "CONTRIBUTING.md:Contributing Guidelines"
        "CODE_OF_CONDUCT.md:Code of Conduct"
        "SECURITY.md:Security Policy"
        "README.md:Readme"
    )
    
    for doc_entry in "${doc_files[@]}"; do
        IFS=':' read -r filename type <<< "$doc_entry"
        if [ -f "$WORKSPACE_PATH/$filename" ]; then
            write_report "✓ Found $type: $filename"
        elif [ "$filename" = "README.md" ]; then
            write_report "✗ ISSUE: Missing README.md"
            ((FOUND_ISSUES++))
        else
            write_report "⚠ Missing $type: $filename"
        fi
    done
}

generate_summary() {
    write_section "Summary"
    
    write_report "Files Checked: $CHECKED_FILES"
    write_report "Issues Found: $FOUND_ISSUES"
    write_report ""
    
    if [ $FOUND_ISSUES -eq 0 ]; then
        write_report "✓ No license compliance issues detected!"
        write_report ""
        write_report "Project appears to be in good standing for license compliance."
    else
        write_report "✗ $FOUND_ISSUES license compliance issues found."
        write_report ""
        write_report "Please review the issues above and take corrective action."
    fi
    
    write_report ""
    write_report "═══════════════════════════════════════════════════════"
    write_report "Report generated: $(date '+%Y-%m-%d %H:%M:%S')"
    write_report ""
}

# Main execution
write_logo
check_license_file
check_gitignore
check_source_files
check_cmakelists
check_dependencies
check_documentation
generate_summary

echo ""
echo -e "${GREEN}Report saved to: $report_file${NC}"

# Exit with appropriate code
if [ $FOUND_ISSUES -gt 0 ]; then
    exit 1
else
    exit 0
fi
