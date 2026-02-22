#!/bin/bash
set -e

# ── sys-monitor installer ───────────────────────────────────────────────
# Usage: curl -sSL https://raw.githubusercontent.com/YOURUSERNAME/sys-monitor/main/install.sh | bash
# ─────────────────────────────────────────────────────────────────────────

REPO="YOURUSERNAME/sys-monitor"   # ← Replace with your GitHub username
BINARY="sys-monitor"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${CYAN}→${NC} $1"; }
ok()    { echo -e "${GREEN}✓${NC} $1"; }
warn()  { echo -e "${YELLOW}⚠${NC} $1"; }
fail()  { echo -e "${RED}✗${NC} $1"; exit 1; }

echo ""
echo -e "${BOLD}  sys-monitor installer${NC}"
echo -e "  ─────────────────────"
echo ""

# ── Detect OS ────────────────────────────────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)  PLATFORM="darwin"  ;;
    Linux)   PLATFORM="linux"   ;;
    *)       fail "Unsupported OS: $OS (only macOS and Linux are supported)" ;;
esac

case "$ARCH" in
    x86_64|amd64)   ARCH="x86_64"  ;;
    arm64|aarch64)   ARCH="aarch64" ;;
    *)               fail "Unsupported architecture: $ARCH" ;;
esac

info "Detected: ${BOLD}${PLATFORM}-${ARCH}${NC}"

# ── Find latest release ─────────────────────────────────────────────────
info "Fetching latest release..."

LATEST_URL="https://api.github.com/repos/${REPO}/releases/latest"

if command -v curl &> /dev/null; then
    RELEASE_JSON=$(curl -sS "$LATEST_URL")
elif command -v wget &> /dev/null; then
    RELEASE_JSON=$(wget -qO- "$LATEST_URL")
else
    fail "Neither curl nor wget found. Please install one."
fi

# Extract the download URL for the matching binary
ASSET_NAME="${BINARY}-${PLATFORM}-${ARCH}"
DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "\"browser_download_url\": *\"[^\"]*${ASSET_NAME}\"" | head -1 | cut -d'"' -f4)

if [ -z "$DOWNLOAD_URL" ]; then
    fail "Could not find a release binary for ${PLATFORM}-${ARCH}.\n   Check https://github.com/${REPO}/releases for available downloads."
fi

VERSION=$(echo "$RELEASE_JSON" | grep -o '"tag_name": *"[^"]*"' | head -1 | cut -d'"' -f4)
info "Latest version: ${BOLD}${VERSION}${NC}"

# ── Download binary ──────────────────────────────────────────────────────
TMPDIR=$(mktemp -d)
TMPFILE="${TMPDIR}/${BINARY}"

info "Downloading ${ASSET_NAME}..."

if command -v curl &> /dev/null; then
    curl -sSL "$DOWNLOAD_URL" -o "$TMPFILE"
else
    wget -qO "$TMPFILE" "$DOWNLOAD_URL"
fi

chmod +x "$TMPFILE"
ok "Downloaded successfully"

# ── Install ──────────────────────────────────────────────────────────────
info "Installing to ${BOLD}${INSTALL_DIR}/${BINARY}${NC}..."

# Check if we can write to the install dir
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMPFILE" "${INSTALL_DIR}/${BINARY}"
else
    warn "Need sudo to write to ${INSTALL_DIR}"
    sudo mv "$TMPFILE" "${INSTALL_DIR}/${BINARY}"
fi

# Clean up
rm -rf "$TMPDIR"

# ── Verify ───────────────────────────────────────────────────────────────
if command -v "$BINARY" &> /dev/null; then
    ok "Installed! Run it with: ${BOLD}sys-monitor${NC}"
else
    warn "Installed to ${INSTALL_DIR}/${BINARY}, but it's not in your PATH."
    echo ""
    echo "  Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
    echo ""
    echo "  Then restart your terminal or run: source ~/.zshrc"
fi

echo ""
