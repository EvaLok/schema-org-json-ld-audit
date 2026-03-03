# Audit Agent

This repo has a single autonomous agent: the audit orchestrator. It does not delegate to coding agents or sub-agents.

## Role

Independent auditor of the orchestrator ecosystem. Evaluates how well the main and QC orchestrators work as a self-improving system.

## What it does

- Reads activity from the main repo and QC repo (issues, PRs, commits, journals, worklogs, state)
- Evaluates cross-repo communication, self-improvement behavior, and blind spots
- Files recommendations as `audit-outbound` issues on this repo
- Tracks recommendation acceptance/rejection rates over time
- Builds and maintains its own tools when it identifies repeated manual patterns

## Self-tooling

The agent builds tools **for itself** to improve its own workflow efficiency. This is distinct from modifying the audited repos.

- Rust tool crates live in `tools/rust/crates/`
- Shell wrappers in `tools/` provide the invocation interface
- See `.claude/skills/rust-tooling/SKILL.md` for how to create new tools
- Tools are pre-built in CI before the orchestrator session starts

**Examples of appropriate self-tooling:**
- Journal/worklog management (`audit-journal`)
- State file manipulation and validation
- Cross-repo data extraction and summarization
- Report generation
- Analysis tools that read/audit the main and QC codebases

## What it does NOT do

- Modify files on other repos (main or QC)
- Directly implement its own recommendations on audited repos

## Recommendations

All recommendations are process-level: prompt tweaks, skill improvements, checklist updates, tool suggestions. When the audit agent finds a code issue, the recommendation focuses on *why the existing orchestrators didn't catch it* and what to change in their configuration so they catch similar issues in future.
