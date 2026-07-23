# Engineering Release 1.2

| Property | Value |
|---|---|
| **Release** | 1.2 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 12 complete (Capability Scaling Architecture: Capability Identity procedure derived, adversarially reconciled, and validated; Repository Closeout) |
| **Scope** | The platform's first explicit, historically-validated Capability Identity decision procedure — and the first adversarial-verification finding applied to a purely architectural planning document, with zero implementation involved at any point |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.1.md` (Sprint 11) |
| **Governing ADRs** | None new — this Sprint applies the Collector Composition Architecture and GOV-012's Rule dispatch model rather than establishing a new durable principle |
| **Governing Plan** | `docs/implementation/SPRINT12.md` (v1.1.0), `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md` |

---

## 1. Executive Summary

Sprint 11 proved this platform could implement its first real capability in a new architectural axis deterministically. Its own Chief Architect review then found that the naive follow-up question — "how do we compose a third Collector, or a second Rule?" — silently presupposed a classification (extension vs. new participant vs. new axis) this project had made five times without ever stating the procedure. Sprint 12 was authorized to derive that procedure explicitly, from the platform's own completed engineering history, and to resolve Collector and Rule Composition strictly as its consequences.

The first draft of that procedure was not the final one. This Sprint's own adversarial verification pass — checking the draft against every historical decision rather than only their headline labels — found that "Sprint 4" actually contains two separable architectural decisions, and the second (Phase 3C, the Duplicate Archive Entry Policy, GOV-011) directly falsifies the draft's central assumption: that recognizing a new kind of fact always implies a new Collector. Per this project's standing discipline, the contradiction was reported rather than silently resolved. On Chief Architect confirmation, the procedure was corrected to three independent axes, and the Sprint's single most consequential finding followed directly: **Sprint 11 is reclassified from Capability Introduction to Capability Expansion.**

This release document describes both the original proposal and the reconciliation that followed it, deliberately, so the repository's own record shows the platform strengthened through explicit correction — exactly the outcome Sprint 11's own architectural reconciliation modeled, applied here one level further back, to a document that never touched a line of Rust.

---

## 2. Sprint Objective

Per `SPRINT12.md`: derive an explicit Capability Identity decision procedure from the platform's own completed engineering history — not theory, not a hypothetical future capability — distinguishing Capability Enrichment, Capability Expansion, and Capability Introduction, and resolve Collector Composition and Rule Composition strictly as consequences of that procedure. Write no code, add no fixture, modify no test, introduce no runtime mechanism.

---

## 3. Architectural Problem Addressed

Sprint 11's own Chief Architect review found that a direct Collector-Composition/Rule-Composition Sprint objective was reachable only by presupposing an answer to a question neither one asked: how does the platform know a new observation needs a new Collector, or a new Rule, at all? Five real historical decisions had answered this correctly by implicit judgment, never by explicit, citable procedure. Sprint 12 exists to close that gap — evaluated against the platform's own completed history rather than a hypothetical future case, a stronger evidentiary basis than the two questions it replaces would have offered on their own.

---

## 4. Contradiction Discovered During Adversarial Verification

**Where:** the first draft of `SPRINT12.md` (v1.0.0), Section 7 — a single cascading decision tree (Kind test → Mechanism test → Judgment test).

**What the draft assumed:** that recognizing a new kind of fact (implying a new `EvidenceCategory`) was, by itself, sufficient to conclude a new Collector was required — category novelty and Collector novelty treated as one question, not two.

**What falsified it:** Sprint 4 Phase 3C — the Duplicate Archive Entry Policy (`GOVERNANCE.md`, GOV-011). `ArchiveCollector`'s own central-directory enumeration cannot fully resolve an archive containing two identically-named entries (confirmed empirically, independently, across three tools, during Sprint 4 Phase 2's own Boundary-Proving investigation). The platform's real, shipped resolution: represent this as a new, deliberately distinct `EvidenceCategory` — `StructuralDuplication` — produced not by a new Collector, but by *extending the already-existing* `ArchiveCollector` with a second, sequential re-read of the same archive it already opens. The original model's Step 1 predicts a new Collector for a new kind of fact; the real, already-tested outcome is a new category from an unmodified-in-identity, extended Collector. The model has no path that produces this outcome, and is therefore wrong, not merely incomplete.

**How it was handled:** reported, not silently corrected — the same discipline `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`'s own mid-Sprint-11 reconciliation established. Full narrative in Section 5 below and in `SPRINT12_ARCHITECTURAL_RESOLUTION.md`.

---

## 5. Reconciliation Process

1. The contradiction was identified and reported in full — original assumption, contradicting evidence, and the specific outcome the original model could not express — without proposing a silent fix.
2. Chief Architect review confirmed the finding and directed reconciliation, explicitly instructing that the engineering record be preserved, not overwritten.
3. `SPRINT12.md` was amended in place (v1.0.0 → v1.1.0): a Reconciliation Record was added preserving the original model, the falsifying evidence, and the correction; Section 7 was replaced with the corrected model; Sections 4, 5, 8, 9, 10, 12, and 16 were updated for consistency; the Document Status footer records the version history.
4. `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md` was produced as the permanent, detailed record: the full contradiction, the complete seven-decision validation table, and both Secondary Question findings.
5. Both documents were cross-checked for terminology consistency (Collection Axis / Evidence Axis / Interpretation Axis used identically in both) before this release was produced.

---

## 6. Final Capability Identity Model

**Three independent axes**, evaluated separately for any new engineering decision:

- **Collection Axis** — same inspection mechanism suffices → Enrichment (extend the Collector); different mechanism, existing Collector to compare against → Expansion (new sibling Collector); no Collector exists yet → the founding case.
- **Evidence Axis** — independent of the Collection Axis (the axis Sprint 4 Phase 3C proves is separable) — existing `EvidenceCategory` suffices → no change; no existing category fits, or a dormant one activates for the first time → new instance.
- **Interpretation Axis** — same judgment, richer content only → Enrichment (extend the Rule); new judgment → Expansion (sibling Rule, via the two-stage filter pattern every category-specific Rule already uses).

**Capability Introduction — an orthogonal fourth check, not a fourth axis:** reserved for capabilities that require inventing composition or dispatch machinery the platform has never exercised before, independent of how many of the three axes above are individually novel. Sprint 7 is the only confirmed non-degenerate instance (the first time any Assessment ever ran more than one Collector). **Sprint 11 — novel at all three axes — is nonetheless Expansion, not Introduction, because it reused Sprint 7's own additive-composition machinery and GOV-012's own fixed-order dispatch model unchanged.**

**Validated against all seven historical decisions** (Sprint 3, 4, 4 Phase 3C, 7, 8, 9, 11) without contradiction — full table in `SPRINT12_ARCHITECTURAL_RESOLUTION.md`, Section 6.

**Two consequence findings, recorded as Collector and Rule Guidance respectively:** mutually-exclusive routing vs. additive composition (discriminated by whether two Collectors' applicability conditions can co-occur on the same Assessment Input — independent of, and not a replacement for, the unchanged Collector Composition Architecture threshold), and the Interpretation Axis's judgment test now confirmed by four independent instances rather than three, with the "second Rule, same category" case still explicitly disclosed as untested.

---

## 7. Repository Impact

| Area | Change |
|---|---|
| Rust source, tests, fixtures | None touched at any point this Sprint |
| Crates | None added, none removed — 9 workspace crates, unchanged |
| Public entry points / `RuleEngine::evaluate` | Unchanged |
| `EvidenceCategory` / `FindingSeverity` | Unchanged |
| Governance Register | Unchanged — 13 items, 8 Resolved, 5 Open |
| ADRs | None new |
| Collector Composition Architecture, GOV-012, `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` v1.2.0 | All reviewed directly; none amended |
| New documents | `docs/implementation/SPRINT12.md` (v1.1.0), `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`, this release document |
| Governance documentation | `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, `CrateRoadmap.md`, `docs/README.md` — all reconciled |

---

## 8. Validation Status

```
cargo fmt --check      → clean
cargo check --workspace → clean, zero warnings
cargo test --workspace  → 238/238 passing, unchanged from Engineering Release 1.1
```

Sandbox (`apps/sandbox/src-tauri`) reverified separately: 7/7 passing, unchanged. Both results are exactly as expected for a Sprint that touched no source at any point — reconfirmed directly this session, not assumed.

---

## 9. Outstanding Limitations

- **The Capability Introduction test has exactly one confirmed non-degenerate historical instance (Sprint 7).** A real precedent, not yet a statistically robust pattern — the same caveat GOV-004 applied before treating three converging subsystems, not one, as settled.
- **Rule Composition's "second Rule, same already-interpreted category" case remains genuinely untested by direct precedent.** The judgment test's answer for it is a disciplined extrapolation of an already-three-times-proven internal pattern, not a confirmed fifth data point — stated as such, not overstated.
- **This reconciliation does not claim to have exhaustively re-examined every sub-decision in the repository's history** — only the seven named by the Chief Architect. GOV-005/GOV-006 and GOV-013 were not evaluated against this model, since none added a Collector, Rule, or category.
- **The reconciliation itself was found by looking past headline Sprint labels, not by re-deriving from first principles** — itself now a named, recorded methodology observation (`ENGINEERING_LOG.md`) worth future sessions' attention when validating any historically-derived model.

---

## 10. Technical Director Assessment

**This Sprint succeeded on its second pass, and that is the correct, expected outcome of applying this project's own adversarial discipline to itself — including to a document that never touched code.** The corrected model is more precise (it separates two dimensions the original conflated), more conservative in what it calls Introduction (one confirmed instance instead of an implicit three), and better evidenced at the Interpretation Axis (four confirming instances instead of three) than the draft it replaces. The Sprint 11 reclassification is this Sprint's single most consequential finding, and both governance documentation and the Resolution document itself repeat, deliberately, that it changes an architectural label, not Sprint 11's own real product significance.

**Risk:** Low. No code, test, or fixture was touched at any point. The residual risk is intellectual — an unexamined eighth historical sub-decision, or a second-order flaw in the corrected model — named honestly in Section 9, not engineered away.

**Expected repository impact realized exactly as planned:** two new architecture documents, five governance documents reconciled, one Engineering Release. No crate, dependency, test, fixture, Governance Register item, or ADR changed.

---

## 11. Final Release Recommendation

**Sprint 12 is complete.** The Capability Identity procedure exists, is historically validated against all seven named decisions without contradiction, and is recorded with its own contradiction-and-correction history preserved rather than overwritten. Collector and Rule Composition are both resolved strictly as its consequences, with their respective evidentiary weights disclosed honestly. The repository remains architecturally consistent: the Collector Composition Architecture, GOV-012's Rule dispatch model, and `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` all required no amendment.

**Recommend:** Chief Architect final approval of this release; on approval, the two recommended commits (architecture + reconciliation; closeout) proceed per the completion report accompanying this document. The repository is ready for the next Capability Proposal — none yet named — to be the Capability Identity procedure's first genuinely prospective test.

---

## Repository Status

Sprint 12 is complete. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` are all clean; Sandbox is reverified unchanged. No Governance Register item, no ADR, and no Documentation Release amendment resulted from this Sprint. The repository is ready for the next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Documentation Release 1.0
        ↓
Sprint 0 → Sprint 9, Engineering Releases v0.1.0-alpha, 0.2–0.9
        ↓
Sprint 10 — Runtime Fixture Corpus Acquisition
        ↓
Engineering Release 1.0
        ↓
Sprint 11 — Runtime Evidence Processing Architecture, Implementation, Reconciliation
        ↓
Engineering Release 1.1
        ↓
Sprint 12 — Capability Scaling Architecture (Capability Identity procedure derived)
        ↓
Sprint 12 — Adversarial Verification (contradiction found: Sprint 4 Phase 3C falsifies the original model)
        ↓
Sprint 12 — Architectural Reconciliation (three-axis model; Sprint 11 reclassified Introduction → Expansion)
        ↓
Sprint 12 Closeout
        ↓
Engineering Release 1.2
```
