#!/usr/bin/env bash
set -euo pipefail

VERSION="v0.2.0"
BINARY_NAME="dot-setup"
REPO="carlosyslas/dot-files"
TMP_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "Downloading dot-setup $VERSION..."

ARCH=$(uname -m)
if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="x86_64"
elif [[ "$ARCH" == "aarch64" ]]; then
    ARCH="aarch64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$VERSION/dot-setup-$ARCH"
BINARY_PATH="$TMP_DIR/$BINARY_NAME"

curl -sL "$URL" -o "$BINARY_PATH"
chmod +x "$BINARY_PATH"

echo "Running dot-setup..."
exec "$BINARY_PATH"
