#!/usr/bin/env bash
set -euo pipefail
cargo clippy --workspace --all-targets -- -D warnings
pnpm --filter @paugeran/web typecheck
