# Audit-side `_notes/`

Archive directory for `state.json::redesign_mode.audit_*_cycle_N` narrative fields older than 5 cycles. Mirrors main repo's `docs/redesign/_notes/` pattern.

Created cycle 215 per [#458](https://github.com/EvaLok/schema-org-json-ld-audit/issues/458) ("state.json passed Read-tool 256KB limit").

## Purpose

`state.json` is append-only. Narrative fields tagged with cycle numbers (e.g., `audit_engagement_cycle_212`) accumulate without decay. By cycle 214 the file had reached 271KB, exceeding the Read tool's default 256KB limit and forcing offset/limit reads.

Archival moves long narrative fields out of `state.json` once they are >5 cycles old, replacing them with one-line pointers. This bounds `state.json` growth from the redesign_mode side.

(The `recommendations.accepted` array is the larger size driver. Its compaction is a separate decision deferred from cycle 215.)

## Convention

- File name: `audit-cycle-N-state.md` where N is the audit cycle number.
- Content: header + the original narrative text, copied verbatim from the state.json field.
- After creation: the corresponding `redesign_mode.audit_*_cycle_N` field in `state.json` is replaced with a one-line pointer like `"phase_1_research_landed_cycle_205": "Archived to docs/redesign/_notes/audit-cycle-205-state.md (cycle 215 archival per #458)"`.

## Threshold (per STARTUP_CHECKLIST Step 13.1, added cycle 215)

- **Advisory at 100KB**: archive narrative fields older than 5 cycles before adding new ones.
- **Mandatory at 200KB**: archive any narrative field older than 5 cycles before commit.
- **Hard limit at 250KB**: do not write new narrative; archive immediately.

The 256KB Read tool limit is the operative ceiling. Thresholds are conservative against it.

## Files

| File | Cycle | Source field(s) |
|---|---|---|
| `audit-cycle-205-state.md` | 205 | `phase_1_research_landed_cycle_205` |
| `audit-cycle-207-state.md` | 207 | `phase_1_phase_2_prep_cycle_207`, `audit_a4_recurrence_filed_cycle_207` |
| `audit-cycle-211-state.md` | 211 | `audit_a4_rate_escalation_cycle_211`, `phase_1_continuation_cycle_211` |
