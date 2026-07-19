# Engineering Release 0.2

| Property | Value |
|----------|-------|
| **Release** | 0.2 |
| **Documentation Release** | 2.0 (unchanged — no frozen specification was modified during Sprint 2) |
| **Milestone** | Sprint 2 complete |
| **Scope** | Runtime Domain content model and Assessment aggregate relationship integration |
| **Predecessor** | ENGINEERING_RELEASE_v0.1.0-alpha.md (Sprint 1) |
| **Governing ADR** | ADR-0007 (Runtime Entity Design Pattern) |
| **Governing Plan** | SPRINT2_IMPLEMENTATION_PLAN.md |

---

## Overview

Sprint 1 proved that the Assessment pipeline could execute end to end using content-free marker types for Evidence, Finding, and Recommendation. Sprint 2 gave those three entities real, evaluable content, and extended `Assessment` with the relationship-resolution behavior an aggregate root needs to make that content useful: resolving a Finding's supporting Evidence and a Recommendation's originating Finding(s) against the Assessment's own state.

This release is an architectural freeze, not a new capability. No Rule Engine decision logic changed, no Reporting behavior changed, no Knowledge Domain integration began, and no new Runtime entity was introduced beyond the three already defined by DataModel.md. Two content-level invariants were identified during implementation and deliberately left unenforced, recorded as open Governance Register items (GOV-005, GOV-006) rather than resolved informally.

---

## Scope

### In Scope

- Real field content for `Evidence`, `Finding`, and `Recommendation` in `modiq-runtime`.
- `EvidenceId`, `FindingId`, `RecommendationId` — process-local, monotonic identity types following the existing `AssessmentId` pattern.
- `EvidenceCategory` and `FindingSeverity` — closed classification enums drawn from Glossary.md.
- `RuleReference` and `RepairRecipeReference` — opaque Runtime-owned reference types.
- Constructor-level validation (`EvidenceError`, `FindingError`, `RecommendationError`).
- `Assessment` relationship-resolution methods: `evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`.
- The minimum necessary production-code changes to `modiq-rules::RuleEngine` (the only component outside `modiq-runtime` that constructs Finding/Recommendation values) and mechanical test-fixture updates across `modiq-rules`, `modiq-report`, and `modiq-engine`.

### Out of Scope (Deferred)

- Rule abstraction and multi-rule dispatch.
- Knowledge Domain integration (`modiq-knowledge` wiring into `modiq-rules`).
- An explainability engine or reasoning fields beyond the approved `description`/`action` fields.
- Reporting evolution (`FindingSummary`, `RecommendationSummary`, `ReportFormatter`, `TraceabilityReport`).
- Persistence, storage architecture, or any persistent identity scheme.
- CLI wiring (`modiq-cli` to `modiq-engine`).
- Version Profile integration (`modiq-versioning`).
- Enforcement of the two governance-pending invariants (GOV-005, GOV-006).

---

## Completed Work

Sprint 2 proceeded in four phases, each reviewed and approved before the next began:

1. **Evidence** — fielded (`id`, `category`, `description`, `location`), with `new`/`with_location` constructors and empty/whitespace validation.
2. **Finding** — fielded (`id`, `severity`, `description`, `evidence_ids`, `rule_reference`), with a single `new` constructor. Required a minimal, non-behavioral change to `modiq-rules::RuleEngine::evaluate`, the sole out-of-crate producer of Finding values, to construct real content instead of a bare marker.
3. **Recommendation** — fielded (`id`, `action`, `finding_ids`, `repair_recipe_reference`), with a single `new` constructor. Required the equivalent minimal change to `RuleEngine::evaluate` for the same reason.
4. **Assessment Aggregate Integration** — added `evidence_by_id`, `finding_by_id`, `evidence_for_finding`, and `findings_for_recommendation`, making Assessment the sole resolver of the cross-entity relationships Finding and Recommendation now carry as stored data.

Each phase's production changes outside `modiq-runtime` were limited to what was strictly required for compilation and correct data flow (e.g., populating a Finding's `evidence_ids` from the Evidence actually evaluated) — never a change to Rule Engine decision logic, which remains exactly as it was in Sprint 1: evidence present → one Finding and one Recommendation; evidence absent → neither.

---

## Architecture Summary

Every crate boundary established by Documentation Release 1.0 and reaffirmed at Engineering Release v0.1.0-alpha held throughout Sprint 2, verified directly rather than assumed:

- Runtime owns state; it does not evaluate. `RuleEngine::evaluate` still takes `&[Evidence]` and returns a value — it never receives or mutates an `Assessment`.
- Rules produce Finding/Recommendation content; they do not own Knowledge. `RuleReference` and `RepairRecipeReference` are opaque values stored by Runtime, not references to `modiq-knowledge` types — `modiq-runtime` gained no new dependency.
- Reporting renders; it performs no analysis. `AssessmentReport::generate` is unchanged.
- No new crate dependency was introduced anywhere in the workspace.

The Runtime entity design pattern established across all three entities — aggregate ownership, entity identity, value objects, opaque references, constructor validation, identity-based equality, aggregate-owned relationship resolution, governance-controlled invariants, and deterministic behavior — is recorded formally in ADR-0007 and is now the standard future Runtime entities are expected to follow.

---

## Runtime Domain Summary

| Entity | Fields | Identity | Validation |
|---|---|---|---|
| `Evidence` | `id`, `category` (`EvidenceCategory`), `description`, `location` (optional) | `EvidenceId` | `description`/`location` must not be empty or whitespace-only |
| `Finding` | `id`, `severity` (`FindingSeverity`), `description`, `evidence_ids` (`Vec<EvidenceId>`), `rule_reference` (`RuleReference`) | `FindingId` | `description` must not be empty or whitespace-only; `evidence_ids` may currently be empty (see Known Deferred Work) |
| `Recommendation` | `id`, `action`, `finding_ids` (`Vec<FindingId>`), `repair_recipe_reference` (optional `RepairRecipeReference`) | `RecommendationId` | `action` must not be empty or whitespace-only; `finding_ids` may currently be empty (see Known Deferred Work) |

All three entities are immutable once constructed and compare equal only by identity — two independently constructed entities with identical content are distinct values. `EvidenceCategory` (`XmlInspection`, `LuaAnalysis`, `RuntimeLogs`, `AssetValidation`, `DependencyResolution`, `PerformanceObservations`, `FileStructureAnalysis`) and `FindingSeverity` (`Error`, `Warning`, `Informational`, `BestPractice`) are direct translations of Glossary.md's Evidence and Finding definitions.

---

## Aggregate Integration Summary

`Assessment` gained four read-only methods, none of which changes its existing mutation behavior (`add_evidence`, `add_finding`, `add_recommendation`, and the lifecycle transitions are byte-for-byte unchanged from Sprint 1):

- `evidence_by_id` / `finding_by_id` — primitive lookups into the aggregate's own collections.
- `evidence_for_finding` — resolves a Finding's `evidence_ids` against the Assessment's Evidence collection.
- `findings_for_recommendation` — resolves a Recommendation's `finding_ids` against the Assessment's Finding collection.

These methods are deliberately **permissive**: a reference that does not resolve is silently omitted from the result, never treated as an error. This reflects a specific, considered scope boundary — see Known Deferred Work.

---

## Testing Summary

The workspace test suite grew from 55 tests (Engineering Release v0.1.0-alpha) to 97 tests, with zero failures at every phase gate:

| Crate | Tests |
|---|---|
| `modiq-runtime` | 84 |
| `modiq-engine` (unit) | 4 |
| `modiq-engine` (integration) | 3 |
| `modiq-report` | 3 |
| `modiq-rules` | 3 |

Testing philosophy from Sprint 1 was preserved without exception: failed operations leave aggregate state unchanged, and every new invariant-adjacent behavior is tested for both its positive and negative case. Two new testing patterns were established this release, specific to real Runtime content:

- **Identity-based equality is explicitly tested**, not assumed — every entity has a test confirming that two independently constructed, identical-content instances are unequal, and a separate test confirming a clone equals its source.
- **Permissive relationship resolution is explicitly tested** — `evidence_for_finding` and `findings_for_recommendation` each have a dedicated test constructing a dangling reference (an id never added to the Assessment) and asserting the method returns an empty result rather than erroring.

A structural note for future work: two "determinism" tests (`modiq-rules::evaluate_is_deterministic_for_identical_input` and both of `modiq-engine`'s "independent and deterministic" tests) required revision during this release, since `RuleEngine` assigns each Finding and Recommendation a fresh identity per invocation by design. These tests now compare content fields explicitly rather than whole-value equality — a pattern future tests involving freshly-constructed identified entities should follow.

---

## Known Deferred Work

Deferred from Sprint 1, still deferred:

- Rule abstraction, multiple Rules, and rule selection.
- Knowledge Domain integration.
- Explainability and Traceability output.
- Engine service expansion (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`).
- Version Profile integration.
- Advanced reporting (`FindingSummary`, `RecommendationSummary`, `ReportFormatter`, `TraceabilityReport`).
- Persistence.
- `modiq-cli` wiring to `modiq-engine`.
- Resolution of `modiq-common`'s architectural purpose (GOV-003).
- Resolution of Engine service granularity (GOV-004).
- Resolution of Assessment Report generation timing (GOV-001).
- Runtime invariant reconciliation with `RuntimeInvariants.md` (GOV-002).

Newly identified and deferred during Sprint 2:

- **GOV-005 — Finding-Evidence reference requirement.** Should a Finding be required to reference at least one Evidence item, validated against the Assessment's own collection? Not enforced; `Finding::new` currently accepts empty `evidence_ids`.
- **GOV-006 — INV-005 refinement.** Should a Recommendation be required to reference specific, existing Finding(s), rather than merely coexist with some Finding in the Assessment (today's enforcement)? Not enforced; `Recommendation::new` currently accepts empty `finding_ids`, and `Assessment::add_recommendation`'s check is unchanged from Sprint 1.

Both are recorded in `GOVERNANCE.md`'s Governance Register and require governance approval — a Level 3 (Behavioral) change — before any enforcement code is written.

Two pre-existing documentation defects were also identified during this release's preparation and are flagged, not corrected, since accepted ADRs are not to be modified per `docs/adrs/README.md`:

- `docs/adrs/0002-domain-model-boundaries.md` internally titles itself "ADR-0001," despite its filename and every cross-reference treating it as ADR-0002.
- `docs/adrs/0006-documentation-release-1.0-freeze.md` references git tag `v0.1.0-docs`, which was never created; `CHANGELOG.md`'s own Documentation Release 2.0 entry already corrected this reference elsewhere but missed the ADR itself.

---

## Sprint 3 Entry Criteria

Sprint 3 should not begin until:

- This release document, the accompanying ADR-0007, and all governance/status documentation updates are reviewed and accepted.
- GOV-005 and GOV-006 have either been resolved through the governance process defined in `GOVERNANCE.md`, or a deliberate decision has been made to defer them further with Sprint 3 scoped around that constraint.
- Sprint 3's actual scope is chosen deliberately from the Known Deferred Work list above (e.g., Rule abstraction, Knowledge integration, or CLI wiring) rather than assumed by default — this release does not select it.
- The workspace remains fully green (`cargo fmt`, `cargo check --workspace`, `cargo test --workspace`), as verified below, and is treated as the baseline Sprint 3 inherits.

---

## Repository Timeline

```
Documentation Release 1.0
        ↓
Sprint 0
        ↓
Sprint 1
        ↓
Engineering Release v0.1.0-alpha
        ↓
Sprint 2
        ↓
Engineering Release 0.2
        ↓
Sprint 3 (pending scope selection)
```
