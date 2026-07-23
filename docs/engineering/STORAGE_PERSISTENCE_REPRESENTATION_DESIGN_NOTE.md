# Storage Persistence Representation — Engineering Design Note

| Property | Value |
|---|---|
| **Document** | STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md |
| **Project** | modIQ |
| **Purpose** | Investigate how an `AssessmentReport` can be persisted and retrieved faithfully without requiring Runtime identities (`AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId`) to become externally serializable or reconstructable, and without modifying `modiq-runtime` or `modiq-report`. Engineering Design Note only. |
| **Origin** | Chief Architect directive, pausing Phase 1 after implementation-time verification found that `AssessmentReport` and its nested Runtime types cannot be round-tripped through storage without either modifying two crates the Sprint Plan designates unmodified, or fabricating identity on read — a real conflict, reported rather than resolved. |
| **Status** | **Design investigation only. No code, no crate, no Cargo workspace change, no architectural decision altered. Awaiting Chief Architect review before Phase 1 resumes.** |

---

## 1. Restating the Conflict, Precisely

Verified directly against source (`crates/modiq-runtime/src/assessment/assessment_id.rs`, `evidence.rs`, `finding.rs`; `crates/modiq-report/src/report/assessment_report.rs`):

- `AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId` are private-field wrappers around a `u64`, with exactly one public method each: `generate()`. No accessor exposes the inner value; no constructor accepts a chosen one.
- `generate()` draws from a **function-local, process-scoped `static NEXT: AtomicU64 = AtomicU64::new(1)`** — the counter restarts at 1 every process invocation. Two unrelated Assessments, in two separate process runs, routinely receive numerically identical ids. Identity is not merely hard to serialize — **it is not a well-defined, durable concept across a process boundary today, by design.**
- `Evidence::new`, `Finding::new`, and (by the same shape) `Recommendation::new` each call their own id type's `generate()` internally; none has a raw constructor accepting a pre-existing identity. `AssessmentReport` has exactly one public constructor, `generate(&Assessment)`, requiring a live `Assessment`.
- This is a deliberately tested invariant, not an oversight: `Finding`'s own test suite includes `finding_with_identical_content_but_different_identity_is_not_equal`; `Evidence`'s includes the equivalent. `PartialEq` is identity-inclusive by explicit design.

Any read path that reconstructs these types from persisted data would necessarily mint new identities, different from the originals — meaning a "retrieved" report could never satisfy `retrieved == original` under `modiq_report::AssessmentReport`'s own `PartialEq`, no matter what storage technology is chosen.

---

## 2. Should Storage Introduce Its Own Persistence Model? — Yes.

`modiq-storage` should define its own, independent representation of what a report says — not attempt to store or reconstruct `modiq_report::AssessmentReport` itself. This is not new architectural machinery; it is the same shape ADR-0007's Opaque Runtime References pattern already establishes one layer upstream: a domain that needs to represent something it does not own defines its own local type for that purpose, rather than depending on the owning domain's real type or forcing the owning domain to change shape for its benefit. `RuleReference`, `RepairRecipeReference`, and `VersionProfileReference` already solve a structurally identical cross-boundary problem this way, three times, without exception. A Storage-owned representation is the fourth instance of an already-proven pattern, not a new one.

---

## 3. Should Report Own That Mapping? — No.

Two candidate designs:

- **(a) `modiq-report` gains an export method** producing a storage-friendly shape. Rejected: this modifies `modiq-report`, which the Sprint Plan explicitly designates unmodified for this Sprint, and it would blur Reporting's own stated boundary — "The Reporting System performs no analysis... its responsibility is presentation" (`Architecture.md`) — the identical reasoning the Architecture Evaluation already used to reject folding Storage into `modiq-report` in the first place (`STORAGE_ARCHITECTURE_EVALUATION.md` §3). Teaching Report to know about persistence-shape concerns is the same boundary violation in the opposite direction.
- **(b) `modiq-storage` performs the mapping itself**, using only `AssessmentReport`'s already-public read-only getters (`assessment_id()`, `status()`, `evidence()`, `findings()`, `recommendations()`) and the nested types' own already-public getters (`Evidence::category()`/`description()`/`location()`, `Finding::severity()`/`description()`, etc.). This requires zero modification to `modiq-report` or `modiq-runtime` — it is an ordinary, additive consumer relationship, the same one `modiq-cli` and `apps/sandbox` already have with `modiq-report` today.

**Recommendation: (b).** Report does not own the mapping; Storage does, using only what Report already exposes publicly.

---

## 4. Where Should Transformation Occur? — Entirely Within `modiq-storage`.

Two transformation points, both inside Storage's own crate boundary:

- **At write time:** `AssessmentReport`'s public getters are read and copied into Storage's own representation. Nothing upstream is touched or altered.
- **At read time:** Storage's own persisted form is reconstructed into Storage's own return type — not into `modiq_report::AssessmentReport`. There is no attempt, at any point, to produce a value of the original live type from persisted data.

This keeps the entire transformation inside the one crate the Architectural Resolution already assigned this responsibility to (Decision 2), touching only already-public APIs of the crates it depends on.

---

## 5. How Should Retrieved Reports Be Represented? — A Storage-Owned Read Representation, Not a Reconstructed `AssessmentReport`.

The read path should return a type `modiq-storage` itself defines — not `modiq_report::AssessmentReport`. This is not a novel pattern for this platform: `AssessmentReport` is already exactly this idea, one layer upstream — "an owned, point-in-time snapshot" of an `Assessment`, deliberately not the live `Assessment` itself, so that a Reporting consumer gets a faithful, read-only view rather than the original live type. Storage's own persisted representation is the identical pattern applied one further step downstream: a snapshot of a snapshot, independently typed, read-only, faithful in content.

**A sub-problem worth naming explicitly:** `Finding.evidence_ids` cross-references `Evidence` within the same report (`INV-013`/GOV-005's "must reference at least one Evidence item"). Since `EvidenceId`'s own value is not stable or meaningful outside the process that generated it, this relationship cannot be preserved by reusing the original id. It does not need to be: within a single persisted report, a locally-scoped correlation (for instance, an ordinal position in the persisted evidence list) preserves the *relationship* Finding-to-Evidence faithfully, without needing the *value* to mean anything beyond that one persisted document. The referential intent survives; the specific numeric token does not need to.

---

## 6. Can the Persisted Representation Remain Faithful Without Preserving Runtime Identity? — Yes.

This is directly supported by a principle this platform already states for itself, not a new standard invented for this document: `PROJECT_HANDOFF_v1.1.md` §5 — "every Runtime entity's own identity is freshly assigned per construction/evaluation, by design... Determinism is judged by content and order, never by incidental identity." That principle was stated for same-process determinism testing; Section 1's finding (the process-restarting atomic counter) shows it applies with at least equal force across a persistence boundary, since identity is not even durably meaningful there today.

**Faithfulness, redefined accordingly:** a retrieved representation matches the original when its content and order match — same status; same evidence entries (category, description, location) in the same order; same finding entries (severity, description, evidence-correlation, rule reference) in the same order; same recommendation entries in the same order. This is content-and-order equality against Storage's own representation, not Rust-level `PartialEq` against `modiq_report::AssessmentReport`. Any future acceptance test for this Sprint should be written against that understanding explicitly, not assumed.

---

## 7. Relationship to the Four Fixed Architectural Decisions

- **Decision 1 (`AssessmentReport` is the persisted domain object):** unaffected. This decision fixed the *conceptual granularity* of what gets persisted — a whole report, not the live `Assessment`, not individually addressable `Evidence`/`Finding`/`Recommendation` records — not the literal Rust type or byte representation on disk. Storage persisting its own representation of a report's content satisfies this decision at the granularity it was actually made at.
- **Decision 2 (`modiq-storage` owns persistence):** reinforced, not merely preserved — the transformation and its own representation live entirely inside `modiq-storage`, the cleanest possible expression of this ownership.
- **Decision 3 (strictly downstream of Reporting):** unaffected — the transformation still occurs strictly after Report generation, reading only Report's already-public output.
- **Decision 4 (`AssessmentService` unchanged):** unaffected — no relationship to this question.

No architectural decision is revisited by this resolution.

---

## 8. What This Resolves, and What It Leaves Open

**Resolves:** the Phase 1 blocking conflict, without modifying `modiq-runtime` or `modiq-report`, and without conflicting with any of the four fixed decisions.

**Left open, deliberately, for implementation:** the exact name and field shape of Storage's own representation type; the exact serialization mechanism (unrelated to this question, already deferred); whether Storage's read path is consumed directly by callers or converted again at the CLI/Sandbox layer. These remain Lead-Engineer-level representational details, the same category Sprint 8 and Sprint 9 both left open once policy was resolved (`VersionProfileReference`'s own shape, `RepairRecipeReference`'s own shape) — not additional architecture questions.

---

## Recommendation

Storage should define and own its own persisted representation of a report's content, populated from `AssessmentReport`'s already-public API at write time and returned as its own type at read time — never attempting to reconstruct `modiq_report::AssessmentReport` or any Runtime entity's original identity. This resolves the blocking conflict entirely within `modiq-storage`'s own crate boundary, requires no change to `modiq-runtime` or `modiq-report`, and revisits none of the four fixed Architectural Resolution decisions. This is a design investigation, not an authorization — Phase 1 remains paused pending your review.
