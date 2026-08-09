#!/usr/bin/env bash
set -euo pipefail

OWNER="OliverHeffernan"
REPO="locaLearn"
BIN="loca"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
Darwin) os_part="apple-darwin" ;;
Linux) os_part="unknown-linux-gnu" ;;
*)
  echo "Unsupported OS: $os"
  exit 1
  ;;
esac

case "$arch" in
x86_64 | amd64) arch_part="x86_64" ;;
arm64 | aarch64) arch_part="aarch64" ;;
*)
  echo "Unsupported architecture: $arch"
  exit 1
  ;;
esac

target="${arch_part}-${os_part}"
asset="${BIN}-${target}.tar.gz"
url="https://github.com/${OWNER}/${REPO}/releases/latest/download/${asset}"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

echo "Downloading ${asset}..."
curl -fL "$url" -o "$tmp_dir/$asset"
tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"

bin_path="$(find "$tmp_dir" -type f -name "$BIN" | head -n1 || true)"
if [ -z "${bin_path}" ]; then
  echo "Downloaded archive did not contain ${BIN}"
  exit 1
fi

install_dir="${HOME}/.local/bin"
mkdir -p "$install_dir"
install -m 755 "$bin_path" "$install_dir/$BIN"

echo "Installed ${BIN} to ${install_dir}/${BIN}"
echo "If needed, add this to your shell config:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
