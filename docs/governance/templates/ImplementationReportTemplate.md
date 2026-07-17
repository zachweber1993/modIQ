# Implementation Report Template

## Purpose

This template defines the standard implementation report format for all engineering work completed on the modIQ platform.

Every implementation task—whether performed by a human engineer or an AI assistant—shall conclude with an implementation report following this structure.

The goal is to ensure consistent code reviews, engineering traceability, and specification compliance.

---

# Implementation Report

## Summary

Provide a concise overview of the feature or change that was implemented.

---

## Files Modified

List every modified file.

Example:

- crates/modiq-runtime/src/assessment/assessment.rs
- crates/modiq-runtime/src/assessment/assessment_error.rs

---

## Public API Changes

Describe any additions, removals, or modifications to public APIs.

If none:

> None.

---

## Specification References

Identify the engineering documents implemented by this work.

Examples:

- RuntimeInvariants.md
- AssessmentCreation.md
- Sprint1.md

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

If none:

> None.

---

## Known Limitations

Document any intentional limitations, deferred work, or future considerations.

Do not list bugs.

---

## Architectural Concerns

Document any conflicts between implementation and the Engineering Specification.

If none:

> None.

---

## Verification

Confirm successful execution of:

- cargo fmt
- cargo check --workspace
- cargo test --workspace

Example:

cargo fmt ✅

cargo check --workspace ✅

cargo test --workspace ✅