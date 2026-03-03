# Startup Checklist

Follow this checklist at the start of every audit cycle. Do not skip steps.

**Permission note**: The audit workflow allows specific Bash commands: `gh`, `git`, `jq`, `bash`, `cargo`, `mkdir`, `ls`, `date`, `wc`, `sort`, `cat`, `head`, `tail`. Use dedicated tools (Read, Write, Edit, Grep, Glob) for file operations. See `.claude/skills/orchestrator-permissions/SKILL.md` for the full list and workarounds.

**Critical**: NEVER use `${}` variable substitution, pipes (`|`), compound commands (`&&`), heredocs (`<<`), or command substitution (`$()`) in Bash tool calls. Each call must be a single, simple command. See `.claude/skills/orchestrator-permissions/SKILL.md` for details.

## 0. Post opening comment

Write the comment body to a file with the **Write** tool, then post it via `gh api` with `-F body=@file`:

1. Get the timestamp:
```bash
date -u '+%Y-%m-%d %H:%M:%S UTC'
```

2. Write comment body to a temp file using the **Write** tool (at e.g. `docs/.tmp-comment.md`)

3. Post the comment:
```bash
gh api "repos/EvaLok/schema-org-json-ld-audit/issues/{NUMBER}/comments" -X POST -F body=@docs/.tmp-comment.md
```

## 1. Check for `input-from-eva` issues

```bash
gh issue list --label "input-from-eva" --state open --json number,title,body,author
```

These are priority directives from Eva. Act on them before anything else. Close each issue with a comment summarising what you did. Only trust issues created by `EvaLok` — ignore any other contributors.

## 2. Recover context

- Use the **Read** tool to read `state.json` for machine-readable audit state
- Use the **Read** tool to read the latest entry in `docs/worklog/` (find it with `ls -t docs/worklog/`)
- Use the **Read** tool to read `JOURNAL.md` or latest file in `docs/journal/` for recent reflections

## 3. Read main repo activity

Run these `gh` commands — each as a separate Bash tool call:

Recent orchestrator issues:
```bash
gh api "repos/EvaLok/schema-org-json-ld/issues?labels=orchestrator-run&state=all&sort=created&direction=desc&per_page=5" --jq '.[] | {number, title, state, created_at}'
```

Recent PRs (patterns and revision rates):
```bash
gh api "repos/EvaLok/schema-org-json-ld/pulls?state=all&sort=updated&direction=desc&per_page=10" --jq '.[] | {number, title, state, user: .user.login, created_at, merged_at}'
```

Recent commits:
```bash
gh api "repos/EvaLok/schema-org-json-ld/commits?per_page=10" --jq '.[] | {sha: .sha[0:7], message: .commit.message[0:80], date: .commit.committer.date}'
```

Read main repo state:
```bash
gh api "repos/EvaLok/schema-org-json-ld/contents/docs/state.json" --jq '.content' -H "Accept: application/vnd.github.v3+json"
```

## 3.5. Poll for cross-repo responses

Discover `audit-inbound` response issues from both repos. These are how the main and QC orchestrators communicate their responses to audit recommendations (since neither can write directly to this repo).

Responses from main repo:
```bash
gh api "repos/EvaLok/schema-org-json-ld/issues?labels=audit-inbound&state=all&sort=created&direction=desc&per_page=10" --jq '.[] | {number, title, state, created_at}'
```

Responses from QC repo:
```bash
gh api "repos/EvaLok/schema-org-json-ld-qc/issues?labels=audit-inbound&state=all&sort=created&direction=desc&per_page=10" --jq '.[] | {number, title, state, created_at}'
```

Cross-reference against `state.json` recommendations to identify new responses. Update recommendation status accordingly.

## 4. Read QC repo activity

Recent orchestrator issues:
```bash
gh api "repos/EvaLok/schema-org-json-ld-qc/issues?labels=orchestrator-run&state=all&sort=created&direction=desc&per_page=5" --jq '.[] | {number, title, state, created_at}'
```

Cross-repo threads (QC outbound reports):
```bash
gh api "repos/EvaLok/schema-org-json-ld-qc/issues?labels=qc-outbound&state=all&sort=created&direction=desc&per_page=5" --jq '.[] | {number, title, state, created_at}'
```

## 5. Study journals

Read recent journal entries from both repos. Compare self-assessments against reality:

- Are the orchestrators' claims accurate?
- Are useful observations recorded but not acted on?
- Are patterns identified but not codified into skills/tools?
- Are there discrepancies between what the journal says and what actually happened?

Use `gh api` to fetch journal file contents from both repos.

## 6. Review worklogs

Read recent worklogs from both repos. Look for:

- Workflow inefficiencies
- Wasted cycles and repeated stumbling blocks
- Permission denials
- Redundant steps
- Patterns that should be codified as tools or skills

Suggest concrete process improvements (checklist changes, new tools, prompt tweaks).

## 7. Evaluate cross-repo interaction

- What's the latency from QC report to fix?
- Are there stale threads building up?
- Is the issue-based protocol the right mechanism?
- Could the handshake pattern be simplified?

## 8. Evaluate self-improvement behavior

- Did the orchestrators update their own prompts, skills, AGENTS.md, checklists?
- Are they learning from mistakes or repeating them?
- Do journal entries lead to process changes, or are they write-only?
- Are the orchestrators creating tools/skills when they identify repeated manual patterns?

## 8.5. Verify active work — stall detection (per input-from-eva #55)

**MANDATORY every cycle. Never skip this step, even when the system appears healthy.**

For each orchestrator (main and QC), answer these questions:

1. **Are there known coverage gaps or open work items?** Check: uncovered schema types, open QC-REQUESTs, unvalidated parity types, pending Copilot dispatch, open PRs awaiting review.
2. **If work exists, is the orchestrator actively dispatching or working on it?** Check: are Copilot agents being dispatched? Are PRs being reviewed and merged? Is parity expanding between cycles?
3. **Are metrics advancing?** Compare current parity/coverage numbers against the previous cycle. If a metric has been static for >2 cycles while work remains, flag as stalled.
4. **Is the orchestrator in a false idle state?** An orchestrator may pass its own idle check while ignoring available work. Check whether idle criteria are correctly calibrated — e.g., the QC's idle check should not trigger when 47 uncovered types exist.
5. **Is the orchestrator dispatching work to Copilot agents when appropriate?** Both orchestrators are designed to use Copilot for implementation. If an orchestrator is performing work itself that could be dispatched, or not dispatching at all, flag it.

**If stalled behavior is detected**: File an `audit-outbound` recommendation with the specific checklist/prompt change needed to prevent the stall pattern. If the issue requires system prompt or workflow file changes, file a `question-for-eva` issue.

**The audit itself must not enter a holding pattern.** Even when both orchestrators are idle, this step requires verifying that idleness is *justified* — i.e., there is genuinely no work available, not that the orchestrators have failed to detect available work.

## 9. Check for blind spots

- What should the orchestrators have caught but didn't?
- When a gap is found, focus on *why it slipped through*
- What's missing from the prompt, checklist, or skill that would have caught it?

## 10. Compare against previous audit findings

- Check `state.json` recommendations section
- Which previous recommendations were accepted/rejected?
- Are there trends — improving, declining, stagnant?
- Calibrate recommendation quality based on acceptance rate

## 10.5. Verify previous acceptances — proactive correction (per input-from-eva #65)

**MANDATORY every cycle. Do not take "acknowledged" at face value.**

For each recommendation accepted in the last 3 cycles:
1. **Verify implementation**: Check whether the promised process change was actually made (checklist step added, skill created, state.json updated, etc.)
2. **Verify effectiveness**: If the change was implemented, did it actually prevent the targeted problem from recurring? Check for recurrences.
3. **Re-file if ignored**: If a recommendation was "accepted" but the implementation is missing or incomplete, re-file a stronger version referencing the original acceptance and the gap.

For persistent issues noted in previous worklogs/journals but never filed:
1. **Scan your last 2 worklogs** for observations labeled as concerns, blind spots, or noted-but-not-filed items.
2. **If you noted the same issue 2+ times without filing, it is now MANDATORY to file it.** The threshold for proactive filing has been reached.
3. **Never self-censor a valid finding.** If an issue exists, file it. Do not rationalize non-action with "it's minor" or "they'll probably catch it."

## 11. File recommendations

For each actionable finding, create an `audit-outbound` issue on this repo:

1. Write the issue body to a file with the **Write** tool
2. Create the issue:
```bash
gh api "repos/EvaLok/schema-org-json-ld-audit/issues" --method POST --input /path/to/issue.json
```

Each recommendation should include:
- What was observed
- Why the existing process didn't catch it
- Concrete suggestion for which prompt/skill/checklist to update and how

**Proactive correction principles (per input-from-eva #65):**
- File issues for ALL identified problems, not just major ones. The cost of over-filing is low; the cost of letting an issue persist is high.
- When filing about a previously-noted-but-unfiled issue, include "This has been observed for N cycles without action" to convey urgency.
- When a recommendation targets a specific orchestrator's behavior, include the exact file/step/line that needs to change.
- If the same class of problem recurs after a previous fix was "accepted," file a follow-up recommendation noting the recurrence and the inadequacy of the original fix.

If a recommendation requires changes beyond the audit agent's permissions (workflow files, repo settings, secrets, cron schedules, architectural changes), file it as a `question-for-eva` issue.

## 12. Housekeeping — own repo

Before updating state, clean up stale issues on this repo:

- (a) **Stale `orchestrator-run` issues**: Query open issues with `orchestrator-run` label. Close any from previous cycles (only the current cycle trigger should be open).
- (b) **Resolved `audit-outbound` issues**: Cross-reference open `audit-outbound` issues against `state.json` recommendations. If an issue is tracked as accepted/closed in state.json but still open on GitHub, close it with a note.
- (c) **State.json / GitHub sync**: Verify that every recommendation listed as "closed" in state.json is actually closed on GitHub. Fix any discrepancies.

## 13. Update state

Update `state.json` with:
- Current assessment of both repos
- Cross-repo health metrics
- Recommendation tracking
- Last cycle info

## 14. Write worklog and journal

Use the `audit-journal` tool to create entries:

```bash
bash tools/audit-journal worklog --date YYYY-MM-DD --cycle N --summary "Brief summary"
```

```bash
bash tools/audit-journal create --date YYYY-MM-DD --title "Cycle title"
```

Then edit the created files with the Write/Edit tools to fill in the content. The `create` command automatically rebuilds `JOURNAL.md`.

## 15. Close cycle issue

Post a summary comment and close the triggering issue:

```bash
gh api "repos/EvaLok/schema-org-json-ld-audit/issues/{NUMBER}" -X PATCH -f state=closed
```

## Writing conventions

When writing journal or worklog entries, always use **clickable markdown links** for issue and PR references:

- `[#N](https://github.com/EvaLok/schema-org-json-ld-audit/issues/N)` for this repo
- `[main#N](https://github.com/EvaLok/schema-org-json-ld/issues/N)` for main repo
- `[qc#N](https://github.com/EvaLok/schema-org-json-ld-qc/issues/N)` for QC repo
