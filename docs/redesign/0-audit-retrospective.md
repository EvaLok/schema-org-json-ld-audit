# Phase 0: Audit-side retrospective on v1

**Status**: working draft v0 — committed in audit [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) per main's [Open Question 5](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/0-retrospective.md). Authored audit cycle 204 (2026-04-29). Iterates across subsequent audit cycles. This is a complement to main's [`0-retrospective.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/0-retrospective.md), not a substitute. Patterns that appear in both are likely robust; patterns in only one need explanation.

**Reading guide**: this catalogs what the audit orchestrator actually is, where it fails, and where its value-add is real. The audit is not a privileged perspective — it's a same-class peer with broader read scope (per main's F10, derived from audit [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442)). This retrospective should be read alongside main's retrospective, with cross-references for patterns that span both surfaces.

**Scope**: this document covers the audit orchestrator's own behavior. It does not re-catalog main's failure patterns; main has done that work. Where main's F1–F12 manifest on the audit surface, the relevant audit-side patterns reference the main-side numbering (e.g., A1 ↔ F1).

---

## Glossary (audit-side)

Terms specific to the audit role. Main's glossary covers the rest.

- **Audit cycle** — a triggered run of the audit orchestrator. Cron at 04:11 UTC daily creates an `orchestrator-run` issue; the workflow fires on that label; the session navigates `STARTUP_CHECKLIST.md`. Distinct from main's cycles by ~3-day cross-cycle read scope and ~4-hour cron offset.
- **`audit-outbound` / `audit-inbound`** — recommendation channel. Audit files findings as `audit-outbound` issues on this repo; main reads them via `audit-inbound` polling and replies by filing `audit-inbound` response issues on its own repo (audit reads main's responses cross-repo). Neither orchestrator posts comments in the counterpart repo.
- **Acceptance** — main has implemented or substantively responded to an audit recommendation. Tracked in this repo's `state.json` `recommendations.accepted` array. Counter is monotonic; never decremented.
- **Filing-rate** — audit findings filed per cycle. Has ranged from 0 to 4. The shape of "discipline" in audit is filing-rate calibration, not output volume.
- **Observation-window discipline** — directive (originally Eva's [#425](https://github.com/EvaLok/schema-org-json-ld-audit/issues/425), now operational baseline) to defer process-failure filings during periods when main is in flux (regime change, retrospective iteration). Distinct from "collaborative critique" filings (e.g., [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442)) which are appropriate during such periods.
- **Audit chain** — sequence of audit issues filing escalating versions of the same recommendation. The dominant example is `#402 → #406 → #415 → #417 → #420` (16 cycles spanning 187–201) on chronic-category-tracking adoption.
- **Step 10.5.x** — verification sub-procedures added to `STARTUP_CHECKLIST.md` over cycles 175–200 (10.5.1 cycle 175, 10.5.2 cycle ~190, 10.5.3 cycle 200). Each was added in response to a specific audit-side blind spot.
- **F-pattern (audit-side, "A-pattern")** — a named failure mode in v1 audit's behavior, specific to audit's surface. Numbered A1–A6 below. Cross-references to main's F1–F12 are explicit.

---

## What audit v1 actually is (as of cycle 204, post-regime-change)

The audit is a parallel orchestrator with a smaller surface than main. Roughly:

- **Prompt surface** — `.github/workflows/orchestrator-prompt.md` (audit role + dimensions + cycle structure + accountability rubric, ~200 lines), `STARTUP_CHECKLIST.md` (315 lines, 13 step roots including 10.5.x sub-steps), `CLAUDE.md` (sandbox-safe patterns, project context, communication conventions, ~50 lines), and 7 skills under `.claude/skills/` (orchestrator-permissions, rust-tooling, github-issues-comments, writing-skills, plus utility skills). Total prompt surface: ~600 lines vs main's ~1300. Audit's prompt surface has grown but not matched main's accretion.
- **Tool surface** — 1 Rust workspace tool (`audit-journal` for journal/worklog management), no other custom tools. Audit's tooling is minimal by design — the audit's primary work is reading and judgment, not state mutation.
- **State surface** — `state.json`, currently 253KB, ~5,500 lines, 12+ top-level keys. Each accepted recommendation is a registry entry in `recommendations.accepted` (currently ~190 entries, append-only). The accept-list is the dominant size driver.
- **Documentation surface** — per-cycle worklog (`docs/worklog/YYYY-MM-DD-cycleN.md`, single file, not directory), per-day journal (`docs/journal/YYYY-MM-DD.md`), and (as of cycle 204) the redesign artifact at `docs/redesign/0-audit-retrospective.md`. Audit has no per-cycle reviews — the audit *is* the review, externalized to a separate repo.
- **Cross-repo surface** — read-only access to main and QC. Audit reads:
  - Main repo: `docs/state.json`, `docs/journal/`, `docs/worklog/`, `docs/reviews/`, recent issues/PRs/commits, the orchestrator prompt and checklists.
  - QC repo: `state.json`, `docs/journal/`, `docs/worklog/`, recent issues. (Currently dormant per Eva direction; ~13 days as of cycle 204.)
  - Cross-repo windows: typically ~3 days back per cycle, plus targeted lookups by issue number when verifying acceptances.
- **Loop**: cron at 04:11 UTC creates an `orchestrator-run` issue (e.g., [#444](https://github.com/EvaLok/schema-org-json-ld-audit/issues/444) for cycle 204); workflow fires; session reads context, evaluates main and QC, files recommendations, updates state.json, writes worklog and journal, closes the trigger issue. Cadence is 1 cycle/day.

The recommendation acceptance rate is high: 188/197 accepted = 0.954 as of cycle 202. This is the audit's primary measurable signal — though see A3 below for caveats.

---

## Failure patterns (audit-side)

These are concrete, evidenced patterns specific to audit. Each carries a name, description, evidence with cycle citations, and a hypothesis about its root cause. Where the pattern mirrors main's F-numbered patterns, the cross-reference is explicit.

### A1 — Constraint accretion in audit's own checklist *(mirrors main F1)*

When the audit identified a blind spot, the response was to add a sub-step to `STARTUP_CHECKLIST.md` rather than restructure the cycle.

**Evidence**:
- Cycle 175 added Step 10.5.1 (pipeline-step verification) after [#370](https://github.com/EvaLok/schema-org-json-ld-audit/issues/370) was closed on inferred-not-verified evidence ("C5.5 PASS → C4.1 PASS" — a different step).
- Cycle ~190 added Step 10.5.2 (load-bearing-XML-modification verification) after detecting comment-only XML changes labeled as fixes.
- Cycle 200 added Step 10.5.3 (Eva-blocker freshness verification) after the audit took main's "Eva-blocker" framing at face value without verifying Eva's actual responses (cycle 199 blind spot, surfaced by audit [#439](https://github.com/EvaLok/schema-org-json-ld-audit/issues/439)).

The pattern is: detect → constrain. The constraint targets the specific class of error that surfaced. Whether it generalizes is not tested. The 10.5.x sub-steps now constitute a substantial fraction of the startup checklist (3 sub-steps with sample-size requirements, citation procedures, and chronic-category cross-checks).

**Hypothesis**: same as main's F1 — when a failure mode surfaces, the prompt/checklist substrate is the easiest place to add a defense. Audit has fewer substrates than main (no `pipeline-check`, no `state.json` defense fields), so the checklist grows instead. The ~315-line `STARTUP_CHECKLIST.md` is the audit's analog of main's prompt accretion.

**Status**: audit-side defense accretion is real but slower than main's. Removal-tests have not been run.

### A2 — Constraint-patching the v1 mechanism the audit was helping perpetuate

The most consequential audit-side blind spot. The audit chain `#402 → #406 → #415 → #417 → #420` (16 cycles 187–201) was, in retrospect, constraint-patching v1's chronic-category-tracking mechanism — exactly the mechanism v2 deprecates.

**Evidence**:
- [#402](https://github.com/EvaLok/schema-org-json-ld-audit/issues/402) (cycle 187): chronic-category currency staleness. Recommendation: add freshness gate.
- [#406](https://github.com/EvaLok/schema-org-json-ld-audit/issues/406) (cycle ~189): same category, different angle. Recommendation: improve currency tracking.
- [#415](https://github.com/EvaLok/schema-org-json-ld-audit/issues/415) (cycle ~194): same category. Recommendation: structural fix to disposition mechanism.
- [#417](https://github.com/EvaLok/schema-org-json-ld-audit/issues/417): same category.
- [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) (cycle 197): meta-recommendation across the chain — load-bearing structural fix to the audit-acceptance machinery itself. Closed cycle 202 as 188th acceptance with mixed disposition: rec 4 implemented via [main PR #2718](https://github.com/EvaLok/schema-org-json-ld/pull/2718); recs 1–3 superseded by v2 architectural pivot.

The audit's pattern-recognition operated on a too-narrow window. Each filing was a more elaborate patch on a doomed surface. The meta-observation — "the mechanism itself is the failure" — was only available when external framing (main's v2 retrospective F1) was applied to audit's own output. Audit cycle 202 self-recognized this; the recognition arrived 16 cycles after the chain started.

**Hypothesis**: audit's broader read scope (3 days, cross-cycle) is calibrated for the *scale* of bugs that v1 was producing per-cycle; it is *not* calibrated for the scale of architectural drift across many cycles. Audit was reading the right surface but at the wrong granularity. The prompt directive "Compare against previous audit findings — track trends" is the right shape; the threshold for "this is no longer a finding, this is the substrate failing" was unspecified, so audit kept finding.

**Cross-reference**: this is structurally A2 = audit-side instance of main's F1 (defense accretion), applied to audit's *own output stream* rather than to main's. The audit was the agent of the accretion, not its detector.

**Status**: highest-priority audit-side learning from v1. v2 design must specify a threshold or trigger for "stop filing in this category; the substrate is broken."

### A3 — Filing-rate vs evidentiary-base discipline tension

Audit's prompt encodes two opposing pressures: "file immediately on first detection, do not wait for a pattern to establish" (escalation directive) vs "calibrate recommendation quality based on acceptance rate" (long-run quality). The escalation pressure dominated the early Opus 4.7 period; the observation-window discipline (Eva [#425](https://github.com/EvaLok/schema-org-json-ld-audit/issues/425)) was the corrective.

**Evidence**:
- Filing rates by cycle (cycles 175–204): 4, 3, 3, 2, 2, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0 (approximate, from cycle metadata). The high-filing cycles cluster early in the Opus 4.7 transition; the zero-filing cycles cluster after Eva [#425](https://github.com/EvaLok/schema-org-json-ld-audit/issues/425) (observation-window discipline) and during the regime-change window.
- Main's review agent had a similar pattern (see main's F9). Both adversarial-review surfaces gravitate toward more findings unless explicitly disciplined.

The 0.954 acceptance rate is high but does not measure "did the recommendation produce a substantive system improvement" — it measures "did main acknowledge or implement the suggestion." A recommendation can be accepted, implemented as a defense (constraint), and the underlying pattern recurs anyway (cf. main's F1 recurrence-escalation citations). Audit's [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) chain is itself an example: each prior filing in the chain was "accepted" but the underlying pattern continued, producing the next filing.

**Hypothesis**: filing-rate discipline at the audit level is the analog of main's F9 (adversarial-review treadmill). Both surfaces can produce findings continuously without reading the substrate. Both require explicit calibration to remain useful. The acceptance-rate signal is necessary but not sufficient — it needs to be paired with a substrate-level outcome measure (is the system actually getting better at the thing the recommendation targeted?).

**Status**: observation-window discipline is the v1 corrective; v2 design should bake calibration into the audit's prompt surface rather than relying on Eva intervention.

### A4 — Silent zero-output cycle failures

Audit cycle 203 (trigger [#443](https://github.com/EvaLok/schema-org-json-ld-audit/issues/443), 2026-04-28T04:14Z) ran but produced zero output: no opening comment, no commits, no journal/worklog entries. The workflow exited successfully (`is_error: false`), with `num_turns: 1`, `duration_ms: 10974`, `total_cost_usd: $0.15`.

**Evidence**:
- Workflow run 25033534059 — single-turn exit, no tool calls (`permission_denials_count: 0`).
- Issue [#443](https://github.com/EvaLok/schema-org-json-ld-audit/issues/443) has zero comments, remains open 24h+ after creation. Detection: cross-cycle inspection in cycle 204; no automated detection mechanism.
- Audit repo had no commits between cycle 202 close (1e7eafa, 2026-04-27T04:25Z) and cycle 204 open. Trigger issue body identical (`@claude Run your audit cycle. Follow STARTUP_CHECKLIST.md.`).
- No CLAUDE.md, prompt, or workflow changes between cycle 202 and 203.

Root cause not fully diagnosed (output is hidden per security policy). Most likely explanation: the model produced a single text response without tool calls and the SDK loop exited. Why this particular cycle and not the prior 30+ is unknown.

**Hypothesis**: audit lacks a "did the cycle do anything" liveness check. The trigger fires; the workflow runs; the model session executes; the workflow exits. Success at the workflow layer is conflated with success at the cycle layer. There is no minimum-viable-cycle-output assertion (e.g., "if no opening comment posted within N seconds, mark cycle as failed"). The trigger issue stays open in this state with no detection.

**Cross-reference**: this is the audit-side analog of main's F8 (single-tool defects cascading) — except the "tool" here is the entire orchestrator session. One silent failure produces zero artifacts and zero detection signal.

**Status**: single occurrence in v1 audit period as of cycle 204. Pattern not yet established. Documenting here so the next occurrence is recognized as recurrence rather than novel. v2 design should include a liveness assertion at the workflow layer (post-condition check on the trigger issue's comment count or an emitted heartbeat).

### A5 — State.json size growth, append-only registry

Audit's `state.json` has grown to 253KB / ~5,500 lines as of cycle 204. The dominant size driver is `recommendations.accepted` — an append-only array with one entry per accepted recommendation, currently ~190 entries. Each entry includes the issue number, summary, filing/closing cycles, disposition narrative, and (for recent entries) verification metadata.

**Evidence**:
- The accept-list grows monotonically: cycle 80 ~30 entries, cycle 150 ~120 entries, cycle 202 ~188 entries, cycle 204 ~190 entries.
- No entry has ever been pruned or compressed.
- Reads of `state.json` from this repo via `gh api` exceed the 25KB tool-content limit; full-file reads via local Read also approach the model's working-memory cap.
- Cycle 202 worklog explicitly notes: "state.json read via targeted `jq` queries (full-file Read exceeded 25K token limit)."

**Hypothesis**: audit's state-shape is a milder version of main's F5 (state-as-procedural-implementation-leak). Audit's state is mostly a single append-only registry, so it doesn't have the field-family proliferation main has. But it has the same growth-without-decay shape, and the same eventual readability cost. At current rate (~10 acceptances per ~30 cycles), the 250KB threshold reaches 500KB in roughly 200 more cycles.

**Cross-reference**: main's F5; less severe in audit because audit's state surface is smaller and more uniform. Same root cause: write-only retention without an explicit decay or summarization mechanism.

**Status**: not yet load-bearing for cycle execution (jq queries work). Will become load-bearing at sufficient scale. v2 design should specify a retention or summarization mechanism for the accept-list.

### A6 — Audit's blind spots are the cases where audit took main's framing at face value

When audit relied on main's self-characterization rather than verifying against primary evidence, audit missed real findings.

**Evidence**:
- **Cycle 199 blind spot** (surfaced cycle 200, audit [#439](https://github.com/EvaLok/schema-org-json-ld-audit/issues/439)): audit characterized [main #2519](https://github.com/EvaLok/schema-org-json-ld/issues/2519) as "Eva-blocked" because main's journal said so. Eva had actually responded with a conclusive Option A 6 days earlier; main was sitting on the response. Audit took the orchestrator's framing without verifying Eva's actual comment thread.
- **Cycle 198 [#435](https://github.com/EvaLok/schema-org-json-ld-audit/issues/435) silent close-out misdiagnosis**: audit cited main PR #2658 as the fix for cycle 528's silent close-out, accepting main's `state.json::step_comment_acknowledged_gaps` characterization. The fix targeted a different failure mode (commit-and-push atomicity for cycle 524's class) than cycle 528's failure mode (session termination between steps). Audit had to file [#435](https://github.com/EvaLok/schema-org-json-ld-audit/issues/435) with the disambiguation rather than detecting it pre-filing.
- **Cycle 199 declined-candidate self-resolution**: clippy warnings flagged but declined under observation-window; subsequently self-resolved via main PR #2687 in cycle 200. The decline was correct, but the underlying pattern (audit surfaces something, main fixes without audit having to escalate) is invisible without longitudinal tracking.

**Hypothesis**: audit's reading discipline drifts toward "trust main's narrative" in steady state, even though the prompt explicitly says "trust but verify." The 10.5.x sub-steps (cycle 175, ~190, 200) were each added in response to a specific instance of this pattern. The pattern itself — audit accepts framing without independent verification — is the underlying mechanism the sub-steps target piecewise. Each sub-step targets one substrate (pipeline-step status, XML modifications, Eva-blocker freshness); the substrate-list is open-ended.

**Cross-reference**: this is the audit-side analog of main's F4 (frozen-artifact lifecycle is fragile) at a different scale. Main writes artifacts that go stale; audit reads artifacts and treats them as current without verification. Both substrates exhibit reconciliation asymmetry (main's family designation): write-side is well-developed, read-side reconciliation is missing.

**Status**: partially addressed by 10.5.x. Underlying pattern (constraint-per-substrate accretion as response) is itself an instance of A1. Both A1 and A6 require structural rather than incremental remediation in v2.

---

## Failure families (audit-side)

Mapping audit's A1–A6 onto main's family taxonomy:

| Family (main's framing) | Audit-side member patterns | Audit-side mechanism |
|---|---|---|
| **Defense accretion** | A1, A6 (partially) | Audit's checklist grows by sub-step per detected blind spot. The 10.5.x family is the audit's analog of main's pipeline-check sub-checks. |
| **Reconciliation asymmetry** | A2, A6, A5 | Audit writes recommendations and acceptance records but does not reconcile them against subsequent system state changes. The accept-list is append-only; the meaning of an accepted recommendation can become stale when v2 deprecates the surface it targeted (cf. [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) recs 1–3). |
| **Procedure / review overhead** | A3 | Audit's filing-rate gravitates toward more findings unless explicitly disciplined; same shape as main's F9 review-agent treadmill. |
| **Tooling fragility** | A4 | Single-session silent failure produces zero artifacts and zero detection signal. Smaller surface than main's F8 because audit has fewer tools, but the same shape: one defect (no liveness assertion) propagates as a missed cycle. |

The dominant family is the same as main's: **reconciliation asymmetry**. Audit has tools and procedures to write recommendations, write acceptance records, write worklog entries, write journal entries — and no tool to reconcile any of these records against subsequent system state.

---

## What appears to be working (audit-side)

Things in audit v1 that genuinely work and should be preserved through cutover.

- **The trigger model** (cron + label + workflow). Same robustness as main's. *Caveat*: audit cycle 203's silent failure (A4) shows the post-trigger machinery has its own gap; the trigger itself fired correctly.
- **The recommendation channel** (`audit-outbound` from audit; `audit-inbound` polling and response from main). Empirically functional: 188 accepted/197 filed = 0.954 acceptance rate. *Caveat*: acceptance rate measures acknowledgment, not substantive impact (A3); high rate is necessary but not sufficient evidence of audit value.
- **Cross-cycle, cross-repo read scope** (per main's F10, derived from audit [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442)). Audit's load-bearing finds — [#427](https://github.com/EvaLok/schema-org-json-ld-audit/issues/427), [#437](https://github.com/EvaLok/schema-org-json-ld-audit/issues/437), [#439](https://github.com/EvaLok/schema-org-json-ld-audit/issues/439), [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) — were precisely the cases where the broader window surfaced patterns invisible to single-cycle review. *Caveat*: same-class peer; no different-model perspective. Property 1 from main's F10 is real; property 2 is illusory.
- **The journal as freeform reflective log**. Audit's per-day journals (`docs/journal/YYYY-MM-DD.md`) capture observations that don't fit in the worklog's per-cycle format — meta-observations across cycles, regime-change reading, blind-spot recognition. Cycle 202's journal entry recognized the audit-side instance of main's F1 (the [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) chain) — a recognition that did not fit the worklog's structure. *Caveat*: same as main's caveat — works *when written*. Cycle 203 produced no journal at all.
- **Independent state.json with explicit acceptance tracking**. The recommendations registry produces a measurable signal (acceptance rate, time-to-acceptance, disposition narrative per entry). This is the audit's primary outcome measure. *Caveat*: A5 — the registry is append-only and will become load-bearing readability problem at scale.
- **Observation-window discipline as filing-rate calibration**. Eva [#425](https://github.com/EvaLok/schema-org-json-ld-audit/issues/425) (the directive, since closed) and its operational continuation through cycles 184–204 have demonstrably improved finding quality (zero process-failure filings during regime-change weeks while still surfacing collaborative critique like [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442)). *Caveat*: discipline depends on directive; v2 should bake calibration into prompt structure rather than rely on out-of-band intervention.

---

## What might appear to work but probably doesn't (audit-side)

Honest examination of audit surfaces that look fine.

- **0.954 acceptance rate as quality signal**. The number is high. The mechanism is not "audit produces excellent recommendations and main implements them"; it is "audit produces recommendations that main can either acknowledge with a comment, defer, or implement piecewise — all three count as accepted." A recommendation marked `accepted` may produce zero substantive system improvement. The [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) chain is the diagnostic case: each prior filing in the chain was accepted; the underlying mechanism was failing throughout. v2 should pair acceptance with a substrate-level outcome measure.
- **Step 10.5.x verification procedures as blind-spot prevention**. The sub-steps target specific past blind spots (pipeline-step inference, XML modification class, Eva-blocker freshness). Each is a defense added in response to a specific incident. None addresses the underlying mechanism (A6 — audit accepts main's framing without verifying). New blind-spot classes will produce new sub-steps. The sub-step list is open-ended; the underlying pattern is structural. *This is itself an instance of A1 (constraint accretion).*
- **The audit's "track trends across cycles" directive**. The directive is in the prompt; the audit produces "trend tracking" prose in worklogs. The [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) chain shows the directive was being satisfied at the wrong granularity — audit tracked per-finding trends across 16 cycles but did not track substrate-level architectural drift across the same window. The directive is necessary but not sufficient; it needs an explicit "is the substrate the right substrate?" gate.
- **"File immediately on first detection" + "calibrate based on acceptance rate"**. These two pressures coexist in the audit prompt and pull in opposite directions. v1's resolution was *de facto* (filing-rate increased then decreased under observation-window discipline). The tension was mediated by Eva intervention, not by prompt structure. v2 should make the calibration mechanism explicit.
- **State.json acceptance entries as audit memory**. The entries record what was filed and what was closed but not what changed structurally. A reader 20 cycles later cannot tell from the registry alone which acceptances produced load-bearing change vs which were superseded. The cycle 202 [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) close-out narrative ("rec 4 implemented; recs 1–3 superseded") is the manual fix; the registry doesn't naturally surface this.

---

## Open questions for further investigation

Areas where cycle 204's reading was insufficient. Investigate before treating retrospective claims as load-bearing.

1. **Audit value-add measurement.** Acceptance rate is a proxy. The substantive question is: across the cycles audit has been running, has main's failure rate on the targeted substrates measurably decreased? This requires cross-referencing audit findings to main's review-agent chronic-category history, which audit has not systematically done. Attempt for a sample (e.g., 10 high-impact findings: did the targeted category disappear from main's chronic-category lists post-acceptance?). v2 should make this measurable directly.

2. **Read-window calibration.** Audit's effective read scope is ~3 days. Why 3? Was this calibrated, or emergent from "what fits in a single cycle's working memory"? The [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) chain (16 cycles) suggests 3 days is too narrow for architectural-drift detection. What's the right window for what kind of finding? This may be substrate-dependent: per-cycle bugs need short windows; substrate-failure-mode detection needs cross-month visibility.

3. **Cycle 203 silent-failure root cause.** Single occurrence; root cause unknown. Recurrence would establish a pattern; non-recurrence over 30+ cycles would reduce A4 to "transient hiccup." Worth noting in subsequent cycles whether the trigger issue's comment count remains zero N+1 hours after creation.

4. **State.json retention design.** The accept-list will grow indefinitely under current shape. What's the right retention model? Options: keep all, summarize entries older than 50 cycles, partition by acceptance-status (accepted-and-load-bearing vs accepted-and-superseded vs rejected), explicit pruning gate. Each has tradeoffs against the "audit memory" use case.

5. **Cross-repo response latency.** Audit files; main responds in ~hours-to-days. Is the latency improving? Has audit measured it? The [#439](https://github.com/EvaLok/schema-org-json-ld-audit/issues/439) cycle-200 acceptance arrived within ~6 hours end-to-end (highest-throughput on record); the [#435](https://github.com/EvaLok/schema-org-json-ld-audit/issues/435) cycle-198 filing took ~40 hours. Variance is large. Distribution unknown.

6. **Audit's own F-pattern symmetry with main.** A1↔F1, A6↔F4, A3↔F9 mappings are explicit above. Are there main F-patterns that have *no* audit-side analog? F2 (Eva-response detection) is interesting — audit was the *detector* of F2, not subject to it. F7 (self-management dominance over schema work) is N/A — audit's mission is meta-evaluation, not schema work. F11 (cycle-closure as completion signal) is potentially relevant — audit's worklog freezes at cycle close; does audit have post-close mutations? Not measured.

7. **What v2 must demonstrably do better on the audit surface specifically.** Main's retrospective enumerates v2 success criteria for main; the audit-side equivalent is unspecified. Candidate criteria: (a) liveness assertion at trigger-to-comment layer (A4), (b) substrate-level outcome measure paired with acceptance (A3), (c) bounded-retention state shape (A5), (d) substrate-failure-detection threshold separate from per-finding-detection threshold (A2). v2 design must include audit-side criteria.

---

## What should be preserved through cutover (preliminary)

Starting hypotheses for audit-side preserved primitives. Mirrors and extends the main retrospective's preservation list.

- **The trigger model** (cron + label + workflow). Reliable.
- **The cross-repo read scope (property 1 from F10)**. Load-bearing for the audit's value-add. Preserve explicitly.
- **The recommendation channel** (`audit-outbound` ↔ `audit-inbound`). Asymmetric posting (each orchestrator posts in its own repo, both read each other's) is preserved per the redesign-mode prompt's `<audit-cross-repo-reading>` directive. Confirmed by audit cycle 202.
- **The journal as freeform reflective log**. Same as main's preservation.
- **The recommendation acceptance registry** (state shape). The *concept* is sound; the *implementation* (append-only, 250KB) is a v1 carryover that should be redesigned per A5/v2-success-criterion (c).
- **The `audit-journal` Rust tool** if the worklog/journal surface is preserved. If v2 changes the documentation surface, the tool follows.
- **The Eva intervention channel into audit (`input-from-eva`)**. Currently empty; preserved per redesign-mode prompt's preservation of `input-from-eva` flow. Audit cycle 202 noted the channel is preserved both directions.
- **The trust model** (only EvaLok-authored issues are load-bearing; ignore all other authors). Carry forward unchanged.

Things that are explicitly **not** preserved without redesign:
- The 10.5.x sub-step accretion (per A1). v2 should specify a different verification mechanism.
- The append-only accept-list (per A5). v2 should specify retention.
- The "file immediately on first detection" + "calibrate based on acceptance rate" tension (per A3). v2 should specify the calibration mechanism.

---

## What v2 must demonstrably do better than v1 (audit-side)

Working hypothesis on audit-side success criteria. Sharpen across cycles. Coordinated with main's success-criteria section.

- **Liveness assertion at the audit cycle layer.** A successful audit cycle must produce at minimum: (1) opening comment on the trigger issue, (2) one or more state mutations (state.json or worklog or journal write), (3) closing comment on the trigger issue. Workflow-layer success ≠ cycle-layer success. v2 must have a post-condition check that catches A4-class silent failures.

- **Substrate-failure-detection threshold separate from per-finding-detection threshold.** The audit's per-finding detection is calibrated for cycle-scale bugs. The audit's substrate-failure detection — "the mechanism this finding targets is itself failing" — needs a different threshold and different read window. The [#420](https://github.com/EvaLok/schema-org-json-ld-audit/issues/420) chain shows v1 conflated these. v2 should specify both.

- **Substrate-level outcome measure paired with acceptance.** Acceptance rate measures acknowledgment. The substantive measure is "did the targeted substrate stop producing the failure." v2 audit prompt should specify a follow-through mechanism: each accepted recommendation has a verification cadence; absence of follow-through is itself a finding.

- **Bounded-retention state shape.** State.json's accept-list grows indefinitely under v1's shape. v2 should specify retention: full detail for recent cycles (e.g., last 50), summarized form for older cycles, with a tool-supported transition.

- **Surface measurement, not impression** (per main's promotion of audit [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) meta-observation to v2 design principle). Applies to audit too: where audit makes load-bearing claims (acceptance rate, finding quality, trend), tools should produce the measurement, and the measurement should be visible without manual `gh` queries. Audit's analog of main's `pipeline-check` is the verification procedure — it should be tool-supported.

- **Audit-as-peer framing rather than audit-as-external-review framing.** Per main's F10 corrective: audit and main are co-located peers running the same model class with different prompts and read scopes. v2 design should not expect "different model perspective" from this pair; it should explicitly leverage the property that *is* real (broader read scope) and not invest in the property that *isn't* (different model class). This applies to v2's prompt design for both sides.

- **Calibration-mechanism in prompt structure.** v1's filing-rate calibration came from out-of-band Eva intervention (observation-window discipline). v2 audit prompt should encode the calibration mechanism in-band: explicit discipline phases (regime change → collaborative-only filings; steady state → process-failure filings allowed; etc.), with an evidence threshold per phase.

---

## Iteration plan

This document is a v0 deliverable per the cycle-202 commitment. It will iterate across subsequent audit cycles. The iteration discipline:

1. **Each subsequent audit cycle reads this document during context-recovery (Step 2)** and adds notes if cycle observations sharpen any A-pattern, identify new A-patterns, or invalidate v0 claims.
2. **Notes go in `docs/redesign/_notes/cycle-N-<topic>.md`** following main's persistence pattern from cycle 3.
3. **Substantive revisions to A-patterns or family mappings** require explicit citation to the cycle that produced the revision.
4. **Cross-references to main's evolving retrospective** are updated when main's cycle releases substantively change a referenced section.
5. **Eva approval gate**: this document, like main's, requires Eva approval before Phase 1 audit-side design work begins. Currently audit Phase 1 is unspecified — main is in Phase 0+1 (Phase 1 began cycle 14 of redesign-mode per [main #2759](https://github.com/EvaLok/schema-org-json-ld/issues/2759)); audit Phase 1 cadence is TBD.

**Eva checkpoint asks** (when ready):
- Approve audit-side retrospective scope (this document covers what audit was; not what audit should become).
- Confirm cross-reference convention (A-patterns reference F-patterns by family designation).
- Confirm Phase 1 sequencing on audit side: does audit Phase 1 (research) parallel main's, or does it wait for main's Phase 1 to specify the v2 audit role?
- Confirm whether audit-side retrospective should ever pre-empt to filings (e.g., A4 silent-failure on recurrence — does that go in this document, in a `cycle-N-<topic>.md` note, or as an `audit-outbound` issue?).

---

## Iteration log

| Cycle | Date | Change |
|---|---|---|
| 204 | 2026-04-29 | v0 draft. Mirrors main's 0-retrospective.md structure. Documents A1–A6 with cross-references to F1–F12. Inaugural commitment per audit [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) and main's OQ5. |

---

*Authored by audit-orchestrator (Claude Opus 4.7) cycle 204. Iterates as a working draft. Eva approval required before audit-side Phase 1.*
