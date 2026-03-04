#!/usr/bin/env bash
# scripts/build_macos.sh
# Idempotent macOS universal binary build script.
# Produces dist/macos/FreePDFEditor.app and a merged universal binary.
#
# Version/channel/build_number are read exclusively from release/release.json.
# CI pre-populates that file via scripts/generate_release_json.py.
#
# Optional:
#   SKIP_SIGNING - set to "1" to skip signing/notarization steps
set -euo pipefail

APP_NAME="FreePDFEditor"
BINARY_NAME="pdf-editor"

# ── Read from release/release.json ────────────────────────────────────────────
APP_VERSION="$(python3 -c "import json; d=json.load(open('release/release.json')); print(d['version'])")"
APP_CHANNEL="$(python3 -c "import json; d=json.load(open('release/release.json')); print(d['channel'])")"
APP_BUILD_NUMBER="$(python3 -c "import json; d=json.load(open('release/release.json')); print(d['build_number'])")"

export APP_VERSION APP_CHANNEL APP_BUILD_NUMBER STORE_BUILD=1

DIST_DIR="dist/macos"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"
CONTENTS="$APP_BUNDLE/Contents"

echo "==> Building $APP_NAME $APP_VERSION (build $APP_BUILD_NUMBER, channel $APP_CHANNEL, universal binary)"

# ── 1. Build for both architectures ──────────────────────────────────────────
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# ── 2. Merge into universal binary ───────────────────────────────────────────
mkdir -p "$DIST_DIR"
lipo -create \
  "target/aarch64-apple-darwin/release/$BINARY_NAME" \
  "target/x86_64-apple-darwin/release/$BINARY_NAME" \
  -output "$DIST_DIR/$BINARY_NAME-universal"

echo "==> Universal binary created: $DIST_DIR/$BINARY_NAME-universal"

# ── 3. Assemble .app bundle ───────────────────────────────────────────────────
rm -rf "$APP_BUNDLE"
mkdir -p "$CONTENTS/MacOS" "$CONTENTS/Resources"

cp "$DIST_DIR/$BINARY_NAME-universal" "$CONTENTS/MacOS/$APP_NAME"
chmod +x "$CONTENTS/MacOS/$APP_NAME"

# Generate Info.plist from template
SHORT_VERSION="$(echo "$APP_VERSION" | cut -d. -f1-3)"
sed \
  -e "s/__VERSION__/$APP_VERSION/g" \
  -e "s/__SHORT_VERSION__/$SHORT_VERSION/g" \
  "platform/macos/Info.plist.template" > "$CONTENTS/Info.plist"

# Copy icons if available
if [ -f "assets/icon-150.png" ]; then
  cp "assets/icon-150.png" "$CONTENTS/Resources/AppIcon.png"
fi

echo "==> .app bundle assembled: $APP_BUNDLE"

# ── 4. Sign (delegated to sign_macos.sh) ─────────────────────────────────────
if [ "${SKIP_SIGNING:-0}" != "1" ]; then
  bash scripts/sign_macos.sh
fi
