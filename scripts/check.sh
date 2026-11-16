#!/usr/bin/env sh
#
# The repository quality gate. Reports, never rewrites.
#
#   sh scripts/check.sh
#
# Runs rustfmt, clippy and the Rust tests, then filet. It first refuses a local dependency
# (`path =` or a `file://` git source): a tag built that way only builds here.

set -eu

cd "$(git rev-parse --show-toplevel)"

if grep -nE '(^|[{,[:space:]])path[[:space:]]*=|file://' Cargo.toml crates/planche/Cargo.toml Cargo.lock; then
  echo "check: a local dependency only builds on this machine; depend on a published version" >&2
  exit 1
fi

echo "==> crates/planche"
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked

echo "==> filet"
if command -v filet >/dev/null 2>&1; then
  filet check .
else
  echo "check: no 'filet' on PATH, skipping the filet pass (CI still runs it)" >&2
fi
