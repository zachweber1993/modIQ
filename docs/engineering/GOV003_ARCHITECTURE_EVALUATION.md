# GOV-003 Architecture Evaluation — Role of `modiq-common`

| Property | Value |
|---|---|
| **Document** | GOV003_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Governance item addressed** | GOV-003 (`docs/engineering/GOVERNANCE.md`) — "Role of `modiq-common`," Open since Engineering Release v0.1.0-alpha (Sprint 1), 13 Sprints unresolved |
| **Purpose** | Architecture Evaluation of whether `modiq-common` should remain the repository for shared platform types, be retired, or be reconsidered on some other basis — following `SPRINT_14_PROPOSAL.md` Candidate C and Sprint 14's own formal objective selection (`docs/engineering/ENGINEERING_LOG.md`, "Sprint 14 — Objective Selected: GOV-003") |
| **Origin** | Chief Architect authorization, following Sprint 14 objective selection |
| **Status** | **Architecture Evaluation complete; Architectural Resolution complete — Option A (retire) Accepted by the Chief Architect (Section 6). Implementation Authorization and Sprint Planning follow as separate documents. No `Cargo.toml` change, no crate removal, no implementation of any kind has occurred as of this document.** |

---

## 1. Evaluation Scope

Per this project's own Decision Framework (`CHIEF_ARCHITECT_HANDOFF_v1.1.md` §6), this document evaluates concrete alternatives against repository evidence and recommends one — it does not decide. It addresses exactly the question GOV-003 already asks: *"Should `modiq-common` become the repository for shared platform types, or should its architectural purpose be reconsidered?"*

**Deliberately excluded:**
- No Sprint Planning, no implementation plan, no phasing.
- No decision on what (if anything) should eventually live in `modiq-common`, should it be retained.
- No ADR — per `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §4, not every resolved GOV item requires one, and this evaluation does not presuppose that GOV-003's resolution will establish a new durable principle rather than apply existing ones (Capability-before-abstraction, Repository truth).

---

## 2. Evidence, Verified Directly This Session

- **Workspace membership:** `modiq-common` is declared in the root `Cargo.toml`'s `members` list.
- **Zero consumers, confirmed directly:** `grep -rn "modiq-common" --include="Cargo.toml" .` across the entire repository returns exactly one match — `modiq-common`'s own manifest. No other crate in the ten-crate workspace (`modiq-runtime`, `modiq-collection`, `modiq-rules`, `modiq-report`, `modiq-engine`, `modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-storage`) depends on it, nor does `apps/sandbox`.
- **Zero real content, confirmed directly:** `crates/modiq-common/src/` contains exactly four files:
  - `lib.rs` — `//! Shared platform types.` plus three `pub mod` declarations (`error`, `id`, `prelude`).
  - `error.rs` — `//! Shared error types.` and nothing else.
  - `id.rs` — `//! Shared identifier types.` and nothing else.
  - `prelude.rs` — `//! Common imports.` and nothing else.

  Each module is a single doc comment. No struct, enum, trait, function, or constant exists anywhere in the crate. This is qualitatively different from `modiq-versioning`'s and `modiq-knowledge`'s own pre-Sprint-8/9 state — both of those crates are on record as scaffolded-but-empty *pending a named, specified architectural dimension* (`VersionProfile`, `RepairRecipe`) already described in `Architecture.md`. `modiq-common`'s own README describes a *purpose* (shared types with no single domain owner) but names no specific type, dimension, or subsystem awaiting activation — there is nothing dormant to activate, only an empty container.
- **Stated design intent (`crates/modiq-common/README.md`):** the crate is meant to stay "intentionally small," to avoid becoming a "general utility" library, and to hold a type only when it is "demonstrably shared across multiple domains and has no single owner." Thirteen Sprints have produced no such type — every cross-cutting concern that has arisen (opaque references, error handling, identifiers) has instead been solved inside the owning domain crate (`modiq-runtime`'s own `AssessmentId`/`EvidenceId`/`FindingId`; ADR-0007's Opaque Runtime References pattern, now four instances, none of which required a shared crate).
- **Historical precedent for on-demand crate creation:** `modiq-collection` did not exist as a scaffold before Sprint 3 — it was created directly when a real boundary was proven (`CrateRoadmap.md`, Sprint 3 Phase 3→4). `modiq-storage` did not exist even as an empty scaffold before Sprint 13 — `STORAGE_ARCHITECTURE_EVALUATION.md` §3 records this as "not unprecedented in the repository generally," citing `modiq-collection` directly. The platform's own demonstrated pattern is that a crate is created when a concrete, evidenced need arrives, not provisioned in advance of one — `modiq-common` is the one instance in the workspace where the reverse happened, and it has not yet been tested against this same standard.
- **Convergent-absence evidence, per this project's own standard:** `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §4 requires *convergent* evidence for governance decisions, "not a single implementation attempt." GOV-004 was decided from three subsystems converging on a shape; `modiq-report`'s Sprint 6 scaffold retirement was decided from confirmed zero real use. GOV-003's own evidence is, if anything, stronger by the same standard: 13 Sprints, ten crates, zero consumers, checked directly, with no single crate ever having reached for it even once.

---

## 3. Alternatives Considered

**Option A — Retire `modiq-common` entirely.** Remove it from the workspace `Cargo.toml` and delete `crates/modiq-common/`. Rationale: the crate has produced zero real content and zero consumers across the entirety of this project's history, confirmed directly, not assumed. This mirrors the exact evidentiary standard and outcome already applied to `modiq-report`'s four unused scaffold types at Sprint 6 (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter` — retired on confirmed zero test-coverage impact) — the same logic applied one level up, to an entire crate rather than four types within one. Consistent with `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §8's "Speculative extensibility" failure mode: "scaffolding built to match a specification's described shape before any real subsystem exists to test that shape against... prefer building the minimal real thing and letting the specification catch up." Should a genuine cross-cutting need arise later, the `modiq-collection`/`modiq-storage` precedent shows the platform creates a crate from nothing without difficulty when the need is real.

**Option B — Retain as deliberately dormant scaffolding, purpose reaffirmed, no change.** Rationale: the crate is architecturally cheap to keep (zero dependents, trivial build cost), and its own README already states a narrow, disciplined promotion criterion. Absence of use to date does not, on this view, prove absence of future need. Risk: this is the same "we'll probably need this" reasoning `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §8 names directly under "Premature abstraction" and "Speculative extensibility" — and it is the reasoning this project has, on record, rejected every other time it has been evaluated against actual evidence (GOV-004, Sprint 6 scaffold retirement).

**Option C — Retain, GOV-003 resolved as "confirmed, no change required."** Distinct from Option B: this treats GOV-003's original question as already answered by the crate's own README (a designed, deliberately-empty placeholder is the intended state, not a gap), the same way GOV-001 was found, this Sprint, to already be answered by existing specification. Risk: unlike GOV-001 — where `DataModel.md`'s Runtime Lifecycle diagram is a specific, checkable, already-authoritative claim the implementation was verified against — no equivalent specification-level document names a concrete future consumer or type for `modiq-common`. There is no "Runtime Lifecycle diagram" equivalent to check conformance against here; the README states a policy for *what kind of type* would qualify, not evidence that one is coming. This option would be affirming a design intent, not verifying a specification claim, and the distinction matters given `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §5's own question: "What would have to be true for this to be wrong, and did anyone check?"

---

## 4. Recommendation

**Option A — retire `modiq-common`.** Of the three alternatives, it is the only one that acts on the evidence rather than around it. The crate's own stated design principle — "only promote a type into `modiq-common` when it is demonstrably shared across multiple domains and has no single owner" — has never once been satisfied in 13 Sprints, and the platform has independently demonstrated, twice (`modiq-collection`, `modiq-storage`), that creating a crate on demand when a real need arrives carries no meaningful cost or friction. Retaining an empty crate indefinitely does not protect against a future need; it only sits, unexercised, as exactly the kind of "reflexive" dependency target `SPRINT_14_PROPOSAL.md` §1.5 itself warned against ("every Sprint it remains unresolved is a Sprint in which a new crate could reflexively be pointed at it out of habit rather than evidence").

This recommendation is offered as **Engineering's own recommendation, not a decision** — consistent with `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §1's standing pattern (GOV-011, GOV-012, GOV-013, and Sprint 12's own Capability Identity procedure were all Engineering-drafted, Chief-Architect-confirmed, never the reverse).

---

## 5. Explicit Non-Goals

- No decision on what, if anything, would replace `modiq-common` as a home for a future genuinely-shared type — that question is deferred entirely, to be answered on demand, per the `modiq-collection`/`modiq-storage` precedent, if and when a real need is evidenced.
- No `Cargo.toml` edit, no file deletion, no crate creation or modification of any kind.
- No ADR.
- No Sprint Planning, no Implementation Authorization.

---

## 6. Chief Architect Decision Summary — Architectural Resolution

Per `PROJECT_HANDOFF_v1.1.md` §5, Architectural Resolution "marks every open architectural question Accepted, Rejected, or Deferred; nothing may carry forward silently into implementation." The Chief Architect has reviewed Section 3's three alternatives and resolved the question.

| Decision | Recommendation | Resolution |
|---|---|---|
| Role of `modiq-common` | Option A — retire the crate | **Accepted** |

**Disposition:** `modiq-common` is retired. Removal is authorized as a subsequent, separately-recorded Implementation Authorization and Sprint Plan (`GOV003_IMPLEMENTATION_AUTHORIZATION.md`, `GOV003_SPRINT_PLAN.md`) — this Resolution decides *that* the crate is retired, not the mechanics of *how* its removal is sequenced or verified, consistent with this document's own Section 1 scope boundary and this project's standing Governance-Decisions-vs-Implementation-Work separation.
