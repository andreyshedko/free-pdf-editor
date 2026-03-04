#!/usr/bin/env python3
"""scripts/validate_metadata.py
Validates store/metadata.json for required fields and asset existence.
Used as a CI quality gate.
"""
import json
import os
import re
import sys

METADATA_PATH = "store/metadata.json"

REQUIRED_FIELDS = [
    "name",
    "display_name",
    "description",
    "version",
    "publisher",
    "publisher_display_name",
    "privacy_policy_url",
    "bundle_id",
    "windows_package_name",
    "macos_app_name",
]

REQUIRED_ASSETS = [
    "assets/icon-44.png",
    "assets/icon-150.png",
    "assets/icon-310.png",
    "assets/splash.png",
]

errors = []

# ── Load metadata ─────────────────────────────────────────────────────────────
if not os.path.exists(METADATA_PATH):
    print(f"ERROR: {METADATA_PATH} not found", file=sys.stderr)
    sys.exit(1)

with open(METADATA_PATH, encoding="utf-8") as f:
    meta = json.load(f)

# ── Check required fields ─────────────────────────────────────────────────────
for field in REQUIRED_FIELDS:
    if not meta.get(field):
        errors.append(f"Missing or empty field: '{field}'")

# ── Check privacy policy URL ──────────────────────────────────────────────────
privacy_url = meta.get("privacy_policy_url", "")
if not privacy_url.startswith("http"):
    errors.append(f"privacy_policy_url must be a valid URL, got: '{privacy_url}'")

# ── Check version format (MAJOR.MINOR.PATCH) ─────────────────────────────────
version = meta.get("version", "")
if not re.match(r"^\d+\.\d+\.\d+$", version):
    errors.append(f"version must be MAJOR.MINOR.PATCH format, got: '{version}'")

# ── Check required assets exist ───────────────────────────────────────────────
for asset_path in REQUIRED_ASSETS:
    if not os.path.exists(asset_path):
        errors.append(f"Required asset not found: '{asset_path}'")

# ── Report ────────────────────────────────────────────────────────────────────
if errors:
    print("Store metadata validation FAILED:", file=sys.stderr)
    for err in errors:
        print(f"  - {err}", file=sys.stderr)
    sys.exit(1)

print(f"Store metadata validation PASSED (version={version}, {len(REQUIRED_ASSETS)} assets OK)")
