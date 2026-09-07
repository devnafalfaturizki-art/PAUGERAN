#!/usr/bin/env bash
set -euo pipefail
pnpm --filter @paugeran/web dev &
exec cargo run --manifest-path apps/server/Cargo.toml
