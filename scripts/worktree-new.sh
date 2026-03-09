#!/usr/bin/env bash
set -euo pipefail

# Creates a git worktree as a sibling directory for parallel devcontainer usage.
# Run from the host (Mac), not inside a container.
#
# Usage: bash scripts/worktree-new.sh <branch-name>
# Result: ../todo-rs-ts--<branch-name>/ with its own .devcontainer/

BRANCH="${1:?Usage: $0 <branch-name>}"
SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_NAME="$(basename "$REPO_ROOT")"
WORKTREE_DIR="$(dirname "$REPO_ROOT")/${REPO_NAME}--${BRANCH}"

if [ -d "$WORKTREE_DIR" ]; then
    echo "Worktree already exists: $WORKTREE_DIR"
    exit 1
fi

BARE_DIR="$(dirname "$REPO_ROOT")/.bare"
WORKTREE_NAME="$(basename "$WORKTREE_DIR")"

echo "Creating worktree at: $WORKTREE_DIR"
git -C "$REPO_ROOT" worktree add "$WORKTREE_DIR" -b "$BRANCH"

# Rewrite absolute paths to relative so git works inside devcontainers too.
# Worktree .git file: points to .bare/worktrees/<name>
echo "gitdir: ../.bare/worktrees/${WORKTREE_NAME}" > "$WORKTREE_DIR/.git"
# Bare repo gitdir: points back to worktree .git
echo "../../../${WORKTREE_NAME}/.git" > "$BARE_DIR/worktrees/${WORKTREE_NAME}/gitdir"

echo ""
echo "Done. Open in Zed to start a devcontainer:"
echo "  zed $WORKTREE_DIR"
