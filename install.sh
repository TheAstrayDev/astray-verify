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
if curl --fail --location --silent --show-error \
  --connect-timeout 10 --max-time 60 --retry 2 --retry-delay 1 \
  "$url" -o "$tmp"; then
  chmod 755 "$tmp"
  mv "$tmp" "$prefix/astray-verify"
  trap - EXIT INT TERM
  echo "Installed prebuilt Astray Verify to $prefix/astray-verify"
else
  rm -f "$tmp"
  trap - EXIT INT TERM
  echo "A prebuilt binary could not be downloaded within 60 seconds." >&2
  if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust is not installed, so Astray Verify cannot fall back to a source build." >&2
    echo "Install Rust at https://rustup.rs, then run this command again." >&2
    exit 1
  fi
  echo "Falling back to a source build. This can take several minutes the first time…"
  cargo install --git "https://github.com/${repo}.git" --locked --force astray-verify
  prefix="${CARGO_HOME:-$HOME/.cargo}/bin"
fi

echo "Astray Verify is ready: $prefix/astray-verify"
case ":${PATH}:" in
  *":${prefix}:"*) ;;
  *) echo "Add $prefix to PATH, then open a new terminal." ;;
esac
"$prefix/astray-verify" --help >/dev/null
