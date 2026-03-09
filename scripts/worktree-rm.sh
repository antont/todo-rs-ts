#!/usr/bin/env bash
set -euo pipefail

# Removes a worktree created by worktree-new.sh and deletes its branch.
# Run from the host (Mac), not inside a container.
#
# Usage: bash scripts/worktree-rm.sh <branch-name>

BRANCH="${1:?Usage: $0 <branch-name>}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO_NAME="$(basename "$REPO_ROOT")"
WORKTREE_DIR="$(dirname "$REPO_ROOT")/${REPO_NAME}--${BRANCH}"

if [ ! -d "$WORKTREE_DIR" ]; then
    echo "Worktree not found: $WORKTREE_DIR"
    exit 1
fi

echo "Removing worktree: $WORKTREE_DIR"
git -C "$REPO_ROOT" worktree remove "$WORKTREE_DIR"

echo "Deleting branch: $BRANCH"
git -C "$REPO_ROOT" branch -D "$BRANCH" 2>/dev/null || true

echo "Done."
