# schema-org-json-ld-audit

Independent auditor of the [schema-org-json-ld](https://github.com/EvaLok/schema-org-json-ld) orchestrator ecosystem.

## What this is

This is not a code quality tool. The [QC repo](https://github.com/EvaLok/schema-org-json-ld-qc) handles output validation. This repo evaluates whether the orchestrators themselves are working well as a self-improving system:

- Are the main and QC orchestrators communicating effectively?
- Are they improving their own processes, or repeating mistakes?
- Are their prompts, skills, and checklists driving the right behaviors?
- What blind spots exist that neither orchestrator has surfaced?

## How it works

An autonomous audit agent runs every 3 hours (staggered from the main and QC orchestrators). Each cycle:

1. Reads recent activity from both the main and QC repos
2. Studies orchestrator journals and worklogs
3. Evaluates cross-repo interaction quality
4. Checks for self-improvement behavior
5. Detects blind spots
6. Files recommendations as `audit-outbound` issues

The agent is **advisory only** — it reads and evaluates, it doesn't write code or open PRs on other repos. Recommendations are process-level changes: prompt tweaks, skill improvements, checklist updates.

## Evaluation dimensions

| Priority | Dimension | Focus |
|----------|-----------|-------|
| 1 | Cross-repo interaction | Main-QC communication efficiency and protocol quality |
| 2 | Self-improvement effectiveness | Are orchestrators updating their own processes? |
| 3 | Journal & worklog review | Comparing self-assessments against reality |
| 4 | Skill & tool effectiveness | Are skills discovered, used, and maintained? |
| 5 | Blind spot detection | What should orchestrators have caught but didn't? |
| 6 | Agent dispatch quality | Are Copilot issue specs clear and effective? |
| 7 | Process efficiency | Are cycles productive or spinning? |
| 8 | Output quality spot-checks | Periodic deep dives into code quality |

## Communication protocol

- `audit-outbound` label on this repo: recommendations filed by the audit agent
- `audit-inbound` label on the main repo: for the main orchestrator to track processed recommendations
- `input-from-eva` label on this repo: human operator guidance

## Repository structure

```
.github/workflows/     # Orchestrator workflow and cron trigger
.claude/skills/        # Agent skills
tools/                 # Shell scripts for safe GitHub/git operations
docs/worklog/          # Per-cycle work logs
state.json             # Machine-readable audit state
STARTUP_CHECKLIST.md   # Per-cycle checklist
JOURNAL.md             # Reflective log
CLAUDE.md              # Sandbox-safe patterns
AGENTS.md              # Agent role description
```

## Related repos

- [schema-org-json-ld](https://github.com/EvaLok/schema-org-json-ld) — Main library (subject of audit)
- [schema-org-json-ld-qc](https://github.com/EvaLok/schema-org-json-ld-qc) — QC validation orchestrator (subject of audit)
