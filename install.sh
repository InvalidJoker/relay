#!/usr/bin/env sh
set -e

REPO="InvalidJoker/relay"
BIN_NAME="relay"
INSTALL_DIR="${RELAY_INSTALL_DIR:-/usr/local/bin}"

# Detect OS
OS=$(uname -s)
case "$OS" in
  Linux)  OS="linux" ;;
  Darwin) OS="macos" ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
  x86_64 | amd64) ARCH="x86_64" ;;
  aarch64 | arm64) ARCH="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

ASSET="relay-${OS}-${ARCH}"

# Resolve release tag
if [ -n "$RELAY_VERSION" ]; then
  TAG="$RELAY_VERSION"
  RELEASE_URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"
else
  RELEASE_URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"
fi

echo "Downloading relay CLI for ${OS}/${ARCH}..."
echo "  -> ${RELEASE_URL}"

TMP=$(mktemp)
if command -v curl > /dev/null 2>&1; then
  curl -fsSL "$RELEASE_URL" -o "$TMP"
elif command -v wget > /dev/null 2>&1; then
  wget -qO "$TMP" "$RELEASE_URL"
else
  echo "Error: curl or wget is required."
  exit 1
fi

chmod +x "$TMP"

# Install — try without sudo first, fall back if needed
if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP" "${INSTALL_DIR}/${BIN_NAME}"
else
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo mv "$TMP" "${INSTALL_DIR}/${BIN_NAME}"
fi

echo "relay installed to ${INSTALL_DIR}/${BIN_NAME}"
echo "Run 'relay --help' to get started."
