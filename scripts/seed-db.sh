#!/usr/bin/env bash
set -euo pipefail
DATA_DIR="${DATA_DIR:-./data}" cargo run --manifest-path apps/server/Cargo.toml
