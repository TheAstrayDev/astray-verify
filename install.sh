#!/usr/bin/env sh
set -eu

repo="TheAstrayDev/astray-verify"
prefix="${ASTRAY_VERIFY_INSTALL_DIR:-$HOME/.local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux) platform="linux" ;;
  Darwin) platform="macos" ;;
  *) echo "Unsupported operating system: $os" >&2; exit 1 ;;
esac

case "$arch" in
  x86_64|amd64) arch="x86_64" ;;
  arm64|aarch64) arch="aarch64" ;;
  *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;
esac

asset="astray-verify-${platform}-${arch}"
url="https://github.com/${repo}/releases/latest/download/${asset}"

mkdir -p "$prefix"
tmp="$(mktemp "${TMPDIR:-/tmp}/astray-verify.XXXXXX")"
trap 'rm -f "$tmp"' EXIT INT TERM

echo "Downloading ${asset}…"
curl -fL --retry 3 --retry-delay 1 "$url" -o "$tmp"
chmod 755 "$tmp"
mv "$tmp" "$prefix/astray-verify"
trap - EXIT INT TERM

echo "Installed Astray Verify to $prefix/astray-verify"
case ":${PATH}:" in
  *":${prefix}:"*) ;;
  *) echo "Add $prefix to PATH, then open a new terminal." ;;
esac
"$prefix/astray-verify" --help >/dev/null
