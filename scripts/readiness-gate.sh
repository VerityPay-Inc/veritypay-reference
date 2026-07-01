#!/usr/bin/env bash
# Readiness gate for veritypay-reference.
# Run from the repository root or any directory; resolves paths relative to this script.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> cargo fmt --check"
cargo fmt --check

echo "==> cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> cargo test --workspace"
cargo test --workspace

echo "==> cargo run -p vp-reference-cli --bin vp-reference"
cargo run -p vp-reference-cli --bin vp-reference

SPEC="${ROOT}/../veritypay-spec"
if [[ -d "$SPEC" ]]; then
    echo "==> cargo run -p vp-reference-cli --bin vp-reference -- load-spec --spec ${SPEC}"
    cargo run -p vp-reference-cli --bin vp-reference -- load-spec --spec "$SPEC"
else
    echo "==> skipping load-spec: ${SPEC} not found"
    echo "    clone veritypay-spec alongside this repository to run end-to-end spec loading"
fi

echo "readiness gate passed"
