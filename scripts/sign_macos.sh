#!/usr/bin/env bash
# scripts/sign_macos.sh
# Idempotent macOS code-signing script.
# Uses a temporary keychain so credentials are never written permanently.
#
# Required environment variables:
#   APPLE_CERT_BASE64    - base64-encoded Developer ID certificate (p12)
#   APPLE_CERT_PASSWORD  - certificate password
#   APPLE_TEAM_ID        - Apple Developer Team ID
set -euo pipefail

APP_NAME="FreePDFEditor"
DIST_DIR="dist/macos"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"
KEYCHAIN_NAME="ci-build-$(date +%s)"
KEYCHAIN_PATH="$TMPDIR/$KEYCHAIN_NAME.keychain-db"
CERT_PATH="$TMPDIR/apple_cert.p12"

cleanup() {
  echo "==> Cleaning up temporary keychain and certificate"
  security delete-keychain "$KEYCHAIN_PATH" 2>/dev/null || true
  rm -f "$CERT_PATH"
}
trap cleanup EXIT

echo "==> Setting up temporary keychain"
KEYCHAIN_PASS="$(openssl rand -hex 16)"
security create-keychain -p "$KEYCHAIN_PASS" "$KEYCHAIN_PATH"
security set-keychain-settings -lut 3600 "$KEYCHAIN_PATH"
security unlock-keychain -p "$KEYCHAIN_PASS" "$KEYCHAIN_PATH"

echo "==> Importing certificate"
echo "${APPLE_CERT_BASE64}" | base64 --decode > "$CERT_PATH"
security import "$CERT_PATH" -k "$KEYCHAIN_PATH" \
  -P "${APPLE_CERT_PASSWORD}" \
  -T /usr/bin/codesign -T /usr/bin/productbuild
security set-key-partition-list \
  -S apple-tool:,apple:,codesign: \
  -s -k "$KEYCHAIN_PASS" "$KEYCHAIN_PATH"

# Add to search list
PREV_KEYCHAINS=$(security list-keychains -d user | tr -d '"' | tr '\n' ' ')
security list-keychains -d user -s "$KEYCHAIN_PATH" $PREV_KEYCHAINS

echo "==> Code-signing $APP_BUNDLE"
codesign \
  --deep \
  --force \
  --options runtime \
  --entitlements "platform/macos/entitlements.plist" \
  --sign "Developer ID Application: ${APPLE_TEAM_ID}" \
  --timestamp \
  "$APP_BUNDLE"

echo "==> Verifying signature"
codesign --verify --deep --strict --verbose=2 "$APP_BUNDLE"

echo "==> Code-signing complete"
