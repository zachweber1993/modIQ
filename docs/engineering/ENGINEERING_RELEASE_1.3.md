# Engineering Release 1.3

| Property | Value |
|---|---|
| **Release** | 1.3 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 13 complete (Storage Architectural Activation: `modiq-storage` given its first real content, wired through `modiq-cli` and `apps/sandbox`; Repository Closeout) |
| **Scope** | The platform's first subsystem-level activation since Sprint 9, and the first Sprint to rediscover and reconnect an earlier, pre-Capability-Identity governance classification (Sprint 8's "Architectural Activation") rather than deriving a new one |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.2.md` (Sprint 12) |
| **Governing ADRs** | None new — this Sprint applies ADR-0007's Opaque Runtime References pattern (its fourth instance) rather than establishing a new durable principle |
| **Governing Documents** | `docs/engineering/INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`, `docs/engineering/GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`, `docs/engineering/STORAGE_ARCHITECTURE_EVALUATION.md`, `docs/engineering/STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`, `docs/engineering/STORAGE_SPRINT_PLAN.md` |

---

## 1. Executive Summary

Sprint 13 gave the Storage subsystem — named in `Architecture.md`'s own System Overview since the document's own baseline, never implemented — its first real content. Unlike every capability Sprint since Sprint 10, this one did not proceed through Sprint 12's own Capability Identity procedure: an investigation (`INV-002`) found that procedure could not classify a subsystem-level candidate at all, its three axes and Introduction test being scoped exclusively to Collector/Rule-shaped decisions. Rather than deriving a new procedure for this shape of question, a governance review found the repository already contained one: Sprint 8's own "Architectural Activation" classification, applied to Version Profiles' first real content a full Sprint before Capability Identity existed, and never cross-referenced by it. A two-sentence amendment to `PROJECT_HANDOFF_v1.1.md` §5 reconnected the two, introducing no new procedure, taxonomy, or architectural decision.

Architecture Evaluation and Resolution then fixed four decisions — persisted domain object, subsystem ownership, lifecycle boundary, and public-entry-point impact — all Accepted without modification. Implementation preparation surfaced a genuine design conflict (Runtime entity identity cannot be serialized, reconstructed, or meaningfully preserved across a process boundary) and resolved it entirely within `modiq-storage`'s own crate boundary, without touching `modiq-runtime` or `modiq-report`. Three independently validated implementation phases followed, each verified beyond in-test coverage by a genuine cross-process round trip against the real, built `modiq-cli` binary.

---

## 2. Sprint Objective

Per `STORAGE_SPRINT_PLAN.md`: activate the Storage subsystem's first real content — durable, cross-process persistence and retrieval of a single `AssessmentReport` — closing the gap between "modIQ produces an assessment" and "modIQ preserves one," within the boundaries fixed by Architectural Resolution, introducing no general persistence framework, no new query capability, and no change to `AssessmentService`'s public entry points.

---

## 3. Architectural Problem Addressed: A Procedure's Scope, Not Its Correctness

`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` established Storage as a valid, well-evidenced capability candidate — `Vision.md`'s "Historical knowledge preservation," `Principles.md`'s "Knowledge Preservation" ("rather than remaining isolated within individual assessments"), and `ProductSpecification.md`'s own MKB content list ("Historical Assessments") and Assessment Workflow step 7 all describe a persistence-dependent capability never built. Applying the Sprint 12 Capability Identity procedure directly, however, produced no verdict: the Collection, Evidence, and Interpretation Axes, and the Capability Introduction test, are each phrased in terms of Collectors, Rules, and `EvidenceCategory` — none of which describes what Storage does. This was not a defect in the procedure; its own derivation set (Sprint 3, 4, 4 Phase 3C, 7, 8[rule], 9[rule], 11) contains no subsystem-level example to have generalized from.

---

## 4. Governance Reconciliation

`GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md` generalized `INV-002`'s finding into a formal Observation, per this project's own Decision Framework (Observation → Evidence → Investigation → Governance). Its own first draft concluded no governing concept existed for subsystem-level activation at all. On Chief Architect review, this was found incomplete: `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 had already named and applied **Architectural Activation** to Version Profiles' own first real content — "This Sprint is the first to realize an already-specified-but-dormant architectural dimension, not to invent a new one or merely extend an already-live one" — a classification Sprint 12's own historically-derived model never checked itself against, despite predating it by a full Sprint. Both `INV-002` and the Governance Observation were revised in place, recorded explicitly as amendments, not silent rewrites.

`PROJECT_HANDOFF_v1.1.md` §5 was amended (commit `fd2db36`) with two sentences: the standing Capability Identity gate applies to candidates answerable by its own three axes — Collector/Rule-shaped work — and an already-specified-but-dormant architectural subsystem receiving its first real content is not such a candidate; it follows Sprint 8's own Architectural Activation precedent directly to Architecture Evaluation instead. Neither Sprint 8's nor Sprint 12's own documents were altered. No Governance Register item and no ADR were opened — this reconnected two already-existing, already-approved classifications rather than establishing a new one.

---

## 5. Architecture Evaluation and Resolution

`STORAGE_ARCHITECTURE_EVALUATION.md` evaluated four decisions, each with alternatives, mirroring the rigor `SPRINT8_ARCHITECTURAL_RESOLUTION.md` applied to Version Profiles:

1. **Persisted domain object:** `AssessmentReport` — not `Assessment` itself (which would require deciding lifecycle re-entry and mutation-after-load questions with no precedent to draw on) and not individually addressable `Evidence`/`Finding`/`Recommendation` records (which would require resolving the still-open GOV-005/GOV-006 referential-integrity question as a precondition). `AssessmentReport` alone is already inert, read-only, and the exact artifact both real consumers already produce.
2. **Subsystem ownership:** a new `modiq-storage` crate — mirroring every other named subsystem's one-crate-per-subsystem shape. Disclosed explicitly: unlike `modiq-versioning`/`modiq-knowledge`, no `modiq-storage` crate existed even as an empty scaffold before this Sprint — this activation's one genuine asymmetry from the Sprint 8/9 precedent.
3. **Lifecycle boundary:** strictly downstream of Reporting, deliberately agnostic to GOV-001's own still-Open report-generation-timing question, with minimum-viable retrieval (lookup by identifier only, no querying or cross-Assessment operation).
4. **`AssessmentService` public entry points:** unchanged — Storage is consumed by callers the same way both `modiq-cli` and `apps/sandbox` already consume `modiq-report` directly.

All four were **Accepted** by the Chief Architect exactly as recommended, with no modification.

`STORAGE_IMPLEMENTATION_AUTHORIZATION.md` then translated the Resolution into an engineering envelope — deliberately narrower in scope than `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`'s own precedent, explicitly excluding implementation sequencing, crate order, and per-phase testing strategy. This scope difference was recorded, on request, as reflecting the repository's own evolved separation between Implementation Authorization and Sprint Planning as distinct stages — not any difference in what Storage's own architecture requires.

---

## 6. Design Conflict and Resolution

Direct inspection of `AssessmentReport` and its nested Runtime types, performed before writing any Phase 1 code, found:

- None of `AssessmentReport`, `Evidence`, `Finding`, `Recommendation`, or their identifier types derive `Serialize`/`Deserialize` anywhere in the codebase.
- `AssessmentId`/`EvidenceId`/`FindingId` are private-field wrappers with exactly one public method, `generate()`, drawing from a function-local, process-scoped `AtomicU64` counter that **restarts at 1 on every process invocation** — meaning Runtime identity is not merely hard to serialize, but not a durable, cross-process concept at all, by design.
- `Evidence::new`/`Finding::new`/`Recommendation::new` each mint a fresh identity internally; none has a raw constructor accepting a pre-existing one. `Finding`'s and `Evidence`'s own test suites include named tests (`..._with_identical_content_but_different_identity_is_not_equal`) confirming identity-inclusive equality is a deliberate, tested platform invariant, not an oversight.

This was reported as a real conflict with the Sprint Plan's own "`modiq-runtime`/`modiq-report` unmodified" constraint, not resolved unilaterally. `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` resolved it entirely within `modiq-storage`'s own crate boundary: Storage defines and owns its own persisted representation of a report's content, populated from `AssessmentReport`'s already-public getters, never attempting to reconstruct the original type or its identity — the **fourth instance of ADR-0007's Opaque Runtime References pattern** (`RuleReference`, `RepairRecipeReference`, `VersionProfileReference`), applied one layer downstream of where it has been applied before. Finding→Evidence and Recommendation→Finding cross-references are preserved as positions within the persisted report itself, not the original process-local identifiers. Faithfulness is judged by content and order — a direct extension of this platform's own pre-existing convention ("Determinism is judged by content and order, never by incidental identity," `PROJECT_HANDOFF_v1.1.md` §5) from same-process determinism testing to cross-process persistence fidelity, made decisive rather than merely permissible by the process-restarting counter finding. No modification to `modiq-runtime` or `modiq-report` was required; none of the four fixed Architectural Resolution decisions was revisited.

---

## 7. Implementation Summary

**Phase 1 — `modiq-storage` (new crate).** `PersistedAssessmentReport` and its nested `Persisted*` types; `ReportKey` (an opaque identifier Storage mints itself, since `AssessmentId` cannot serve as an external key); `ReportStore` (real filesystem-backed write/read, one JSON file per report, no new external dependency beyond the workspace's already-declared `serde`/`serde_json`). One implementation-time refinement, reported rather than silently absorbed: the Sprint Plan assumed a `modiq-report`-only dependency, but converting `Evidence`/`Finding`/`Recommendation`'s own field types requires naming them directly from `modiq_runtime::assessment` (not re-exported by `modiq-report`) — an additional `modiq-storage → modiq-runtime` edge, the same diamond-dependency shape already common throughout this workspace, conflicting with none of the four fixed decisions.

**Phase 2 — `modiq-cli`.** `AssessCommand::run` hands a successful assessment's report to `ReportStore::store`, reporting the key; a storage failure is reported as a warning, never as a change to the assessment's own exit code. A new `retrieve <key>` command reads a report back independent of running a new Assessment. Verified with the real, built binary run as **two separate process invocations** — the load-bearing proof the Sprint Plan itself named, not only in-test coverage.

**Phase 3 — `apps/sandbox`.** The identical pattern: `create_assessment`'s IPC summary DTO gains `stored_report_key`; a new `retrieve_report` Tauri command mirrors `modiq-cli`'s own, returning a new `PersistedReportSummary` DTO built from `PersistedAssessmentReport`. Both commands split into a thin `#[tauri::command]` wrapper plus a storage-root-parameterized core function, mirroring Phase 2's own shape, so tests run against a real, hermetic temporary directory rather than the fixed, manifest-relative default.

`modiq-runtime`, `modiq-report`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, and `modiq-knowledge` are unmodified across all three phases. `AssessmentService`'s two public entry points required zero signature change.

---

## 8. Repository Impact

| Area | Change |
|---|---|
| Crates | `modiq-storage` added (new) — 10 workspace crates, up from 9 |
| `modiq-cli` | `Cargo.toml` gains `modiq-storage`; `assess` now stores, new `retrieve` command |
| `apps/sandbox` (separate workspace) | `Cargo.toml` gains `modiq-storage`; `create_assessment` gains `stored_report_key`, new `retrieve_report` command |
| `modiq-runtime`, `modiq-report`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, `modiq-knowledge` | Unmodified |
| `AssessmentService` public entry points | Unchanged |
| `EvidenceCategory` / `FindingSeverity` | Unchanged |
| Governance Register | Unchanged — 14 items, 8 Resolved, 6 Open |
| ADRs | None new — ADR-0007's pattern reused (fourth instance) |
| `GOVERNANCE.md` | New Storage Crate Boundary Rule pair (Owns / Must never) |
| `.gitignore` | `.modiq-storage/` added |
| New documents | `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`, `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`, `STORAGE_ARCHITECTURE_EVALUATION.md`, `STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`, `STORAGE_SPRINT_PLAN.md`, this release document |
| Amended documents | `PROJECT_HANDOFF_v1.1.md` (§5, two sentences) |
| Governance documentation | `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, `CrateRoadmap.md`, `docs/README.md` — all reconciled |

---

## 9. Validation Status

```
Root workspace:
cargo fmt --check      → clean
cargo check --workspace → clean, zero warnings
cargo test --workspace  → 253/253 passing (238 prior + modiq-storage 10 + modiq-cli +5)

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check → clean
cargo check       → clean, zero warnings
cargo test        → 9/9 passing (7 prior + 2 new)
```

Beyond automated tests: the real, built `modiq-cli` binary was run against a scratch fixture as two genuinely separate process invocations (`assess`, then `retrieve` in a distinct process), confirming the persisted JSON's content and the retrieved output matched exactly — the true cross-process proof this Sprint's own Testing Strategy required, not merely an in-test proxy.

---

## 10. Outstanding Limitations

- **Two product-forcing-functions named in `INV-002` remain explicitly undecided:** whether cross-mod collection validation, MKB accumulation from real Assessments, or single-mod history is the actual next product problem Storage should grow to solve. This Sprint's minimum-viable slice (single-report write/read) does not presuppose or foreclose any of the three.
- **Extension Layer** — the platform's other dormant System Overview subsystem — remains untouched, unscoped, and would face the identical Capability-Identity-vs-Architectural-Activation question this Sprint just resolved for Storage, should it ever be activated.
- **GOV-001 (Assessment Report Generation Timing)** remains Open; Storage's own lifecycle boundary was deliberately designed to remain correct regardless of how it eventually resolves, but it has not itself been advanced.
- **The `modiq-versioning` Crate Boundary Rules gap**, named at Sprint 8 planning, remains open — `modiq-storage` gained its own Crate Boundary Rule pair directly this Sprint, `modiq-versioning`'s own gap is unaffected and unclosed.
- **The storage root location is a fixed, non-configurable default** (`.modiq-storage`, relative to CWD for `modiq-cli`, relative to `CARGO_MANIFEST_DIR` for the Sandbox) in both consumers — the smallest slice this Sprint's own authorization covers; a configurable location is a separate, later capability.

---

## 11. Technical Director Assessment

**This Sprint's central finding was not a new architectural insight but the successful recovery of an existing one.** Sprint 8 had already solved the exact problem this Sprint faced — what governs giving a dormant, already-specified subsystem its first real content — five Sprints before Sprint 12 derived a narrower, more heavily-verified procedure that superficially resembled but never actually covered it. Recovering that precedent, rather than deriving a new one, is the more conservative, more evidence-respecting outcome, and it is worth stating plainly: this Sprint added no new governance machinery at all, only a two-sentence cross-reference — the smallest possible correction, exactly per this project's own "make the smallest change supported by evidence" standard.

**The design conflict found during implementation preparation is this Sprint's second most consequential finding.** It would have been straightforward, under implementation pressure, to add a `Serialize`/`Deserialize` derive to `modiq-runtime`'s own types and call the problem solved — a change the Sprint Plan's own scope explicitly excluded, and one that would not have restored meaningful cross-process identity in any case, given the process-restarting counter. Resolving it instead within `modiq-storage`'s own boundary, as the fourth instance of an already-three-times-proven pattern, is the more disciplined outcome, and required no revision to any fixed Architectural Resolution decision.

**Risk:** Low. Every implementation phase was independently validated; the one design conflict was found before code was written, not discovered mid-implementation; the cross-process claim was verified against the real binary, not asserted from unit tests alone. The residual risk is the same category named in Sprint 12's own assessment — an unexamined adjacent precedent the repository's history may still contain, for some future question not yet asked.

---

## 12. Final Release Recommendation

**Sprint 13 is complete.** Storage has undergone its first Architectural Activation: `modiq-storage` is real, tested, and wired through both real consumers, verified with a genuine cross-process round trip. The governance reconciliation that preceded implementation leaves the repository more consistent than it found it — two previously-unreconciled taxonomies are now explicitly connected, in the fewest words that could do so. No architectural decision was revisited during implementation; the one design conflict found was resolved within the crate boundary already assigned to it.

**Recommend:** Chief Architect final approval of this release. The repository is ready for its next engineering objective — Extension Layer's own eventual activation, Storage's own next-phase product-forcing-function question, or Lua Analysis once GOV-014 resolves — none yet scoped by this document.

---

## Repository Status

Sprint 13 is complete. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` are all clean at 253/253; Sandbox is reverified at 9/9. No Governance Register item and no ADR resulted from this Sprint; `GOVERNANCE.md` gained one new Crate Boundary Rule pair. The repository is ready for the next engineering objective, not yet scoped by this document.

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
Engineering Release 1.2
        ↓
INV-001 — Lua Analysis Capability Investigation (Recommendation A; GOV-014 opened)
        ↓
INV-002 — Platform Persistence Capability Investigation (Capability Identity cannot classify Storage)
        ↓
Governance Observation — Subsystem Activation (Sprint 8's Architectural Activation rediscovered)
        ↓
PROJECT_HANDOFF_v1.1.md §5 amended (Capability Identity scope clarified)
        ↓
Storage Architecture Evaluation → Architectural Resolution (four decisions Accepted)
        ↓
Storage Implementation Authorization → Sprint Plan
        ↓
Sprint 13 Phase 1 — modiq-storage
        ↓
Sprint 13 Phase 2 — modiq-cli integration
        ↓
Sprint 13 Phase 3 — apps/sandbox integration
        ↓
Sprint 13 Closeout
        ↓
Engineering Release 1.3
```
