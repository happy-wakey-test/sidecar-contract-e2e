#!/usr/bin/env bash
set -euo pipefail

revision="6bee12449fb421b142d3c5836bbc0547f805462a"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

git clone --quiet https://github.com/happy-wakey/happy-wakey-sidecar.rs "$tmp/source"
git -C "$tmp/source" checkout --quiet "$revision"
cargo install --quiet --locked --path "$tmp/source" --root "$tmp/install"

export HAPPY_WAKEY_INSTALLED_BIN="$tmp/install/bin/happy-wakey-sidecar"
cargo test --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
