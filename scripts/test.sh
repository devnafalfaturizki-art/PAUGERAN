#!/usr/bin/env bash
set -euo pipefail
pnpm --filter @paugeran/web typecheck
cargo test --workspace
