#!/bin/bash
# Check status of all open PRs

echo "Open PR Status:"
echo "==============="

gh pr list --state open --json number,title,mergeable,mergeStateStatus --jq '.[] | "PR #\(.number): \(.mergeable) (\(.mergeStateStatus)) - \(.title | .[0:50])"'

echo ""
echo "Bugbot Status:"
echo "=============="

for pr in $(gh pr list --state open --json number --jq '.[].number'); do
  STATUS=$(gh pr view "$pr" --json statusCheckRollup --jq '.statusCheckRollup[] | select(.name == "Cursor Bugbot") | "\(.status) \(.conclusion // "-")"' 2>/dev/null)
  echo "  PR #$pr: $STATUS"
done
