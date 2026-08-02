# Frontend Architecture Necessity/Justification Evaluation

| Property | Value |
|---|---|
| **Document** | FRONTEND_ARCHITECTURE_NECESSITY_JUSTIFICATION_EVALUATION.md |
| **Project** | modIQ |
| **Purpose** | Determine whether a frontend architectural layer is a constitutional necessity, grounded in one primary argument and reinforced by converging support — without deciding whether or how to act on that necessity. |
| **Status** | Evaluation complete. Necessity demonstrated. No lineage authorized, no Documentation Authority decision made, no subsequent stage opened. |
| **Precedes** | A Documentation Authority decision, if and when separately sought, on where a Frontend Architecture lineage would live |
| **Follows** | Frontend Architecture Readiness Assessment (this session); `docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md`, Initiative 5 |
| **As of** | 2026-08-01 |

---

## 1. Method

For each candidate responsibility, three tests are applied, in order:

- **(a) Required** — is this responsibility demonstrably required by the frozen Product Design / Interaction Design corpus, or acknowledged as a boundary by the Engineering Architecture itself?
- **(b) Discharged** — does an existing frozen document (Platform Architecture, Product Design, Interaction Design, `Architecture.md`, or Initiative 5's Architectural Resolution) already assign and resolve this responsibility — even only at the ownership level?
- **(c) Consequential** — would leaving it unarchitected risk violating an existing Engineering Principle (determinism, explainability, maintainability, crate-boundary-equivalent discipline) or reproduce the class of drift ADRs and Governance Register items exist to prevent?

| (a) | (b) | (c) | Disposition |
|---|---|---|---|
| false | — | — | **Not Required** — out of scope |
| true | true | — | **Discharged** — no gap |
| true | false | false | **Unclaimed, Inconsequential** — a gap, but ordinary implementation judgment suffices |
| true | false | true | **Unclaimed, Consequential** — evidence toward necessity |

No candidate is designed, specified, or resolved here — only classified. One candidate is treated as the primary constitutional argument; the remainder are treated as converging support, not independently load-bearing.

---

## 2. Analysis

### 2.1 Primary Argument — Enforcement of the Engine/Consumer Responsibility Split (Initiative 5, Decision 2)

**(a) Required.** Initiative 5's Architectural Resolution, Decision 2 (Adopted), binds the consumer to never contain business logic, evaluate Evidence, generate Findings/Recommendations/Reports, or mutate Assessment state directly. This is not a proposal — it is already-adopted repository architecture.

**(b) Not discharged.** The *rule* is adopted. The *mechanism that keeps a real, growing frontend codebase from silently drifting across that line over time* does not exist anywhere. Nothing plays, for the consumer side, the role a Cargo workspace boundary and this repository's Crate Boundary Rules play for every Rust crate.

**(c) Consequential, directly.** This is the exact failure mode the repository's entire crate-boundary discipline — ADR-0007, the Crate Boundary Rules in `GOVERNANCE.md`, the "must never contain business logic" language already governing `modiq-cli` — exists to prevent. It is currently unenforced on the one side of the system that has no structural boundary at all. A rule adopted without an enforcement mechanism is not yet architecture; it is intent.

**Disposition: Unclaimed, Consequential. This is the primary argument.** It alone is sufficient to demonstrate necessity: an already-adopted constraint on the production interaction layer currently has no architectural home in which to be enforced, and no existing frozen document — Platform Architecture, Product Design, Interaction Design, or `Architecture.md` — claims that role.

### 2.2 Converging Support

The following are independently real gaps that reinforce the primary argument without being required to establish it on their own.

**Application shell / composition structure.** The Interaction Design corpus certifies discrete, behaviorally-complete experiences (Intake, Assessing, Finding, Evidence, Report, Navigation) that a user moves *between*. `Architecture.md` places "User" outside its subsystem diagram and names "User interfaces" as an intentionally separated boundary, never elaborated; Platform Architecture disclaims UI behavior outright; Interaction Design excludes implementation decisions by its own charter. No document says what holds the flows together as one running program — and without a shell, the enforcement mechanism the primary argument requires has no structure to attach to. → **Unclaimed, Consequential.**

**Consumer-side state architecture (mechanism, not ownership).** Initiative 5 Decision 4a/4b adopts consumer *ownership* of navigation, interaction, presentation, and workspace state, but explicitly excludes the mechanism — how that state is structured, where it lives — from its own scope, deferring workspace-state derivation to Initiative 1. Unowned state mechanics is the frontend-side analogue of the field-shape divergence Initiative 3 exists to fix on the engine side. → **Unclaimed, Consequential**, with its derivation half not yet ripe pending Initiative 1.

**Navigation / routing implementation.** `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` specifies transition *behavior* in detail but, per Interaction Design's own charter, none of its implementation — no URL structure, deep-linking, or history mechanism is addressed anywhere. An unowned implementation risks silently contradicting behavioral invariants Interaction Design already certified complete. → **Unclaimed, Consequential.**

**Client-side identity/session handling.** `IdentityAndAccess.md` defines Organization/User/Membership conceptually and explicitly defines no auth mechanism. The gap is real but sits on the Platform Architecture lineage's own boundary as much as the frontend's — a cross-lineage question, not resolved here. → **Unclaimed, Consequential, but cross-lineage.**

Two further candidates were considered and do not add support: **presentation rendering/component decomposition** is real but inconsequential on its own — closer to `RuleEngine::evaluate`'s internal dispatch shape (GOV-012's own "implementation detail, not fixed by governance" precedent) than to a boundary question. **Failure/latency presentation discipline** resolves into the shell and state-mechanism items above rather than standing independently.

**Boundary-crossing contract shape** (payload/serialization/transport) is real and consequential in principle, but the Alignment Program itself already named it as downstream of Initiatives 1–4's field-level outcomes and not yet decidable — it is claimed by the same necessity this evaluation demonstrates, but not yet ripe for resolution.

---

## 3. Summary

| Responsibility | Role | Disposition |
|---|---|---|
| Enforcement of engine/consumer responsibility split | **Primary argument** | **Unclaimed, Consequential** |
| Application shell / composition structure | Converging support | Unclaimed, Consequential |
| Consumer-side state architecture (mechanism) | Converging support | Unclaimed, Consequential (partly not yet ripe) |
| Navigation/routing implementation | Converging support | Unclaimed, Consequential |
| Client-side identity/session handling | Converging support | Unclaimed, Consequential — cross-lineage |
| Presentation rendering / component decomposition | Considered | Unclaimed, Inconsequential |
| Failure/latency presentation discipline | Considered | Folds into shell/state items |
| Boundary-crossing contract shape | Considered | Unclaimed, Consequential — not yet ripe |

---

## 4. Verdict

Necessity is demonstrated primarily by one fact: Initiative 5 already adopted a binding constraint on the production interaction layer — the consumer must never contain business logic, evaluate Evidence, or mutate Assessment state — and no existing frozen document provides, or claims to provide, any structural mechanism to enforce it. That is not a stylistic gap; it is an adopted rule with no architectural home, on the one boundary of this system that currently has no structural discipline at all. The shell, state-mechanism, navigation, and identity gaps identified in §2.2 converge on the same conclusion independently, but none of them is required to reach it — the primary argument stands on its own.

This evaluation demonstrates necessity. It does not authorize a Frontend Architecture lineage, does not make or presuppose a Documentation Authority decision, and does not open any subsequent stage. Whether, when, and how to act on the necessity shown here is a separate decision, reserved entirely to whoever holds that authority. Two scope constraints should carry forward into that decision, not be dropped: the contract-shape responsibility (§2.2) is claimed but not yet resolvable, and the identity/session responsibility (§2.2) is claimed but its lineage ownership — Frontend or Platform Architecture — is itself undecided.

---

## 5. Explicit Non-Actions

- No Documentation Map placement decided.
- No governance classification mechanism decided (Initiative 5's own Governance Observation remains open).
- No shell, state, navigation, or boundary-enforcement design performed.
- No Frontend Architecture lineage authorized.
- No Governance Register item opened.
- No ADR proposed.

---

## Subsequent History

This evaluation is preserved exactly as it represented the repository's understanding on 2026-08-01, at the time it was written. Its original Status and Precedes fields, above, are not revised to reflect what followed.

Following this evaluation, in sequence: a Documentation Authority Decision determined Frontend Architecture to be a Governed Specification of `Architecture.md`; a Frontend Architecture Evaluation dispositioned every candidate responsibility as Architecturally Ready, Blocked, or Excluded; a Frontend Architecture Architectural Resolution converted that Evaluation into binding determinations; and `docs/architecture/FrontendArchitecture.md` was synchronized into the repository as the resulting governing specification. This evaluation's own necessity finding was the fixed evidentiary basis for all four, and was not reopened by any of them.
