#!/usr/bin/env python3
"""scripts/generate_release_json.py
Generates release/release.json from a git tag and CI run number.

Usage:
    python3 scripts/generate_release_json.py <tag> <ci_run_number>

Example:
    python3 scripts/generate_release_json.py v2.1.0-beta 42

Outputs release/release.json and prints the computed values to stdout.

Channel detection:
    v2.1.0        -> stable
    v2.1.0-beta   -> beta
    v2.1.0-alpha  -> alpha
"""
import json
import os
import re
import sys
from datetime import datetime, timezone

# ── Parse arguments ───────────────────────────────────────────────────────────
if len(sys.argv) < 3:
    print(f"Usage: {sys.argv[0]} <tag> <ci_run_number>", file=sys.stderr)
    sys.exit(1)

raw_tag = sys.argv[1].lstrip("v")
ci_run_number = int(sys.argv[2])

# ── Detect channel from tag suffix ───────────────────────────────────────────
CHANNEL_SUFFIXES = {
    "-alpha": "alpha",
    "-beta":  "beta",
}
channel = "stable"
version_core = raw_tag
for suffix, ch in CHANNEL_SUFFIXES.items():
    if raw_tag.endswith(suffix):
        channel = ch
        version_core = raw_tag[: -len(suffix)]
        break

# ── Validate semver ───────────────────────────────────────────────────────────
m = re.match(r"^(\d+)\.(\d+)\.(\d+)$", version_core)
if not m:
    print(f"ERROR: cannot parse semver from tag '{raw_tag}' (core='{version_core}')", file=sys.stderr)
    sys.exit(1)

major, minor, patch = int(m.group(1)), int(m.group(2)), int(m.group(3))

# ── Build number: MAJOR*1000 + MINOR*100 + PATCH*10 + CI_RUN_NUMBER ──────────
# CI_RUN_NUMBER is taken modulo 10 to fit in the units digit.
# NOTE: this means run numbers that share the same last digit (e.g. 1 and 11)
# will produce the same build_number for an identical tag.  Retrying a failed
# release run is therefore safe only if the version or channel differs.
# For CI tracking purposes the raw run number is also logged to the workflow.
run_digit = ci_run_number % 10
build_number = major * 1000 + minor * 100 + patch * 10 + run_digit

# ── Load existing release.json to preserve unknown fields ────────────────────
release_path = "release/release.json"
existing: dict = {}
if os.path.exists(release_path):
    with open(release_path, encoding="utf-8") as f:
        existing = json.load(f)

# ── Merge ─────────────────────────────────────────────────────────────────────
existing.update(
    {
        "version": version_core,
        "build_number": build_number,
        "channel": channel,
        "release_date": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "min_supported_version": existing.get("min_supported_version", "1.0.0"),
    }
)

os.makedirs(os.path.dirname(release_path), exist_ok=True)
with open(release_path, "w", encoding="utf-8") as f:
    json.dump(existing, f, indent=2)
    f.write("\n")

print(
    f"release.json written: version={version_core}, build={build_number}, "
    f"channel={channel}"
)
