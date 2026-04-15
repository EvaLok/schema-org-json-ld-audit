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

**Phase awareness**: The main repo now uses a phased cycle architecture. Check `cycle_phase.phase` in the decoded state.json. Valid phases: `work`, `doc_dispatched`, `doc_review`, `close_out`, `complete`. A cycle in `doc_dispatched` or `doc_review` is waiting for a Copilot documentation agent — this is normal intermediate state, not a stall (see Step 8.5).

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

**Note**: Main repo journals may now be written by a Copilot documentation agent (dispatched during `doc_dispatched` phase) rather than the orchestrator itself. Agent-written entries should be more accurate (derived from committed state), but verify this claim — check whether in-flight counts, self-modification records, and commit receipts actually match reality.

Use `gh api` to fetch journal file contents from both repos.

## 6. Review worklogs

Read recent worklogs from both repos. Look for:

- Workflow inefficiencies
- Wasted cycles and repeated stumbling blocks
- Permission denials
- Redundant steps
- Patterns that should be codified as tools or skills

**Note**: Main repo worklogs may now be agent-generated (same as journals above). Compare quality and accuracy of agent-generated worklogs against earlier orchestrator-written ones. The phased architecture was introduced specifically to fix systematic inaccuracies in self-reporting — evaluate whether it's working.

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

### Main repo phased cycle calibration

The main orchestrator now uses multi-phase cycles that span multiple cron invocations. When evaluating stall behavior, account for normal phase wait times:

- **`doc_dispatched`** (< 2 hours): Normal. A Copilot documentation agent is working. Not a stall.
- **`doc_dispatched`** (> 2 hours): Likely stale. The orchestrator should have fallen back to direct documentation. Flag if it hasn't.
- **`doc_review`** (< 1 cron interval): Normal. The orchestrator is reviewing the doc PR.
- **`doc_review`** with `review_iteration` approaching `review_max` (3): The retry loop may be churning without converging. Check whether the doc agent is actually fixing the flagged issues.
- **`close_out`** (> 1 cron interval): Should complete quickly. Flag if lingering.
- **Any phase persisting across 3+ cron invocations without progress**: Genuine stall. The orchestrator may be stuck in resume mode. Check `cycle_phase.phase_entered_at` age.

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

### 10.5.1. Step-level verification for pipeline-step audit issues (per #383)

When closing or verifying any audit issue that targets a specific orchestrator pipeline step (C4.1, C5.5, current-cycle-steps, doc-validation, receipt-validate, etc.), the following procedure is MANDATORY. **Inferring step status from a different step is fabrication.** Cycle 175 closed [#370](https://github.com/EvaLok/schema-org-json-ld-audit/issues/370) on the basis of "C5.5 PASS → C4.1 PASS" inference, which was wrong (cycles 449/451/453 had C4.1 FAIL while C5.5 PASS).

1. **Step-level verification (mandatory).** For each cycle in the verification window, fetch the cited step's literal comment from the cycle's issue thread:
   ```bash
   gh api "repos/EvaLok/schema-org-json-ld/issues/<N>/comments" --jq "[.[] | select(.body | contains(\"Step C4.1\")) | .body]"
   ```
   Record the literal status text in the audit's worklog. Do NOT infer step status from a different step.

2. **Chronic-category cross-check (mandatory).** Read `docs/state.json::review_agent.chronic_category_responses` from the main repo (via `gh api ... -H "Accept: application/vnd.github.v3.raw" --jq ".review_agent.chronic_category_responses"`). For each entry whose `category` matches a cited finding category:
   - Is the entry's `verification_cycle` older than the latest cycle in the verification window?
   - Is the entry still flagged in the most recent reviews?
   - If either, the audit issue is NOT verified — the chronic category is still producing findings.

3. **Recent-review scan (mandatory).** Fetch the most recent 3 adversarial reviews from `docs/reviews/cycle-<N>.md` (via `gh api "repos/EvaLok/schema-org-json-ld/contents/docs/reviews/cycle-<N>.md" -H "Accept: application/vnd.github.v3.raw"`). If any of them flag the targeted chronic category, the audit issue is NOT verified.

4. **Sample size minimum.** Require at least **5 cycles** in the verification window for chronic-category fixes (categories with `chronic_category_responses` entries). 3 cycles is too few to distinguish a fix from coincidence.

5. **Receipt persistence.** Record the literal step output for each cycle in the audit's worklog (not just "all 5 PASS"). This forces the audit to actually fetch the data and creates an audit trail for retrospective review.

### 10.5.2. Load-bearing modification verification (per cycle 191 audit blind spot)

When verifying any acceptance whose claimed implementation is a change to a non-Rust file (XML, MD, JSON, YAML, or other config/spec), do **not** treat the file change as the verification. The change must also be **read by Rust code at runtime**.

This sub-step exists because cycles 187-190 tracked a four-link sub-categorization adoption chain (audit [#402](https://github.com/EvaLok/schema-org-json-ld-audit/issues/402) → [#406](https://github.com/EvaLok/schema-org-json-ld-audit/issues/406) → [#415](https://github.com/EvaLok/schema-org-json-ld-audit/issues/415) → [#417](https://github.com/EvaLok/schema-org-json-ld-audit/issues/417) → [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420)) without ever asking whether the cycle-492 self-modification of `COMPLETION_CHECKLIST.xml` was actually read by any code path. Eva diagnosed in [main#2519](https://github.com/EvaLok/schema-org-json-ld/issues/2519): the review prompt body is hardcoded in `tools/rust/crates/cycle-runner/src/review_body.rs`; no code reads `COMPLETION_CHECKLIST.xml` at dispatch time. The XML modification was inert; the audit chain was tracking a fix that did nothing.

**Procedure (mandatory when verifying a non-Rust file modification):**

1. **Identify the modified path.** From the acceptance evidence, determine the exact file path that was changed (e.g., `COMPLETION_CHECKLIST.xml`, `docs/review-spec.md`).
2. **Grep Rust source for runtime references.** Use the Grep tool against the main repo (or its mirror via `gh api .../contents/...`) for both literal path matches and known-import patterns:
   ```bash
   gh api "repos/EvaLok/schema-org-json-ld/search/code?q=COMPLETION_CHECKLIST+language:rust" --jq '.items[] | {path, name}'
   ```
   Distinguish between (a) source code that reads/parses the file, (b) comments that mention the file, and (c) doc-lint style references that only check whether the file *exists*.
3. **Verdict:**
   - If at least one Rust file actually reads/parses the modified path at runtime → the modification is load-bearing; verification can proceed normally.
   - If no Rust file reads the path (only comments and existence checks) → the modification is **inert**. The acceptance is **not verified**. Re-file the original recommendation with the load-bearing diagnosis and request a code-side fix that actually consumes the modified content.
4. **Persist the receipt.** Record the grep output (or "no matches") in the audit's worklog for the cycle — this creates an audit trail that the verification was performed.

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
