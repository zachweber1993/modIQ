# Engineering Release 1.5

| Property | Value |
|---|---|
| **Release** | 1.5 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 20 complete (Historical Assessment Analysis) — recognized as a Sprint during repository reconciliation, not originally scoped as one. First implementation Sprint since the Sprint 15–19 governance-reconciliation cycle (GOV-002, GOV-015, ADR-0011). |
| **Scope** | A minimum-viable, read-only Storage-domain capability giving an engineer visibility into recurring Rule/Severity patterns across every persisted `AssessmentReport` — the platform's first capability built on top of Storage's own Sprint 13 activation, rather than a further activation of a dormant subsystem |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.4.md` (Sprint 14). Sprints 15–19 produced no Engineering Release of their own — consistent with this project's standing convention that an Engineering Release records Sprint *implementation* state, and none of those five Sprints touched Rust source. |
| **Governing ADRs** | None new |
| **Governing Documents** | `docs/engineering/CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md` (suspended), `docs/engineering/ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md`, `docs/engineering/CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`, `docs/engineering/IMPLEMENTATION_REPORT_HISTORICAL_ASSESSMENT_ANALYSIS.md` |

---

## 1. Executive Summary

A Capability Definition for "MKB Accumulation from Real Assessments" surfaced a genuine conflict among frozen specifications before any implementation began. A dedicated Architecture Evaluation, scoped exclusively to that question, found no actual conflict — `ProductSpecification.md`/`Glossary.md` and `KnowledgeModel.md` describe different levels of abstraction — and recorded a binding Adopted Architectural Constraint: knowledge accumulation must occur entirely outside live Assessment execution. Under that constraint, the original capability was suspended in favor of a narrower one, **Historical Assessment Analysis**, scoped, classified, implemented, verified, and committed (`ebc10c5`) in full.

This release performs the one step that sequence had not yet completed: Sprint recognition and Repository Closeout. The engineering retrospective following implementation found that this work had, in every material respect, already followed the repository's standard Sprint lifecycle — it simply began from a governance/architecture discussion rather than a conventional Sprint-scoping session, and was never assigned a number as a result. Reconciling the repository's own Sprint numbering found that `CHANGELOG.md` already uses `[Sprint 15]` through `[Sprint 19]` for the preceding governance-reconciliation cycle — a fact `PROJECT_STATUS.md`'s own prose (corrected during Package B to describe that cycle as non-Sprint governance work) did not carry over into `CHANGELOG.md` itself. This work is therefore recognized as **Sprint 20**, not Sprint 15, preserving the repository's committed numbering rather than rewriting it.

---

## 2. Sprint Recognition Determination

**Question:** does Historical Assessment Analysis qualify for formal recognition as a numbered Sprint, and if so, which number?

**Grounding, exclusively in documented lifecycle and precedent:**

- **The repository's own bright line between Sprint work and non-Sprint work is whether Rust source was touched.** `INV-001`, `INV-002`, the post-Sprint-13 GOV-001 evaluation, and Sprints 15–19 (GOV-002, GOV-015, ADR-0011) are all explicitly recorded as "Not a Sprint" / architecture-and-governance-only, each stating directly that no Rust source was touched. Historical Assessment Analysis touched Rust source substantially: `ReportStore::list_keys`, a new `history_analysis` module, a new `modiq-cli history` command, and 11 new tests. It falls unambiguously on the Sprint side of that line.
- **Every canonical lifecycle stage was satisfied, in order, without exception:** Capability Definition (produced, `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`) → Capability Identity Classification (performed, §3 of that document) → Architecture Evaluation determination (performed, §4, concluding none was separately required — itself a legitimate lifecycle outcome, mirroring how Sprint 13 proceeded under Sprint 8's existing Architectural Activation precedent rather than a fresh evaluation) → Implementation Authorization (the Chief Architect's explicit "Proceed with implementation... exactly as scoped" direction) → Implementation (three phases, as scoped) → Validation (`cargo fmt`/`check`/`test`, both workspaces) → Implementation Report (produced, including the Architectural Invariants/Implementation Assumptions distinction added on review) → Architectural Conformance Review (the Chief Architect's review and approval of that report) → Commit (`ebc10c5`) → Merge (no separate Sprint branch was used, consistent with every Sprint since Sprint 7's own precedent) → Repository Closeout (this release).
- **Direct precedent exists for retroactively recognizing already-completed work without rewriting history:** `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §9 reclassified Sprint 11 from Capability Introduction to Capability Expansion after the fact — a correction to how already-completed work is *categorized*, explicitly not a rewriting of what happened. Recognizing this work as a Sprint, after the fact, during reconciliation, follows the same discipline: the work itself is not altered; only its repository categorization is being correctly recorded now.
- **`CHANGELOG.md` already contains `[Sprint 15]` through `[Sprint 19]`**, formally headed, for the GOV-002 → GOV-015 → ADR-0011 governance-reconciliation cycle. This was not caught during Package B's own reconciliation, which corrected `PROJECT_STATUS.md`'s prose but did not check `CHANGELOG.md`'s own entry headings — recorded here as a process observation (§8), not corrected as part of this release, per explicit direction.

**Determination: recognized as Sprint 20.** The evidence supports Sprint recognition without qualification; it does not support "Sprint 15" specifically, since that number is already committed to different content in the repository's own record. Sprint 20 is the next number actually free, following `CHANGELOG.md`'s own existing numbering as committed rather than renumbering it.

---

## 3. Sprint Objective

Per `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` §2: give `modiq-storage` a minimum-viable, read-only capability to enumerate every persisted `AssessmentReport`, deterministically compare them, and surface recurring `(rule_reference, severity)` patterns as a candidate output for future architectural consideration — without touching the Knowledge Domain, without participating in `AssessmentService::execute`, and without introducing any new domain type absent evidence one was required.

---

## 4. Capability Definition, Classification, and Architecture Evaluation Determination

Full detail lives in the governing documents (header, above); summarized here for the release record:

- **Capability Definition** — narrowed, by explicit Chief Architect direction, from the originally-proposed "Knowledge Feedback Loop" (suspended, not withdrawn) after `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` recorded its Adopted Architectural Constraint. Historical Assessment Analysis was scoped to stay entirely within `modiq-storage`, never touching `modiq-knowledge`.
- **Capability Identity Classification** — neither the three-axis Capability Identity procedure nor Architectural Activation cleanly classifies this candidate, for the same structural reason `INV-002` found for Storage's own original activation: it operates downstream of Evidence Collection and Rule Evaluation entirely, and neither `modiq-storage` nor `modiq-knowledge` is dormant. Recorded as evidence, not resolved by inventing a new taxonomy (§8, Observation 1).
- **Architecture Evaluation determination** — found not separately required, on three grounds verified directly rather than assumed: the capability satisfies `GOVERNANCE.md`'s Storage Crate Boundary "Must never" list by construction; `STORAGE_ARCHITECTURE_EVALUATION.md` (Sprint 13) explicitly named cross-report comparison as deferred to "its own, later, separately-evaluated capability" rather than prohibited; and the Adopted Architectural Constraint already establishes the one binding rule this capability's read-only, out-of-band scope satisfies by construction.

---

## 5. Implementation Summary

Three phases, exactly as scoped, no deviation:

1. **`ReportStore::list_keys`** (`crates/modiq-storage/src/storage/report_store.rs`) — enumerates every key at a store's root, sorted for deterministic ordering; a non-existent root returns an empty list, not an error.
2. **`history_analysis` module** (`crates/modiq-storage/src/storage/history_analysis.rs`, new) — `PatternFrequency` (a plain struct reusing `PersistedFindingSeverity` and the existing `rule_reference` string; no new enum) and `recurring_patterns`, a deterministic, read-only aggregation with its own dedicated determinism test (reports stored out of sorted order still produce sorted output) and a dedicated non-mutation test.
3. **`modiq-cli history`** (`crates/modiq-cli/src/commands/history.rs`, new) — an argument-free, read-only subcommand surfacing the result; `Application::run`'s dispatch, doc comment (four → five commands), and `help.rs`'s usage text and test were all updated to match.

No change to `AssessmentService`'s two public entry points, `RuleEngine::evaluate`, any Runtime type, or `modiq-knowledge`.

---

## 6. Repository Impact

| Area | Change |
|---|---|
| Crates | Unchanged — 9 workspace crates. No new crate. |
| `modiq-storage` | `ReportStore::list_keys` added; new `history_analysis` module (`PatternFrequency`, `recurring_patterns`) |
| `modiq-cli` | New `history` command; `app.rs` dispatch and doc comment updated; `help.rs` usage text and test updated |
| Every other crate (`modiq-runtime`, `modiq-collection`, `modiq-rules`, `modiq-report`, `modiq-engine`, `modiq-knowledge`, `modiq-versioning`) | Unmodified |
| `apps/sandbox` | Unmodified |
| `AssessmentService` public entry points | Unaffected |
| Governance Register | Unaffected by this Sprint's own scope — 15 items, 11 Resolved, 4 Open at Sprint 20's own implementation and commit (`ebc10c5`). Now 16 items, 11 Resolved, 5 Open at current HEAD, following GOV-016's opening (`5db8cbb`) — a separate, unrelated governance investigation (see §8, addendum, below), not opened by and not affecting Sprint 20's own scope. |
| ADRs | None new |
| `GOVERNANCE.md` | Unmodified — this Sprint's own Architecture Evaluation determination (§4) confirmed the existing Storage Crate Boundary Rule already correctly describes this capability's behavior; nothing required updating |
| New documents | `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md`, `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md` (suspended), `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`, `IMPLEMENTATION_REPORT_HISTORICAL_ASSESSMENT_ANALYSIS.md`, this release document |

---

## 7. Validation Status

```
Root workspace:
cargo fmt --check      → clean
cargo check --workspace → clean, zero warnings
cargo test --workspace  → 264/264 passing (253 → 264: modiq-storage 10 → 18, modiq-cli 15 → 18)

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check → clean
cargo check       → clean, zero warnings
cargo test        → 9/9 passing (unchanged)
```

Re-verified fresh for this release, not carried forward from the Implementation Report's own account.

---

## 8. Governance / Process Observations

Recorded as evidence only, per explicit Chief Architect direction — no process change, new governance mechanism, or Governance Register item follows from either observation here.

**Observation 1 — a second candidate neither Capability Identity Classification nor Architectural Activation cleanly classifies.** Storage's own original activation (`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`) and this Sprint's Historical Assessment Analysis both operate on a shape — extending an already-activated subsystem's own existing contract, without touching a Collector, Rule, or dormant subsystem — that neither existing classification procedure was derived to describe. Two instances is below this project's own three-instance convergent-evidence bar (GOV-004's precedent); recorded as evidence toward a possible future pattern, not itself grounds for a new procedure.

**Observation 2 — no documented procedure exists for deciding whether a proposed change needs a dedicated Architecture Evaluation at all.** This Sprint's own Capability Definition (§4) had to construct that determination directly from evidence (checking the relevant Crate Boundary Rule's "Must never" list; checking whether a prior Architecture Evaluation had already named the change as deferred) rather than apply an established procedure, the way Capability Identity Classification at least offers three named axes. Recorded as a process observation only.

**Additionally, and separately from both of the above:** this Sprint's own reconciliation found that `CHANGELOG.md`'s `[Sprint 15]`–`[Sprint 19]` headings were never corrected to match `PROJECT_STATUS.md`'s own Package B relabeling of that same work as a non-Sprint governance cycle. Per explicit direction, this is recorded here only, not corrected as part of this release.

**A fourth, concurrent observation** (`docs/engineering/GOVERNANCE_OBSERVATION_EBC10C5_AUTHORIZATION_EVIDENCE.md`, committed `7eb7b5f`, from a separate Sprint 22 Chief Architect session running concurrently with this one) found that `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md` and `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` each retained a stale closing line ("Awaiting Chief Architect review...") from their initial drafting, left uncorrected when their header `Status` fields were later updated to Suspended and Approved-and-Implemented respectively — a genuine, narrow documentation defect, evaluated and confirmed on its own merits rather than assumed from the observation's own more sweeping framing (which read the absence of a separate git-tracked authorization artifact as grounds to treat Chief Architect authorization itself as unestablished — a standard this repository has never applied to any prior Sprint's own "Chief Architect accepted this recommendation" language, and not adopted here). Both stale lines were corrected as part of this Sprint's own closeout.

**That same concurrent Sprint 22 session subsequently opened GOV-016** ("Evidentiary Standard for Establishing Governance Decisions as Repository Fact," Open, `5db8cbb`) — a general, constitutional-level governance question, explicitly not scoped to decide anything about Sprint 20 or commit `ebc10c5` (its own Resolution field defers any application "to historical cases, including commit `ebc10c5` and the capability documents and implementation it concerns" to "a separate, subsequent milestone"). **By explicit Chief Architect direction, GOV-016 is treated here as an unrelated, separate open governance investigation** — accepted as part of this document's current-state accounting (Repository Impact, above) but not evaluated, resolved, or acted upon as part of Sprint 20's own closeout. Sprint 20's own recognition, implementation, and Repository Closeout are unaffected by GOV-016's pendency.

---

## 9. Outstanding Limitations

- No minimum-occurrence threshold — every observed `(rule_reference, severity)` pair is reported regardless of count (`CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` §13, Question 3, not decided by this implementation).
- No pagination or size limit on `list_keys`/`recurring_patterns` — explicitly deferred, consistent with this project's own "smallest real slice" discipline.
- This capability has not yet been exercised against a real, multi-report `.modiq-storage` directory produced by ordinary usage over time — only against test-constructed reports.
- The `CHANGELOG.md`/`PROJECT_STATUS.md` Sprint-numbering inconsistency named in §8 remains unresolved, by explicit direction, pending separate authorization.

---

## 10. Technical Director Assessment

**This Sprint's most significant finding was not architectural but process-evidentiary: the repository's own Sprint-recognition question required checking a document (`CHANGELOG.md`) the immediately preceding reconciliation (Package B) had not checked.** That this surfaced during a routine retrospective, rather than during Package B itself, is itself worth naming plainly: verifying a correction's completeness against every document it touches, not only the one it was drafting in, is a discipline this Sprint's own evidence suggests deserves more consistent application, not a criticism of any single pass.

**Risk: low.** The implementation itself carries the same low risk profile recorded in its own Implementation Report; the Sprint-numbering question this release resolves is a repository-bookkeeping correction, not a code or architectural risk.

---

## 11. Final Release Recommendation

**Sprint 20 is complete.** Historical Assessment Analysis is implemented, verified, and committed; the repository's Governance Register, `Cargo.toml`, and living documents all agree on this. The `CHANGELOG.md`/`PROJECT_STATUS.md` numbering inconsistency named in §8 is recommended as a future, separately-authorized reconciliation item — not performed here.

**Recommend:** Chief Architect final approval of this release.

---

## Repository Status

Sprint 20 is complete. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` are all clean at 264/264; Sandbox is reverified at 9/9. No Governance Register item or ADR resulted from this Sprint. The repository is ready for its next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Engineering Release 1.4 (Sprint 14)
        ↓
Sprint 15 — GOV-002 Architecture Evaluation and Resolution (governance only)
        ↓
Sprint 16 — GOV-015 Governance Initiation (governance only)
        ↓
Sprint 17 — GOV-015 Architecture Evaluation and Resolution (governance only)
        ↓
Sprint 18 — ADR-0011 Proposal (governance only)
        ↓
Sprint 19 — ADR-0011 Created and Accepted (governance only)
        ↓
Repository Reconciliation Package A (ADR-0011 index, GOVERNANCE.md, CHANGELOG.md, PROJECT_STATUS.md)
        ↓
Platform Architecture Track / Product & Interaction Design Track (independent lineage, 2026-07-28 – 2026-08-01)
        ↓
Repository Reconciliation Package B (living handoff documents synchronized)
        ↓
Capability Definition — Knowledge Feedback Loop (suspended)
        ↓
Architecture Evaluation — Product Specification / Knowledge Model Consistency (no conflict; Adopted Architectural Constraint recorded)
        ↓
Capability Definition — Historical Assessment Analysis (approved)
        ↓
Sprint 20 Implementation — list_keys, history_analysis, modiq-cli history
        ↓
Commit ebc10c5
        ↓
Engineering Release 1.5 — Sprint 20 recognized during Repository Closeout
```
