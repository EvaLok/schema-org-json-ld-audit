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

**Axis-coverage gap instance (named cycle 218)**: Step 13.1 (cycle 215 archival pattern) targets *one* growth axis — `redesign_mode.audit_*_cycle_N` narrative fields. Other growth axes remain unbounded:

- `metrics.trend` cumulative narrative (rewritten each cycle but tends to grow when summarizing prior cycles)
- `last_cycle.summary` watch-item list (cycle 220 had 6 items; pattern shows accumulation)
- `recommendations.accepted` array (~190 entries, ~190KB — explicitly deferred as larger structural decision)
- New top-level redesign_mode.* fields that may be added during Phase 3

Step 13.1 is a defense added against one specific overflow vector. The cycle 218 archival of cycle_217 saved ~1.3KB but cycle 218 narrative additions in metrics.trend / last_cycle.summary / recommendations.open initially exceeded archival savings (+2.5KB net), requiring trim to bring net to +224 bytes. This is the **defense-accretion-on-one-axis-while-others-unbounded** pattern — A1 instantiated against Step 13.1 itself. v2 design should specify *holistic* state-size discipline (per-axis caps or whole-file rotation), not per-incident defenses.

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

**Status (revised cycle 221)**: A4 has now occurred 8 times across cycles 203 / 206 / 208 / 209 / 210 / 213 / 216 / 219. The ~50% rate over 17 cycles since cycle 203 is broadly stable; rate has neither escalated nor self-bounded. [#448](https://github.com/EvaLok/schema-org-json-ld-audit/issues/448) is the question-for-eva tracking the pattern (filed cycle 207, updated cycles 211 / 214 / 220 with rate-escalation evidence; Eva-silent 14+ days as of cycle 221).

Cycle 211's hypothesis-C (model-side guardrail/classifier intersecting with cycle context) remains most plausible given deterministic 27-54s "Run Audit Orchestrator" step durations. The cycle 211/212 success-then-cycle-213-revert sequence falsified the transient-cluster hypothesis; A4 is not self-bounded.

**Substrate-replacement-shifts-A4-framing (named cycle 220, post-Q7-resolution)**: A4 is empirical evidence about *single-orchestrator substrate*. Q7 RESOLVED 2026-05-13 (main [#2930](https://github.com/EvaLok/schema-org-json-ld/issues/2930)) selected Candidate B (multi-agent role-specialization) with drastic architectural change required. Main shifted to Phase 3 across cycles 134-148 and produced first-end-to-end-smoke evidence cycle 153 (5 v2-* primitives + 3 live multi-agent cycles in 94-98ms warm wallclock; SCAFFOLD topology operationally verified). Audit's own substrate remains single-orchestrator (this very cycle is single-orchestrator). Phase 3 multi-agent topology replaces main's substrate; A4 framing under that new substrate is empirically open:

- **Amplify** prediction: per-role-context risk × N agents may sum, making zero-output cycles more likely.
- **Mitigate** prediction: one role's failure may not kill the cycle (other roles still produce work via the channel-router super-step boundary).
- **Transform** prediction: failure mode shifts entirely (e.g., from "zero output cycle" to "incomplete cycle with N roles silent and M roles output"; classifier-class failure may no longer have a single observable shape).

None of these are empirically supported. Audit will observe under Phase 3 cycle-1 minimal end-to-end pipeline first cron execution (currently SCAFFOLD; cron-driven live execution not yet active). **Future A4 observations must be tagged by substrate-type** (audit-side single-orchestrator vs main-side Phase 3 multi-agent vs other). The pattern-name "A4" remains; the underlying mechanism analysis is now substrate-dependent. This is the deepest substrate-level reframing the retrospective has undergone since v0.

**Commitment-thread cascade interaction (named cycle 215)**: A4 produces a secondary failure mode beyond zero-output. When cycle N filed a commitment to take action in cycle N+1, and cycle N+1 silently fails, the commitment cascades to cycle N+2 with no user-visible notification. This has occurred twice:

- Cycle 207 → cycle 211 [#448](https://github.com/EvaLok/schema-org-json-ld-audit/issues/448) escalation: cycle 207's "re-file only if rate increases" commitment was scheduled to be evaluated cycle 208; cycles 208/209/210 silently failed; cycle 211 (cycle 207 + 4) executed the escalation.
- Cycle 213 → cycle 214 [#455](https://github.com/EvaLok/schema-org-json-ld-audit/issues/455) Option A: cycle 212's "audit will adopt Option A in cycle 213 if no Eva direction" commitment was scheduled for cycle 213; cycle 213 silently failed; cycle 214 (cycle 212 + 2) implemented Option A.

The commitment-thread discipline (see "What appears to be working") survives the cascade, but with a cycle-count latency penalty proportional to the A4 run length. v2 design must specify both a liveness assertion (catches A4) AND a commitment-thread observability mechanism (catches A4 ∩ commitment-thread cascade).

v2 design must include a liveness assertion at the workflow layer (post-condition check on the trigger issue's comment count or an emitted heartbeat) AND a commitment-thread observability mechanism that surfaces deferred commitments at cycle N+1 entry.

### A5 — State.json size growth, append-only registry

Audit's `state.json` has grown to 253KB / ~5,500 lines as of cycle 204. The dominant size driver is `recommendations.accepted` — an append-only array with one entry per accepted recommendation, currently ~190 entries. Each entry includes the issue number, summary, filing/closing cycles, disposition narrative, and (for recent entries) verification metadata.

**Evidence**:
- The accept-list grows monotonically: cycle 80 ~30 entries, cycle 150 ~120 entries, cycle 202 ~188 entries, cycle 204 ~190 entries.
- No entry has ever been pruned or compressed.
- Reads of `state.json` from this repo via `gh api` exceed the 25KB tool-content limit; full-file reads via local Read also approach the model's working-memory cap.
- Cycle 202 worklog explicitly notes: "state.json read via targeted `jq` queries (full-file Read exceeded 25K token limit)."
- **Cycle 214 ([#458](https://github.com/EvaLok/schema-org-json-ld-audit/issues/458))**: state.json reached 273674 bytes (267KB), exceeding the Read tool's 256KB default limit. A5 crossed operational threshold.
- **Cycle 215 (named threshold + first archival applied)**: STARTUP_CHECKLIST Step 13.1 added with size thresholds (100KB advisory / 200KB mandatory / 250KB hard). First archival applied — cycles 205/207/211 narrative fields moved to `docs/redesign/_notes/audit-cycle-N-state.md` mirroring main's `_notes/` pattern. State.json reduced 271KB → 264KB (-8KB / ~3%).

**Hypothesis**: audit's state-shape is a milder version of main's F5 (state-as-procedural-implementation-leak). Audit's state is mostly a single append-only registry, so it doesn't have the field-family proliferation main has. But it has the same growth-without-decay shape, and the same eventual readability cost. At current rate (~10 acceptances per ~30 cycles), the 250KB threshold reaches 500KB in roughly 200 more cycles.

**Cross-reference**: main's F5; less severe in audit because audit's state surface is smaller and more uniform. Same root cause: write-only retention without an explicit decay or summarization mechanism.

**Status (revised cycle 215)**: load-bearing for cycle execution as of cycle 214. Cycle 215 implementation:

- **Named threshold**: STARTUP_CHECKLIST Step 13.1 specifies 100KB advisory / 200KB mandatory / 250KB hard, against the 256KB Read tool ceiling.
- **Archival pattern**: `redesign_mode.audit_*_cycle_N` narrative fields older than 5 cycles are moved to `docs/redesign/_notes/audit-cycle-N-state.md`, replaced with one-line pointers.
- **Cycle 215 first application**: cycles 205/207/211 archived; cycle 212 retained (within 5-cycle window); cycle 214 had no narrative fields per [#458](https://github.com/EvaLok/schema-org-json-ld-audit/issues/458) commitment to bound growth during design phase.

**Remaining gap**: the dominant size driver (`recommendations.accepted` at ~190 entries / ~190KB) is NOT addressed by the cycle 215 archival. That decision is structurally larger — full archival of older entries vs summarization vs partition by acceptance-status — and is deferred separately from the cycle 215 redesign_mode-narrative archival. v2 design must specify the recommendations-registry retention shape; this is the audit-side instance of main's F5 at full severity.

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

- **Commitment-thread discipline (named cycle 215, extended cycle 221)**. The pattern: when cycle N identifies a forward action that should be evaluated cycle N+M, encode the threshold *and* the response in writing as a watch item on cycle N's `last_cycle.summary`. Cycle N+M's first action is to evaluate the watch item against the named criterion. **Eight V2-era instances** across cycles 207→211, 211→212, 212→214, 214→215, 215→217, 217→218, 218→220, 220→221 all produced high-value outcomes:

  - Cycle 207 → cycle 211 [#448](https://github.com/EvaLok/schema-org-json-ld-audit/issues/448) escalation. Cycle 207 commitment: "re-file only if rate increases or cause becomes actionable." Cycle 211 (after 4-cycle A4 blackout) executed the escalation mechanically: rate had increased (5 occurrences in 8 days vs 2 in 24 days), so re-filing was procedural, not a fresh judgment call.
  - Cycle 211 → cycle 212 [#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454) / [#455](https://github.com/EvaLok/schema-org-json-ld-audit/issues/455) filings. Cycle 211 commitment: "audit retrospective iteration discipline — A4 rate-escalation under V2 NOT yet retrospective-warranting." Cycle 212 surfaced the audit-request channel-gap and engaged substantively on [main#2849](https://github.com/EvaLok/schema-org-json-ld/issues/2849) — the substantive engagement was explicit because cycle 211 had named the surfaces audit *was not* iterating on.
  - Cycle 212 → cycle 214 absorption discovery + Option A implementation. Cycle 212 commitment: "audit will adopt Option A in cycle 213 if no Eva direction." Cycle 213 silently failed (A4 occurrence #6); cycle 214 (cycle 212 + 2) executed Option A AND discovered main cycle 85's 100% verdict-level absorption of [#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454).
  - Cycle 214 → cycle 215 [#458](https://github.com/EvaLok/schema-org-json-ld-audit/issues/458) self-implementation. Cycle 214 commitment: "implement state.json archival pattern." Cycle 215 mechanically executed with bounded scope (archival + Step 13.1 + retrospective revision) and bounded execution (one cycle).
  - Cycle 215 → cycle 217 [#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) cluster framework critique + [#463](https://github.com/EvaLok/schema-org-json-ld-audit/issues/463) retention-shape question. Cycle 215 commitment: "audit-side substantive Phase 2 candidate critique production" + "question-for-eva on retention-shape preference if no Eva direction emerges." Cycle 217 filed both.
  - Cycle 217 → cycle 218 [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) M1 sharpening. Cycle 217 surfaced cycle 130's M1 deferral conflation; cycle 218 sharpened as Phase 2 evidence specifically.
  - Cycle 218 → cycle 220 closure work + audit-side format revision deferral. Cycle 218 commitment: "[#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) M1 absorbed → close; [#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) M1 sharpened separately → partial close; cycle 219+ work if [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) accepted." Cycle 220 executed closure + deferred format revision to cycle 221.
  - Cycle 220 → cycle 221 audit retrospective revision. Cycle 220 commitment: "4 candidates accumulated across 3 cycles ready for retrospective update concurrent with format-revision implementation cycle 221+." Cycle 221 (this cycle) executed both — retrospective A4 substrate-replacement reframing, A1 axis-coverage instance, two new positive patterns, plus this commitment-thread extension.

  The pattern is structurally similar to main's `<promotion-shape-tracker>` discipline (track promotions across cycles to surface stable patterns). Audit's analog is forward-action-promotion: a commitment named in writing at cycle N is the substrate against which cycle N+M evaluates. *Caveat*: the discipline survives A4 cascades but with cycle-count latency penalty (see A4 revised cycle 221). **Cycle 221 status**: this is the single most load-bearing primitive in audit-side V2 operations — produced 5 substantive cross-repo absorption events ([#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454), [#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462), [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465), [#458](https://github.com/EvaLok/schema-org-json-ld-audit/issues/458) self, and this revision), 8-cycle operational record, zero degradation. v2 design should preserve this as an audit primitive AND specify a commitment-thread observability mechanism that surfaces deferred commitments at cycle N+1 entry.

- **Audit-as-Priority-1-input pattern (NOVEL@1 cycle 218, HARDENING-AT-2 cycle 220)**. Main has named audit's cross-repo critiques as **forward priority #1** for the next cycle's substantive focal at least twice:

  - Cycle 133 main housekeeping note explicitly listed "audit cycle 218 critique absorption" as forward-priority #1 for cycle 134.
  - Cycle 134 then absorbed [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) M1 at 100% verdict-level within 53 minutes of filing — the deepest substrate-level absorption audit has produced under V2 (3 load-bearing A-findings consumed, 3 pattern promotions, 2 new candidate-emergent patterns named).
  - Earlier instance: main cycle 85 absorbed [#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454) within 50 minutes of filing with similar 100% verdict-level discipline.

  The pattern: audit-engagement filings become explicit forward-priority inputs to main's next cycle, not just background reading. This is structurally different from "audit files, main absorbs eventually" — it is "audit files, main names absorption as next-cycle substantive focal." 5-cycle operational arc (cycle 217 filing → cycle 220 forward-priority naming with cycle 134 absorption demonstration). *Caveat*: the pattern depends on audit producing critique substantive enough to anchor a main cycle. Low-cadence findings or process-failure filings don't anchor cycles; substantive cross-perspective critiques do. v2 design should preserve the V2 cross-repo audit-engagement format (5+5+5+N) as the substrate that produces forward-priority-anchorable critique.

- **V2 cross-repo audit-engagement format (named cycle 215, extended cycle 221)**. Audit produces substantive critique on surfaces main hasn't yet cold-readed; main absorbs at high verdict-level. **Four V2-era instances**:

  - [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) (cycle 202) — first V2 collaborative critique on main's 0-retrospective.md. Main absorbed cycle 3 with structural revisions to F10/F11/F12 + family taxonomy adoption + meta-observation promotion.
  - [#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454) (cycle 212) — second V2 collaborative critique on main's clusters.md cycles 65-75 cluster framework. Main absorbed cycle 85 at 100% verdict-level (5/5 strongly-agree PRESERVED + 5/5 disagree ACCEPTED + 5/5 missing patterns ACCEPTED + 6/6 Phase 2 implications ACCEPTED). 50-min round-trip.
  - [#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) (cycle 217) — third V2 collaborative critique on main's Phase 2 candidate-selection deliverable. Main absorbed across cycles 126-130 at 100%-D-absorption (D1-D5 absorbed at substantive depth across 5 dedicated absorption notes within ~24h of filing).
  - [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) (cycle 218) — first follow-up-sharpening filing under V2 (recursive engagement-of-engagement). Main absorbed cycle 134 within 53 minutes — deepest substrate-level absorption (3 A-findings + 3 pattern promotions + 2 new candidate-emergent patterns).

  Format precedent: 5 strongly-agree + 5 disagree-or-sharpen + 5 missing patterns + N implications, structured around the surface main authored. All four delivered when audit's broader read scope surfaced patterns invisible to main's per-cycle cold-reader rhythm. **Cycle 221 format documentation (per [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) recommendation 2)**: the Missing Patterns bucket benefits from sub-categorization into (a) **Phase-2-evidence to consult** (observations from V2 operational substrate that bear on main's design choices), (b) **Phase-3-design requirements** (specifications or constraints for forthcoming substrate), (c) **framework-categorization questions** (clarifying queries about the surface being critiqued). The sub-categorization is implicit in past engagements but documenting it here makes the format reproducible and absorbable without re-derivation. *Caveat*: low cadence — four instances over ~19 V2 audit cycles. Engagement requires either an explicit audit-request from main OR an implicit ask channel.

- **Copilot-as-adversarial-critique-parallel-pattern (NOVEL@1 cycle 220)**. Main's [main#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) directive (2026-05-14: "Better leverage Copilot dispatch for development work") catalyzed adversarial-critique-via-Copilot pattern at intra-repo scope. Main exercised it cycles 146-148 (PR #2951 24-finding ledger) and cycle 155 (PR #2961 29-finding cycle 152 v2-cycle-runner critique with 0 DISAGREE). This pattern structurally parallels audit's V2 cross-repo audit-engagement format (adversarial mandate, structured per-finding deliverable, absorption-vs-disagreement tracking) — but at intra-repo scope. The audit-as-peer relationship may transform now that main has a parallel intra-repo mechanism for adversarial critique.

  **Audit's distinctive value-add under Phase 3 (hypothesized, not yet empirically tested under new substrate)**:

  - Cross-repo perspective: audit reads main + QC + audit-self from outside; Copilot dispatches are intra-repo (one PR at a time, no cross-repo view).
  - Retrospective-corpus access: audit holds A1-A6 retrospective + ~19 V2-era operational substrate instances; Copilot dispatches see the PR's local context only.
  - Async-of-main-cycle cadence: audit reads at depth main hasn't cold-readed yet; Copilot dispatches synchronize with main's cycle rhythm.
  - Cross-iteration memory: audit tracks patterns across many cycles (8 V2 commitment-thread instances, 8 A4 occurrences); Copilot dispatches are single-cycle.

  These distinctions are **empirically testable under Phase 3**: if audit-engagements still produce 100%-verdict-level absorption with new candidate-emergent patterns under Phase 3 substrate, the audit-as-peer relationship is preserved with cross-perspective specialization. If they don't, audit's role may need to evolve. Cycle 221 cannot answer; first Phase 3 cron-driven multi-agent live execution + first audit-engagement against Phase 3 substrate will begin to. v2 design should explicitly leverage the audit-distinctive properties (cross-repo + retrospective-corpus + async-cadence + cross-iteration memory) rather than treat audit-as-peer and Copilot-as-adversarial-critique as redundant.

- **V2 cross-repo audit-engagement format (named cycle 215)**. Audit produces substantive critique on surfaces main hasn't yet cold-readed; main absorbs at high verdict-level. Two V2-era instances:

  - [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) (cycle 202) — first V2 collaborative critique on main's 0-retrospective.md. Main absorbed cycle 3 (`docs/redesign/_notes/cycle-3-audit-442-integration.md`) with structural revisions to F10/F11/F12 + family taxonomy adoption + meta-observation promotion. ~25-cycle window.
  - [#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454) (cycle 212) — second V2 collaborative critique on main's clusters.md cycles 65-75 cluster framework. Main absorbed cycle 85 at 100% verdict-level (5/5 strongly-agree PRESERVED + 5/5 disagree ACCEPTED + 5/5 missing patterns ACCEPTED + 6/6 Phase 2 implications ACCEPTED). 50-min round-trip from audit cycle 212 close.

  Format precedent: 5 strongly-agree + 5 disagree + 5 missing patterns + N implications, structured around the surface main authored. Both instances delivered when audit's broader read scope (F10 property 1) surfaced patterns invisible to main's per-cycle cold-reader rhythm. *Caveat*: low cadence — two instances over ~13 V2 audit cycles. Engagement requires either an explicit audit-request from main OR an implicit ask channel (cycle 215 observed: main's `2-candidates/README.md` includes the "audit critique solicitation — this directory's content is read by the audit-repo orchestrator on its next cycle" directive without filing a `[audit-request]` issue). v2 design should formalize both the explicit and implicit channels.

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
| 215 | 2026-05-10 | A4 revised: 6 occurrences (cycles 203/206/208/209/210/213); status changed from "single occurrence" to "sustained ~50% rate over 12 cycles since cycle 203, NOT self-bounded"; commitment-thread cascade interaction named (A4 ∩ commitment-thread produces cycle-count latency penalty). A5 revised: cycle 214 [#458](https://github.com/EvaLok/schema-org-json-ld-audit/issues/458) crossed operational threshold; cycle 215 added named threshold (Step 13.1 in STARTUP_CHECKLIST: 100KB advisory / 200KB mandatory / 250KB hard) + first archival application (cycles 205/207/211 → `_notes/audit-cycle-N-state.md`). Two new positive patterns added to "What appears to be working": commitment-thread discipline (3 V2-era instances cycles 207→211 / 211→212 / 212→214) and V2 cross-repo audit-engagement format (2 V2-era instances [#442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) and [#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454)). Recommendations.accepted retention shape remains explicitly deferred. |
| 221 | 2026-05-16 | **A4 substrate-replacement reframing** (deepest substrate-level revision since v0): A4 occurrences extended to 8 (cycles 203/206/208/209/210/213/216/219; ~47% rate over 17 cycles broadly stable). Q7 RESOLVED 2026-05-13 selected Candidate B (multi-agent role-specialization); main shifted to Phase 3 cycles 134-156 producing first-end-to-end-smoke evidence cycle 153 (5 v2-* primitives + 3 live multi-agent cycles in 94-98ms warm wallclock). A4 framing now substrate-dependent: amplify/mitigate/transform predictions enumerated, none empirically supported; future A4 observations must be tagged by substrate-type. **A1 axis-coverage instance**: Step 13.1 defends one growth axis (`redesign_mode.audit_*_cycle_N` narrative) while metrics.trend / last_cycle.summary / recommendations.accepted remain unbounded; A1 instantiated against the cycle 215 defense itself; v2 should specify holistic state-size discipline. **Commitment-thread discipline extended**: 3 V2-era instances → 8 (cycles 214→215, 215→217, 217→218, 218→220, 220→221 added); named the single most load-bearing primitive in audit-side V2 operations with 8-cycle operational record + zero degradation. **V2 cross-repo audit-engagement format extended**: 2 instances → 4 ([#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) cycle 217, [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) cycle 218 added — [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) is first follow-up-sharpening filing under V2); Missing Patterns sub-categorization documented per [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) recommendation 2 (Phase-2-evidence / Phase-3-design-requirements / framework-categorization). **Two new positive patterns added**: (1) audit-as-Priority-1-input pattern (NOVEL@1 cycle 218, HARDENING-AT-2 cycle 220 — main cycle 133 named "audit cycle 218 critique absorption" as forward-priority #1 for cycle 134; cycle 134 then absorbed [#465](https://github.com/EvaLok/schema-org-json-ld-audit/issues/465) M1 at 100% verdict-level in 53min); (2) Copilot-as-adversarial-critique-parallel-pattern (NOVEL@1 cycle 220 — main's [main#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) directive catalyzed intra-repo adversarial-critique-via-Copilot; PR #2951 24-finding + PR #2961 29-finding 0-DISAGREE absorbed; audit's distinctive value-add under Phase 3 enumerated: cross-repo perspective + retrospective-corpus access + async-cadence + cross-iteration memory). Recommendations.accepted retention shape remains explicitly deferred ([#463](https://github.com/EvaLok/schema-org-json-ld-audit/issues/463) Eva-pending). |

---

*Authored by audit-orchestrator (Claude Opus 4.7) cycle 204. Iterates as a working draft. Eva approval required before audit-side Phase 1.*
