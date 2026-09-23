#!/bin/sh
# install.sh — Universal installer for docgov
# Usage: curl -fsSL https://raw.githubusercontent.com/ming/docs-governance/main/install.sh | sh
set -e

REPO="ming/docs-governance"
BIN_NAME="docgov"

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     OS="unknown-linux-musl" ;;
    Darwin*)    OS="apple-darwin" ;;
    *)          echo "Unsupported OS: $OS" >&2; exit 1 ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64|amd64)   ARCH="x86_64" ;;
    aarch64|arm64)  ARCH="aarch64" ;;
    *)              echo "Unsupported Architecture: $ARCH" >&2; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"

# Determine version (latest if not specified)
VERSION="${DOCGOV_VERSION:-latest}"

if [ "$VERSION" = "latest" ]; then
    RELEASE_URL="https://github.com/${REPO}/releases/latest/download/docgov-${TARGET}.tar.gz"
else
    RELEASE_URL="https://github.com/${REPO}/releases/download/${VERSION}/docgov-${TARGET}.tar.gz"
fi

# Determine install directory
if [ -n "$DOCGOV_INSTALL_DIR" ]; then
    INSTALL_DIR="$DOCGOV_INSTALL_DIR"
elif [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -w "$HOME/.local/bin" ] || mkdir -p "$HOME/.local/bin" 2>/dev/null; then
    INSTALL_DIR="$HOME/.local/bin"
else
    INSTALL_DIR="/usr/local/bin"
fi

echo "==> Installing ${BIN_NAME} (${TARGET}) to ${INSTALL_DIR}..."

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

ARCHIVE="${TMP_DIR}/docgov.tar.gz"

if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$RELEASE_URL" -o "$ARCHIVE"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "$ARCHIVE" "$RELEASE_URL"
else
    echo "Error: curl or wget is required to download docgov" >&2
    exit 1
fi

tar -xzf "$ARCHIVE" -C "$TMP_DIR"

if [ -w "$INSTALL_DIR" ]; then
    mv "${TMP_DIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
    chmod +x "${INSTALL_DIR}/${BIN_NAME}"
else
    echo "==> Elevated permissions required to install to ${INSTALL_DIR}"
    sudo mv "${TMP_DIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
    sudo chmod +x "${INSTALL_DIR}/${BIN_NAME}"
fi

echo "==> Successfully installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

# PATH warning if installed in ~/.local/bin and not in PATH
case ":$PATH:" in
    *:"$INSTALL_DIR":*) ;;
    *)
        echo ""
        echo "WARNING: ${INSTALL_DIR} is not in your \$PATH."
        echo "Add it by running:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
        ;;
esac

echo "Run '${BIN_NAME} --help' to get started."
