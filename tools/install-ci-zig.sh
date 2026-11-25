#!/usr/bin/env bash
set -euo pipefail

version=$(cat .zigversion)
case "$(uname -sm)" in
  'Linux x86_64')
    arch=x86_64-linux
    sha=3581863e3cbcfd0fce0efc682ce7e9e93da96668a977791d356a26d2a7e9b17a ;;
  'Darwin arm64')
    arch=aarch64-macos
    sha=75f1bc7a1e9852f17ec9a0c1895f767c4ec56d46bc8a834920eece1d0359dd8d ;;
  *) echo "Unsupported Zig CI host: $(uname -sm)" >&2; exit 1 ;;
esac

archive="zig-$arch-$version"
cd "$RUNNER_TEMP"

verify_archive() {
  printf '%s  %s\n' "$sha" "$archive.tar.xz" | shasum -a 256 -c
}

if curl --fail --silent --show-error --location --max-time 30 \
    "https://pkg.hexops.org/zig/$archive.tar.xz" -o "$archive.tar.xz" && verify_archive; then
  echo 'Verified Zig download from the Hexops mirror'
else
  echo 'Fetching Zig from ziglang.org'
  curl --fail --silent --show-error --location --retry 3 \
    "https://ziglang.org/download/$version/$archive.tar.xz" -o "$archive.tar.xz"
  verify_archive
fi

tar xf "$archive.tar.xz"
echo "$RUNNER_TEMP/$archive" >> "$GITHUB_PATH"
