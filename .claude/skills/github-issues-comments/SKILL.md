---
name: github-issues-comments
description: How to create well-formatted GitHub issues and comments without shell escaping problems. Use when creating issues, editing issue bodies, commenting on issues/PRs, or creating pull requests via the gh CLI.
---

# Creating GitHub Issues and Comments

How to create well-formatted GitHub issues and comments without shell escaping problems.

## The core problem

When creating issues or comments via `gh issue create` with inline `--body` text, shell escaping mangles markdown formatting — backticks get escaped as `\``, breaking inline code rendering. This also affects fenced code blocks and any other markdown that uses backticks.

The same applies to `gh issue comment`, `gh pr create`, and any other `gh` subcommand that accepts a `--body` flag.

## Solution: always use --body-file or -F body=@file

Write the body content to a temporary file first, then pass it with `--body-file` or `-F body=@file`. This completely avoids shell escaping.

### Creating an issue

1. Write the body to a file using the **Write** tool (not `cat`, `echo`, or heredocs):

   ```
   Write tool → /tmp/issue-body.md
   ```

   The file should contain raw markdown — backticks, code blocks, links, etc. all written literally with no escaping.

2. Create the issue:

   ```bash
   gh issue create --repo EvaLok/schema-org-json-ld-audit --title "Issue title here" --label "audit-outbound" --body-file /tmp/issue-body.md
   ```

### Alternative: JSON input for issues

Write a JSON payload with the **Write** tool, then use `--input`:

```bash
gh api "repos/EvaLok/schema-org-json-ld-audit/issues" --method POST --input /path/to/issue.json
```

The JSON file should contain `title`, `body`, and `labels` fields.

### Commenting on an issue

1. Write the comment to a file using the **Write** tool.

2. Post it via `gh api`:

   ```bash
   gh api "repos/EvaLok/schema-org-json-ld-audit/issues/NUMBER/comments" -X POST -F body=@/path/to/comment.md
   ```

### Editing an issue body

```bash
gh issue edit <NUMBER> --repo EvaLok/schema-org-json-ld-audit --body-file /tmp/issue-body.md
```

## What NOT to do

These approaches all cause escaping problems:

- **Inline --body with backticks**: `gh issue create --body "Use \`foo\` here"` — backticks get mangled
- **$(cat <<'EOF' ... EOF)**: Heredoc inside command substitution — backticks may be escaped depending on shell context
- **$(cat file)**: Command substitution — still subject to shell processing

## Formatting guidelines

- Use standard GitHub-flavored markdown in the body file
- Backticks for inline code: `` `example` ``
- Triple backticks for code blocks (with language hint)
- Use `**bold**` for emphasis, not ALL CAPS
- Structure with `## Headings` for scannable sections
- Keep issue bodies focused: Observation → Root cause → Recommendation

## Orchestrator note

When running under the orchestrator workflow (prefix-based permission model), the `gh api` approach with `-F body=@file` is the most reliable. See the orchestrator-permissions skill for details.
