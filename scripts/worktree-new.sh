#!/usr/bin/env bash
set -euo pipefail

# Creates a git worktree as a sibling directory for parallel devcontainer usage.
# Run from the host (Mac), not inside a container.
#
# Usage: bash scripts/worktree-new.sh <branch-name>
# Result: ../todo-rs-ts--<branch-name>/ with its own .devcontainer/

BRANCH="${1:?Usage: $0 <branch-name>}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO_NAME="$(basename "$REPO_ROOT")"
WORKTREE_DIR="$(dirname "$REPO_ROOT")/${REPO_NAME}--${BRANCH}"

if [ -d "$WORKTREE_DIR" ]; then
    echo "Worktree already exists: $WORKTREE_DIR"
    exit 1
fi

echo "Creating worktree at: $WORKTREE_DIR"
git -C "$REPO_ROOT" worktree add "$WORKTREE_DIR" -b "$BRANCH"

echo ""
echo "Done. Open in Zed to start a devcontainer:"
echo "  zed $WORKTREE_DIR"
