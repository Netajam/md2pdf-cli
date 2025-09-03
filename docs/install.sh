#!/bin/sh
# install.sh
#
# This script downloads and installs the latest md2pdf-cli binary for Linux and macOS.

set -e

# --- Configuration ---
GITHUB_REPO="Netajam/md2pdf-cli"
CMD_NAME="md2pdf-cli"
INSTALL_DIR="${HOME}/.local/bin"

# --- Helper Functions ---
echo_green() { printf "\033[0;32m%s\033[0m\n" "$1"; }
echo_error() { printf "\033[0;31mError: %s\033[0m\n" "$1" >&2; exit 1; }

# --- Main Logic ---
os=$(uname -s | tr '[:upper:]' '[:lower:]')
arch=$(uname -m)

case "$os" in
  linux)
    case "$arch" in x86_64) ;; *) echo_error "Unsupported architecture: $arch" ;; esac
    asset_os="linux"
    ;;
  darwin)
    case "$arch" in x86_64 | arm64) ;; *) echo_error "Unsupported architecture: $arch" ;; esac
    asset_os="macos"
    ;;
  *) echo_error "Unsupported OS: $os" ;;
esac

echo "Fetching latest version of ${CMD_NAME}..."
latest_tag=$(curl -s "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$latest_tag" ]; then
  echo_error "Could not fetch the latest release tag. Check repository name and ensure a release exists."
fi
echo_green "Latest version is ${latest_tag}"

asset_name="${CMD_NAME}-${asset_os}-x86_64.tar.gz"
download_url="https://github.com/${GITHUB_REPO}/releases/download/${latest_tag}/${asset_name}"

echo "Downloading from ${download_url}..."
temp_file=$(mktemp)
curl -L --progress-bar -o "$temp_file" "$download_url"

echo "Installing to ${INSTALL_DIR}..."
mkdir -p "$INSTALL_DIR"

temp_extract_dir=$(mktemp -d)
tar -xzf "$temp_file" -C "$temp_extract_dir"
mv "${temp_extract_dir}/${CMD_NAME}" "${INSTALL_DIR}/"
rm "$temp_file"
rm -rf "$temp_extract_dir"

echo "Setting execute permission..."
chmod +x "${INSTALL_DIR}/${CMD_NAME}"

if ! command -v "${CMD_NAME}" >/dev/null; then
    echo "Warning: '${INSTALL_DIR}' is not in your PATH."
    echo "Please add 'export PATH=\"${INSTALL_DIR}:\$PATH\"' to your shell's startup file (e.g., ~/.bashrc)."
fi

echo_green "${CMD_NAME} installed successfully!"
"${INSTALL_DIR}/${CMD_NAME}" --version