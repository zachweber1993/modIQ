# Architectural Resolution — Store Item Reference Integrity

| Property | Value |
|---|---|
| **Document** | ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md |
| **Project** | modIQ |
| **Questions Resolved** | (A) May a Rule conclude that a declared path is not present in the collected archive inventory, given that adopted collection policy intentionally omits certain archive entries from Evidence without a Rule-visible signal? (B) What path representation and comparison semantics govern that conclusion, and does the Archive Traversal Boundary Policy's own normalization sentence govern the value recorded as Evidence `location`? |
| **Origin** | A four-stage investigation sequence approved by the Product Owner — Cross-Evidence Relationship Architecture Evaluation, Mod-Local Reference Semantics follow-up, Process Determination, and Architecture Evaluation (Store Item Reference Integrity). **Those four antecedent documents were delivered in-session and are not present in this repository's working tree**; this Resolution therefore records their load-bearing findings directly rather than citing repository paths that do not exist. That absence is named here as a documentation-currency gap, following this repository's own discipline of naming a discrepancy at the record that first encounters it rather than resolving it silently — it is not corrected by this document. |
| **Adopted Precedent** | GOV-010 and GOV-011 in their entirety, unamended; GOV-012 and `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` Decisions 1–3; ADR-0007, ADR-0008, ADR-0010/GOV-004; `docs/architecture/EvidenceCollection.md` (Collector Contract, Collection Outcomes, Collection Atomicity, Symbolic Link Policy, Archive Traversal Boundary Policy, Duplicate Archive Entry Policy); `docs/architecture/RuleEngine.md` Execution Principles; `docs/architecture/DataModel.md` (Evidence Anatomy, Finding Severity); `docs/implementation/RuntimeInvariants.md` INV-002, INV-003, INV-004, INV-013; Initiative 1 AC-4; Initiative 3 Items 1–4 and 10; Initiative 4 Q1a/Q1b. All treated as binding and unreopened. |
| **Status** | **Architectural Resolution — Adopted by the Product Owner. This Resolution does not authorize implementation.** No source file, test, configuration, dependency, schema, `GOVERNANCE.md` entry, or ADR has been created or modified. No Implementation Authorization or Implementation Plan exists. The capability is not implemented and is not released. |

---

## §1 — Method

This Resolution records decisions the Product Owner has approved, using the disposition vocabulary this repository's own Architectural Resolutions already use (`INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` §1, `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` §1): **Adopted / Adopted Architectural Constraint / Deferred / Rejected**. No new disposition category is introduced.

Evidence, reasoning, and decision are kept in separate sections. Adopted decisions (§5–§9) are distinguished throughout from deferred questions (§11); nothing recorded as Deferred is implicitly approved for future implementation.

---

## §2 — Governing Evidence

Verified directly against source during the Architecture Evaluation that preceded this Resolution.

**E1 — The complete inventory of conditions under which a ZIP entry is omitted from collected Evidence is exactly two.**

| Condition | Mechanism | Rule-visible |
|---|---|---|
| Traversal-invalid entry name — the as-stored name is an absolute path, or normalizes outside the archive's conceptual root via `..` segments | Filtered in `crates/modiq-collection/src/collection/archive_collector.rs` (`is_valid_entry_name`); no Evidence produced; collection continues for remaining valid entries, per the Archive Traversal Boundary Policy | No signal |
| Duplicate entry names | The underlying archive reader's central directory is keyed by filename, collapsing same-named physical entries before enumeration; documented in `archive_collector.rs` | Yes — `EvidenceCategory::StructuralDuplication` Evidence |

**E2 — There is no silent per-entry omission below that filter.** `ArchiveReader::entries()` and `entry_sizes()` (`crates/modiq-collection/src/collection/archive_reader.rs`) propagate any per-entry failure as `ArchiveReadError::InvalidArchive`; they never skip an entry. Resource-limit violations likewise reject the whole archive as Unsupported Input. Every non-omission failure is therefore whole-archive, and architecturally distinct from an omission under GOV-010.

**E3 — Archive entry names reach Evidence verbatim.** `ArchiveEntry::name()` is documented as "the entry's name exactly as stored in the archive. Not yet normalized or validated," and `ArchiveEvidenceBuilder` records it unchanged as the Evidence `location`. A dedicated test asserts that an entry named `a/../b.txt` is recorded with that exact `location`.

**E4 — Declared `storeItem xmlFilename` values are mod-root-relative.** Established at verified confidence by the Mod-Local Reference Semantics follow-up (real declarations resolving from mod root across independent mods; no instance of any path-prefix expression found in this attribute in any examined mod), and consistent with GIANTS's own published schema documentation for the attribute, which describes it only as the path to a store item's XML file and states no cross-mod resolution rule. A declared dependency was found not to alter path resolution for this attribute; dependency declarations govern mod presence and load order.

**E5 — Platform-divergent path representation originates solely in filesystem collection.** `EvidenceCollector` renders relative paths through `Path::display()`, which yields host-native separators and is lossy for non-UTF-8 paths. ZIP-routed assessments do not invoke it.

**E6 — Discrimination the capability requires is already carried by adopted Evidence fields.** `source` distinguishes `"Archive Collection"` from `"Filesystem Collection"`; `label` distinguishes `"File Discovered"` from `"Directory Discovered"`; `StructuralDuplication` signals inventory collapse; `location` carries the raw entry name.

**E7 — INV-013 requires a Finding to reference at least one Evidence item**, as cardinality only; referential integrity of cited identifiers remains explicitly unenforced (GOV-005 Resolution).

---

## §3 — Architectural Reasoning

Reasoning only; no decision is stated in this section.

**On whether a traversal-invalid omission can conceal a declared store-item file.** It cannot, and the reason is structural rather than probabilistic. For such an omission to conceal a declared file, the archive would have to store that file under a name that is either absolute or root-escaping. A declared path is mod-root-relative (E4), and is resolved as such. An entry stored under an absolute or root-escaping name is therefore not at the location the manifest declares. The omission and the non-match are not two facts that might disagree — they are the same fact observed twice. A conclusion that no collected entry matched the declared mod-root-relative path is true of such an archive, not falsely true.

**On whether duplicate collapse can cause a false conclusion.** It cannot. Collapse is keyed by identical name, so the name survives in the inventory exactly once; membership is preserved while count fidelity is not. This capability performs a membership test and never a count, so the specific hazard the Duplicate Archive Entry Policy names — a Rule "counting discovered files" — is not the hazard this capability presents. The Policy's instruction that such a Rule must not *silently absorb* `StructuralDuplication` nevertheless remains binding as an authorship obligation.

**On why this reasoning is ZIP-specific and does not generalize.** The reasoning above depends on the omitted entry being identifiable as non-conforming from its name alone. The filesystem omissions have no such property: a symbolic link at a declared path is omitted under the Symbolic Link Policy while being, from the game's perspective, exactly where the declaration says it is. The same applies to skipped non-regular filesystem objects. ZIP-only scope therefore does not postpone the filesystem problem — it excludes the only input mode in which a false missing-file conclusion can arise.

**On the four states that must remain distinguishable.** Each is already representable through adopted mechanisms, with no new concept: a complete inventory with no matching path (inventory Evidence present, no match, no `StructuralDuplication`); an inventory with a known omission (`StructuralDuplication` present); an invalid or unsupported archive (Collection Atomicity means no Assessment reaches rule evaluation at all); and a genuinely empty archive (Empty Collection under GOV-010 — a successful observation, not an error, with no declarations to check).

**On the scope of the Archive Traversal Boundary Policy's normalization sentence.** The sentence opens a policy section whose entire subject is which entries are invalid; the sentences following it define the two invalidity conditions; and the implementation normalizes by path components for exactly that determination while recording the raw name (E3). Reading the sentence as governing the recorded Evidence `location` would also place GOV-011 in tension with its own Duplicate Archive Entry Policy, which requires that collection not silently resolve what it cannot fully observe — rewriting a recorded name is the same category of silent resolution. Interpreting it as governing traversal validation only is therefore the reading most consistent with GOV-011 read as a whole. This is recorded as an adopted interpretation, not as the only textually possible reading, following C2 Decision 1's own precedent of adopting an interpretation explicitly rather than claiming the source text settles it unambiguously.

**On why case-sensitive, non-normalized comparison is platform-independent here.** Both sides of the comparison originate inside one archive: the declared path from the manifest's own bytes, the entry name from the central directory's bytes. Neither passes through a host filesystem. The same archive therefore yields the same comparison result on every operating system. Byte-faithful comparison is the only representation-faithful policy, and under ZIP-only scope it introduces no platform dependence.

---

## §4 — Alternative Dispositions

| Alternative | Disposition | Basis |
|---|---|---|
| Emit an explicit Evidence observation for every skipped traversal-invalid entry | **Rejected for this capability** | Consistent in spirit with the Duplicate Archive Entry Policy's own precedent, but it changes archive collection output for every assessment, expands scope into Collection, risks being read as weakening GOV-011's "not recorded as Evidence" instruction, and addresses a failure mode §3 shows does not arise in ZIP scope |
| Represent inventory completeness through a new Collection Outcome or Evidence field | **Rejected** | GOV-010 was extended to the archive case explicitly "without a fifth outcome"; no new Evidence concept is necessary under §3 |
| Suppress the Rule whenever any omission may have occurred | **Rejected** | Traversal omissions are undetectable from the Rule's position, so this is not implementable; suppressing on `StructuralDuplication` would discard correct conclusions for an unrelated reason |
| Normalize at collection so Evidence `location` carries a canonical form | **Rejected** | Destroys the raw as-stored name and with it traceability to the exact archive entry; in tension with GOV-011's anti-silent-resolution discipline |
| Retain the raw name in `location` and add a canonical form to the currently-unused `content` field on archive Evidence | **Deferred**, not Rejected | Preserves traceability using only adopted fields, but changes archive Evidence for every assessment and surfaces through an existing consumer DTO while remaining absent from the persisted representation — for the benefit of one Rule. Available for reconsideration if a second consumer ever requires canonical paths |
| Evidence unchanged; comparison performed inside the Rule over raw names, with unsupported forms excluded | **Adopted** (§7) | Changes no Evidence item, no persisted content, and nothing any existing consumer reads; traceability holds by construction |

---

## §5 — Decision A: Assessment Scope

**A1 — Adopted.** The capability's first release supports **ZIP archive assessments only**.

**A2 — Adopted.** **Filesystem-directory assessments are explicitly excluded** from this capability. A directory-based assessment produces no store-item reference finding of any kind. This is a deliberate silence, not an oversight, and follows directly from §3: the filesystem omission classes can produce a false missing-file conclusion, and the ZIP scope excludes them.

**A3 — Adopted.** The capability evaluates `storeItem` declarations in `modDesc.xml`.

**A4 — Adopted.** It determines whether an ordinary mod-root-relative declared path matches a **file** entry in the collected ZIP inventory.

**A5 — Adopted Architectural Constraint.** The capability **does not establish runtime loadability or functional correctness**, and shall make no claim of either.

---

## §6 — Decision B: Negative Membership and Inventory Completeness

**B1 — Adopted.** For ZIP archive inputs, a Rule **may** conclude that **no collected ZIP archive entry matched a supported declared path**. Such a conclusion is supported by collected Evidence within the meaning of `RuleEngine.md`'s Evidence-Based principle: the inventory of `FileStructureAnalysis` Evidence is itself the positive Evidence, and whole-set structural reasoning over that category is already contemplated by the Duplicate Archive Entry Policy.

**B2 — Adopted Architectural Constraint.** Finding language **shall remain bounded to the collected archive inventory**. The conclusion is stated as a fact about the declared location and what the inventory contained, and shall not assert absolute file absence, mod breakage, failure to load, or any runtime consequence.

**B3 — Adopted Architectural Constraint.** A Rule performing membership reasoning over `FileStructureAnalysis` **shall explicitly account for `StructuralDuplication` Evidence** rather than silently absorbing it, per the Duplicate Archive Entry Policy. For a membership test, accounting consists of recording why collapse cannot affect the conclusion; it imposes no runtime mechanism.

**B4 — Adopted.** Existing archive traversal and invalid-entry handling **remain unchanged**. GOV-011 is neither amended nor weakened by this Resolution; no entry that policy excludes becomes recordable, matchable, or resolvable.

**B5 — Adopted.** **No new inventory-completeness domain concept and no Evidence model change is introduced.** The four states in §3 remain distinguishable through already-adopted mechanisms.

**B6 — Adopted.** The Finding cites the declaration Evidence identifier, satisfying INV-013 (E7). No Evidence of the absent file is required, and none exists.

---

## §7 — Decision C: Path Representation and Comparison

**C1 — Adopted interpretation (GOV-011 unamended).** The Archive Traversal Boundary Policy's normalization sentence governs the **traversal-validity determination only**, not the value recorded as Evidence `location`. Archive-entry Evidence continues to carry the entry's name exactly as stored. GOV-011's text is not amended; a cross-reference to this Resolution is named as reasonable future housekeeping, not performed here, following C2's own precedent for GOV-012.

**C2 — Adopted.** Comparison is performed **inside the Rule**, over **raw ZIP entry names**, with no change to any Evidence item. `location` remains the traceability anchor to the exact archive entry.

**C3 — Adopted comparison policy.**

| Aspect | Adopted policy |
|---|---|
| Path separator | `/` is the recognized separator |
| Case | Comparison is **case-sensitive** |
| Unicode | **No Unicode normalization is applied** |
| Directory entries | A directory entry **never satisfies** a file declaration |
| Unsupported forms | **Never normalized or silently reinterpreted** |

**C4 — Adopted.** The following declared path forms are **excluded from missing-file conclusions**:

- absolute paths and paths with a leading separator;
- `$`-based path expressions;
- paths containing a backslash;
- paths containing `.` or `..` segments.

**C5 — Adopted.** An excluded declaration **remains traceable as declaration Evidence**. It does **not** require a separate Finding merely because it is unsupported, and shall not produce a missing-file conclusion.

**C6 — Adopted limitation, recorded.** ZIP filename encoding is a known limitation of this comparison policy. The archive format permits entry names stored under a legacy code page as well as names flagged as UTF-8, while a manifest's declared path is read as XML text. **This Resolution makes no claim that all legacy-encoded archives will be interpreted identically**, and adopts no mechanism for resolving the difference. Resolution of legacy encoding is Deferred (§11).

---

## §8 — Decision D: Architectural Boundaries

All **Adopted**, and all stated as boundaries this capability does not cross:

- No new domain concepts.
- No new Evidence categories. The dormant `EvidenceCategory` variants remain dormant.
- No changes to `Evidence` or `Finding` structures.
- No changes to the `AssessmentService` workflow, including its public entry points.
- No Storage, persistence, DTO, transport, or UI changes.
- No Knowledge subsystem changes.
- No changes to existing collectors, beyond the separately approved future **additive** extraction of `storeItem` declarations in `XmlCollector` — which this Resolution does not itself authorize.
- Cross-category Rule evaluation is **consistent with the existing adopted architecture**: GOV-012, as interpreted and adopted by C2 Decision 1, already establishes that every Rule receives the full Evidence set and that dispatch carries no concept of category ownership. No change to `RuleEngine` dispatch, and no trait, registry, factory, or plugin mechanism, is introduced (ADR-0010, GOV-004).

---

## §9 — Decision E: Finding Policy

All **Adopted**:

- **Severity: `Warning`.** Consistent with `DataModel.md`'s Finding Severity definitions, which reserve `Error` for a defect Evidence directly proves and assign `Warning` to a genuine, Evidence-supported concern where Evidence does not conclusively establish that the mod is broken.
- **Dimension: `Structure`.**
- **The initial release does not require a Recommendation.** A Finding may stand alone (Initiative 3, Item 3).
- The Finding **shall identify the declared path and cite the exact declaration Evidence**.
- **Adopted Architectural Constraint:** the Rule **shall not make runtime or functional claims**.

No new Finding status, severity value, Confidence model, or uncertainty mechanism is introduced. `FindingStatus` semantics (Initiative 1 AC-4) and Confidence's Recommendation-only scope with deferred representation (Initiative 4 Q1a/Q1b) are untouched.

---

## §10 — Consequences and Affected Components

| Area | Consequence of this Resolution |
|---|---|
| `crates/modiq-collection` | Unchanged by this Resolution. A future, separately authorized additive extraction in `XmlCollector` is anticipated but not authorized here. `ArchiveCollector`, `ArchiveReader`, `ArchiveEvidenceBuilder`, and `EvidenceCollector` are unaffected |
| `crates/modiq-rules` | Unchanged by this Resolution. A future Rule would be dispatched through the existing fixed declaration sequence, with no dispatch abstraction |
| `crates/modiq-runtime` | Unchanged. No category, field, invariant, or entity change |
| `crates/modiq-engine` | Unchanged. No signature change; GOV-008 untouched |
| `crates/modiq-knowledge`, `crates/modiq-storage`, `crates/modiq-versioning`, `crates/modiq-cli` | Unchanged |
| `apps/console`, `apps/sandbox` | Unchanged. Note that `apps/console` supports directory intake today, so a directory-based assessment will produce no store-item finding under A2 |
| Persisted representation and history analysis | Schema unchanged. When a Rule is eventually implemented, a new `rule_reference` would appear in Storage's read-only history aggregation and therefore in `modiq-cli history` output — a behavioral consequence of future implementation, not a contract change, and not performed by this Resolution |
| GOV-010, GOV-011, GOV-012 | Unamended. GOV-011's normalization sentence is interpreted (C1), not revised |
| ADRs | None created; none required (§12) |

---

## §11 — Explicit Deferrals

Recorded as **Deferred — not Rejected, and not implicitly approved for future implementation**. Each would require its own evidence and its own authorization.

1. **Filesystem-directory support.** Blocked on the Symbolic Link Policy's and skipped non-regular-object omissions, which can produce false missing-file conclusions (§3).
2. **Case-insensitive matching, and case-only near-match findings.**
3. **Legacy ZIP filename encoding resolution** (§7, C6).
4. **Duplicate store-item declaration handling.**
5. **New inventory omission signaling** for traversal-invalid entries.
6. **Broader XML and configuration reference analysis.**
7. **Vehicle XML, specialization, localization, and base-game reference intelligence**, including vehicle-type name resolution and dependency-supplied name resolution.
8. **A canonical path representation carried in archive Evidence `content`** (§4).

---

## §12 — Governance and Documentation Implications

- **GOV-011 is not amended.** Its Archive Traversal Boundary Policy, Duplicate Archive Entry Policy, and archive-specific Collection Outcomes stand exactly as resolved. C1 adopts an interpretation of one sentence's scope without revising the text.
- **No ADR is created, and none is required.** Both decisions are interpretations of already-adopted text plus a scope limitation, not new durable principles superseding an existing architectural decision. This follows C2's precedent directly, which adopted an interpretation of GOV-012, created no ADR, and amended no Governance Register entry.
- **No new Governance Register entry is created**, consistent with this repository's precedent of resolving capability-scoped architectural questions in a dedicated Resolution (C2, C12, C15).
- **C2 Decision 3 — prospective instance identified, empirical confirmation still outstanding.** C2 Decision 3 holds that its Non-Contradiction Constraint is confirmed or corrected only against "a real, shipped instance" — "a real Rule… to inspect against Decision 2's Constraint." This Resolution identifies the **first prospective such instance**: a Rule under this capability would be the third interpreter of `XmlInspection` Evidence, alongside `VersionCompatibilityRule` and `DeclaredDependencyDuplicationRule`. The Constraint is dischargeable at design time by complete filter disjointness, since the three content-shape prefixes diverge lexically and no Evidence item can satisfy more than one. **The empirical confirmation C2 Decision 3 contemplates has not occurred and is not claimed here** — no Rule exists yet to inspect. Whichever document eventually authorizes that Rule should record the three-way disjointness verification explicitly, since C2 noted that no repository mechanism performs or prompts that check.
- **Antecedent documents.** The four approved investigation documents preceding this Resolution are not present in this repository (see Origin, above). Whether they are reconstructed into the repository is a separate question this Resolution does not decide.
- **Observation, not corrected here:** `docs/engineering/README.md`'s category table records the naming pattern `*_ARCHITECTURAL_RESOLUTION.md`, while the two most recent capability-scoped resolutions (C12, C15) and this document use the `ARCHITECTURAL_RESOLUTION_*` prefix form. This pre-existing drift is named for a future housekeeping pass, not altered by this document.

---

## §13 — Implementation Boundary

If and when implementation is separately authorized, the following must remain unchanged:

- `AssessmentService::execute` and `execute_from_assessment_input` signatures.
- `RuleEngine::evaluate`'s dispatch shape — no trait, registry, factory, or dispatch table.
- `EvidenceCategory`'s existing variants; dormant variants stay dormant.
- `Evidence`, `Finding`, and `Recommendation` field sets, and ADR-0007's construct-once pattern.
- `FindingStatus` semantics and Confidence's absence from `Finding`.
- Storage schema, Knowledge content, and every DTO, transport, and UI surface.
- Collector independence — no collector may consume another collector's output.
- Archive traversal and invalid-entry handling, and every recorded Evidence `location` value.

---

## §14 — Verification Criteria

A future, separately authorized implementation would satisfy this Resolution if and only if:

1. A supported declared store item present in the ZIP produces no Finding; one absent produces a Finding naming the exact declared path and citing the declaration Evidence identifier.
2. Each excluded form in C4 produces no missing-file Finding, and is neither normalized nor silently reinterpreted.
3. `ArchiveCollector`, `ArchiveReader`, `ArchiveEvidenceBuilder`, and `EvidenceCollector` are unchanged, and no traversal-invalid entry becomes recordable or matchable.
4. A directory-input assessment produces no store-item Finding.
5. No new `EvidenceCategory`, Runtime field, `AssessmentService` signature, DTO, UI, or storage-schema change appears.
6. `RuleEngine` gains no dispatch abstraction.
7. The three-way `XmlInspection` content-shape disjointness verification is recorded in the authorizing document.
8. Finding wording satisfies B2, A5, and §9 — no absolute-absence, runtime, or functional claim.
9. A directory entry never satisfies a file declaration.
10. Repeated assessment of an identical archive yields identical Findings.

---

## §15 — What This Resolution Does Not Do

- **It does not authorize implementation.** No Implementation Authorization and no Implementation Plan exists for this capability.
- It does not claim the capability is implemented, built, verified, or released.
- It does not create, modify, or delete any source file, test, configuration file, dependency, or schema.
- It does not amend `GOVERNANCE.md`, any ADR, any architecture document, or any prior Architectural Resolution.
- It does not create an ADR or a Governance Register entry.
- It does not assign a capability number or a release designation.
- It does not decide the deferred questions in §11, and does not treat any of them as pre-approved.
- It does not resolve the antecedent-document absence named in Origin and §12.

---

## §16 — Final Resolution

**Adopted.** For ZIP archive assessments only, a Rule may conclude that no collected archive entry matched a supported, ordinary mod-root-relative `storeItem` declared path — a conclusion supported by the collected inventory itself, bounded in its wording to that inventory, accounting explicitly for `StructuralDuplication`, and making no runtime or functional claim. Comparison is performed in the Rule over raw ZIP entry names, with `/` as the recognized separator, case-sensitively, without Unicode normalization, never satisfied by a directory entry, and never normalizing or silently reinterpreting an unsupported path form. The Archive Traversal Boundary Policy's normalization sentence is interpreted as governing traversal validation only, leaving every recorded Evidence `location` exactly as stored. Filesystem-directory assessments are excluded. No new domain concept, Evidence category, Evidence or Finding structure, collector change, dispatch behavior, service signature, Storage, persistence, DTO, transport, UI, or Knowledge change is introduced, and GOV-011 is unamended.

**This Resolution's responsibility ends here. It does not authorize implementation.**
