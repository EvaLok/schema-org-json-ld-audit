#!/usr/bin/env bash
# tools/git-commit.sh — Sandbox-safe git commits
#
# Usage:
#   bash tools/git-commit.sh "Commit message" [FILES...]
#   bash tools/git-commit.sh --help
#
# Stages specified files (or all tracked changes), commits with the given
# message, and pushes. Each git operation is a simple command to stay
# compatible with the orchestrator sandbox.
#
# Requires: git

set -euo pipefail

show_help() {
	echo "tools/git-commit.sh — Sandbox-safe git commits"
	echo ""
	echo "Usage:"
	echo "  bash tools/git-commit.sh \"Commit message\" [FILES...]"
	echo "  bash tools/git-commit.sh --help"
	echo ""
	echo "If no FILES are specified, stages state.json and docs/."
	echo "Commits and pushes in separate steps for sandbox compatibility."
}

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
	show_help
	exit 0
fi

if [ -z "${1:-}" ]; then
	echo "Error: commit message required" >&2
	show_help >&2
	exit 1
fi

MESSAGE="$1"
shift

# Stage files
if [ $# -gt 0 ]; then
	git add "$@"
else
	git add state.json docs/ JOURNAL.md
fi

# Check if there's anything to commit
if git diff --cached --quiet; then
	echo "Nothing to commit."
	exit 0
fi

# Commit
git commit -m "$MESSAGE"

# Push
git push

echo "Committed and pushed: $MESSAGE"
