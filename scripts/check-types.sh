#!/usr/bin/env bash
set -euo pipefail

GENERATED="web/src/types/generated/types.ts"
TMP_FILE=$(mktemp)
trap 'rm -f "$TMP_FILE"' EXIT

echo "Regenerating types into temp file..."
typeshare ./src --lang=typescript --output-file="$TMP_FILE"

echo "Comparing against $GENERATED..."
if ! diff -q "$TMP_FILE" "$GENERATED" > /dev/null 2>&1; then
  echo "MISMATCH:"
  diff "$TMP_FILE" "$GENERATED" || true
  echo "Types are out of date. Run scripts/generate-types.sh to update."
  exit 1
fi

echo "Types are up to date."

echo "Running TypeScript type check..."
cd web && npx tsc --noEmit
echo "All checks passed."
