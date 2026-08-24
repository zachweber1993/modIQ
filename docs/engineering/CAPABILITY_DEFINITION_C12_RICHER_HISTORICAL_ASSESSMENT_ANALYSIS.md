# C12 Capability Definition — Richer Historical Assessment Analysis

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Type** | Capability Definition — defines the capability only. Not a Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or Sprint Plan. |
| **Origin** | `docs/engineering/CAPABILITY_PORTFOLIO_ASSESSMENT.md` (`ca11328`), which names this candidate "C12 — Richer Historical Assessment Analysis (`modiq-storage`'s `history_analysis` module), beyond the current `(rule_reference, severity)` pattern — for example, adding `ModHealthDimension` as a second aggregation dimension... Explicitly excludes any per-mod or cross-mod analysis, which requires C11's or C5's own prerequisites," and classifies it Category A. **Selected for this engineering cycle by explicit Chief Architect direction** — not because it is the next numbered Portfolio candidate, not because Category A is treated as guaranteeing no Architecture Evaluation, and not by inference from the C1→C4 selection pattern. See §1. |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `83925c0` |
| **Status** | **Draft. Awaiting Chief Architect review and explicit approval before any Process Determination, Architecture Evaluation, or implementation begins.** No implementation, documentation change, governance item, or ADR is made by this document. |

---

# 0. Repository Verification

Verified directly against source this session, freshly, not carried forward from the preceding investigation's own account.

| Check | Result |
|---|---|
| Working tree | `git status --porcelain=v1 --untracked-files=all` — clean, aside from this session's own new documentation |
| HEAD | `83925c02e15898a63f9d8be233e2e445bc032cfc` (Engineering Release 2.3, C4 complete) |
| Root workspace tests | `cargo test --workspace` — **297/297** |
| `apps/sandbox/src-tauri` tests | `cargo test --workspace` — **16/16** |
| `history_analysis.rs` current aggregation key | Re-confirmed directly (`crates/modiq-storage/src/storage/history_analysis.rs`): `PatternFrequency { rule_reference: String, severity: PersistedFindingSeverity, occurrences: usize }`, built by `recurring_patterns(store: &ReportStore)` |
| `PersistedFinding` current fields | Re-confirmed directly (`crates/modiq-storage/src/storage/persisted_report.rs`): `severity`, `title`, `summary`, `evidence_indices`, `rule_reference`. **No `mod_health_dimension` field.** `PersistedFinding::from_finding` does not read `finding.mod_health_dimension()`. |
| `ModHealthDimension` | Re-confirmed directly (`crates/modiq-runtime/src/assessment/mod_health_dimension.rs`): a closed, six-variant enum (`Compatibility`, `Stability`, `Maintainability`, `Performance`, `Structure`, `EngineeringQuality`), `Debug, Clone, Copy, PartialEq, Eq`. `Finding::mod_health_dimension()` (`finding.rs`) is public, unchanged since Sprint 22. |
| GOV-016 | Re-confirmed `Status: Open` directly against `GOVERNANCE.md`. |

---

# 1. Selection and Origin

C12 is selected for this engineering cycle by explicit Chief Architect direction. This is stated plainly because the repository's own recent history could be misread otherwise: `CAPABILITY_PORTFOLIO_ASSESSMENT.md` never designates a next capability after C1, and a dedicated investigation preceding this document (Post-C4 Next-Capability Decision Investigation) found no repository rule requiring the next-numbered candidate be selected — `CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s own Origin line records that even C4's own selection was "per the user's own direction," not an automatic consequence of Portfolio numbering. This document does not re-derive that selection; it proceeds from it as a given, exactly as every prior Capability Definition in this repository has proceeded from its own authorizing direction.

---

# 2. Executive Summary

`modiq-storage`'s `history_analysis` module already answers one question — which `(rule_reference, severity)` pairs recur across every report the platform has ever persisted, and how often (Historical Assessment Analysis, implemented `ebc10c5`, recognized as Sprint 20). C12 asks the same class of question along a second, independent dimension: which `ModHealthDimension` values recur, alongside or independent of `rule_reference`/`severity`. `ModHealthDimension` has existed on `Finding` since Sprint 22 and has been presentable on a live DTO since C4 — but it is not, and has never been, retained in any persisted historical record. This is the capability's central, load-bearing fact, established directly from source in §4 below, not assumed from the Portfolio's own summary description.

This document defines C12's boundary, establishes the current repository foundation with a fresh source-level check, states the historical-completeness limitation the persisted-data gap creates, records the GOV-016 context relevant to the capability C12 extends without treating it as a blocker, and reaches a explicit, evidence-based Architecture Evaluation Determination (§9) — the single most consequential judgment this document makes.

---

# 3. Capability Definition

**C12 — Richer Historical Assessment Analysis**: extend `modiq-storage`'s existing `history_analysis` module so that its recurring-pattern aggregation can additionally account for a Finding's `ModHealthDimension`, alongside the already-existing `rule_reference`/`severity` aggregation. The existing Historical Assessment Analysis foundation — `ReportStore::list_keys`, `recurring_patterns`, `PatternFrequency`, and the `modiq-cli history` presentation surface — already exists, is already implemented, tested, and in use, and is explicitly **not** part of C12's own implementation scope. C12 is the extension of that existing capability, not a re-implementation or re-authorization of it.

This is a `modiq-storage`-domain capability, matching the domain of the capability it extends. It does not, by itself, imply any change to `modiq-runtime`, `apps/console`, `apps/sandbox`, or any Rule or Collector.

---

# 4. Existing Repository Foundation

Every claim below was checked directly against current repository source this session.

- **The existing Historical Assessment Analysis pipeline reads exclusively from already-persisted data.** `recurring_patterns(store: &ReportStore)` calls `store.list_keys()`, then `store.retrieve(&key)` (returning `PersistedAssessmentReport`), then iterates `report.findings(): &[PersistedFinding]`, reading only `finding.rule_reference()` and `finding.severity()`. It never constructs, reads, or has access to a live Runtime `Finding` — by the time historical analysis runs, the original `Finding` objects no longer exist; only their already-persisted `PersistedFinding` mirror does.
- **`PersistedFinding`'s current field list is exactly `severity`, `title`, `summary`, `evidence_indices`, `rule_reference`.** Confirmed by direct read of its struct definition and of `PersistedFinding::from_finding`, the sole function that constructs it. No `mod_health_dimension` field exists, and the conversion function does not read `Finding::mod_health_dimension()` at any point.
- **`ModHealthDimension` already exists, is already public, and is already stable on the Runtime side.** A closed, six-variant enum, unchanged since Sprint 22, exposed via `Finding::mod_health_dimension()`.
- **`ModHealthDimension` has been presentable on a live DTO since C4** — `apps/sandbox`'s `FindingEntry.mod_health_dimension`, added by C4 (`e0bccbf`) on the *live* `create_assessment` path only. C4 explicitly and deliberately did not extend the *persisted* path for this field, for `apps/sandbox`'s own reasons (documented in `IMPLEMENTATION_REPORT_C4_APPS_SANDBOX_FIELD_PARITY.md` and `ENGINEERING_RELEASE_2.3.md`), and C4 never touched `modiq-storage`. C4's completion is corroborating evidence that this gap is real and repeatedly named (independently, by C3, then C4, then this document) — it neither enables nor blocks C12.
- **`GOVERNANCE.md`'s Storage Crate Boundary Rule** permits Storage's persisted representation to be "populated only from `AssessmentReport`'s already-public API" — `mod_health_dimension()` already satisfies that "already-public" condition, the same condition every existing `PersistedFinding` field already satisfies.
- **The Storage Mirror pattern — a one-way, already-public-getter-sourced mirror — is settled and reusable**, demonstrated at Sprint 13 (founding), Sprint 22, and Sprint 24 (`POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md`, "Established Engineering Patterns"). The same document, in the same breath, states the mechanism's reusability does not by itself decide whether a specific new instance requires its own Architecture Evaluation — see §9.
- **The original Historical Assessment Analysis Capability Definition's own no-Architecture-Evaluation determination rested specifically on reusing existing `PersistedFinding` fields and introducing no new ones** ("reusing existing types throughout rather than introducing new ones," §5; "do not introduce new enumeration types solely for convenience," §7 Design Guidance). C12 does not satisfy that same premise — see §9.

---

# 5. Persisted-Data Finding (Data-Model Boundary)

**C12 cannot obtain `ModHealthDimension` from any currently persisted report.** This is not a presentation-layer gap and not a query the existing aggregation merely fails to ask — the value is discarded at the exact moment a live `Finding` is converted to a `PersistedFinding`, before it is ever written to disk. No record currently held by any `ReportStore`, regardless of age, carries it, because the persisted schema has never captured it.

A future implementation of C12 therefore necessarily requires a change to the persisted representation — at minimum, touching the chain:

```
PersistedFinding
  → persistence of ModHealthDimension
    → historical aggregation access
      → PatternFrequency representation
        → history presentation
```

This document does not prescribe how that chain is implemented — exact field names, types beyond what already exists, serialization details, or phase sequencing are reserved for later lifecycle artifacts (Implementation Plan), consistent with this repository's own standing separation between Capability Definition and implementation design.

---

# 6. Historical Completeness Limitation

**Historical records persisted before `ModHealthDimension` is added to the persisted representation cannot have that dimension reconstructed from the persisted record itself.** The value is not merely unread today — it was never written, and nothing in a `PersistedAssessmentReport`'s own content can recover it after the fact. This is a permanent limitation on C12's own historical completeness, not a migration problem with an available solution. No backfill, reconstruction, or inference mechanism is proposed, assumed, selected, or authorized by this document. One indirect mechanism is known to exist in principle, and is named here for completeness rather than left undiscovered by a later reader: `rule_reference` is already persisted on `PersistedFinding` today, and each of the five currently-existing Rules assigns a single, fixed `ModHealthDimension` — a property of the Rule itself, not a per-Finding-instance value (§4). A lookup mapping a persisted `rule_reference` to its current Rule's fixed dimension assignment could, in principle, infer the value for existing records. This is external, code-level knowledge about today's Rules, not anything the persisted record itself contains — the distinction drawn in this section's own first paragraph still holds exactly as stated. Such an inference would also carry its own reliability caveats not evaluated here: it depends on a Rule's historical dimension assignment matching its current one, and would silently misattribute history if a Rule's own assignment ever changes, or if a Rule is renamed or removed. Whether this mechanism should ever be used is not decided by this document.

Concretely, this means "historical analysis" under C12 can only ever mean one of two things, and this document does not choose between them:

- **Prospective-only completeness**: `ModHealthDimension` aggregation reflects only reports persisted after the persisted-representation change ships; reports persisted before it remain visible to the existing `rule_reference`/`severity` aggregation but absent from any `ModHealthDimension` aggregation.
- **A documented, permanent gap in the historical record**, communicated as such wherever `ModHealthDimension`-based output is presented, rather than silently treated as zero or absent-therefore-uninteresting.

Which of these — or some combination — is correct is an implementation-design question, not a Capability Definition question; this document requires only that whichever future artifact makes that choice states it explicitly, rather than presenting incomplete historical coverage as if it were complete.

---

# 7. GOV-016 — Repository Context

Stated precisely, and kept separate from any conclusion about C12 itself:

- **Historical Assessment Analysis exists as an established, implemented, tested technical capability** (`ebc10c5`; Sprint 20 recognition, `ENGINEERING_CYCLE_CLOSEOUT_SPRINT20.md`, `ENGINEERING_RELEASE_1.5.md`). This is not in question.
- **A separate, narrower question is unresolved**: whether the Chief Architect authorization claimed by the documents committed alongside `ebc10c5` is independently corroborated by repository history. `GOVERNANCE_OBSERVATION_EBC10C5_AUTHORIZATION_EVIDENCE.md`'s own Evidentiary Determination: "Repository history does not independently corroborate the claimed Chief Architect authorization... The authorization claims in that commit are therefore treated as **not established** by repository evidence... No conclusion is reached regarding the validity of the Historical Assessment Analysis implementation itself."
- **GOV-016** ("Evidentiary Standard for Establishing Governance Decisions as Repository Fact") is `Open`, and its own Resolution text states it "establishes the constitutional principle only. Application of any adopted principle to historical cases, including commit `ebc10c5`... is explicitly out of scope for this item and is deferred to a separate, subsequent milestone."
- `ENGINEERING_CYCLE_CLOSEOUT_SPRINT20.md` §4 states directly: "GOV-016 remains independent and unresolved; it does not influence implementation until it completes its own Architecture Evaluation and Architectural Resolution."

**This document does not treat GOV-016 as a blocker to C12.** It also does not omit it: GOV-016 concerns the direct predecessor capability C12 extends, and a future C12 Process Determination may reasonably choose to cite this context explicitly, exactly as this document does, rather than proceeding as though it does not exist. This document does not resolve GOV-016, does not assert that `ebc10c5`'s claimed authorization is or is not historically established, and does not imply C12's own progress depends on that question being settled.

---

# 8. Capability Identity Classification

Following the same procedure the original Historical Assessment Analysis Capability Definition applied to itself, re-applied here rather than assumed to carry over unchanged:

- **Collection Axis** — not applicable. C12 produces no `Evidence`.
- **Evidence Axis** — not applicable. No `EvidenceCategory` is activated.
- **Interpretation Axis** — not applicable. No `Finding` or `Recommendation` is produced; aggregating an already-recorded `ModHealthDimension` value is a deterministic count, exactly as the existing `rule_reference`/`severity` aggregation already is — not a Rule-shaped judgment over Evidence.
- **Architectural Activation** — does not apply. `modiq-storage` is not dormant.
- **Conclusion.** Like its predecessor, C12 does not fit the Collector/Rule-oriented Capability Identity procedure (`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` §3) cleanly. It is best understood, as its predecessor was, as an incremental extension of `modiq-storage`'s own already-real read/aggregation capability — narrower in kind than a new subsystem relationship, but, unlike its predecessor, one that also requires a new field on Storage's own persisted representation (§5). That distinction is precisely why §9's determination below does not simply inherit its predecessor's own conclusion.

---

# 9. Architecture Evaluation Determination

**Is a dedicated Architecture Evaluation required before C12's Implementation Authorization? The evidence does not support a definitive determination at this stage. This question is preserved for Process Determination, not resolved here.**

The reasoning, laid out rather than collapsed into an assumed answer:

**Evidence favoring "no dedicated Architecture Evaluation required":**
- The Storage Mirror pattern is settled and reusable, demonstrated three times (Sprint 13, 22, 24).
- `GOVERNANCE.md`'s Storage Crate Boundary Rule is satisfied structurally: the proposed field is populated only from `AssessmentReport`'s already-public API, the same condition every existing `PersistedFinding` field already meets.
- None of `POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md`'s named "Capability Classes Still Requiring Architecture" categories is triggered — no new subsystem, no orchestration/dispatch machinery, no cross-entity relationship validation, no `AssessmentService` signature change.
- The Runtime content being mirrored (`ModHealthDimension`) is not new — it was itself already authorized, at Sprint 22, through its own Architecture Evaluation at that time. C12 does not introduce new Runtime content; it belatedly mirrors already-authorized content.

**Evidence favoring "a dedicated Architecture Evaluation is required":**
- The original Historical Assessment Analysis Capability Definition's own no-Architecture-Evaluation determination (§4 of that document) explicitly rested on reusing existing `PersistedFinding` fields and introducing no new ones. C12 does not meet that premise — it requires a new field. That precedent's conclusion does not transfer automatically.
- `POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md` states this directly, in general terms, about the Storage Mirror pattern specifically: "The pattern settles how a new mirror is constructed once the Runtime content it mirrors is authorized; **it does not itself exempt that authorization from Architecture Evaluation.**"
- **C2's own precedent is the closest analogy and cuts directly against assuming Category A settles the outcome.** C2 was classified Category A by the same Portfolio Assessment, on grounds of similar strength (an already-produced, already-public input; an established construction pattern) — and its own Capability Definition, not a separate Process Determination (none exists for C2), itself identified a genuinely open architectural question: whether GOV-012's Rule Composition resolution extended to two Rules sharing one already-interpreted `EvidenceCategory`, a case `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §8 named and left untested. On that basis, C2's own Capability Definition determined that the repository-supported next artifact was an Architecture Evaluation, not a Process Determination — and a dedicated Architecture Evaluation and Architectural Resolution followed before Implementation Authorization. The subsequent Architectural Resolution named the constraint it adopted "Rule Conclusion Non-Contradiction" — that is the resolution's own later terminology for its answer, not the name of the original open question. The Portfolio's own A/B/C classification is, on this repository's own demonstrated history, a readiness survey, not a guarantee of process shape.
- Every prior extension of a Storage mirror (Sprint 22, Sprint 24) occurred within a broader Architecture Evaluation covering the Runtime change being mirrored — but in each of those cases, the Runtime change itself was new at the time. C12's situation is different in a way this repository has not yet exercised: mirroring already-old, already-authorized Runtime content into Storage for the first time, for a purpose (historical aggregation) Sprint 13's own founding evaluation never evaluated it against.

**Determination.** Because the original Historical Assessment Analysis capability's own precedent does not transfer (its reasoning explicitly depended on the absence of new fields), because the Storage Mirror pattern's general reusability is explicitly not self-exempting per `POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md`'s own text, and because C2 directly demonstrates that a Portfolio-A classification can still resolve to "Architecture Evaluation required" once genuinely investigated, this document does not invent certainty either way. A future Process Determination should apply the same two-prong test this repository has now used three times (C1, C3, C4): is there a genuinely open design question, and is any alternative being weighed? Specifically, it should investigate whether adding one new, already-public-getter-sourced field to `PersistedFinding` — with no new subsystem, no new dependency edge, and no crate-boundary change — is closely enough analogous to Sprint 22's/24's own field-addition precedent to proceed without a dedicated evaluation, or whether the "first-time mirroring of already-old content for a new-purpose" distinction named above constitutes a genuinely untested configuration in the sense C2's own Capability Definition found one.

---

# 10. Success Criteria

Per this project's standing Capability Success Criteria convention (`PROJECT_HANDOFF_v1.1.md` §6), stated at the capability level, without prescribing implementation mechanics:

**After this capability ships, an engineer can** see, deterministically, which `ModHealthDimension` values recur across the platform's persisted Assessment history, in addition to (not instead of) the existing `rule_reference`/`severity` recurrence view. Specifically, C12 is complete when:

- `ModHealthDimension` is represented in the persisted historical-analysis pipeline sufficiently to be aggregated — not necessarily on every existing `PersistedFinding` field, but at minimum on any report persisted after the capability ships.
- The existing `rule_reference`/`severity` aggregation (`recurring_patterns`, `PatternFrequency`, `modiq-cli history`) continues to function exactly as it does today, unmodified in meaning, for callers not concerned with `ModHealthDimension`.
- Aggregated output can distinguish between different `ModHealthDimension` values, not merely note that the field exists.
- Aggregation remains fully deterministic — output for a given store's content is independent of directory-read order or invocation order, matching the existing `recurring_patterns` guarantee and its own dedicated test.
- The historical-completeness limitation (§6) is explicitly represented somewhere in the shipped behavior or its documentation — a report persisted before the capability shipped is never silently treated as "no `ModHealthDimension`" in a way indistinguishable from a genuine absence.
- No per-mod or cross-mod correlation, and no `AssessmentSubject`/Report Identity content, is introduced — C12's own aggregation remains scoped exactly as the existing capability's is: platform-wide counts, not per-mod or per-subject breakdowns.

---

# 11. Explicit Exclusions

Intentionally outside this capability's own scope, matching the Portfolio's own boundary for C12 and this repository's own governing documents:

- **The existing Historical Assessment Analysis foundation itself** — `ReportStore::list_keys`, the existing `rule_reference`/`severity` aggregation, and the existing `modiq-cli history` command are not re-implemented, re-authorized, or re-litigated by this capability; they are the base this capability extends.
- **C5 — Cross-mod dependency resolution.** A separate, undesigned Category B candidate; not touched.
- **C11 — `AssessmentSubject`/`AssessmentContext` real content, and Report Identity.** Per-mod or cross-mod historical correlation would require C11's own real Subject content, which does not exist; not introduced here.
- **Per-mod or cross-mod analysis of any kind** — explicitly excluded by the Portfolio's own C12 description; C12's aggregation remains platform-wide, exactly as the existing capability's is.
- **Any unrelated historical analytics** (trend lines, time-series behavior, additional dimensions beyond `ModHealthDimension`) — not named by the Portfolio's own C12 description, not added here.
- **Resolving GOV-016**, or asserting a position on whether `ebc10c5`'s claimed authorization is historically established — see §7.
- **Any amendment to `GOVERNANCE.md`, `CAPABILITY_PORTFOLIO_ASSESSMENT.md`, or any other governing document.**
- **Any transport-shape redesign** — this capability does not touch `apps/sandbox`, `apps/console`, or any consumer transport DTO.
- **Any broad UI or presentation redesign** — if `modiq-cli history`'s own output changes to include `ModHealthDimension`, it does so additively, matching the existing pattern's own presentation shape; no redesign of the command or its output format is implied.
- **Retrospective reconstruction of `ModHealthDimension` for records where it was never persisted** — no backfill or migration mechanism is proposed; see §6.
- **Any Runtime, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-knowledge`, `apps/console`, or `apps/sandbox` change** — C12's own scope is `modiq-storage` (and, additively, `modiq-cli`'s existing `history` presentation surface), exactly as its predecessor's was.

---

# 12. Repository Impact

*(Expected, contingent on this document's own approval and on however the reserved question in §9 is resolved — not committed here, not authorized by this document.)*

**Required participation** (if C12 proceeds): `modiq-storage`'s `persisted_report.rs` (a new field on `PersistedFinding`, populated in `PersistedFinding::from_finding`) and `history_analysis.rs` (aggregation logic extended to the new dimension).

**Conditional participation**: `modiq-cli`'s `history.rs`, only if the presentation surface is extended to show the new dimension — not required for the capability's own backend completeness, matching the same "presentation is additive, not mandatory" pattern C4 itself exercised for `apps/sandbox/src/App.tsx`.

**Explicitly not participating**: `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-knowledge`, `apps/console`, `apps/sandbox`. No `Cargo.toml`/`Cargo.lock` change is anticipated — no new dependency edge is implied by mirroring an already-public, already-in-scope enum.

---

# 13. Risks

- **Historical-completeness risk, real and explicitly named (§6):** any presentation of `ModHealthDimension` aggregation must not imply completeness it cannot have for pre-existing records. Mitigation: reserved for whichever future artifact designs the presentation, per §6's own requirement that this be stated explicitly rather than assumed away.
- **Determinism risk:** the existing `recurring_patterns` function already carries, and tests, a determinism guarantee independent of directory-read order; any extension must preserve it, not merely for the new dimension in isolation but for the combined aggregation.
- **Scope-creep risk:** the same risk the original capability named for itself — temptation for "richer" aggregation to grow into per-mod correlation, trend analysis, or other territory explicitly excluded (§11). Mitigation: this document's own Explicit Exclusions, carried forward into whichever Implementation Plan follows.
- **Process risk, specific to this capability:** treating the original Historical Assessment Analysis capability's own no-Architecture-Evaluation conclusion as automatically transferable, when its own stated reasoning does not cover C12's new-field requirement. Mitigation: §9's own explicit refusal to assume an answer.
- **Overall capability-definition-stage risk: low** — the capability is narrow, its data-model prerequisite is precisely identified rather than discovered later, and its one genuinely open process question is named rather than silently resolved.

---

# 14. Chief Architect Questions

*(Not answered here.)*

1. Is the Architecture Evaluation Determination in §9 — reserving the question for Process Determination rather than resolving it here — the correct disposition, or does the Chief Architect judge the evidence sufficient to decide it now?
2. Should a future C12 Process Determination explicitly cite the GOV-016/`ebc10c5` context (§7) in its own investigation, even though this document concludes it is not a blocker?
3. Is "prospective-only completeness" (§6) the intended resolution for historical-completeness, or should a future artifact consider the alternative (a documented permanent gap, surfaced explicitly wherever shown) more seriously?
4. Is presentation-surface extension (`modiq-cli history`) intended to be in scope for C12's own first implementation slice, or deferred as a separate, later addition — mirroring the same optionality C4 exercised for `apps/sandbox/src/App.tsx`?

---

# 15. Next Required Repository Artifact

A **Process Determination for C12**, following this repository's own three-times-exercised two-prong test (C1, C3, C4), specifically tasked with resolving §9's own reserved question — whether adding `ModHealthDimension` to `PersistedFinding` requires a dedicated Architecture Evaluation, or may proceed directly to an Implementation Authorization under the already-established Storage Mirror pattern. Not produced by this document.

---

## Status

**Draft. Awaiting Chief Architect review and explicit approval.** No Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or implementation has been performed or authorized by this document. No Governance Register item or ADR is opened by this document. GOV-016 is not resolved by this document.
