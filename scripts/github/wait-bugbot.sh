#!/bin/bash
# Wait for Cursor Bugbot to complete on a PR and show new comments

set -e

PR_NUMBER=$1
TIMEOUT=${2:-300}  # default 5 minutes

# Auto-detect PR number from current branch if not provided
if [ -z "$PR_NUMBER" ]; then
  PR_NUMBER=$(gh pr view --json number --jq '.number' 2>/dev/null || true)
  if [ -z "$PR_NUMBER" ]; then
    echo "Usage: $0 [PR_NUMBER] [TIMEOUT_SECONDS]"
    echo "Waits for Bugbot to complete and shows new review comments."
    echo "PR_NUMBER is auto-detected from current branch if omitted."
    exit 1
  fi
fi

echo "Waiting for Bugbot on PR #$PR_NUMBER (timeout: ${TIMEOUT}s)..."

# Get the latest commit SHA and timestamp in a single call
COMMIT_INFO=$(gh pr view "$PR_NUMBER" --json commits --jq '.commits[-1] | "\(.oid) \(.committedDate)"')
LATEST_COMMIT=${COMMIT_INFO%% *}
LATEST_COMMIT_TIME=${COMMIT_INFO#* }

echo "Latest commit: ${LATEST_COMMIT:0:7} ($LATEST_COMMIT_TIME)"

# Wait for Bugbot to complete
ELAPSED=0
SEEN_CHECK=false
while true; do
  STATUS=$(gh pr view "$PR_NUMBER" --json statusCheckRollup --jq '.statusCheckRollup[] | select(.name == "Cursor Bugbot") | .status' 2>/dev/null)

  if [ "$STATUS" = "COMPLETED" ]; then
    CONCLUSION=$(gh pr view "$PR_NUMBER" --json statusCheckRollup --jq '.statusCheckRollup[] | select(.name == "Cursor Bugbot") | .conclusion' 2>/dev/null)
    echo ""
    echo "Bugbot completed: $CONCLUSION"
    break
  fi

  if [ -n "$STATUS" ] && [ "$SEEN_CHECK" = false ]; then
    SEEN_CHECK=true
    echo -n " (check is $STATUS)"
  fi

  if [ "$ELAPSED" -ge "$TIMEOUT" ]; then
    echo ""
    if [ -z "$STATUS" ]; then
      echo "Timeout: Bugbot check never appeared after ${TIMEOUT}s."
    else
      echo "Timeout: Bugbot still $STATUS after ${TIMEOUT}s."
    fi
    exit 1
  fi

  echo -n "."
  sleep 5
  ELAPSED=$((ELAPSED + 5))
done

# Fetch review comments and filter for new ones (created after latest commit)
echo ""
echo "Checking for new review comments..."

COMMENTS=$(gh api "repos/{owner}/{repo}/pulls/$PR_NUMBER/comments" \
  --jq ".[] | select(.created_at > \"$LATEST_COMMIT_TIME\")")

if [ -z "$COMMENTS" ]; then
  echo "No new comments found."
else
  echo "$COMMENTS" | jq -r '
    def strip_cursor_fix_links:
      split("\n")
      | map(
          select(
            (test("cursor\\.com/(open|agents)\\?data=") | not)
            and (test("<p><a href=\"https://cursor\\.com/") | not)
            and (test("<picture>|</picture>|<img alt=\"Fix in") | not)
          )
        )
      | join("\n")
      | gsub("\n{3,}"; "\n\n")
      | sub("[\\n\\r\\t ]+$"; "");
    "--- \(.path):\(.original_line // "file") ---\n\(.body | strip_cursor_fix_links)\n"
  '
fi
