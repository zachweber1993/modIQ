# GOV-017 Readiness Assessment — Role of `AssessmentReport` Re-export from `modiq-engine`

| Property | Value |
|---|---|
| **Document** | GOV017_READINESS_ASSESSMENT.md |
| **Project** | modIQ |
| **Governance Item Addressed** | GOV-017 — Role of `AssessmentReport` Re-export from `modiq-engine` (Resolved: Not adopted) |
| **Purpose** | Determine whether Sprint 21 produced sufficient new evidence to justify reopening GOV-017. Not a Governance Reconsideration. Does not decide GOV-017. |
| **Primary Evidence Sources** | GOV-017 and GOV-004 (`docs/engineering/GOVERNANCE.md`), `GOV017_ARCHITECTURE_EVALUATION.md` (the source evaluation GOV-017's own resolution cites), `PLATFORM_VALIDATION_GOV-004.md`, `ENGINEERING_RELEASE_1.6.md`, `IMPLEMENTATION_REPORT_FRONTEND_CONSOLE.md`, `FRONTEND_ARCHITECTURE_EVIDENCE_LEDGER.md`. |
| **Repository Verification** | Dependencies verified directly against current source, not from summary: `apps/console/src-tauri/src/assessment.rs` and `Cargo.toml`, `crates/modiq-cli/src/commands/assess.rs` and `Cargo.toml`, `apps/sandbox/src-tauri/src/lib.rs`, `crates/modiq-engine/src/engine/mod.rs` and `Cargo.toml`. |
| **Status** | **Readiness Assessment only. No Governance Reconsideration is performed. GOV-017's disposition (Not adopted) is unchanged by this document.** |

---

## 1. What precise architectural question does GOV-017 leave open?

None, formally — GOV-017's Status is Resolved. What it leaves open is narrower: it establishes a standing evidentiary test ("consuming another crate's API" — redundant, countable — versus "declaring the type in the dependent's own public API" — structurally necessary, not countable) and states plainly that the count under that test currently stands at two, "below this project's own three-point convergence bar." The question this readiness assessment addresses is whether that count has since changed under the same, unaltered test.

## 2. What evidence existed when GOV-017 reached "Not Adopted"?

Verified directly in `GOV017_ARCHITECTURE_EVALUATION.md` §2: three real crates existed at decision time — `modiq-cli` and `apps/sandbox`, both depending on `modiq-engine` for orchestration and separately importing `AssessmentReport` from `modiq-report` only because `modiq-engine`'s `execute`/`execute_from_assessment_input` returns that type without re-exporting it; and `modiq-storage`, which imports `AssessmentReport` because its own public functions (`ReportStore::store`, `PersistedAssessmentReport::from_report`) declare it as a parameter type, independent of `modiq-engine` — confirmed structurally: `modiq-storage`'s `Cargo.toml` has no dependency on `modiq-engine` at all, in either direction.

## 3. What evidentiary threshold did GOV-017 explicitly establish?

Stated as the resolution's own precedent (§5 of the Evaluation, restated verbatim in `GOVERNANCE.md`): a shared dependency counts toward GOV-004's three-point bar only if it arises from consuming another crate's API redundantly — i.e., the consumer already depends on `modiq-engine` and separately names `AssessmentReport` solely because `modiq-engine` doesn't re-export a type it already returns. A dependency arising from the consumer's own, independent public-API need does not count. "Two points remain... the three-point standard is therefore not met."

## 4. What new implementation evidence now exists after Sprint 21?

Verified directly against current source, not summary:

- `apps/console/src-tauri/Cargo.toml` — depends on `modiq-engine` and, separately, on `modiq-report`, added under a comment dated "Phase 2 (Sprint 21): the first real dependency on the engine."
- `apps/console/src-tauri/src/assessment.rs` — `use modiq_engine::engine::AssessmentService;` and `use modiq_report::report::AssessmentReport;`, the latter used only to type `impl From<&AssessmentReport> for ReportSummary`.
- `modiq-engine/src/engine/mod.rs` — re-confirmed: re-exports `AssessmentExecutionError` and `AssessmentService` only. `AssessmentReport` is still not re-exported. Nothing about the target of the question has changed.

This crate did not exist when `GOV017_ARCHITECTURE_EVALUATION.md` was written. It is genuinely new evidence, not a re-reading of evidence already considered.

## 5. Is Console genuinely an independent convergent point under GOV-004's own methodology, or merely another manifestation of an already-counted dependency?

Independent, and structurally, not stylistically, so. Applying GOV-017's own discriminating test exactly: Console already depends on `modiq-engine` (to call `execute_from_assessment_input`) and separately imports `AssessmentReport` from `modiq-report` only to name the return type of the method it is already calling — the identical shape as `modiq-cli` and `apps/sandbox`, and structurally distinct from `modiq-storage`'s shape (no dependency on `modiq-engine` at all; the type is needed for Storage's own unrelated public API).

One distinction is worth naming precisely, because it strengthens rather than weakens the classification: `IMPLEMENTATION_REPORT_FRONTEND_CONSOLE.md` records that Console reused `apps/sandbox`'s DTO-building technique as validated precedent. That reuse does not make this dependency a copied or coordinated data point. Reusing a technique (how to build a boundary-crossing DTO) is a stylistic choice; needing to import `AssessmentReport` at all is a structural consequence forced on any consumer of `execute_from_assessment_input` that wants to use its return value, given that `modiq-engine` still doesn't re-export the type — not a decision Console was free to make differently by imitating or not imitating anything. This is exactly the "independently, structurally forced" character GOV-004's own precedent (three subsystems independently converging on direct composition, without coordination) requires.

## 6. Does Sprint 21 satisfy the exact threshold GOV-017 said had not yet been reached?

**The repository evidence indicates that it does, under the test as GOV-017 itself defined it.** The count of instances passing the "consuming another crate's API redundantly" test moves from two (`modiq-cli`, `apps/sandbox`) to three (`modiq-cli`, `apps/sandbox`, `console`), verified directly against current source for all three. GOV-017's own stated bar was three; the observed count now matches it.

## 7. Has anything appeared that weakens GOV-017's original reasoning, or only changes the amount of evidence?

Only the amount of evidence changed; the reasoning is untouched and, if anything, reconfirmed. GOV-017's discriminating test (consumption-of-API versus declaration-in-own-API) was applied to Console here without any ambiguity or edge case — Console falls cleanly on the countable side, the same way `modiq-cli` and `apps/sandbox` did, and the same way `modiq-storage` did not. Nothing about Sprint 21 calls the test itself, or the correctness of `modiq-storage`'s exclusion, into question. The original decision was correct given the evidence available when it was made. What has changed is that a new, real, independent instance now exists that did not exist then.

---

## Is a Governance Reconsideration now justified?

**Yes.** GOV-017 explicitly and precisely named the condition under which its own disposition should be revisited — a genuine third instance of the specific, defined pattern — and the repository evidence indicates that, applying GOV-017's own test to evidence that did not exist when it was written, that condition now appears to be satisfied.

This does not decide the outcome. Whether `modiq-engine` should in fact re-export `AssessmentReport` remains for the reconsideration itself to determine — a reconsideration could, on review, still decline to authorize a re-export for reasons beyond the count (implementation cost, absence of a forcing function beyond convenience, or another consideration this readiness assessment does not evaluate). This assessment finds only that the specific evidentiary gate GOV-017 itself set now appears satisfied, not that a re-export is warranted.

---

## Recommendation

**A narrow GOV-017 Governance Reconsideration is now justified.**

Scope, if pursued: confirm the three-point count this assessment observed, confirm no fourth or fifth structural exception exists beyond `modiq-storage`'s already-settled case, and decide — with the evidentiary threshold now appearing satisfied — whether `modiq-engine` should re-export `AssessmentReport`. This recommendation names readiness for that reconsideration; it does not perform it, and no implementation, ADR, or Crate Boundary Rule change is proposed or implied here.
