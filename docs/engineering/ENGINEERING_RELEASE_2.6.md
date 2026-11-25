# Engineering Release 2.6

| Property | Value |
|---|---|
| **Release** | 2.6 |
| **Documentation Release** | 2.1 (Frozen — unchanged this capability; no specification amendment) |
| **Milestone** | Store Item Reference Integrity complete — the platform's first capability to relate a fact declared in one collected document to the inventory collected from another source, determining whether ordinary mod-root-relative files declared by `storeItem xmlFilename` entries in a mod's root `modDesc.xml` are present in the assessed ZIP archive's collected inventory |
| **Scope** | Store-item declaration extraction in `XmlCollector`, one new Rule (`StoreItemReferenceRule`) registered last in the existing fixed evaluation order, and test coverage across collection, Rule, dispatch, and engine-integration layers — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_2.5.md` (C15) |
| **Governing ADRs** | None new — `ADR-0007`, `ADR-0008`, `ADR-0010`/GOV-004 applied, not amended |
| **Governing Documents** | `docs/engineering/ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md` (the only lifecycle artifact persisted in this repository for this capability). GOV-010, GOV-011, GOV-012 and `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` applied, not amended. The capability's remaining lifecycle artifacts were completed and delivered during the engineering process but were not persisted in this repository — see §9, an accepted documentation gap recorded by explicit Product Owner decision |
| **Capability Number** | None assigned. This capability carries no `C`-number; none was authorized |

---

## 1. Executive Summary

Store Item Reference Integrity gives modIQ its first capability that evaluates a *relationship between two independently collected facts*: a path a mod declares in its own manifest, and the inventory of entries collected from the archive that manifest ships inside. Every prior Rule reasoned over Evidence of a single category; this one reads declarations (`XmlInspection`) and archive entries (`FileStructureAnalysis`) together, and concludes only what the collected inventory establishes.

The capability is deliberately narrow. It answers one question — was a declared file present in what collection actually observed — and refuses several adjacent ones. It is restricted to ZIP archive assessments, because adopted collection policy omits certain filesystem entries from Evidence without any Rule-visible signal (the Symbolic Link Policy and skipped non-regular objects), and those omissions are the only conditions under which a false missing-file conclusion could arise. For ZIP archives the two omission conditions that exist cannot conceal a declared mod-root-relative path: a traversal-invalid entry name is, by construction, not at the location the manifest declares, and duplicate-name collapse preserves membership exactly while degrading only count fidelity.

Four path forms are excluded from missing-file conclusions rather than normalized or guessed at — absolute and leading-separator paths, `$`-based expressions, backslash-containing paths, and paths carrying `.` or `..` segments. Excluded declarations remain fully traceable as Evidence; they simply yield no conclusion. Every Finding is bounded in its own wording to the collected inventory and makes no claim about runtime loadability, functional correctness, or in-game behavior.

This release also records the resolution of one architectural question the implementation surfaced: `StoreItemReferenceRule` is the first Rule to report more than one outcome from a single evaluation. A dedicated Architectural Impact Review found this permissible under the existing design, with no downstream defect — §8.

---

## 2. Lifecycle Status and Authorization

Every stage below was completed and approved by the Product Owner. No dates are recorded, consistent with this document series' own convention.

| Stage | Status |
|---|---|
| Capability Definition | Completed and approved |
| Process Determination | Completed — concluded that Architecture Evaluation was required (the C2 path), not the shorter no-Evaluation path C1/C3/C4/C14 followed |
| Architecture Evaluation | Completed — identified exactly two genuine architectural gaps (absence-based conclusions; archive path representation) and classified everything else as already-adopted permission or implementation convention |
| Architectural Resolution | **Adopted** — `docs/engineering/ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md` |
| Implementation Plan | Reviewed and approved; five implementation questions (Q1–Q5) surfaced rather than silently decided |
| Implementation Authorization | Issued, resolving Q1–Q5 and adding an explicit ZIP-only provenance safeguard |
| Implementation | Completed, phase-gated across Collection, Rule, and Integration |
| Architectural Impact Review | Completed — recommendation B (release as implemented; clarify GOV-012 at closeout) |
| Engineering Release 2.6 | **Approved by the Product Owner** |

The Architectural Resolution states, correctly and in three places, that it does not itself authorize implementation. Implementation was authorized separately, by the Implementation Authorization recorded above. Both statements are accurate; §11 carries a closeout note so the two are not read as contradictory.

---

## 3. Scope Delivered

Exactly the following behavior, and no more:

**Collection.**
- Direct-child `storeItem` elements under the canonical `storeItems` element in root `modDesc.xml` are recognized. Unrestricted descendant matching is deliberately not used, so a `storeItem` element appearing elsewhere in the document is not extracted.
- One `XmlInspection` Evidence item per recognized declaration, in document order, appended strictly after every pre-existing manifest Evidence item — positions of the manifest-found, declared-version, and declared-dependency items are unchanged.
- Every structurally valid declaration is preserved, including degenerate forms: an absent `xmlFilename` is represented without inventing a path (`content` is `None`), and a present-but-empty or whitespace-only value is preserved verbatim in `content`. Both degenerate forms carry their own distinct description rather than the declaration prefix, so neither can produce a missing-file conclusion.

**Evaluation.**
- ZIP archive assessments only. Archive provenance is established from the presence of at least one collected archive file entry — never from whether a declared target itself resolves — so a ZIP carrying a valid manifest and a genuinely missing target is still evaluated, while a filesystem-directory assessment produces nothing at all.
- Supported declarations are compared against collected archive **file** entries, over raw as-stored names, with `/` as the recognized separator, case-sensitively, with no Unicode normalization applied.
- Directory entries never satisfy a file declaration.
- Unsupported path forms — absolute and leading-separator (including drive-qualified), `$`-based, backslash-containing, and `.`/`..`-segment-bearing — are skipped entirely: never normalized, never silently reinterpreted, never reported as missing. They remain fully traceable as declaration Evidence.
- `StructuralDuplication` is accounted for as the approved Resolution specifies — explicitly, in documentation and by an invariance test, with no runtime mechanism introduced. The membership test is unaffected by duplicate-name collapse, which preserves membership while degrading only count fidelity.

**Findings.**
- One independent Finding per unmatched supported declaration.
- Each Finding cites exactly its own corresponding declaration Evidence item.
- Severity `Warning`; Mod Health dimension `Structure`; status `Final`; no Recommendation.
- Finding wording is bounded to the collected archive inventory and asserts no runtime loadability, functional correctness, or in-game consequence.

**Engine.**
- The Rule is registered **last** in the existing fixed declaration order, after `DeclaredDependencyDuplicationRule`.
- All five pre-existing Rules are unaffected in content, severity, and ordering; no Rule source file was edited.

---

## 4. Architectural Decisions

As adopted in `docs/engineering/ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md`. Nothing below was decided by this release; it is recorded here for the historical record.

| Area | Adopted decision |
|---|---|
| **Assessment scope** | ZIP archive assessments only. Filesystem-directory assessments explicitly excluded. Establishes no runtime loadability or functional correctness |
| **Negative membership** | A Rule may conclude that no collected ZIP archive entry matched a supported declared path. Finding language shall remain bounded to the collected archive inventory. `StructuralDuplication` shall be accounted for explicitly. Existing archive traversal and invalid-entry handling remain unchanged. No new inventory-completeness concept and no Evidence model change |
| **Path representation** | The Archive Traversal Boundary Policy's normalization sentence is interpreted as governing the traversal-validity determination only, not the recorded Evidence `location`; archive-entry Evidence continues to carry the entry's name exactly as stored. Comparison is performed inside the Rule over raw names |
| **Comparison policy** | `/` separator; case-sensitive; no Unicode normalization; directory entries never satisfy a file declaration; unsupported forms never normalized or silently reinterpreted |
| **Excluded declared forms** | Absolute and leading-separator paths; `$`-based expressions; backslash-containing paths; paths containing `.` or `..` segments. Excluded declarations remain traceable and require no separate Finding merely for being unsupported |
| **Duplicate handling** | Duplicate archive entry names are governed by the existing Duplicate Archive Entry Policy; duplicate *store-item declarations* are out of scope for this capability |
| **Evidence representation** | Existing fields only: declaration prefix in `description`, declaring file in `location`, raw declared path in `content`, declaration context in `label`/`source`. No new Evidence field or category |
| **Cross-category Rule comparison** | Consistent with existing adopted architecture — `RuleEngine::evaluate` passes the full, unpartitioned Evidence set to every Rule and the dispatch mechanism carries no concept of category ownership (GOV-012 Question 3, as interpreted and adopted by C2 Decision 1) |
| **Finding granularity** | One Finding per unmatched declaration, each citing exactly one declaration Evidence item |
| **Rule ordering** | Registered last in the existing fixed declaration order |
| **Severity / dimension** | `Warning` / `Structure` |
| **Recommendation** | None required; a Finding may stand alone (Initiative 3, Item 3) |
| **ZIP-only safeguard** | Archive provenance established independently of whether the declared target exists |

**C2 Rule Conclusion Non-Contradiction Constraint — discharged by complete disjointness.** This Rule is the third interpreter of `XmlInspection` Evidence. Its content-shape filter is lexically disjoint from `VersionCompatibilityRule`'s and `DeclaredDependencyDuplicationRule`'s — the three diverge at the first character after `declares `, and no prefix is a prefix of another — so no Evidence item can satisfy more than one filter, and no item can support two Rules' conclusions at all. The three-way verification was performed against both sibling Rules' own constants at implementation time and is recorded in the Rule's own doc comment. C2 Decision 3's empirical confirmation, which requires a real shipped Rule to inspect, now has its first such instance available; that inspection is not performed by this release.

**No ADR was created. No Governance Register entry was created. GOV-010, GOV-011, and GOV-012 are unamended.**

---

## 5. Implementation Summary

| File | Nature of change |
|---|---|
| `crates/modiq-collection/src/collection/xml_collector.rs` | **Modified.** Adds a declaration prefix constant and `declared_store_item_evidence`; extracts direct-child `storeItem` elements under canonical `storeItems`, appended after all pre-existing manifest Evidence. Plus 10 collection tests |
| `crates/modiq-rules/src/rules/store_item_reference_rule.rs` | **New.** `StoreItemReferenceRule` — the ZIP provenance gate, supported-path filtering, case-sensitive membership comparison, and one Finding per unmatched declaration. Plus 20 Rule unit tests |
| `crates/modiq-rules/src/rules/mod.rs` | **Modified.** Registers and exports the Rule (2 lines) |
| `crates/modiq-rules/src/rules/engine.rs` | **Modified.** Registers the Rule last in the existing fixed dispatch order; extends the dispatch doc comment. Plus 5 dispatch and non-interference tests |
| `crates/modiq-engine/src/engine/assessment_service.rs` | **Modified — `#[cfg(test)]` only.** 8 integration tests using the pre-existing ZIP helper. No production code or signature change |

Diff shape: **768 insertions, 4 deletions** across 4 tracked files, plus 1 new source file. All four deletions were audited: one test-import line (extended, not removed) and three doc-comment lines (replaced with longer text). **No production logic and no test assertion was deleted, and no existing test was weakened, altered, or removed.**

**Test coverage added: 43 tests.**

| Crate | Before | After |
|---|---|---|
| `modiq_collection` | 70 | 80 (+10) |
| `modiq_rules` | 46 | 71 (+25) |
| `modiq_engine` | 23 | 31 (+8) |
| **Root workspace total** | **311** | **354 (+43)** |

Coverage spans: declaration extraction and ordering; degenerate-attribute preservation; canonical-position restriction; present and absent targets; independent multi-declaration outcomes; directory entries; case-only differences; each excluded path form; `StructuralDuplication` invariance; filesystem-assessment silence; provenance established independently of the declared target; determinism; and pre-existing-Rule non-interference.

---

## 6. Repository Impact

| Area | Impact |
|---|---|
| `crates/modiq-collection` | `xml_collector.rs` only. `ArchiveCollector`, `ArchiveReader`, `ArchiveEvidenceBuilder`, `EvidenceCollector` **unchanged** |
| `crates/modiq-rules` | One new Rule; `engine.rs` registration; `mod.rs` export. **No pre-existing Rule source file edited** |
| `crates/modiq-runtime` | **Unchanged.** No new `EvidenceCategory`, no entity field, no invariant change. Dormant categories remain dormant |
| `crates/modiq-engine` | Tests only. `AssessmentService`'s two public entry points unchanged; GOV-008 untouched |
| `crates/modiq-storage` | **Unchanged.** Persisted schema untouched |
| `crates/modiq-knowledge`, `modiq-versioning`, `modiq-cli` | **Unchanged** |
| `apps/console`, `apps/sandbox` | **Unchanged** |
| DTO / transport / UI / Dashboard | **Unchanged** |
| Cargo manifests, dependencies, schemas, configuration | **Unchanged** |
| `GOVERNANCE.md`, ADRs, architecture documents | **Unchanged** |
| `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md` | **Intentionally not synchronized by this release** — deferred to a separate Repository Closeout, following the identical choice Engineering Releases 2.4 and 2.5 each made |

**One behavioral consequence, recorded:** a new `rule_reference` will appear in `modiq-storage`'s read-only history aggregation and therefore in `modiq-cli history` output once assessments carrying this Rule's Findings are persisted. This is a consequence of the Rule existing, not a contract or schema change.

---

## 7. Validation Status

All gates re-run fresh at release-review time.

```
Root workspace:
cargo fmt --check                     → clean
cargo check --workspace --all-targets → clean
cargo test --workspace                → clean, 0 failures, 354 total passing
  console_lib 7 · modiq_cli 27 · modiq_collection 80 · modiq_engine 31(+3 e2e)
  modiq_knowledge 5 · modiq_report 4 · modiq_rules 71 · modiq_runtime 93
  modiq_storage 29 · modiq_versioning 4

apps/sandbox (separate workspace):
cargo fmt --check                     → clean
cargo check --all-targets             → clean
cargo test                            → 16/16, unaffected

apps/console:
tsc --noEmit                          → clean
vite build                            → clean

git diff --check                      → clean
```

Baseline preserved: the pre-capability root suite stood at 311 passing; it now stands at 354, the entire delta accounted for by the 43 tests this capability added.

---

## 8. Architectural Impact Review

A dedicated read-only review examined one question the implementation surfaced: `StoreItemReferenceRule` returns `Vec<RuleOutcome>` where every other Rule returns `Option<RuleOutcome>`, making it the first Rule to report more than one outcome from a single evaluation. Its conclusions:

- **Multiple outcomes from one Rule are architecturally permissible.** No invariant, ADR, architecture document, or Governance Register item constrains per-Rule Finding cardinality; all fourteen Runtime Invariants were checked.
- **GOV-012 does not establish a one-Finding-per-Rule invariant.** Its Question 1 asked whether `evaluate` returns a single aggregated outcome *or one outcome per matching Rule* — a question about aggregation across Rules. The phrase "one per matching Rule" records the shape chosen in answer to that question; the clause that follows states its purpose, preserving each Finding's independent traceability to the Rule and Evidence that produced it. That purpose is satisfied here, since each Finding cites the one declaration it concerns.
- **`RuleEngine` aggregation handles multiple outcomes naturally.** `AssessmentService`'s consuming loop is a plain iteration with no keying, deduplication, or cardinality assumption.
- **Downstream consumers preserve individual Findings.** No `HashMap`, `HashSet`, `dedup`, or `BTreeMap` anywhere in the workspace is keyed by `rule_reference`; every presentation path — `modiq-cli assess`, `modiq-cli retrieve`, the console's `FindingSummary`, `apps/sandbox`'s `FindingEntry`, `PersistedFinding`, and `history_analysis` — iterates per Finding.
- **No grouping, deduplication, loss, or incorrect-grouping defect was identified.**
- **No implementation change is required.**

The review's recommendation was **B — release as implemented, with a GOV-012 clarification and a history-count nuance deferred to Repository Closeout** (§11).

---

## 9. Accepted Documentation Gap (Product Owner Decision — Option B)

By explicit Product Owner decision, this capability's remaining lifecycle artifacts were **completed and delivered during the engineering process but were not persisted in this repository**:

- Capability Definition
- Process Determination
- Architecture Evaluation
- Implementation Plan
- Implementation Authorization
- Implementation Report
- Architectural Impact Review

Only `ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md` exists in `docs/engineering/` for this capability. **No retrospective reconstruction of the missing artifacts is authorized or performed.** This departs from the precedent Engineering Release 2.5 set, where C15 committed seven lifecycle documents alongside its release; that departure is deliberate and approved, not an oversight.

**This Engineering Release document therefore carries the load-bearing findings, decisions, scope, and verification information those artifacts contained**, so the historical record remains self-sufficient: §2 records every lifecycle stage and its outcome, §3 records the delivered scope, §4 records every adopted architectural decision, §5 records the implementation and its coverage, §7 records verification, §8 records the Architectural Impact Review's conclusions, and §10 records the deferred questions.

This is recorded as an **accepted documentation gap** — not an implementation defect, not a verification gap, and not a release blocker. The Architectural Resolution's own Origin field independently names the same absence.

---

## 10. Deferred Items and Limitations

Recorded as deferred — **not approved future capabilities, and not commitments**. Each would require its own evidence and its own authorization.

1. **Filesystem-directory support.** Blocked on the Symbolic Link Policy's and skipped non-regular-object omissions, which can produce false missing-file conclusions. `apps/console` supports directory intake today, so a directory-based assessment produces no store-item Finding — a deliberate silence.
2. **Case-insensitive matching.**
3. **Case-only near-match detection.**
4. **Legacy ZIP filename encoding.** No claim is made that all legacy-encoded archives will be interpreted identically.
5. **Duplicate store-item declaration handling.**
6. **Inventory omission signalling for traversal-invalid entries.**
7. **Broader XML, configuration, vehicle, specialization, and localization reference intelligence.**
8. **Base-game reference knowledge.**
9. **Canonical path representation in archive Evidence `content`.**

---

## 11. Closeout Housekeeping

Recorded for Repository Closeout. **None is a release blocker; none is performed by this release.**

1. **GOV-012 clarification.** Record that "one per matching Rule" describes cross-Rule aggregation and does not establish a cardinality cap for a single Rule. Proposed wording was drafted during the Architectural Impact Review. GOV-012's text is not amended by this release.
2. **History aggregation nuance.** A single assessment may now contribute multiple occurrences to the same `(rule_reference, severity)` pair in `history_analysis`. This is correct against `PatternFrequency`'s own documented per-Finding semantics, and worth stating so `modiq-cli history` output is not misread as a per-report count.
3. **Stale test name.** `evaluate_dispatches_all_five_rules_independently_when_all_match` (`crates/modiq-rules/src/rules/engine.rs`) still asserts `outcomes.len() == 5` and passes correctly — its Evidence set triggers no store-item outcome. Only the name is numerically stale against a six-Rule engine. **Noted only; deliberately not renamed during this release.**
4. **Architectural Resolution wording in context.** The Resolution states that it does not authorize implementation. Clarify at closeout that a separate Implementation Authorization was subsequently issued, so the two records are not read as contradictory.

---

## 12. Engineering Assessment

The capability was delivered within its authorized envelope with no scope expansion: exactly the five files the Implementation Authorization named, no file outside that set, no production change beyond one additive extraction and one new Rule, and no change to any domain model, service signature, schema, DTO, or existing Rule.

Three characteristics are worth recording as engineering history. First, the Process Determination declined to assume the shorter lifecycle path and correctly routed the capability through Architecture Evaluation, which then found that most of the perceived blockers were already-adopted permissions and narrowed the genuinely open questions to two. Second, five implementation questions were surfaced for Product Owner decision rather than silently resolved, and all five were answered before any code was written. Third, the implementation surfaced one architectural consequence — the first Rule reporting multiple outcomes — which was escalated to a dedicated review rather than absorbed, and that review confirmed it permissible with no downstream defect.

The accepted documentation gap (§9) is the one respect in which this release's record is thinner than its predecessors'. It is mitigated, by design, by this document's own completeness.

---

## 13. Final Release Recommendation

**Release.** The capability is implemented within its authorized scope, verified against all required gates with the pre-existing baseline preserved, consistent with the adopted Architectural Resolution in every particular, and cleared by a dedicated Architectural Impact Review that found no defect and required no implementation change. No release blocker exists.

**Recommend:** Product Owner final approval of this release, followed by Repository Closeout carrying the four housekeeping items in §11.

---

## Repository Status

Store Item Reference Integrity is complete. The root workspace is clean at 354/354; `apps/sandbox` is clean at 16/16, unaffected; `apps/console`'s build is clean. No Governance Register item and no ADR resulted from this capability; GOV-010, GOV-011, and GOV-012 remain textually unmodified. As of this document's own drafting, HEAD remains `4415737` on `feature/runtime-implementation`, and the entire capability — the Architectural Resolution, the implementation across five files, and this release document — exists only as uncommitted working-tree state. `PROJECT_STATUS.md`, `CHANGELOG.md`, and `ENGINEERING_LOG.md` are **intentionally not synchronized by this release**, confirmed by direct inspection to still reflect the C15 closeout state with zero reference to this capability, consistent with the identical choice Engineering Releases 2.4 and 2.5 each made in deferring those three files to a separate Repository Closeout. That separate step remains this release's own explicit exclusion, not performed here.

---

## Repository Timeline

```
Engineering Release 2.5 — C15 complete (Assessment & Report Subject Identity)
        ↓
Offline Assessment Capability Investigation — established that non-manifest XML,
asset references, and declared-path resolution were uninspected
        ↓
Vehicle Configuration Intelligence Investigation — identified three distinct
reference classes and measured the false-positive hazard of conflating them
        ↓
Intra-Mod Declaration Consistency Investigation — measured a 67% false-positive
rate for naive localization-token checking; isolated storeItem → archive file
reference as the one defensible, self-contained candidate
        ↓
Cross-Evidence Relationship Architecture Evaluation — decomposed the perceived
Collector/Rule blocker into three distinct questions, two of which proved
already answered by the adopted dispatch model
        ↓
Mod-Local Reference Semantics — Focused Follow-up — established that a declared
dependency does not make an absent mod-local reference externally satisfiable
        ↓
Capability Definition — Store Item Reference Integrity (approved)
        ↓
Process Determination — Architecture Evaluation required (the C2 path)
        ↓
Architecture Evaluation — two genuine gaps identified: absence-based conclusions,
and archive path representation
        ↓
Architectural Resolution — Adopted: ZIP-only scope resolves both gaps through
an interpretation plus a scope limitation, with no architectural change
        ↓
Implementation Plan — reviewed; five implementation questions surfaced
        ↓
Implementation Authorization — Q1–Q5 resolved; ZIP-only provenance safeguard added
        ↓
Implementation — phase-gated: Collection → Rule → Integration, 311 → 354 tests
        ↓
Architectural Impact Review — multiple outcomes from one Rule permissible;
no downstream defect; no implementation change required
        ↓
Engineering Release 2.6 — this document
        ↓
Repository Closeout — pending, carrying four housekeeping items (§11)
```
