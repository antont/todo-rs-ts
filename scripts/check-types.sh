#!/usr/bin/env bash
set -euo pipefail

GENERATED_DIR="web/src/types/generated"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Regenerating types into temp directory..."
TS_RS_EXPORT_DIR="$TMP_DIR" cargo test export_bindings --quiet

echo "Comparing against $GENERATED_DIR..."
# Only compare .ts files produced by ts-rs (exclude index.ts barrel)
has_diff=0
for f in "$TMP_DIR"/*.ts; do
  name=$(basename "$f")
  if ! diff -q "$f" "$GENERATED_DIR/$name" > /dev/null 2>&1; then
    echo "MISMATCH: $name"
    diff "$f" "$GENERATED_DIR/$name" || true
    has_diff=1
  fi
done

if [ "$has_diff" -ne 0 ]; then
  echo "Types are out of date. Run scripts/generate-types.sh to update."
  exit 1
fi

echo "Types are up to date."

echo "Running TypeScript type check..."
cd web && npx tsc --noEmit
echo "All checks passed."
