#!/usr/bin/env bash
set -euo pipefail

VERSION="v0.2.1"
BINARY_NAME="dot-setup"
REPO="carlosyslas/dot-files"
TMP_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "Downloading dot-setup $VERSION..."

URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME"
BINARY_PATH="$TMP_DIR/$BINARY_NAME"

if curl -sL --fail "$URL" -o "$BINARY_PATH" 2>/dev/null; then
    chmod +x "$BINARY_PATH"
    echo "Running dot-setup..."
    exec "$BINARY_PATH"
else
    echo "Release not found. Building from source..."
    
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
    if [[ -d "$SCRIPT_DIR/dot-setup" ]]; then
        echo "Building dot-setup..."
        (cd "$SCRIPT_DIR/dot-setup" && cargo build --release)
        BUILT_BINARY="$SCRIPT_DIR/dot-setup/target/release/$BINARY_NAME"
        if [[ -x "$BUILT_BINARY" ]]; then
            echo "Running dot-setup..."
            exec "$BUILT_BINARY"
        fi
    fi
    
    echo "Error: Could not download or build dot-setup"
    echo "Please ensure Rust and cargo are installed: https://rustup.rs/"
    exit 1
fi
