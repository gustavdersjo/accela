#!/bin/bash

set -eu

#exec "$@"
cd /app/src/
cargo build --release --package "${TARGET_PKG}" --bin "${TARGET_BIN}"
