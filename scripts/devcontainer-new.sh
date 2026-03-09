#!/usr/bin/env bash
set -euo pipefail

# Creates a git worktree and opens it in Zed as a dev container.
# Run from the host (Mac), not inside a container.
#
# Usage: bash scripts/devcontainer-new.sh <branch-name>
# Set ZED to use a custom Zed binary (e.g. ZED=/path/to/zed/target/release/zed)

BRANCH="${1:?Usage: $0 <branch-name>}"
SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"

"$SCRIPT_DIR/worktree-new.sh" "$BRANCH"

REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_NAME="$(basename "$REPO_ROOT")"
WORKTREE_DIR="$(dirname "$REPO_ROOT")/${REPO_NAME}--${BRANCH}"

echo "Opening in Zed dev container..."
"${ZED:-zed}" --dev-container "$WORKTREE_DIR"
