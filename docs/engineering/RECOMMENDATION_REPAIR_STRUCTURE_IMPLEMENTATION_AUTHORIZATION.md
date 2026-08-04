# Recommendation Repair Structure Implementation Authorization

| Property | Value |
|---|---|
| **Document** | RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Convert the adopted Architectural Resolution ("Runtime-Owned Representation of RepairRecipe-Derived Structure") into a bounded engineering envelope. Authorizes Sprint 24 Planning to translate this principle into a concrete implementation plan. |
| **Origin** | The Architecture Evaluation and Architectural Resolution for "Runtime-Owned Representation of RepairRecipe-Derived Structure" (both this session's fixed inputs, not restated). Initiative 3's Architecture Evaluation, Architectural Resolution, and Implementation Authorization — fixed precedent, not reopened. |
| **Status** | **Approved. Authorizes Sprint 24 Planning within the scope defined below. No Rust type, field name, constructor signature, or code has been produced in preparing this document.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is the Architectural Resolution's own adopted principle, converted into binding form. Nothing in this document revisits how that principle was reached, re-evaluates the rejected alternatives, or reopens any fixed input.

---

## 2. Architectural Basis

Derived exclusively from the Resolution's own dispositions; nothing here is newly reasoned:

- **Adopted principle:** Runtime owns a projected, derived representation of `RepairRecipe`-derived structure, supplementing — never replacing — `Recommendation`'s existing `action` field. The projection occurs exactly once, at Rule-construction time, inside the single crate boundary (`modiq-rules`) that already legitimately sees both `modiq-runtime` and `modiq-knowledge`. Runtime never resolves back to `modiq-knowledge`'s own types at any later point.
- **ADR-0007** governs the shape any new Runtime value must take (plain value-object fields, constructor validation, deterministic construction, identity-based equality where applicable) — confirmed by the Resolution to require no amendment and no new ADR.
- **`GOVERNANCE.md`'s Crate Boundary Rules** are unchanged and fully satisfied without modification — the adopted principle introduces no new dependency edge and crosses no boundary not already permitted.

---

## 3. Authorized Scope

Sprint 24 is authorized to implement:

- A new Runtime-owned value representation, owned by `modiq-runtime`, capturing the informational shape `RepairRecipe`'s steps already carry (a per-step kind and instruction) — its exact type design, field names, and constructor signature are reserved for Sprint Planning, not defined here.
- Population of that representation by the Rule that already constructs a `Recommendation` from real `RepairRecipe` content (today, `VersionCompatibilityRule`), at `Recommendation`-construction time, entirely within `modiq-rules`'s existing dependency on `modiq-knowledge`.
- `Recommendation` continuing to carry `action`, unchanged in meaning and purpose, supplemented — not replaced — by the new representation.
- A corresponding addition to `modiq-storage`'s `PersistedRecommendation`, so that whatever the new representation is, it survives persistence and retrieval. Its own exact shape and migration approach are reserved for Sprint Planning, not defined here.

This authorization is conditioned on all of the following:

- No new Cargo dependency edge is introduced anywhere in the workspace.
- No `modiq-knowledge` type (`RepairRecipe`, `RepairStep`, `RepairStepKind`) is referenced, aliased, or re-exported outside `modiq-knowledge`'s own crate boundary.
- `RepairRecipeReference` remains unresolved everywhere downstream of its construction — no new resolution mechanism is introduced.
- No mutation method is added to `Recommendation`, `Finding`, or `Evidence`.

No responsibility outside this list is authorized by this document.

---

## 4. Explicit Exclusions

**Architecturally out of scope — reserved for a future, separate authorization:**

- IPC transport extension (`apps/console`'s DTOs) — mirroring how Sprint 22's Runtime work and Sprint 23's presentation work were deliberately separated into their own authorizations.
- Frontend presentation of the new representation, in any consumer application.
- Any change to `modiq-report`, `modiq-engine`, `modiq-cli`, `apps/sandbox`, or `apps/console`.
- Any new Rule adopting a `RepairRecipe` beyond `VersionCompatibilityRule`'s own existing usage.
- Any resolution mechanism for `RepairRecipeReference`.
- Any new Cargo dependency edge, anywhere.

**Unrelated, untouched, not implicated by this work:**

- `AssessmentSubject` / Report Identity, Confidence, Initiative 1, Initiative 2, GOV-001, GOV-002, GOV-008, GOV-013, GOV-014, GOV-016.

---

## 5. Participating Crates

**Participating:**

- `modiq-runtime` — new Runtime-owned representation; `Recommendation`'s own shape.
- `modiq-rules` — population at Rule-construction time, at the existing `VersionCompatibilityRule` call site.
- `modiq-storage` — `PersistedRecommendation`'s mirrored addition.

**Not participating — confirmed unchanged by this Authorization:**

- `modiq-knowledge` (`RepairRecipe`, `RepairStep`, `RepairStepKind` untouched)
- `modiq-report`, `modiq-engine`, `modiq-cli`, `modiq-collection`, `modiq-versioning`
- `apps/console`, `apps/sandbox`

---

## 6. Implementation Boundaries

- The projection occurs exactly once, at the single existing crate boundary where both domains are already visible (inside `modiq-rules`, at `Recommendation` construction) — no other location, crate, or later stage may perform it.
- The new representation is populated at construction only, consistent with every existing Runtime entity's constructor-validation pattern (ADR-0007) — no post-construction setter or mutation path.
- `action` remains architecturally valid and unchanged in purpose. It is not deprecated, and no existing consumer of `action()` may be affected by this Sprint's own scope, since transport and presentation are excluded (§4).

---

## 7. Architectural Invariants That Must Not Be Violated

- Runtime must never own Knowledge (`GOVERNANCE.md`) — the new representation must be Runtime's own type, never `modiq-knowledge`'s.
- `RepairRecipe` remains entirely owned by `modiq-knowledge`, unconditionally and without exception.
- No Runtime entity may reference a Knowledge Domain type directly (ADR-0007) — any deviation would require its own ADR, not authorized by this document.
- No new Cargo dependency edge, anywhere in the workspace (Resolution §3, reaffirmed here).
- No mutation method on `Recommendation`, `Finding`, or `Evidence`.
- Aggregate-Owned Relationship Resolution (ADR-0007) is unaffected — this work is a field-level addition to an existing entity, not a new aggregate relationship. Introducing one would exceed this Authorization.

---

## 8. Required Verification Gates

Mirroring this repository's own standing per-task requirement, restated for this Sprint's own scope:

- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test --workspace`
- Direct confirmation (diff/grep, not assertion) that zero new dependency edges exist in any `Cargo.toml` in the workspace.
- Direct confirmation that no reference to `RepairRecipe`, `RepairStep`, or `RepairStepKind` exists anywhere outside `modiq-knowledge`'s own crate boundary — including `use` statements.
- Direct confirmation that `action()`'s existing behavior and every existing consumer (`modiq-cli`, `apps/sandbox`, `apps/console`, `modiq-storage`) remain unaffected, since this Sprint touches `modiq-storage` only for the new mirrored addition, never for `action`'s own handling.

---

## 9. Completion Criteria

- The new Runtime-owned representation exists on or associated with `Recommendation`, populated by `VersionCompatibilityRule` at construction time from real `RepairRecipe` content.
- `Recommendation` continues to expose `action`, unchanged.
- `PersistedRecommendation` carries a corresponding addition, so that the new representation survives persistence and retrieval — its exact shape is Sprint Planning's own decision, not this document's.
- No new Cargo dependency edge exists anywhere in the workspace.
- No `modiq-knowledge` type is referenced outside `modiq-knowledge`.
- Full workspace verification (§8) passes clean.
- No item named in §4 (Explicit Exclusions) is touched, added, or implied.

---

## 10. Expected Next Artifact

**Sprint 24 Implementation Plan** — translating this Authorization's scope into concrete phases, Rust type design, field names, constructor signatures, storage migration detail, and testing strategy. Not produced by this document.

---

## Status

This document defines the engineering envelope for Sprint 24 and authorizes Sprint 24 Planning within it. It does not authorize implementation beyond Section 3, nor any work named in Section 4. No Rust type, field name, constructor signature, or code has been produced in preparing this document.
