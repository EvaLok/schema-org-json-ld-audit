#!/usr/bin/env bash
# tools/session-init.sh — Gather and display session metadata safely
#
# Usage:
#   bash tools/session-init.sh              # Print all session info
#   bash tools/session-init.sh --json       # Output as JSON
#   bash tools/session-init.sh --run-id     # Print just the run ID
#   bash tools/session-init.sh --help
#
# Safely reads GitHub Actions environment variables without triggering
# permission issues from shell variable expansion in the orchestrator.
#
# Requires: gh (for auth check), jq (for JSON mode)

set -euo pipefail

REPO="EvaLok/schema-org-json-ld-audit"

show_help() {
	echo "tools/session-init.sh — Display session metadata"
	echo ""
	echo "Usage:"
	echo "  bash tools/session-init.sh              Print all session info"
	echo "  bash tools/session-init.sh --json       Output as JSON"
	echo "  bash tools/session-init.sh --run-id     Print just the GitHub Actions run ID"
	echo "  bash tools/session-init.sh --timestamp  Print just the UTC timestamp"
	echo "  bash tools/session-init.sh --help       Show this help"
}

get_run_id() {
	if [ -n "${GITHUB_RUN_ID:-}" ]; then
		echo "$GITHUB_RUN_ID"
	else
		echo "unknown"
	fi
}

get_timestamp() {
	date -u '+%Y-%m-%d %H:%M:%S UTC'
}

get_run_url() {
	local run_id
	run_id=$(get_run_id)
	if [ "$run_id" != "unknown" ]; then
		echo "https://github.com/${REPO}/actions/runs/${run_id}"
	else
		echo "unknown"
	fi
}

get_gh_user() {
	gh auth status 2>&1 | grep "Logged in" | sed 's/.*account \(.*\) (.*/\1/' || echo "unknown"
}

case "${1:-}" in
	--help|-h)
		show_help
		;;
	--run-id)
		get_run_id
		;;
	--timestamp)
		get_timestamp
		;;
	--json)
		jq -n \
			--arg run_id "$(get_run_id)" \
			--arg timestamp "$(get_timestamp)" \
			--arg run_url "$(get_run_url)" \
			--arg gh_user "$(get_gh_user)" \
			--arg model "Claude Opus 4.6" \
			'{run_id: $run_id, timestamp: $timestamp, run_url: $run_url, gh_user: $gh_user, model: $model}'
		;;
	"")
		echo "Session Info:"
		echo "  Model:     Claude Opus 4.6"
		echo "  Run ID:    $(get_run_id)"
		echo "  Timestamp: $(get_timestamp)"
		echo "  Run URL:   $(get_run_url)"
		echo "  GH User:   $(get_gh_user)"
		;;
	*)
		echo "Unknown option: $1" >&2
		show_help >&2
		exit 1
		;;
esac
