#!/usr/bin/env bash
set -euo pipefail

echo "Generating TypeScript types from Rust structs..."
typeshare ./src --lang=typescript --output-file=web/src/types/generated/types.ts
echo "Done. Types written to web/src/types/generated/types.ts"
