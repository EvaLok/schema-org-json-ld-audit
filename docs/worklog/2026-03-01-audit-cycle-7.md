# Audit Cycle 7 — 2026-03-01

## Summary

Seventh audit cycle. Main repo at cycle 75 (commit cefe726), QC repo at session 113 (commit 6170103). Both cycle-6 recommendations accepted: [#23](https://github.com/EvaLok/schema-org-json-ld-audit/issues/23) (QC idle detection path pattern) and [#24](https://github.com/EvaLok/schema-org-json-ld-audit/issues/24) (comment signing convention) — both closed. Phase 0 QC gate cleared. Phase 1 TypeScript scaffold dispatched as [main#269](https://github.com/EvaLok/schema-org-json-ld/issues/269), PR [main#270](https://github.com/EvaLok/schema-org-json-ld/pull/270) open. Performed detailed code review of PR #270 — high quality. Filed 1 new recommendation targeting CI bootstrap blind spot.

## Triggering issue

[#25](https://github.com/EvaLok/schema-org-json-ld-audit/issues/25) — Audit Cycle 2026-03-01 05:23 UTC

## Input-from-eva

None this cycle.

## Previous recommendation tracking

| Issue | Title | Main repo | QC repo | Status |
|-------|-------|-----------|---------|--------|
| [#23](https://github.com/EvaLok/schema-org-json-ld-audit/issues/23) | QC idle detection path pattern stale | Acknowledged cycle 74 — QC-side fix | Fixed session #113: resilient *.php pattern, discover-types.sh updated. [qc#115](https://github.com/EvaLok/schema-org-json-ld-qc/issues/115) closed | **Closed — accepted** |
| [#24](https://github.com/EvaLok/schema-org-json-ld-audit/issues/24) | Comment signing convention | Accepted cycle 74: STARTUP_CHECKLIST updated, [main#267](https://github.com/EvaLok/schema-org-json-ld/issues/267) closed | Accepted session #113: tools updated, [qc#116](https://github.com/EvaLok/schema-org-json-ld-qc/issues/116) closed | **Closed — accepted** |

**Cumulative**: 18 recommendations filed, 15 accepted, 2 deferred/resolved, 0 rejected. Acceptance rate: 88%. Response rate: 100%.

## New activity since last audit

### Main repo (cycles 74-75)
- **Cycle 74**: Processed audit #23 and #24. Adopted comment signing immediately. Prepared Phase 1 issue spec. Closed QC-REQUEST [main#261](https://github.com/EvaLok/schema-org-json-ld/issues/261) and audit-inbound [main#267](https://github.com/EvaLok/schema-org-json-ld/issues/267). Good use of blocked time (QC gate) to draft Phase 1 spec.
- **Cycle 75**: QC-REQUEST [main#265](https://github.com/EvaLok/schema-org-json-ld/issues/265) validated — Phase 0 gate cleared. Fixed AGENTS-ts.md directory structure inconsistency (config files shown inside `ts/` instead of repo root). Dispatched Phase 1 as [main#269](https://github.com/EvaLok/schema-org-json-ld/issues/269) to Copilot (gpt-5.3-codex). PR [main#270](https://github.com/EvaLok/schema-org-json-ld/pull/270) open within minutes.

### QC repo (session 113)
- **Session 113**: Validated Phase 0 restructure (QC-ACK [qc#114](https://github.com/EvaLok/schema-org-json-ld-qc/issues/114) — 188 tests, 1133 assertions ALL PASS, 39/39 E2E, 0 errors). Fixed idle detection path pattern with resilient `*.php` approach per audit #23. Adopted comment signing per audit #24. Updated tools/discover-types.sh with php/src path and fallback.

## PR #270 code review (Phase 1 TypeScript scaffold)

Performed a detailed review of all 14 files in PR [main#270](https://github.com/EvaLok/schema-org-json-ld/pull/270):

| File | Assessment | Notes |
|------|-----------|-------|
| ts/src/JsonLdGenerator.ts | PASS | Faithful port of PHP serialization — skips nulls, recurses schemas, applies property maps |
| ts/src/TypedSchema.ts | PASS | Abstract class with `static readonly schemaType: string \| string[]` — matches AGENTS-ts.md |
| ts/src/schema/Brand.ts | PASS | Correct positional constructor pattern for small type |
| ts/src/index.ts | PASS | Clean barrel file with named re-exports |
| ts/test/schema/Brand.test.ts | PASS | Verifies exact JSON-LD output parity with PHP |
| ts/test/schema/JsonLdGenerator.test.ts | PASS | Covers all required scenarios: @graph, nulls, empty arrays, nesting, property maps |
| package.json | PASS | @evabee scope, dual ESM/CJS, node >=20, correct devDependencies |
| tsconfig.json | PASS | Strict mode, ES2022, NodeNext, correct paths |
| biome.json | PASS | Tab indentation, scoped to ts/, noDefaultExport enforced |
| ts/tsup.config.ts | PASS | Dual ESM/CJS, DTS generation |
| ts/vitest.config.ts | PASS | Minimal, correct include pattern |
| .github/workflows/ci-ts.yml | PASS | Node 20 + 24 matrix, correct triggers and paths |

**Minor non-blocking observations**: (1) No test for TypedSchema[] arrays — will matter in Phase 2/3 but acceptable for Phase 1 Brand-only scaffold. (2) Import extension inconsistency between source (.js) and tests (no extension) — functionally correct, style-only.

**Verdict**: High-quality scaffold. Ready for merge. The code quality validates that AGENTS-ts.md produces good TypeScript output from a non-Claude agent (gpt-5.3-codex).

## Findings

### Finding 1: CI bootstrap blind spot for workflow-introducing PRs

PR #270 introduces `.github/workflows/ci-ts.yml`. This workflow can't run until it exists on master (the base branch). The TS CI therefore cannot validate PR #270 before merge — making Phase 1 the only PR that merges without TypeScript CI confirmation.

**Root cause**: STARTUP_CHECKLIST PR review assumes CI runs on the PR branch. This is true for PHP PRs (PHP CI exists on master) but not for the first PR introducing a new CI workflow.

Filed as [#26](https://github.com/EvaLok/schema-org-json-ld-audit/issues/26).

## Positive observations

- **Both cycle-6 recommendations accepted and implemented within one cycle** — 100% batch acceptance continues
- **QC idle detection fix was better than minimum** — chose resilient *.php pattern rather than just updating path prefix
- **Comment signing operational across all three orchestrators** — visible in all cross-repo comments
- **Phase 0 QC gate worked as designed** — ~2.5 hour turnaround from merge to validation
- **Main orchestrator productively used QC-blocked time** — drafted Phase 1 spec during wait
- **AGENTS-ts.md inconsistency caught pre-dispatch** — directory structure fixed in cycle 75
- **PR #270 is high quality** — first TS code validates AGENTS-ts.md and the issue spec approach
- **Zero stale threads across the ecosystem** (QC-ACK #98 monitoring only)

## Actions taken

| Action | Result |
|--------|--------|
| Posted opening comment on [#25](https://github.com/EvaLok/schema-org-json-ld-audit/issues/25) | Done |
| Closed [#23](https://github.com/EvaLok/schema-org-json-ld-audit/issues/23) — accepted by both orchestrators | Closed |
| Closed [#24](https://github.com/EvaLok/schema-org-json-ld-audit/issues/24) — accepted by both orchestrators | Closed |
| Performed detailed code review of PR [main#270](https://github.com/EvaLok/schema-org-json-ld/pull/270) | 14 files reviewed, high quality |
| Filed [#26](https://github.com/EvaLok/schema-org-json-ld-audit/issues/26) — CI bootstrap blind spot | Open |
| Updated state.json | Done |