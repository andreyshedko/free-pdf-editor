#!/usr/bin/env bash
# scripts/notarize_macos.sh
# Idempotent macOS notarization script.
# Reads version from release/release.json (populated by generate_release_json.py).
#
# Required environment variables:
#   APPLE_API_KEY_ID    - App Store Connect API key ID
#   APPLE_API_ISSUER_ID - App Store Connect issuer ID
#   APPLE_API_PRIVATE_KEY - Contents of the .p8 private key file
#   APPLE_TEAM_ID       - Apple Developer Team ID
set -euo pipefail

APP_NAME="FreePDFEditor"
APP_VERSION="$(python3 -c "import json; d=json.load(open('release/release.json')); print(d['version'])")"
DIST_DIR="dist/macos"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"
PKG_PATH="$DIST_DIR/${APP_NAME}_${APP_VERSION}.pkg"
ZIP_PATH="$TMPDIR/${APP_NAME}_${APP_VERSION}.zip"
KEY_PATH="$TMPDIR/apple_api_key.p8"

cleanup() {
  echo "==> Cleaning up temporary key file"
  rm -f "$KEY_PATH" "$ZIP_PATH"
}
trap cleanup EXIT

echo "==> Writing API private key"
printf '%s' "${APPLE_API_PRIVATE_KEY}" > "$KEY_PATH"

# ── 1. Zip the app bundle for notarization ────────────────────────────────────
echo "==> Zipping $APP_BUNDLE for notarization"
ditto -c -k --keepParent "$APP_BUNDLE" "$ZIP_PATH"

# ── 2. Submit for notarization ────────────────────────────────────────────────
echo "==> Submitting to Apple notarization service"
xcrun notarytool submit "$ZIP_PATH" \
  --key "$KEY_PATH" \
  --key-id "${APPLE_API_KEY_ID}" \
  --issuer "${APPLE_API_ISSUER_ID}" \
  --team-id "${APPLE_TEAM_ID}" \
  --wait \
  --timeout 600

echo "==> Notarization succeeded"

# ── 3. Staple ticket ─────────────────────────────────────────────────────────
echo "==> Stapling notarization ticket"
xcrun stapler staple "$APP_BUNDLE"

# ── 4. Build installer package ───────────────────────────────────────────────
echo "==> Building installer: $PKG_PATH"
productbuild \
  --component "$APP_BUNDLE" /Applications \
  --sign "Developer ID Installer: ${APPLE_TEAM_ID}" \
  "$PKG_PATH"

echo "==> macOS package ready: $PKG_PATH"
