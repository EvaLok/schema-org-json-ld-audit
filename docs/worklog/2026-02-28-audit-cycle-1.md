# Audit Cycle 1 — 2026-02-28

## Summary

First audit cycle for the schema-org-json-ld ecosystem. Baseline assessment of both orchestrators (main repo at cycle 63, QC repo at session 90). Both repos are in post-v1.0.0 steady state. Filed 4 recommendations.

## Triggering issue

[#1](https://github.com/EvaLok/schema-org-json-ld-audit/issues/1) — Audit Cycle 2026-02-28 11:09 UTC

## Data gathered

### Main repo (`EvaLok/schema-org-json-ld`)
- **Cycle count**: 63 (started Feb 24)
- **Current state**: Post-v1.0.0 steady state since ~cycle 50
- **Schema types**: 28 implemented, 98 classes, 12 enums
- **Test count**: 321 (PHPStan level 9)
- **Copilot dispatch**: 17/18 merged (94.4% success rate)
- **Skills**: 6 created (github-issues-comments, journal-entries, orchestrator-permissions, pr-review-workflow, schema-implementation, writing-skills)
- **Tools**: 9 bash scripts in `tools/` (non-functional in sandbox)
- **Self-improvement**: AGENTS.md last updated Feb 26, skills last created Feb 27, STARTUP_CHECKLIST last updated Feb 26
- **Eva input issues processed**: 20 (all closed)

### QC repo (`EvaLok/schema-org-json-ld-qc`)
- **Session count**: 90 (started Feb 25)
- **Current state**: Steady-state validation since session 87
- **Tests**: 188 unit tests (1133 assertions), 39/39 E2E pass, 0 errors
- **Warnings**: 15 total — all classified as validator false positives
- **Eva input issues processed**: 4 (all closed)

### Cross-repo interaction
- 3 QC outbound reports filed, all resolved
- 8 inbound validation requests, all completed
- Average report-to-fix latency: ~12 hours
- Average acknowledgment latency: ~1 hour
- All cross-repo threads currently closed
- Protocol working effectively

## Findings

### 1. Idle cycle spinning (highest priority)
Both orchestrators are running full cycles that produce no meaningful output. Main repo has 14+ consecutive identical "maintenance cycle" worklogs. QC repo has 4+ identical "steady-state validation" sessions. Each generates commits, issues, and API calls for no reason.

**Root cause**: No steady-state detection in STARTUP_CHECKLIST. Filed as [#2](https://github.com/EvaLok/schema-org-json-ld-audit/issues/2).

### 2. Non-functional tools directory
9 bash scripts in `tools/` cannot be executed by the orchestrator due to sandbox restrictions. Created before permission model was understood.

**Root cause**: No sandbox compatibility check in tool/skill creation process. Filed as [#3](https://github.com/EvaLok/schema-org-json-ld-audit/issues/3).

### 3. False positives not communicated to users
15 validator false positives are documented internally but not mentioned in user-facing documentation (README). Library users will encounter the same warnings.

**Root cause**: No checklist step for ensuring internal knowledge reaches user-facing docs. Filed as [#4](https://github.com/EvaLok/schema-org-json-ld-audit/issues/4).

### 4. Cron frequency in steady state
Both orchestrators run every ~90 minutes, producing ~22 empty cycles/day combined. Question for Eva about reducing frequency or adding maintenance mode.

Filed as [#5](https://github.com/EvaLok/schema-org-json-ld-audit/issues/5) (question-for-eva).

## Positive observations

- **Cross-repo protocol is effective**: Issue-based communication works well with good latency
- **Self-improvement is genuine**: Orchestrators created skills, updated checklists, refined AGENTS.md based on real experience (especially the 40-permission-denial audit in cycle 35)
- **Agent dispatch quality is excellent**: 94.4% Copilot merge rate, only 1 failed session (re-dispatched successfully)
- **Eva engagement is strong**: 20+ input-from-eva issues processed, all actioned promptly
- **Journal quality is good**: The QC repo's 2026-02-27 journal has genuine observations about validator behavior and warning classification
- **Code quality is high**: PHPStan level 9, 321 tests, all 31 Google Rich Results categories covered

## Actions taken

| Action | Result |
|--------|--------|
| Posted opening comment on [#1](https://github.com/EvaLok/schema-org-json-ld-audit/issues/1) | Done |
| Filed [#2](https://github.com/EvaLok/schema-org-json-ld-audit/issues/2) — idle cycle spinning | Open |
| Filed [#3](https://github.com/EvaLok/schema-org-json-ld-audit/issues/3) — non-functional tools | Open |
| Filed [#4](https://github.com/EvaLok/schema-org-json-ld-audit/issues/4) — false positives docs | Open |
| Filed [#5](https://github.com/EvaLok/schema-org-json-ld-audit/issues/5) — cron frequency | Open |
| Updated state.json | Done |
