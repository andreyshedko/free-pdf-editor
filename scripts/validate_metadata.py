#!/usr/bin/env python3
"""scripts/validate_metadata.py
Validates store/metadata.json and release/release.json for required fields,
asset existence, and cross-file version consistency.
Used as a CI quality gate.
"""
import json
import os
import re
import sys

METADATA_PATH = "store/metadata.json"
RELEASE_PATH  = "release/release.json"

REQUIRED_METADATA_FIELDS = [
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

REQUIRED_RELEASE_FIELDS = [
    "version",
    "build_number",
    "channel",
    "min_supported_version",
]

VALID_CHANNELS = {"alpha", "beta", "stable"}

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

# ── Load release.json ─────────────────────────────────────────────────────────
if not os.path.exists(RELEASE_PATH):
    print(f"ERROR: {RELEASE_PATH} not found", file=sys.stderr)
    sys.exit(1)

with open(RELEASE_PATH, encoding="utf-8") as f:
    release = json.load(f)

# ── Check required metadata fields ───────────────────────────────────────────
for field in REQUIRED_METADATA_FIELDS:
    if not meta.get(field):
        errors.append(f"metadata: missing or empty field: '{field}'")

# ── Check required release fields ────────────────────────────────────────────
for field in REQUIRED_RELEASE_FIELDS:
    if not release.get(field) and release.get(field) != 0:
        errors.append(f"release.json: missing or empty field: '{field}'")

# ── Check privacy policy URL ──────────────────────────────────────────────────
privacy_url = meta.get("privacy_policy_url", "")
if not privacy_url.startswith("http"):
    errors.append(f"metadata: privacy_policy_url must be a valid URL, got: '{privacy_url}'")

# ── Check version format (MAJOR.MINOR.PATCH) ─────────────────────────────────
release_version = release.get("version", "")
if not re.match(r"^\d+\.\d+\.\d+$", release_version):
    errors.append(f"release.json: version must be MAJOR.MINOR.PATCH, got: '{release_version}'")

# ── Cross-file version consistency ───────────────────────────────────────────
meta_version = meta.get("version", "")
if meta_version and release_version and meta_version != release_version:
    errors.append(
        f"version mismatch: store/metadata.json has '{meta_version}' "
        f"but release/release.json has '{release_version}'"
    )
if meta_version and not re.match(r"^\d+\.\d+\.\d+$", meta_version):
    errors.append(
        f"metadata: version must be MAJOR.MINOR.PATCH, got: '{meta_version}'"
    )

# ── Check channel ─────────────────────────────────────────────────────────────
channel = release.get("channel", "")
if channel not in VALID_CHANNELS:
    errors.append(f"release.json: channel must be one of {VALID_CHANNELS}, got: '{channel}'")

# ── Check build_number is a positive integer ──────────────────────────────────
build_number = release.get("build_number")
if not isinstance(build_number, int) or build_number < 0:
    errors.append(f"release.json: build_number must be a non-negative integer, got: {build_number!r}")

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

print(
    f"Store metadata validation PASSED "
    f"(version={release_version}, build={build_number}, channel={channel}, "
    f"{len(REQUIRED_ASSETS)} assets OK)"
)

