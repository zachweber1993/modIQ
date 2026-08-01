# GOV-017 Architecture Evaluation — Role of `AssessmentReport` Re-export from `modiq-engine`

| Property | Value |
|---|---|
| **Document** | GOV017_ARCHITECTURE_EVALUATION.md |
| **Governance item** | GOV-017 (`docs/engineering/GOVERNANCE.md`) |
| **Purpose** | Verify, using repository evidence, whether GOV-004's three-point convergent-evidence standard has genuinely been met for reconsidering `modiq-engine`'s non-re-export of `AssessmentReport` |
| **Scope** | Kept proportional to the size of the candidate change — a single dependency/re-export question, not a redesign |
| **Status** | **Architecture Evaluation and Architectural Resolution both complete. GOV-017 Resolved: not adopted. No implementation performed.** |

---

## 1. Question

Does a genuine third, independent data point exist (alongside `modiq-cli` and `apps/sandbox`) converging on the same redundant-dependency pattern GOV-004's own precedent treats as decided, non-coincidental evidence — such that `modiq-engine` should re-export `AssessmentReport`?

---

## 2. Evidence

Verified directly against source this session:

- `AssessmentService::execute` (`crates/modiq-engine/src/engine/assessment_service.rs:41-46`) returns `AssessmentReport` directly. Any caller of `execute`/`execute_from_assessment_input` already receives the type from `modiq-engine`'s own public API.
- `modiq-cli/src/commands/assess.rs:2,55` — imports `AssessmentReport` from `modiq-report`, uses it as a real parameter type (`report: &AssessmentReport`). `modiq-cli` already depends on `modiq-engine` (to call `AssessmentService`).
- `apps/sandbox/src-tauri/src/lib.rs:2,89-90` — imports `AssessmentReport` from `modiq-report`, uses it in `impl From<&AssessmentReport> for AssessmentSummary`. `apps/sandbox` already depends on `modiq-engine`, identically.
- `modiq-storage/src/storage/report_store.rs:13`, `persisted_report.rs:12` — imports `AssessmentReport` from `modiq-report`, uses it as a real parameter type (`ReportStore::store(&self, report: &AssessmentReport)`, `PersistedAssessmentReport::from_report(report: &AssessmentReport)`).
- `modiq-storage/Cargo.toml` — depends on `modiq-report` and `modiq-runtime` only. **No dependency on `modiq-engine`, in either direction** (`modiq-engine/Cargo.toml` confirmed: no dependency on `modiq-storage` either).
- `STORAGE_ARCHITECTURE_EVALUATION.md` §5 (Sprint 13): "`AssessmentService` itself needs no awareness that Storage exists" — Storage is deliberately wired at the `modiq-cli`/`apps/sandbox` layer, alongside `AssessmentService`, not composed inside it and not dependent on it.

---

## 3. Finding

**The claimed third data point does not hold.** `modiq-cli` and `apps/sandbox` genuinely share the same pattern: both already depend on `modiq-engine` for orchestration, and both separately name `AssessmentReport` *only* because `modiq-engine` doesn't re-export a type its own `execute` method already returns. A re-export would let both obtain the type through a dependency they already carry — this is the redundant-dependency shape GOV-004's convergence standard describes.

`modiq-storage` is structurally different, not a third instance of the same shape: it names `AssessmentReport` because its own public functions take it as a parameter — a need that exists independent of anything `modiq-engine` does or re-exports. It does not, and per Sprint 13's own architecture should not, depend on `modiq-engine` at all. A re-export from `modiq-engine` would be unreachable to `modiq-storage` without adding a new dependency edge — Storage depending on the Engine that is used alongside it, not composed by it — that no evidence here supports and that Sprint 13's own evaluation deliberately avoided creating.

**Genuine count: two, unchanged since Sprint 6.** This is the same count `SPRINT8_INITIALIZATION_REPORT.md` and `SPRINT_14_PROPOSAL.md` each already found and each judged below this project's own three-point bar. Nothing evidenced this session changes that count.

---

## 4. Architectural Consequence Check

- **Crate Boundary Rules** — not implicated either way, since no change is adopted. For completeness: a re-export would not have violated the Engine rule ("must never... generate reports" — re-exporting a type it already returns is not generating one) or the Reporting rule (ownership of `AssessmentReport` is unaffected by who else can name it).
- **`AssessmentService`'s public entry points** — unaffected; this question never touched them.
- **No other architectural consequence found.**

---

## 5. Resolution

**GOV-017 Resolved. Not adopted.**

**Explicit determination, stated as the precedent this resolution establishes:** a shared dependency on a crate is not, by itself, evidence of the same convergent pattern GOV-004 treats as decided — it must be checked whether the dependency arises from *consuming another crate's API* (redundant, and eliminable by that crate re-exporting the type) or from *declaring the type in the dependent's own public API* (structurally necessary, independent of any re-export, and not evidence of anything). The `modiq-storage` dependency is not evidence of the same implementation pattern as `modiq-cli`'s and `apps/sandbox`'s, because it arises from Storage's own public API (`ReportStore::store`, `PersistedAssessmentReport::from_report` each declare `AssessmentReport` as a parameter type) rather than from consumption of `modiq-engine` — `modiq-storage` does not depend on `modiq-engine` in any respect. Future evaluations citing convergent dependency evidence should check which of these two shapes each instance is before counting it.

`modiq-engine` does not re-export `AssessmentReport`. The three-point standard is not met; the two-point count already known since Sprint 6/8 stands, correctly judged insufficient each time it has been checked. No Governance Register count change beyond GOV-017's own entry. No ADR. No implementation performed or authorized.
