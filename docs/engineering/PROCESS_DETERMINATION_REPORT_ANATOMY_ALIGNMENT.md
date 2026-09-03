# PROCESS_DETERMINATION_REPORT_ANATOMY_ALIGNMENT.md

## 1. Capability Under Review

Report Anatomy Alignment — eight console-presentation behaviors bringing Overview/Finding/Evidence display into conformance with already-frozen product/interaction design, per `CAPABILITY_DEFINITION_REPORT_ANATOMY_ALIGNMENT.md` (Determination A).

## 2. Repository Baseline

Branch `feature/runtime-implementation`. Unchanged since the Capability Definition.

## 3. Capability Re-Grounding

Re-confirmed: all eight behaviors consume data already present in the current transport (`FindingSummary`/`EvidenceSummary`), require no Runtime/Storage/transport change, and touch only `apps/console`. Re-confirmed: `apps/console/package.json` declares no component-testing framework (no Vitest, Jest, React Testing Library, or Playwright).

**Governing methodology found, not invented:** this repository has already performed this exact determination three times before — `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`, and its reapplications for C3 and C4 — each explicitly stating the test is reusable ("This determination is answered by applying the same test the repository already used... not a newly invented one"). That same two-prong test is applied fresh below, not assumed to transfer by analogy. One precise clarification carried forward from C3's own document: **"Process Determination" is not a canonical stage in `PROJECT_HANDOFF_v1.1.md`'s own workflow list** — it is a committed precedent (used three times), and what is reused here is its test, not an assumption that this document type is itself mandatory.

## 4. Process Determination Matrix

| Stage | Status | Evidence | Precedent | Trigger to Escalate |
|---|---|---|---|---|
| Architecture Evaluation | **Not Required** | §5 | C1, C3, C4 (skipped); contrast C2, C12 (required) | A future item is found to require a new transport shape, a new inference, or a new engine/consumer boundary decision |
| Architectural Resolution | **Not Required** | Follows from §5 | Same as above | Same as above |
| Implementation Authorization | **Required** | Standard next artifact per C1/C3/C4's identical path | C1, C3, C4 | N/A — always required before implementation |
| Implementation Plan | **Required** | Standard | C1, C3, C4 | N/A |
| Implementation | **Required** (not performed here) | — | — | — |
| Implementation Report | **Required** | Standard closeout artifact | C1, C3, C4 | N/A |
| Engineering Release | **Required** | Standard closeout artifact | C1 (69e33da), C3 (4c46283), C4 (83925c0) | N/A |

## 5. Architecture Evaluation Determination

Applying the established two-prong test fresh:

**Prong 1 — genuinely open design question, or governing principle already Adopted?** `FrontendArchitecture.md`'s Consumer-Owned State and Explainable Continuity already, non-discretionarily, settle that a consumer may display already-engine-produced content, provided nothing is inferred. All eight behaviors reorganize or relocate data the engine already produced and already delivers (`modHealthDimension`, `status`, `recommendation.action`, `evidence.length`, `evidence.content`) — none introduces new engine-side computation or a new fact the consumer originates. **This capability's Prong-1 case is cleaner than any of its three precedents:** C1 had one genuine transport-shape wrinkle to resolve (a new nested DTO for `repair_steps`); C3 had one residual field-shape gap to check (also resolved, no boundary to cross); Report Anatomy Alignment requires **zero** new fields and **zero** transport changes — every field consumed already crosses the IPC boundary today. GOV-008's reservation over "the concrete payload shape" is not even touched, since nothing about the shape changes. The Initiative 1/2 reservations (concerning *when*/*whether* a Report is live or evolving) are inapplicable for the same reason C1 found them inapplicable — this capability concerns only *how* an already-completed Report's existing content is arranged and disclosed.

**Prong 2 — is any alternative being weighed?** No. Nothing proposes a different engine/consumer boundary, state-ownership model, or navigation mechanism — it reorganizes presentation of already-adopted Transport Mirror data within the already-adopted single-expansion-layer pattern.

**Determination:** matches the C1/C3/C4 path on both prongs, more cleanly than any of the three. No Architecture Evaluation or Architectural Resolution required.

**Contrast, confirming the boundary is real, not convenient:** C2 required Architecture Evaluation because it added a *new Rule* interpreting evidence, raising a genuine, previously-unresolved question about Rule composition under GOV-012. C12 required it because it evolved a *persisted schema*, raising a genuine backward-compatibility question. Report Anatomy Alignment adds no new Rule, Collector, evidence category, or persisted field — it is presentation-only, squarely in the C1/C3/C4 category, not the C2/C12 category.

## 6. Architectural Resolution Determination

Not required — follows directly from §5; there is no Evaluation output to resolve.

## 7. Implementation Authorization Determination

Required, as the standard next artifact in every precedent path examined (C1, C3, C4 each proceeded Capability Definition → [this determination] → Implementation Authorization directly).

## 8. Implementation Plan Determination

Required, as the standard artifact following Implementation Authorization in every precedent examined. Not produced here.

## 9. Verification / Testing Process Determination

**Directly answering the six questions posed, using established precedent rather than inventing a new standard:**

1. **Does absence of frontend tests block implementation?** No — `apps/console` has never had a component-testing framework, and C1 (which also modified `Reviewing.tsx`) proceeded to full closeout under exactly this condition, explicitly recorded as "consistent with Sprint 23's own precedent."
2. **Does the capability require establishing frontend test infrastructure?** No — C1's Implementation Report states this directly: "introducing one is outside this Authorization's scope," for the identical underlying reason (no framework exists, and this capability does not itself need one to complete).
3. **Would introducing a frontend test framework constitute...?** Based on twice-applied precedent, it would be its own separate decision — not bundled into a presentation-alignment capability. This determination does not classify it further (architecture vs. process vs. tooling) since no precedent instance has needed to.
4. **Can the eight behaviors be verified reliably without a new stack?** Yes — the already-used method is direct, real-data tracing against actual fixture output (the same interpretive method used at Sprint 23's and C1's closeout), combined with the mechanical build-clean gates below.
5. **What verification discipline should Implementation Authorization require?** The same Required Verification Gates shape C1 used: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`; `apps/console`'s `npm run build` (`tsc && vite build`) clean; and each of the eight behaviors specifically traced against real fixture output, not mocked — consistent with this repository's standing Real-I/O Testing Discipline.
6. **Minimum defensible verification standard:** exactly the above — no new tooling, no new framework, direct tracing against real data for behavioral confirmation, compiler/build-clean gates for mechanical correctness.

This is a process consideration to carry into Implementation Authorization's own Required Verification Gates section, not a decision made here.

## 10. Transport and GOV-008 Boundary

Report Anatomy Alignment can safely proceed against the current provisional transport because it consumes existing fields only, introduces no new field, and does not touch the payload shape GOV-008 reserves. The exact boundary: any future item requiring a field not already in `FindingSummary`/`EvidenceSummary`/`ReportSummary` today falls outside this capability and would itself raise the transport-shape question GOV-008 reserves — none of the eight behaviors defined does. GOV-008 is not resolved, touched, or strained by this determination.

## 11. Capability Boundary Test (Re-Verification)

Re-checked, not merely re-asserted: all eight behaviors remain coherent (single Reviewing-state hierarchy, same three governing documents), independently meaningful, already specified, architecturally supported (§5), governance-clear (§10), testable (§9), and bounded. No behavior requires removal, splitting, or conditional handling — none surfaced a hidden dependency on an excluded item during this re-verification.

## 12. Lifecycle Precedent

| Capability | Shape | AE/AR? | Basis |
|---|---|---|---|
| C1 | Console presentation of already-engine-produced data | No | Two-prong test, satisfied with one resolved wrinkle (new nested DTO) |
| C2 | New Rule interpreting existing evidence | **Yes** | Genuine open question: Rule composition safety under GOV-012 |
| C3 | CLI field parity | No | Two-prong test, satisfied; no transport boundary at all |
| C4 | Sandbox field parity | No | Two-prong test, satisfied; inherited C3's settled boundary answers |
| C12 | Persisted schema evolution | **Yes** | Genuine open question: backward-compatible field addition to Storage |

No single precedent is treated as a universal rule — the pattern across five instances is consistent and the distinguishing principle (new architectural question vs. pure additive presentation) is what does the work, not the count of prior UI-shaped capabilities alone. Report Anatomy Alignment sits unambiguously in the no-AE group, with the cleanest Prong-1 case of any instance examined.

## 13. Minimum Safe Process

Capability Definition → this Process Determination → Implementation Authorization → Implementation Plan → Implementation → Implementation Report → Engineering Release. No stage is skipped relative to precedent; Architecture Evaluation and Architectural Resolution are the two stages this determination finds unnecessary, on the same evidentiary basis this repository has used three times before.

## 14. Findings

**PD-01** — The two-prong test this repository already established (C1) and reused twice (C3, C4) applies cleanly to Report Anatomy Alignment, with no open question on either prong. *Confidence:* High.

**PD-02** — Report Anatomy Alignment's Prong-1 case is stronger than any prior instance — zero new transport fields, versus C1's one resolved wrinkle. *Significance:* reinforces rather than merely matches the no-AE conclusion.

**PD-03** — The distinguishing principle between AE-required (C2, C12) and AE-skipped (C1, C3, C4) capabilities is whether a genuinely new architectural question is raised, not whether the work touches a UI surface. *Significance:* the correct, evidence-grounded test, not an assumption that "UI-only" implies "no AE" or the reverse.

**PD-04** — The absence of a frontend test framework is a real, already-encountered, already-resolved-once condition (C1), not a novel blocker. *Significance:* directly answers the Capability Definition's own open testing question without introducing new tooling.

**PD-05** — This capability touches no field GOV-008 reserves; the transport boundary is exact and does not require GOV-008's resolution. *Confidence:* High.

## 15. Final Determination

**B — Architecture Evaluation is not required; proceed to Implementation Authorization.**

1. **Required lifecycle stages:** Implementation Authorization, Implementation Plan, Implementation, Implementation Report, Engineering Release.
2. **Skipped stages:** Architecture Evaluation, Architectural Resolution — justified by the two-prong test (§5), consistent with C1/C3/C4 and distinguished from C2/C12 on a real, evidence-grounded basis (§12).
3. **Conditional stages and triggers:** none currently active. If a future item within this capability's scope is found to require a new field, a new inference, or a boundary decision, that item would need to be re-evaluated against Architecture Evaluation criteria at that time — no such item was found here.
4. **Why sufficient:** every behavior reorganizes already-produced, already-delivered data within an already-Adopted presentation model; no genuinely open design question exists on repository evidence.
5. **Next immediate artifact:** an Implementation Authorization for Report Anatomy Alignment, following the same procedural shape `C1_IMPLEMENTATION_AUTHORIZATION.md` used, including its own Required Verification Gates section incorporating §9's testing determination.

## Next Process Step

**Implementation Authorization for Report Anatomy Alignment.**
