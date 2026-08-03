# Engineering Release 1.8

| Property | Value |
|---|---|
| **Release** | 1.8 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 23 complete (Frontend Presentation of Sprint 22's Domain Model) — `apps/console` surfaces Title, Mod Health dimension, Status, and Evidence provenance for the first time |
| **Scope** | The five items `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` authorizes: `FindingSummary`/`EvidenceSummary` transport extension (Rust and TypeScript), and their presentation in `Reviewing.tsx`/`Overview.tsx` — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.7.md` (Sprint 22) |
| **Governing ADRs** | None new |
| **Governing Documents** | `docs/engineering/FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/SPRINT23_IMPLEMENTATION_PLAN.md`, `docs/engineering/SPRINT23_IMPLEMENTATION_REPORT.md` |

---

## 1. Executive Summary

Sprint 23 closed the gap between what Sprint 22 gave the Runtime domain model and what any consumer application could show a user. `apps/console`'s transport DTOs and presentation components were extended, in four gated phases, to carry and display Title, `ModHealthDimension`, `FindingStatus`, and Evidence `label`/`source`/`content` — all data the engine has produced since Sprint 22 but that no application previously surfaced.

The Sprint's most consequential finding was not a defect but a boundary confirmation: `FrontendArchitecture.md`'s Consumer-Owned State authority, adopted during Sprint 21's own governing work, required no reinterpretation or extension to cover this work. Every decision the implementation needed was already resolvable from existing precedent. A second, unplanned finding — surfaced during closeout review, not during implementation itself — established that `RepairRecipe`'s structured steps are already derived-from in `VersionCompatibilityRule`'s flat `Recommendation.action` string, but discarded immediately afterward, with no resolution path back to structured content anywhere downstream.

---

## 2. Authorization and Planning Summary

Sprint 23 followed the same procedural pattern Sprint 21 established for `apps/console`: settled architecture (`FrontendArchitecture.md`, Approved) → a dedicated Implementation Authorization (`FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md`) → a Sprint Plan (`SPRINT23_IMPLEMENTATION_PLAN.md`) → implementation, at a proportionally smaller scope than Sprint 21's own foundational work. No Architecture Evaluation or Architectural Resolution preceded this Sprint — none was required, since Consumer-Owned State already, non-discretionarily, settled the only architectural question this work could raise. The Authorization and Sprint Plan both underwent Technical Director review before implementation began, catching and correcting four precision issues (a truncated quotation, an imprecise field-conversion description, an unnecessary test-sequencing dependency, and an inaccurate field count) before any code was written.

---

## 3. Implementation Summary

Four phases, executed in strict sequence, each gated before the next began:

1. **Phase 1 — Rust transport.** `FindingSummary` gained `mod_health_dimension`/`status`; `EvidenceSummary` gained `label`/`source`/`content`. New test assertions landed in this same phase, not deferred.
2. **Phase 2 — TypeScript transport.** `types.ts` interfaces mirrored, field-for-field, in the existing camelCase convention; the module's own stale doc comment corrected.
3. **Phase 3 — Presentation.** `Reviewing.tsx` renders Title on the collapsed row and Mod Health dimension/Status/Evidence provenance in the existing per-Finding expansion — no new expansion level. `Overview.tsx`'s doc comment corrected; no new aggregate content added, per the Phase 3 Authorization's explicit exclusion of grouping.
4. **Phase 4 — Final reverification.** Zero files changed; full workspace, `console`, and Sandbox all reconfirmed clean.

No change to `AssessmentService`'s public entry points, `RuleEngine::evaluate`, or any Runtime type, at any phase.

---

## 4. Repository Impact

| Area | Change |
|---|---|
| `console` crate (Rust) | `assessment.rs` — `FindingSummary`/`EvidenceSummary` extended; test count 4 → 5 |
| `apps/console` (TypeScript) | `types.ts`, `Reviewing.tsx`, `Overview.tsx` extended; no new file |
| Every `modiq-*` crate | **Unmodified** — confirmed via empty diff |
| `apps/sandbox` | **Unmodified** — separate workspace, out of scope |
| `AssessmentService` public entry points | Unaffected |
| Governance Register | Unaffected — 17 items, 12 Resolved, 5 Open, unchanged |
| ADRs | None new |
| New documents | `docs/engineering/SPRINT23_IMPLEMENTATION_REPORT.md`, this release document |

---

## 5. Validation Status

```
Root workspace (default-members):
cargo fmt --check       → clean
cargo check --workspace → clean
cargo test              → 269/269 passing, unchanged

console (explicit -p console):
cargo test -p console  → 5/5 passing (4 → 5, one extended, one added)

Full workspace (--workspace, 10 members):
cargo test --workspace  → 273 → 274 passing

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check → clean
cargo check       → clean
cargo test        → 9/9 passing, unaffected

apps/console (frontend):
npm run build (tsc && vite build) → clean
```

---

## 6. Engineering Observations

Recorded as evidence only — no process change, new governance mechanism, or Governance Register item follows from any observation here.

**Observation 1 — settled architecture, exercised a second time, required nothing new.** `FrontendArchitecture.md`'s Consumer-Owned State clause was adopted during Sprint 21's own governing work and exercised for the first time, at scale, during that same Sprint. Sprint 23 is the second real exercise of that same authority, on a materially different kind of change (extending existing fields, not establishing a new application), and it required zero reinterpretation — real, if modest, convergent evidence that the adopted authority is sufficient as written.

**Observation 2 — the flat Recommendation string is a real derivation, not a disconnected shortcut, but its source structure is still unreachable.** Closeout review (not implementation itself) traced `VersionCompatibilityRule`'s construction of `Recommendation.action` and found it already joins `RepairRecipe::steps()`'s individual instructions into one string, at Rule-evaluation time. The structured form this join discards — per-step `RepairStepKind`, step boundaries — has no resolution path back into existence anywhere between Rule construction and the consumer. This narrows, rather than widens, what a future "expose RepairRecipe steps" initiative would need to design: the content already exists in the right shape at the right moment: the outstanding question is representation and resolution, not authoring.

Neither observation required, or produced, any architectural change.

---

## 7. Outstanding Limitations / Reserved Responsibilities

Named, not resolved, exactly as `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` §6 already reserves or excludes them:

- Initiative 3 Item 6a (Report Identity), Item 4 (Evidence Explanation), 3C — untouched, unaffected.
- Initiative 1 D-1/D-2, Initiative 2 Question 1 — untouched, unaffected.
- `RepairRecipe` structured-step exposure — real opportunity, no architecture authorizing it yet (Observation 2, above).
- `apps/sandbox` parity — available, no forcing function.
- `apps/console`'s lack of a component/frontend test framework — a standing condition, not newly introduced; this Sprint's own verification relied on Rust-side tests and manual/traced verification instead.

---

## 8. Technical Director Assessment

**This Sprint's engineering discipline matched Sprint 22's exactly, at a smaller and lower-risk scale.** Every prediction the Authorization and Sprint Plan made — that no new architecture would be needed, that the existing expansion layer would suffice, that the `.map(str::to_string)` pattern would generalize cleanly — was confirmed by implementation, with no rework at any phase boundary. The one genuinely new finding (Observation 2) emerged from closeout-time source review, not from implementation friction — a reminder that this repository's own discipline of checking real source before drawing conclusions continues to surface facts that narrative alone would not.

**Risk: low.** No deviation expanded scope; all three named deviations (§ Repository Audit) narrowed or clarified rather than exceeded the Authorization.

---

## 9. Final Release Recommendation

**Sprint 23 is complete.** All four phases implemented, verified, and reconfirmed at closeout; every Authorization exclusion remains absent; field parity between Rust and TypeScript transport is exact.

**Recommend:** Chief Architect final approval of this release.

---

## Repository Status

Sprint 23 is complete. Root workspace `cargo fmt --check`, `cargo check --workspace`, and `cargo test` are clean at 269/269, unchanged; `console` is clean at 5/5 (4 → 5); the full workspace is clean at 274/274; Sandbox remains clean at 9/9. No Governance Register item or ADR resulted from this Sprint. The repository is ready for its next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Engineering Release 1.7 (Sprint 22)
        ↓
Frontend Presentation Implementation Authorization (committed)
        ↓
Sprint 23 Implementation Plan (committed)
        ↓
Sprint 23 Phase 1 — Rust Transport
        ↓
Sprint 23 Phase 2 — TypeScript Transport
        ↓
Sprint 23 Phase 3 — Presentation
        ↓
Sprint 23 Phase 4 — Final Reverification
        ↓
Engineering Release 1.8 — Sprint 23 complete
```
