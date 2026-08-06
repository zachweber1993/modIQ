# C2 Implementation Authorization — Declared Dependency Interpretation

| Property | Value |
|---|---|
| **Document** | C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md |
| **Project** | modIQ |
| **Purpose** | Convert `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own adopted decisions into a bounded engineering envelope. Authorizes implementation in principle — participating crates, exclusions, constraints, verification gates, and completion criteria — and nothing beyond that. |
| **Origin** | `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md` (committed `7c7faf8`) and `C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md` (committed `2cf8ba8`), both treated as fixed, unreopened. `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` — drafted, Repository-Validation-Reviewed, targeted-reconciled, and Post-Reconciliation-Verified, all complete — is the fully validated basis this Authorization is drafted against. Its own commit is, as of this Authorization's own drafting, still outstanding (confirmed via `git status`: the file remains untracked). This document's own repository approval accordingly depends on that Resolution first entering repository history as a committed artifact. This Authorization does not itself establish, and does not attempt to establish, what evidentiary standard a claim of repository fact must meet before that commit occurs — that question is `GOVERNANCE.md`'s own GOV-016 (Open), left unresolved and unchanged by anything in this document. |
| **Status** | **Approved, conditioned on `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` entering repository history as a committed artifact.** Authorizes C2 Implementation Planning within the scope defined below, once that condition is met. No Rust type, matching logic, prefix, severity, Mod Health Dimension, Repair Recipe, or code has been produced in preparing this document. |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own conclusion, converted into binding form. Nothing in this document revisits how that Resolution's decisions were reached, reweighs its dispositioned alternatives, or reopens the shared-`EvidenceCategory` composition question it already resolved.

This document's own organization draws on this repository's existing family of Implementation Authorization precedents — `C1_IMPLEMENTATION_AUTHORIZATION.md`'s condensed shape, `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md`'s fuller shape, and `STORAGE_IMPLEMENTATION_AUTHORIZATION.md`'s differently-organized shape — no single one of which is this repository's sole canonical template for the document type. This document's own section structure is a synthesis suited to this capability's own scope, not a claim that one exact, uniform structure governs every Implementation Authorization.

---

## 2. Architectural Basis

Derived exclusively from already-Adopted determinations; nothing here is newly reasoned:

- **GOV-012, interpreted and adopted by the Architectural Resolution (Decision 1) as already governing Rule dispatch and composition for a shared `EvidenceCategory`.** No new dispatch mechanism, precedence rule, or suppression model is required or permitted. This is the entire basis for authorizing a second `EvidenceCategory::XmlInspection`-interpreting Rule to coexist with `VersionCompatibilityRule`.
- **The Architectural Resolution's Decision 2 — the Rule Conclusion Non-Contradiction Constraint.** No single Evidence item may support mutually contradictory specific conclusions from two different Rules; genuinely independent, non-contradictory conclusions about the same Evidence item are not barred. This constrains Rule *authorship*; it introduces no new mechanism into `RuleEngine::evaluate`.
- **`RuleEngine::evaluate`'s existing fixed-order, independently-composing dispatch** (GOV-012, ADR-0010, GOV-004) — confirmed unchanged and sufficient by the Architectural Resolution's own Decision 1; the new Rule is authorized as one additional, fixed-order `if let Some(outcome) = ... { outcomes.push(outcome); }` dispatch line, matching the statement shape every existing Rule already uses. Its own parameter list — `evidence` alone, matching three of the four existing Rules, or `evidence` and `version_profile` together, matching `VersionCompatibilityRule` alone — is not fixed by this authorization.
- **`CAPABILITY_DEFINITION_C2...md`'s own Capability Statement** — "The Rule Engine gains the ability to ask a question of a mod's own declared-dependency content... rather than that content being interpreted only by `EvidencePresenceRule`'s undifferentiated... match" — remains the fixed scope of what is being built.
- **`GOVERNANCE.md`'s Rule Engine Crate Boundary Rule** — "owns deterministic evaluation, rule execution, rule outcomes... must never mutate Assessment directly" — squarely covers this work; not extended or reinterpreted by it.

---

## 3. Authorized Scope

Implementation Planning is authorized to scope work that:

- Introduces one new Rule in `modiq-rules`, dispatched from `RuleEngine::evaluate` as a fifth fixed-order `if let Some(outcome) = NewRule.evaluate(...) { outcomes.push(outcome); }` line — whether the new Rule's own `evaluate` signature takes `evidence` alone or `evidence` and `version_profile` together is a Rule-design question reserved for Implementation Planning (§2), not decided here — interpreting `EvidenceCategory::XmlInspection` Evidence specifically bearing the `"Declared Dependency"` label and `"modDesc.xml declares dependency: "` description prefix — the Evidence `XmlCollector` already produces, unconditionally, for every non-empty declared `<dependency>` element.
- Produces, for Evidence its own content-shape match recognizes as warranting a conclusion, a Finding whose title, summary, and evidence references are specifically about that declared-dependency content — distinguishable from `EvidencePresenceRule`'s undifferentiated "Evidence collected" Finding.
- May, but is not required to, produce a paired Recommendation for that Finding (Engineering Alignment, Initiative 3, Item 3 — a Finding may stand alone), and may, but is not required to, cite a `RepairRecipe` from `modiq-knowledge` for that Recommendation.
- Selects a severity and Mod Health Dimension for the new Finding, subject to `DataModel.md`'s Finding Severity discipline: a Rule "must never assign a severity stronger than what its Evidence conclusively establishes."

This authorization is conditioned on all of the following:

- The new Rule's own content-shape match (its second filtering stage, after the `EvidenceCategory` match) is verified by its own author against every existing Rule already interpreting `XmlInspection` — currently `VersionCompatibilityRule` alone — and satisfies the Architectural Resolution's Decision 2: no single Evidence item may support a contradictory conclusion from the new Rule and `VersionCompatibilityRule` (or any other future `XmlInspection`-interpreting Rule).
- No change to `XmlCollector`, `EvidenceCategory`, or any part of `RuleEngine::evaluate`'s dispatch shape beyond the one new, additive dispatch line.
- No change to any consumer's transport code (`modiq-report`, `modiq-storage`, `modiq-cli`, `apps/console`, `apps/sandbox`) is required, authorized, or permitted for the new Finding to become reachable — it must reach every consumer through their existing, already-generic handling of `Finding`/`Evidence`, exactly as every historical Rule addition has already demonstrated.

No responsibility outside this list is authorized by this document.

---

## 4. Participating Crates

**Participating:**

- `modiq-rules` (`crates/modiq-rules`) — the new Rule's own module, and the one new dispatch line in `engine.rs`.
- `modiq-knowledge` (`crates/modiq-knowledge`) — participating only if Implementation Planning determines the new Rule's Recommendation should cite a `RepairRecipe`; not required by this Authorization.

**Not participating — confirmed unchanged by this Authorization:**

- `modiq-runtime` — `Finding`, `Evidence`, `EvidenceCategory`, `Recommendation`, and every other Runtime type are consumed as-is; no schema change.
- `modiq-collection` — `XmlCollector` produces the Evidence this capability concerns already; untouched.
- `modiq-versioning`, `modiq-report`, `modiq-engine`, `modiq-storage`, `modiq-cli`.
- `apps/console`, `apps/sandbox` (both separate workspaces, out of scope).

---

## 5. Explicit Exclusions

**Architecturally blocked or reserved — unaffected by this document:**

- **Cross-mod dependency resolution** (Capability Portfolio Assessment's own C5) — `AssessmentSubject` remains a zero-field unit struct with no identifying content, and no Evidence records any fact about a second mod; nothing available to this Rule makes cross-mod resolution reachable, exactly as `CAPABILITY_DEFINITION_C2...md`'s own Explicit Exclusions already established.
- **Activating `DependencyResolution`** (the dormant `EvidenceCategory` variant) or any other dormant category — unrelated; declared-dependency Evidence remains, and will remain, `XmlInspection`.
- **Any change to `XmlCollector`**, including recording currently-dropped empty `<dependency>` declarations as Evidence — a Collection-Axis change, not authorized here.
- **GOV-013** (`FindingSeverity` Severity/Kind Conflation) — not reopened by this Authorization or the work it authorizes.
- **The Architectural Resolution's own two recommended, not-performed follow-ups** — a `RuleEngine.md` amendment recording Decision 2 as architecture text, and a `GOVERNANCE.md` cross-reference from GOV-012 to the Resolution — remain separate, later housekeeping steps, not part of this implementation and not a precondition for it.
- **`SPRINT12.md`'s and `SPRINT12_ARCHITECTURAL_RESOLUTION.md`'s own Historical Validation tables** — the Architectural Resolution's Decision 3 names updating them, once this Rule ships, as a future, separate, non-blocking step. Not performed by, and not a completion criterion of, this implementation.
- **C1's own remaining deferred items** (`apps/sandbox`/`modiq-cli` parity for `RecommendationStep`), **C6 (Confidence), or any other Capability Portfolio Assessment candidate.**

**Outside the scope of this authorization by implementation scope — no architectural decision is required for any of these; they simply are not part of this work:**

- The exact Rust type, matching logic, prefix string, severity, Mod Health Dimension, or `RepairRecipe` content for the new Rule — all reserved for Implementation Planning.
- Any presentation, visual, or interaction treatment in `apps/console`, `modiq-cli`, or `apps/sandbox` distinguishing the new Finding from any other — generic presentation already reaches it; a dedicated treatment is a separate, future question.
- Any Sprint Plan, phase breakdown, or testing sequencing beyond §11's own boundary-level requirement.

---

## 6. Architectural Constraints That Must Not Be Violated

- **Rule Conclusion Non-Contradiction** (Architectural Resolution, Decision 2) — no single Evidence item may support mutually contradictory specific conclusions from the new Rule and any existing Rule interpreting the same category. Non-contradictory overlap (two independently true, unrelated conclusions about the same Evidence item) is not itself a violation.
- **`RuleEngine::evaluate`'s fixed-order, independently-composing dispatch** — no suppression model, no precedence mechanism, no trait, registry, or plugin mechanism (GOV-012, ADR-0010, GOV-004).
- **The Evidence Collection boundary** (`GOVERNANCE.md`) — `XmlCollector` "must never evaluate Evidence, or produce Findings or Recommendations"; unaffected and unextended by this work, which is Interpretation-Axis only.
- **`DataModel.md`'s Finding Severity discipline** — the new Rule must never assign a severity its Evidence does not conclusively establish.
- **`Finding`'s existing invariant that it may stand without a Recommendation** (Initiative 3, Item 3) — remains available to the new Rule; not mandatory.
- **The Runtime/Knowledge boundary** — any `RepairRecipe` content the new Rule's Recommendation cites must be authored in `modiq-knowledge`, consumed by `modiq-rules`, never the reverse (mirroring `VersionCompatibilityRule`'s own precedent); an inline-authored Recommendation with no `RepairRecipe` (mirroring `RuntimeLoadFailureRule`'s own precedent) remains equally available and requires no `modiq-knowledge` participation.

---

## 7. Implementation Boundaries

The following remain unchanged by this implementation, as a direct consequence of already-adopted architecture:

- **The Collector Contract and `XmlCollector` itself** — untouched; declared-dependency Evidence is already produced in full.
- **`EvidenceCategory`'s closed set** — untouched; no new or dormant category activated.
- **`modiq-report`, `modiq-storage`, `modiq-cli`, `apps/console`, `apps/sandbox`** — untouched; the new Finding must reach every consumer through already-generic `Finding`/`Evidence` handling, with zero code change, exactly as demonstrated for every historical Rule addition (Sprint 5, 8, 11) and re-confirmed directly, this session's own predecessor documents, against: `modiq-cli`'s own output code (`assess.rs`, `retrieve.rs`, `history.rs`); `apps/console`'s own IPC transport layer (`assessment.rs`, the Rust/TypeScript boundary) and its TypeScript interface mirror (`engine/types.ts`); `apps/console`'s own presentation components consuming that already-mirrored transport (`Reviewing.tsx`, `Overview.tsx`); and `apps/sandbox`'s own transport layer (`lib.rs`).
- **Console's Frontend Architecture boundary** (`FrontendArchitecture.md`) — unaffected; no presentation change is authorized or required.
- **GOV-012, `RuleEngine.md`** — not amended by this implementation; their own recommended future amendments (§5) remain separate, later, non-blocking steps.

---

## 8. Required Verification Gates

- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — both the root workspace and `apps/sandbox/src-tauri`'s own separate workspace (`PROJECT_HANDOFF_v1.1.md` §5's standing discipline).
- New, real-fixture tests extended within `modiq-rules`'s own existing testing convention — no mocking of Evidence content (Real-I/O Testing Discipline).
- Direct confirmation (diff/grep, not assertion) that the new Rule's own content-shape match does not overlap `VersionCompatibilityRule`'s own `descVersion`-prefix match against any real or test-fixture Evidence — a checkable demonstration of Decision 2's Non-Contradiction Constraint, not an assumption of it.
- Direct confirmation that `RuleEngine::evaluate` remains a fixed-order sequence of `if let` statements — no trait, registry, or dispatch table introduced.
- Direct confirmation that no consumer file required any modification for the new Finding to appear — neither transport (`modiq-cli`'s `assess.rs`/`retrieve.rs`/`history.rs`; `apps/console`'s `assessment.rs` and `engine/types.ts`; `apps/sandbox`'s `lib.rs`) nor the presentation components consuming that transport (`apps/console`'s `Reviewing.tsx`, `Overview.tsx`).

---

## 9. Completion Criteria

- A Finding exists, produced by the new Rule, whose title, summary, and evidence references are specifically about a mod's declared-dependency content — distinguishable from `EvidencePresenceRule`'s own undifferentiated Finding.
- An Assessment Subject with no declared dependencies, or with declarations the new Rule's own judgment finds nothing to conclude about, produces no such Finding — a valid, expected outcome, not a gap to fill (mirroring `VersionCompatibilityRule`'s and `RuntimeLoadFailureRule`'s own precedent of returning `None`).
- The new Rule's severity assignment is honestly grounded in what declared-dependency Evidence alone conclusively establishes (§6).
- The Rule Conclusion Non-Contradiction Constraint is satisfied and directly, verifiably demonstrated (§8) — not merely asserted.
- Full verification gates (§8) pass clean.
- No item named in §5 (Explicit Exclusions) is touched, added, or implied.

---

## 10. Repository Impact

| Area | Expected Impact |
|---|---|
| `modiq-rules` | New Rule module; one new dispatch line in `engine.rs`; new tests. |
| `modiq-knowledge` | Unchanged, unless Implementation Planning authorizes a new `RepairRecipe` for the new Rule's Recommendation — not required. |
| `modiq-runtime`, `modiq-collection`, `modiq-versioning`, `modiq-report`, `modiq-engine`, `modiq-storage`, `modiq-cli` | Unchanged. |
| `apps/console`, `apps/sandbox` | Unchanged; out of scope. |
| `GOVERNANCE.md`, `RuleEngine.md` | Unchanged by this implementation. Their own recommended amendments (Architectural Resolution §7) remain separate, later, non-blocking steps. |
| `SPRINT12.md` / `SPRINT12_ARCHITECTURAL_RESOLUTION.md` Historical Validation tables | Unchanged by this implementation. Eligible for a future, separate update once the new Rule ships, per the Architectural Resolution's own Decision 3 — not a completion criterion here. |
| ADRs, Governance Register | None created, none modified. |

---

## 11. Required Implementation Phases

Implementation is authorized to proceed through one or more independently verified implementation phases. Whatever number is used, each phase — or the single phase, if only one is used — must be independently verified (`cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`, both workspaces) before the next begins, or before completion, respectively — this repository's own standing discipline (`PROJECT_HANDOFF_v1.1.md` §5).

The exact phase structure — how many phases this scope warrants, if more than one; their boundaries; and where the new Rule's own construction, its dispatch wiring, and any Recommendation/`RepairRecipe` content are sequenced relative to one another — is not decided here. That determination, translating this Authorization's scope into concrete phases, belongs to the Implementation Plan (§12), exactly as `C1_IMPLEMENTATION_AUTHORIZATION.md` §9 reserved the equivalent determination for `C1_IMPLEMENTATION_PLAN.md`. Nothing in this Authorization presumes this capability's own scope requires more than a single phase.

---

## 12. Expected Next Artifact

**C2 Implementation Plan** — translating this Authorization's scope into concrete phases, the new Rule's own Rust type, matching logic, prefix, severity, and Mod Health Dimension design, and testing sequencing. Not produced by this document.

---

## Status

This document defines the engineering envelope for C2's implementation and authorizes Implementation Planning within it, conditioned on `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` first entering repository history as a committed artifact (Origin, above). It does not authorize implementation beyond Section 3, nor any work named in Section 5. No Rust type, matching logic, prefix, severity, Mod Health Dimension, `RepairRecipe`, or code has been produced in preparing this document.
