#!/usr/bin/env bash
# scripts/notarize_macos.sh
# Idempotent macOS notarization script.
# Reads version from release/release.json (populated by generate_release_json.py).
#
# Required environment variables:
#   APPLE_API_KEY_ID      - App Store Connect API key ID
#   APPLE_API_ISSUER_ID   - App Store Connect issuer ID
#   APPLE_API_PRIVATE_KEY - Contents of the .p8 private key file
#   APPLE_TEAM_ID         - Apple Developer Team ID
#
#   APPLE_INSTALLER_CERT_BASE64    - base64-encoded Developer ID Installer cert (p12)
#   APPLE_INSTALLER_CERT_PASSWORD  - Installer certificate password
#
# Optional environment variables:
#   APPLE_INSTALLER_SIGN_IDENTITY  - Full certificate common name for productbuild --sign.
#                                    For direct distribution: "Developer ID Installer: <Name> (<TEAMID>)"
#                                    For Mac App Store: "3rd Party Mac Developer Installer: <Name> (<TEAMID>)"
#                                    Defaults to "Developer ID Installer: ${APPLE_TEAM_ID}" if not set.
set -euo pipefail

APP_NAME="FreePDFEditor"
APP_VERSION="$(python3 -c "import json; d=json.load(open('release/release.json')); print(d['version'])")"
DIST_DIR="dist/macos"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"
PKG_PATH="$DIST_DIR/${APP_NAME}_${APP_VERSION}.pkg"
ZIP_PATH="$TMPDIR/${APP_NAME}_${APP_VERSION}.zip"
KEY_PATH="$TMPDIR/apple_api_key.p8"
INSTALLER_KEYCHAIN_NAME="ci-installer-$(date +%s)"
INSTALLER_KEYCHAIN_PATH="$TMPDIR/$INSTALLER_KEYCHAIN_NAME.keychain-db"
INSTALLER_CERT_PATH="$TMPDIR/apple_installer_cert.p12"

# Installer signing identity — use explicit env var or fall back to team-ID form.
INSTALLER_SIGN_IDENTITY="${APPLE_INSTALLER_SIGN_IDENTITY:-Developer ID Installer: ${APPLE_TEAM_ID}}"

cleanup() {
  echo "==> Cleaning up temporary files"
  rm -f "$KEY_PATH" "$ZIP_PATH" "$INSTALLER_CERT_PATH"
  security delete-keychain "$INSTALLER_KEYCHAIN_PATH" 2>/dev/null || true
}
trap cleanup EXIT

echo "==> Writing API private key"
printf '%s' "${APPLE_API_PRIVATE_KEY}" > "$KEY_PATH"

# ── 0. Import installer signing certificate ───────────────────────────────────
echo "==> Setting up installer signing keychain"
INSTALLER_KEYCHAIN_PASS="$(openssl rand -hex 16)"
security create-keychain -p "$INSTALLER_KEYCHAIN_PASS" "$INSTALLER_KEYCHAIN_PATH"
security set-keychain-settings -lut 3600 "$INSTALLER_KEYCHAIN_PATH"
security unlock-keychain -p "$INSTALLER_KEYCHAIN_PASS" "$INSTALLER_KEYCHAIN_PATH"

echo "==> Importing installer certificate"
printf '%s' "${APPLE_INSTALLER_CERT_BASE64}" | base64 --decode > "$INSTALLER_CERT_PATH"
security import "$INSTALLER_CERT_PATH" -k "$INSTALLER_KEYCHAIN_PATH" \
  -P "${APPLE_INSTALLER_CERT_PASSWORD}" \
  -T /usr/bin/productbuild
security set-key-partition-list \
  -S apple-tool:,apple:,productbuild: \
  -s -k "$INSTALLER_KEYCHAIN_PASS" "$INSTALLER_KEYCHAIN_PATH"

PREV_KEYCHAINS=$(security list-keychains -d user | tr -d '"' | tr '\n' ' ')
security list-keychains -d user -s "$INSTALLER_KEYCHAIN_PATH" $PREV_KEYCHAINS

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
echo "==> Building installer: $PKG_PATH (identity: $INSTALLER_SIGN_IDENTITY)"
productbuild \
  --component "$APP_BUNDLE" /Applications \
  --sign "$INSTALLER_SIGN_IDENTITY" \
  "$PKG_PATH"

echo "==> macOS package ready: $PKG_PATH"
