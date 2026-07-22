# Implementation Report Template

## Purpose

This template defines the standard implementation report format for all engineering work completed on the modIQ platform.

Every implementation task—whether performed by a human engineer or an AI assistant—shall conclude with an implementation report following this structure.

The goal is to ensure consistent code reviews, engineering traceability, and specification compliance.

**Revision note:** reconciled against reporting practice demonstrated in Sprint 6 (recorded in `ENGINEERING_LOG.md`'s Sprint 6 Closeout entry) and Sprint 7 (`SPRINT7_IMPLEMENTATION_REPORT.md`) — six sections added below reflect real, repeated practice, not theoretical preference; no prior section was removed. See `ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md` for the evidence behind each addition.

---

# Implementation Report

## Summary

Provide a concise overview of the feature or change that was implemented.

---

## Capability Summary

State, in one sentence, what capability this work adds: "After this [Sprint/phase], modIQ can now..." Demonstrated as standing practice since Sprint 7's own Capability Success Criteria — every implementation report should be able to complete this sentence, not only Sprints that introduce a new product-facing capability.

If the work is infrastructure or cleanup with no new user-facing capability:

> No new capability. [State what changed instead — e.g., scaffold retirement, documentation reconciliation.]

---

## Files and Crates Modified

List every modified file, grouped by crate where more than one crate is touched.

Example:

- `crates/modiq-runtime/src/assessment/assessment.rs`
- `crates/modiq-runtime/src/assessment/assessment_error.rs`

---

## Public API Changes

Describe any additions, removals, or modifications to public APIs.

If none:

> None.

---

## Repository Impact

Summarize the change at repository scale: which crates were touched, whether any new dependency was added (and why), and the test count before and after. Demonstrated in both Sprint 6 and Sprint 7 reports as a distinct, repeatedly useful summary — not redundant with Files and Crates Modified, which lists individual files rather than aggregate impact.

Example:

> Crates touched: `modiq-collection`, `modiq-engine`. New dependency: `roxmltree` (XML parsing, authorized in `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md`). Tests: 172 → 187.

---

## Specification References

Identify the engineering documents implemented by this work.

Examples:

- `RuntimeInvariants.md`
- `AssessmentCreation.md`
- `EvidenceCollection.md`

If none:

> None.

---

## Invariants Implemented

List every runtime invariant enforced by this implementation.

Example:

- INV-001
- INV-002
- INV-003

If no invariants are affected:

> None.

---

## Tests Added

Summarize all new or modified tests.

Include both successful-path and failure-path testing where applicable.

---

## Design Decisions

Describe implementation choices that may affect future engineering work.

Focus on decisions rather than implementation details.

---

## Assumptions Made

Document any assumptions required because the specification did not explicitly define behavior.

Distinguish explicitly between what was implemented and what remains empirically unvalidated — Sprint 7's Dependency Extraction Status is the model: implementation can be complete while real-world behavioral validation is still pending, and a report should say so plainly rather than implying more certainty than the work supports.

If none:

> None.

---

## Known Limitations

Document any intentional limitations, deferred work, or future considerations.

Do not list bugs.

---

## Architectural Validation

Where the work was preceded by an Architecture Evaluation or Architectural Resolution stage that made specific predictions (e.g., "a coordinator component is unnecessary," "existing boundaries are sufficient"), record whether implementation confirmed or disproved each one. Demonstrated in Sprint 7's own report as a genuinely useful, previously-missing section — the place a disproven prediction would be recorded, not only confirmed ones.

If no architectural predictions preceded this work:

> Not applicable — no prior Architecture Evaluation or Architectural Resolution stage for this work.

---

## Architectural Concerns

Document any conflicts between implementation and the Engineering Specification.

If none:

> None.

---

## Governance Observations

State plainly whether this work's evidence indicates any Governance Register item, ADR, or crate boundary rule should change. Demonstrated in both Sprint 6 and Sprint 7 reports; if the answer is no, that is itself the required content of this section — say so explicitly rather than omitting the section.

If none:

> No Governance Register, ADR, or crate boundary changes are indicated by this work's evidence.

---

## Implementation Constraints

Confirm that constraints stated at Implementation Authorization were honored (scope boundary, no new dependency beyond what was authorized, no speculative abstraction introduced). If a constraint could not be honored, state why and what was reported instead of silently working around it.

---

## Recommendations

State what this work's evidence suggests for future Sprints — a follow-up validation step, a deferred cleanup item, a governance item worth opening once further evidence exists. Demonstrated in both Sprint 6 and Sprint 7 reports as a distinct closing section.

If none:

> None.

---

## Validation Summary

Confirm successful execution of, with before/after test counts, both workspaces where applicable:

- `cargo fmt` (or `cargo fmt --check`)
- `cargo check --workspace`
- `cargo test --workspace`
- `apps/sandbox/src-tauri` equivalents, where the change could plausibly affect the Sandbox

Example:

cargo fmt ✅

cargo check --workspace ✅ (zero warnings)

cargo test --workspace ✅ (172 → 187 passed)

apps/sandbox/src-tauri ✅ (6 → 7 passed)
