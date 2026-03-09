#!/usr/bin/env bash
set -euo pipefail

# Lists all active worktrees.
# Run from the host (Mac), not inside a container.
#
# Usage: bash scripts/worktree-ls.sh

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
git -C "$REPO_ROOT" worktree list
