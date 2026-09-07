#!/usr/bin/env bash
set -euo pipefail
cargo fmt --all
pnpm exec prettier --write 'apps/web/**/*.{ts,tsx,css}' 'packages/**/*.ts'
