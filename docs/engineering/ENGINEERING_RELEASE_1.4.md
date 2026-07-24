# Engineering Release 1.4

| Property | Value |
|---|---|
| **Release** | 1.4 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 14 complete (GOV-003: `modiq-common` resolved and retired); the post-Sprint-13 GOV-001 Architecture Evaluation (not itself Sprint 14 work) is also recorded here as the immediately preceding governance action |
| **Scope** | The platform's first Governance Register retirement of an entire crate, following the same convergent-absence evidentiary standard already applied to `modiq-report`'s Sprint 6 scaffold-type retirement, applied here one level up — to a crate itself, not individual types within one |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.3.md` (Sprint 13) |
| **Governing ADRs** | None new — this Sprint neither required nor established one; GOV-003's resolution applies the platform's own "capability before abstraction" and convergent-evidence discipline rather than a new durable principle |
| **Governing Documents** | `docs/engineering/GOV003_ARCHITECTURE_EVALUATION.md`, `docs/engineering/GOV003_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/GOV003_SPRINT_PLAN.md` |

---

## 1. Executive Summary

Following Sprint 13's Repository Closeout, two governance actions occurred before Sprint 14 was itself scoped. First, a dedicated Architecture Evaluation of GOV-001 (Assessment Report Generation Timing) — directed by the Chief Architect on the strength of the forcing function `modiq-storage`'s own durable persistence created — found no inconsistency between `DataModel.md`'s specified Runtime Lifecycle and the implementation's actual behavior; no Architectural Resolution was performed or required, and GOV-001 was narrowed and returned to Open. This was explicitly **not** Sprint 14 work.

Sprint 14 itself was then formally selected: **GOV-003 (Role of `modiq-common`)**, the most immediately resolvable of the repository's remaining open Governance Register items — self-contained, zero external dependency, fully resolvable within one Sprint. `GOV003_ARCHITECTURE_EVALUATION.md` evaluated three alternatives against evidence verified directly (zero consumers across the entire workspace, zero real content in the crate's own source, and the platform's own twice-demonstrated pattern of creating a crate on demand rather than pre-provisioning one empty) and recommended retirement. The Chief Architect accepted this recommendation. `GOV003_IMPLEMENTATION_AUTHORIZATION.md` and `GOV003_SPRINT_PLAN.md` scoped a single authorized phase, executed without incident: `modiq-common` was removed from the workspace `Cargo.toml` and its directory deleted in its entirety, with zero other crate requiring any change.

---

## 2. Sprint Objective

Per `GOV003_SPRINT_PLAN.md`: retire `modiq-common` and reconcile the specific living documents that describe it (`README.md`, `CrateRoadmap.md`, `DependencyMap.md`, `CHANGELOG.md`), introducing no replacement crate, module, or type, and touching no other crate's source.

---

## 3. Governance Background: GOV-001 (Not Sprint 14)

`SPRINT_14_PROPOSAL.md` (Candidate A) had recommended resolving GOV-001 on the strength of a forcing function Sprint 13's own `modiq-storage` activation created: a manually smoke-tested, durably persisted report was observed holding `status: EvaluatingRules`, not `Completed`. Before any Architectural Resolution, the Chief Architect directed a determination of `AssessmentReport`'s own semantic contract, and whether its current generation point satisfies it.

**Finding:** no inconsistency exists. `DataModel.md`'s own canonical Runtime Lifecycle diagram documents "Assessment Report Produced" as the step immediately preceding "Assessment Completed" — the specification does not merely permit pre-completion generation, it specifies it as the intended sequence. Both `AssessmentService::execute` and `execute_from_assessment_input` call `AssessmentReport::generate` exactly once, immediately before `Assessment::complete()`, exactly matching the documented sequence, enforced by the existing `execute_reflects_state_at_report_generation_prior_to_completion` test.

**Disposition:** no Architectural Resolution was performed or required. GOV-001 was narrowed, not closed — the live question is now only whether `AssessmentStatus::Completed` being permanently unreachable by any real persisted report is an intended consequence of the documented ordering or an unexamined gap, deliberately left Open pending a future forcing function, mirroring GOV-008's and GOV-013's own standing treatment. Full record: `docs/engineering/ENGINEERING_LOG.md` ("GOV-001 Architecture Evaluation — Post-Sprint 13, Not Resolved").

---

## 4. GOV-003 Architecture Evaluation and Resolution

`GOV003_ARCHITECTURE_EVALUATION.md` evaluated three alternatives for `modiq-common`:

- **Option A — retire entirely.** Zero consumers confirmed directly (every crate's `Cargo.toml` grepped workspace-wide, only the crate's own manifest matched); zero real content confirmed directly (`crates/modiq-common/src/`'s four files, each a single doc comment, no types). The platform has independently demonstrated, twice (`modiq-collection` at Sprint 3, `modiq-storage` at Sprint 13), that a crate can be created on demand when a real need arrives, without cost or friction — undercutting the case for pre-provisioning an empty one.
- **Option B — retain as deliberately dormant scaffolding, purpose reaffirmed.** Rejected: this is the same "we'll probably need this" reasoning the platform's own "Speculative extensibility" failure mode names directly, and the reasoning this project has rejected every other time it has been evaluated against actual evidence (GOV-004, the Sprint 6 scaffold retirement).
- **Option C — retain, resolved as already-satisfied by design.** Rejected: unlike GOV-001 (where a specific, checkable specification claim existed to verify conformance against), no equivalent specification names a concrete future consumer or type for `modiq-common` — this option would affirm a design intent, not verify a specification claim.

**Recommendation: Option A.** The Chief Architect accepted retirement, exactly as recommended, with no modification. `GOVERNANCE.md`'s GOV-003 entry was updated to Resolved.

---

## 5. Implementation Summary

A single authorized phase, per `GOV003_SPRINT_PLAN.md`:

- `"crates/modiq-common"` removed from the root `Cargo.toml`'s `members` list; `crates/modiq-common/` deleted in its entirety (`Cargo.toml`, `README.md`, `src/lib.rs`, `src/error.rs`, `src/id.rs`, `src/prelude.rs`).
- Zero other crate's source or `Cargo.toml` required any change — consistent with the zero-consumer finding both the Evaluation and Authorization recorded.
- Living documents reconciled: `README.md`'s crate table, `docs/implementation/CrateRoadmap.md`'s crate table and dependency diagram (plus a new revision history entry, 1.26.0), and `docs/implementation/DependencyMap.md`'s dependency diagram all had their `modiq-common` entries removed. `docs/governance/CHANGELOG.md` gained a new `[Sprint 14]` entry.
- No historical Sprint/Release document was modified, per the Implementation Authorization's explicit exclusion.

**One finding surfaced during verification, reported rather than silently corrected:** `PROJECT_HANDOFF_v1.1.md` and `CHIEF_ARCHITECT_HANDOFF_v1.1.md` — the repository's current, living handoff documents — were found to still describe `modiq-common` as an existing crate in present tense, and to predate Sprint 13 entirely (zero mention of `modiq-storage`). This was outside `GOV003_IMPLEMENTATION_AUTHORIZATION.md`'s explicitly authorized document list and was not corrected as part of Sprint 14's own implementation — it was subsequently reconciled in a dedicated documentation-synchronization pass (`docs: reconcile repository baseline after Sprint 14`), recorded separately from this release.

---

## 6. Repository Impact

| Area | Change |
|---|---|
| Crates | `modiq-common` removed — 9 workspace crates, down from 10 |
| Root `Cargo.toml` | `"crates/modiq-common"` removed from `members` |
| `README.md`, `CrateRoadmap.md`, `DependencyMap.md` | `modiq-common` entries removed; `CrateRoadmap.md` gained revision history entry 1.26.0 |
| `CHANGELOG.md` | New `[Sprint 14]` entry |
| Every other crate (`modiq-runtime`, `modiq-collection`, `modiq-rules`, `modiq-report`, `modiq-engine`, `modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-storage`) | Unmodified |
| `AssessmentService` public entry points | Unaffected |
| Governance Register | GOV-003 moved from Open to Resolved — now 14 items, 9 Resolved, 5 Open |
| ADRs | None new |
| `GOVERNANCE.md` | GOV-003 entry updated (Status, Description, Resolution); no Crate Boundary Rule to remove, since `modiq-common` never had one |
| New documents | `GOV003_ARCHITECTURE_EVALUATION.md`, `GOV003_IMPLEMENTATION_AUTHORIZATION.md`, `GOV003_SPRINT_PLAN.md`, this release document |

---

## 7. Validation Status

```
Root workspace:
cargo fmt --check      → clean
cargo check --workspace → clean, zero warnings
cargo test --workspace  → 253/253 passing (unchanged — modiq-common had zero tests)

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check → clean
cargo check       → clean, zero warnings
cargo test        → 9/9 passing (unchanged)
```

A residual-reference grep (`modiq-common`/`modiq_common`) across the entire repository, run after all edits, confirmed no remaining reference outside historical Sprint/Release documents (correctly untouched) and this Sprint's own governance record (describing the retirement in past tense, as expected). `docs/architecture/` and `docs/adrs/` — this project's own Normative Authority for architectural intent — contain zero references to `modiq-common`, before or after this change; it never had a `GOVERNANCE.md` Crate Boundary Rule pair to remove.

---

## 8. Outstanding Limitations

- **This release document itself was produced after Sprint 14's own implementation, not at its close** — the first gap in the one-Sprint-one-release discipline established at Sprint 8, closed by this document's own publication rather than left unaddressed.
- **`PROJECT_HANDOFF_v1.1.md` and `CHIEF_ARCHITECT_HANDOFF_v1.1.md`'s two-Sprint staleness** (Section 5) was found during this Sprint but reconciled in a separate, dedicated documentation-synchronization commit, not as part of this release's own implementation phase.
- **GOV-002 (Runtime Invariant Reconciliation)** remains Open, unaddressed by this Sprint, and — unlike GOV-001, GOV-008, and GOV-013 — has never received a dedicated evaluation across any Sprint to date.
- **The `modiq-versioning` Crate Boundary Rules gap**, named at Sprint 8 planning, remains open and unaffected by this Sprint.
- **GOV-014 (Lua Fixture Acquisition Governance)** remains Open, unaffected by this Sprint.

---

## 9. Technical Director Assessment

**This Sprint's finding was not architectural but evidentiary discipline applied to a question the repository had left unexamined for its entire history.** `modiq-common` never satisfied its own stated promotion criterion in 13 Sprints; retiring it required no new principle, only the same convergent-absence standard GOV-004 and the Sprint 6 scaffold retirement already established, applied one level up — to a crate rather than to types within one.

**Risk:** Very low. Zero consumers meant zero blast radius, confirmed both before and after removal; the residual-reference grep and both workspaces' full verification found nothing inconsistent with that expectation.

---

## 10. Final Release Recommendation

**Sprint 14 is complete.** `modiq-common` is retired; the repository's Governance Register, `Cargo.toml`, and living documents all agree on this. The repository is ready for its next engineering objective — Sprint 15 remains unscoped as of this release.

**Recommend:** Chief Architect final approval of this release.

---

## Repository Status

Sprint 14 is complete. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` are all clean at 253/253; Sandbox is reverified at 9/9. GOV-003 is the only Governance Register item this Sprint resolved; no ADR resulted. The repository is ready for the next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Engineering Release 1.3
        ↓
Post-Sprint 13 — GOV-001 Architecture Evaluation (not Sprint 14; no Architectural Resolution)
        ↓
Sprint 14 — Objective Selected: GOV-003 (Role of modiq-common)
        ↓
GOV-003 Architecture Evaluation
        ↓
GOV-003 Architectural Resolution (Option A, retire, Accepted)
        ↓
GOV-003 Implementation Authorization → Sprint Plan
        ↓
Sprint 14 Implementation — modiq-common removed
        ↓
Repository Baseline Reconciliation (PROJECT_HANDOFF_v1.1.md, CHIEF_ARCHITECT_HANDOFF_v1.1.md, PROJECT_STATUS.md, GOVERNANCE.md)
        ↓
Engineering Release 1.4
```
