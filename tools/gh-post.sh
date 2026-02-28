#!/usr/bin/env bash
# tools/gh-post.sh — Safe GitHub comment/issue posting
#
# Usage:
#   bash tools/gh-post.sh comment <ISSUE_NUMBER> <BODY_FILE>
#   bash tools/gh-post.sh issue <TITLE> <BODY_FILE> [LABEL]
#   bash tools/gh-post.sh close <ISSUE_NUMBER>
#   bash tools/gh-post.sh --help
#
# Posts comments and creates issues using file-based body content
# to avoid shell escaping problems. All content is read from files
# written with the Write tool.
#
# Requires: gh, jq

set -euo pipefail

REPO="EvaLok/schema-org-json-ld-audit"

show_help() {
	echo "tools/gh-post.sh — Safe GitHub comment/issue posting"
	echo ""
	echo "Usage:"
	echo "  bash tools/gh-post.sh comment <ISSUE_NUMBER> <BODY_FILE>"
	echo "  bash tools/gh-post.sh issue <TITLE> <BODY_FILE> [LABEL]"
	echo "  bash tools/gh-post.sh close <ISSUE_NUMBER>"
	echo "  bash tools/gh-post.sh --help"
	echo ""
	echo "BODY_FILE should be written first using the Write tool."
	echo "Default label for issues is 'audit-outbound'."
}

post_comment() {
	local issue_num="$1"
	local body_file="$2"

	if [ ! -f "$body_file" ]; then
		echo "Error: body file not found: $body_file" >&2
		exit 1
	fi

	local result
	result=$(gh api "repos/${REPO}/issues/${issue_num}/comments" -X POST \
		-F body=@"${body_file}")

	local comment_url
	comment_url=$(echo "$result" | jq -r '.html_url')
	echo "Comment posted: ${comment_url}"
}

create_issue() {
	local title="$1"
	local body_file="$2"
	local label="${3:-audit-outbound}"

	if [ ! -f "$body_file" ]; then
		echo "Error: body file not found: $body_file" >&2
		exit 1
	fi

	local result
	result=$(gh api "repos/${REPO}/issues" -X POST \
		--input <(jq -n \
			--arg title "$title" \
			--arg body "$(cat "$body_file")" \
			--arg label "$label" \
			'{"title": $title, "body": $body, "labels": [$label]}'))

	local num url
	num=$(echo "$result" | jq '.number')
	url=$(echo "$result" | jq -r '.html_url')
	echo "Created: Issue #${num}"
	echo "URL: ${url}"
}

close_issue() {
	local issue_num="$1"

	gh api "repos/${REPO}/issues/${issue_num}" -X PATCH -f state=closed
	echo "Issue #${issue_num} closed."
}

case "${1:-}" in
	--help|-h)
		show_help
		;;
	comment)
		if [ -z "${2:-}" ] || [ -z "${3:-}" ]; then
			echo "Error: comment requires ISSUE_NUMBER and BODY_FILE" >&2
			exit 1
		fi
		post_comment "$2" "$3"
		;;
	issue)
		if [ -z "${2:-}" ] || [ -z "${3:-}" ]; then
			echo "Error: issue requires TITLE and BODY_FILE" >&2
			exit 1
		fi
		create_issue "$2" "$3" "${4:-audit-outbound}"
		;;
	close)
		if [ -z "${2:-}" ]; then
			echo "Error: close requires ISSUE_NUMBER" >&2
			exit 1
		fi
		close_issue "$2"
		;;
	"")
		show_help
		;;
	*)
		echo "Unknown command: $1" >&2
		show_help >&2
		exit 1
		;;
esac
