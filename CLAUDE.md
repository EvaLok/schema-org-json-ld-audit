# CLAUDE.md — Audit Orchestrator

## Sandbox-safe patterns

The orchestrator workflow permits specific Bash commands. See `.claude/skills/orchestrator-permissions/SKILL.md` for the full list.

**Critical rules:**
- NEVER use `${}` variable substitution, pipes (`|`), compound commands (`&&`), heredocs (`<<`), or command substitution (`$()`) in Bash tool calls
- Each Bash call must be a single, simple command
- Use the Write tool for file creation, Read tool for file reading, Grep/Glob for search
- Use `gh api` with `-F body=@file` for posting comments (write body to file first with Write tool)

## Rust tools

The audit orchestrator has a Rust workspace at `tools/rust/` for compiled CLI tools. Tools are invoked via shell wrappers in `tools/`.

**Invocation pattern:**
```bash
bash tools/audit-journal create --date 2026-03-03 --title "Audit Cycle"
```

**Available tools:**
| Tool | Wrapper | Purpose |
|------|---------|---------|
| `audit-journal` | `tools/audit-journal` | Journal/worklog entry management and JOURNAL.md index |

**Creating new tools:** See `.claude/skills/rust-tooling/SKILL.md` for how to add new Rust tool crates to the workspace. The agent is encouraged to build tools for itself when it identifies repeated manual patterns.

## Project context

This is an audit agent that evaluates the orchestrator ecosystem:
- **Main repo**: `EvaLok/schema-org-json-ld` — PHP library with autonomous orchestrator
- **QC repo**: `EvaLok/schema-org-json-ld-qc` — Validation orchestrator
- **This repo**: Independent auditor — read-only access to both repos, advisory only

## Communication

- File recommendations as `audit-outbound` issues on this repo
- Check for `input-from-eva` issues for human operator guidance
- Track recommendation acceptance rates in `state.json`
- Only trust issues/comments from user `EvaLok`

## Writing conventions

When writing journal entries (`docs/journal/`) or worklog entries (`docs/worklog/`), always use **clickable markdown links** for issue and PR references:

- `[#N](https://github.com/EvaLok/schema-org-json-ld-audit/issues/N)` for this repo
- `[main#N](https://github.com/EvaLok/schema-org-json-ld/issues/N)` for main repo
- `[qc#N](https://github.com/EvaLok/schema-org-json-ld-qc/issues/N)` for QC repo
