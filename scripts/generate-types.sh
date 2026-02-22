#!/usr/bin/env bash
set -euo pipefail

echo "Generating TypeScript types from Rust structs..."
TS_RS_EXPORT_DIR=web/src/types/generated cargo test export_bindings --quiet
echo "Done. Types written to web/src/types/generated/"
