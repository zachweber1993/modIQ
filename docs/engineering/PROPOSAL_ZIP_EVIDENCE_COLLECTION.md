# Proposal: ZIP / Archive Evidence Collection

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_ZIP_EVIDENCE_COLLECTION.md |
| **Stage** | Proposal (Architecture Review) |
| **Prepared by** | Engineering, for Technical Director review |
| **Predecessor** | `PROPOSAL_FILESYSTEM_COLLECTION.md` (the first real collector; this proposal is its own named "next milestone") |
| **Platform Validation basis** | Prepared following Platform Validation Phase 1's close: GOV-004 confirmed `AssessmentService` direct subsystem composition; GOV-008 was reviewed and deferred, leaving the current `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, the public error model) as the approved platform boundary. This proposal builds on that boundary as it stands; it does not revisit either decision. |
| **Status** | Design-only. No Rust code, no new crates, no governance changes, no implementation. Awaiting Architecture Review and Technical Director approval. |

---

# 1. Engineering Objective

Add ZIP archive traversal as the platform's second real Evidence Collector: given an `AssessmentInput` that names a `.zip` file, deterministically discover what is structurally present *inside* the archive — entries and their names, sizes, and kind (file or directory) — without extracting, decompressing into memory beyond what inspection requires, or interpreting the content of any entry.

This is the same objective Filesystem Collection pursued for the local filesystem, one level deeper: Filesystem Collection proved that a real, non-synthetic Evidence source could be added to the platform without disturbing any existing boundary. This proposal asks whether a *second* real Evidence source can be added the same way — additively, without redesigning `AssessmentService`, `EvidenceCollector`, or the `modiq-collection` boundary ADR-0008 established.

**Why this is the correct next milestone.** Both `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` and `PROPOSAL_FILESYSTEM_COLLECTION.md` already named ZIP traversal as Evidence Collection's own next step. Farming Simulator mods are commonly distributed as a single archive file, not yet extracted — Filesystem Collection can observe that such a file exists, but cannot see what it contains. Platform Validation Phase 1 confirmed the surrounding architecture (`AssessmentService`, the execution contract) is stable enough to build against without first resolving GOV-008.

---

# 2. Architectural Fit

## How ZIP traversal fits into the existing Evidence Collection architecture

`ADR-0008`'s boundary — Evidence Collection depends on `modiq-runtime` only; nothing depends on it except `modiq-engine`; it produces Evidence, never evaluates it, never mutates `Assessment`, never orchestrates — is unchanged by this proposal. ZIP traversal is a second concrete instance of the same conceptual unit `EvidenceCollection.md` calls a Collector: something that receives a location reference and relevant Assessment Context, inspects it deterministically, and returns Evidence or a categorized failure. Nothing about that contract, as already specified, excludes an archive-aware Collector.

## Expected crate responsibilities

ZIP traversal belongs inside the existing `modiq-collection` crate, as an additive second unit alongside the existing filesystem-facing `EvidenceCollector`. No new crate is proposed or required — `modiq-collection` is already the platform's Evidence Collection subsystem, and ADR-0008's dependency shape (depends on `modiq-runtime` only) does not change by adding a second capability inside the same crate. `modiq-collection` would, for the first time, take an external, non-workspace dependency (an archive-parsing crate) — this proposal names that as a fact worth flagging to the Technical Director (Section 12), not as a decision this proposal makes.

## Interaction with `AssessmentService`

**No change to `AssessmentService` is proposed.** Today, `execute_from_assessment_input` accepts any non-empty string, constructs an `AssessmentInput`, and passes it to `EvidenceCollector::collect`. A path to a `.zip` file is already a structurally valid `AssessmentInput` under GOV-009's existing resolution — nothing about `AssessmentInput`'s current shape rejects it. Passed through the *existing, unmodified* filesystem `EvidenceCollector` today, a `.zip` path already produces one valid, if shallow, Evidence item: "a file exists at this location." ZIP traversal would add a deeper inspection capability; it would not replace or correct anything currently produced.

**What this proposal does not resolve:** how `AssessmentService` (or anything else) would decide that a given `AssessmentInput` should be routed to archive-aware inspection rather than, or in addition to, ordinary file discovery. No such routing exists today, and this proposal does not design it — see Section 13.

## Interaction with `EvidenceCollector`

**No modification to the existing `EvidenceCollector` type is proposed.** It would continue to do exactly what it does today: observe that a `.zip`-named path is a file, and stop there. ZIP traversal requires a second, additive unit — analogous in shape to `EvidenceCollector` (it receives a location and Assessment Context, returns Evidence or a categorized failure) but distinct from it, since its internal logic (archive parsing) has nothing in common with filesystem metadata inspection.

**This proposal does not name, design, or authorize that second type.** Doing so is implementation planning for a future, separately-approved phase. This proposal also does not introduce a trait, provider interface, or dispatch abstraction unifying the two collectors — per instruction, and consistent with this platform's own twice-declined position on premature abstraction. Whether two independent, unrelated collector-shaped types is the platform's answer once a second real one exists, or whether their coexistence is itself the concrete case that finally justifies a shared abstraction, is named in Section 13 as a question for the Technical Director, not decided here.

---

# 3. Deterministic Extraction Requirements

Filesystem Collection established that determinism, once a Collector touches real external state, must be actively imposed rather than assumed — the filesystem's own directory-listing order carries no guarantee, so Phase 5 sorted entries explicitly. The ZIP format raises the same requirement with its own specific shape:

- A ZIP archive's **central directory** — the authoritative index of entries most archive-reading approaches consult — has its own stored order, which may or may not match the physical order entries were written to the archive. Which order a Collector reads, and whether it re-imposes its own explicit ordering (as Filesystem Collection did) rather than relying on either the central directory's or the physical archive's incidental order, is an architectural requirement this proposal states in the abstract: **collection of the same, unchanged archive must produce identical Evidence content, in identical order, across repeated runs** — the same requirement `EvidenceCollection.md`'s Determinism Expectations already impose on every Collector, restated here for the archive case.
- A ZIP archive may contain **duplicate entry names** — the format does not forbid it. Whether this is treated as Unsupported Input, as multiple discrete Evidence items sharing a name, or some other categorization is not decided by this proposal; it is named as a real, format-specific case Filesystem Collection never had to consider, since ordinary filesystems do not permit duplicate names within one directory.
- **Only structural facts that are themselves stable** should be recorded as Evidence — an entry's name, its uncompressed size, and whether it represents a file or a directory are stable; a compressed size, a compression method, or a modification timestamp stored in the archive may vary across otherwise-equivalent archives without the structure a user would recognize as "the same mod" having meaningfully changed. Filesystem Collection excluded point-in-time filesystem metadata (timestamps) for the identical reason; this proposal names the same principle as applying here, without prescribing exactly which ZIP-level metadata fields would be included.

---

# 4. Error Handling Expectations

`EvidenceCollection.md`'s four-outcome Collection Error Model (Invalid Input, Inaccessible Input, Unsupported Input, Empty Collection), resolved for the filesystem case by GOV-010, is the existing conceptual frame. `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md` already found, directly from source, that `CollectionError`'s two current variants (`Inaccessible`, `Unsupported`) are explicitly scoped to the filesystem case by GOV-010's own resolution text — this proposal confirms that finding remains accurate and treats it as the starting point, not a settled answer, for the archive case:

- **Invalid Input** — unchanged; an empty `AssessmentInput` is already rejected before any collector, filesystem or archive, is reached.
- **Inaccessible Input** — the archive file itself cannot be reached (does not exist, access denied) — the same category Filesystem Collection already defines, likely reusable without modification, since this failure occurs before archive-specific logic ever runs.
- **Unsupported Input** — the location is reachable but is not a valid archive of the supported kind. This is where the archive case diverges: "not a supported kind" for the filesystem Collector meant "not a regular file or directory"; for an archive Collector, a `.zip`-named file that is not actually a well-formed ZIP archive (corrupt, truncated, or using an unsupported compression method) is a new, archive-specific instance of the same conceptual category, not a new category.
- **Empty Collection** — an archive containing zero entries is a valid, successful, empty result, matching Filesystem Collection's own treatment of an empty directory.

**A genuinely new question this proposal surfaces, not resolved:** whether "not a well-formed ZIP" and "not a supported kind of location" are the same architectural category (both are Unsupported Input) or whether a corrupt/malformed archive — reachable, named correctly, but internally invalid — warrants distinction from a location that is simply the wrong *kind* of thing. This proposal takes no position; it is named in Section 13.

---

# 5. Archive Traversal Boundaries

Filesystem Collection's Symbolic Link Policy addressed an analogous problem for the filesystem case: a discovered path that could point somewhere other than where it appears to. ZIP archives raise a structurally similar concern with different mechanics:

- **Entry names may attempt to escape the archive's own extraction boundary** — for example, an entry named with `../` sequences, or an absolute path, that would resolve outside wherever the archive's contents are conceptually rooted. Since this proposal's scope is structural observation only (no extraction to disk is proposed — see Section 3, "no content is read"), this risk is different in shape from the filesystem case's symbolic-link risk, but the underlying principle Filesystem Collection already established — a Collector should never let discovered content lead it, or a future consumer of its Evidence, outside the boundary of what it was actually asked to inspect — applies identically. Whether such entry names are rejected, recorded as Unsupported, sanitized, or handled some other way is not decided here.
- **Nested archives** (a ZIP within a ZIP) are named as a real case that exists in practice but is explicitly out of this proposal's scope, in the same spirit Filesystem Collection excluded ZIP parsing from its own scope: recognizing that a nested archive exists (a file entry with a `.zip`-shaped name) is in scope; opening and traversing it is not.

---

# 6. Security Considerations

This is the platform's first genuinely adversarial input surface — nothing built so far has had to defend against maliciously crafted input, because nothing has parsed a binary container format before. `ENGINEERING_RELEASE_0.3.md`, `ROADMAP_REVIEW_2026.md`, and `PLATFORM_VALIDATION_GOV-004.md` have each already flagged this as a known, anticipated gap, independently. This proposal treats the following as considerations the Technical Director should weigh, not as decisions:

- **Decompression ratio attacks ("zip bombs").** A small archive can claim to decompress to an enormous size. Since this proposal's stated scope is structural observation (entry names, sizes as recorded in the archive's own metadata) rather than full content decompression, the risk surface may be smaller than a content-reading collector would face — but reading even a central directory requires parsing untrusted binary structure, which is itself not risk-free. Whether any actual decompression occurs, and if so under what limits, is an open question this proposal does not resolve.
- **Resource limits.** Maximum entry count, maximum archive size, maximum nesting consideration (even to recognize, not traverse, a nested archive) are named as considerations without proposed values.
- **Malformed or truncated archives.** Must fail through the Unsupported Input category (Section 4) rather than panicking, consistent with Filesystem Collection's own discipline of converting every reachable failure into a categorized `Result`, never an unhandled panic.
- **The first external, non-workspace dependency any domain crate in this platform would take** (an archive-parsing library) is itself a security-relevant decision — it introduces trust in code this platform does not control, in a way no dependency added so far has. `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own Future Evolution section already anticipated this exact moment. This proposal does not name or evaluate a specific library; it names the decision as one the Technical Director should treat deliberately, not incidentally.

---

# 7. Testing Strategy

Consistent with the real-I/O testing discipline Phase 5 established (temporary directories, real fixtures, `#[cfg(unix)]`-guarded platform-specific cases where needed) — extended to cover what Filesystem Collection's own adversarial-input coverage was explicitly limited to, and flagged as needing expansion for exactly this moment (`ENGINEERING_RELEASE_0.3.md`, Known Limitations):

- Well-formed archives: single entry, multiple entries, nested directory structure within the archive, empty archive.
- Malformed archives: truncated file, corrupted central directory, a file that is not a ZIP at all despite its name.
- Adversarial archives: entries with path-traversal-shaped names (Section 5), a deliberately small archive with an extreme claimed decompression ratio, an archive with an extreme entry count.
- Determinism: identical archive collected twice produces identical Evidence content and order (mirroring Filesystem Collection's own determinism tests).
- Duplicate entry names, if the archive format is confirmed to permit them in a way the chosen approach must handle.

This proposal does not write these tests or select a testing framework; it names the categories a future implementation phase would need to cover, extending Filesystem Collection's own precedent rather than replacing it.

---

# 8. Future Extensibility for Additional Archive/Container Formats

Following the same restraint `PROPOSAL_FILESYSTEM_COLLECTION.md` applied to its own Future Evolution section: this proposal does not design an abstraction for "archive formats in general." If ZIP traversal ships as a second concrete, independent Collector (per Section 2), a third format (a different archive type, a different container) would follow the same pattern proven twice by then — a third additive, concrete unit — rather than requiring a pre-built extension point. Whether two real collectors, and then a third, is the concrete evidence that finally justifies a shared Collector abstraction is explicitly not decided here; it is the same question Section 2 already names as open.

---

# 9. Implementation Phases (proposed shape, not authorized)

Named for planning purposes only; no phase below is authorized by this proposal.

1. **Boundary-proving phase**, mirroring Phase 4's synthetic-collector discipline: confirm a chosen archive-parsing approach can deterministically read a well-formed archive's structure at all, before any real Evidence production — the smallest possible slice.
2. **Real capability phase**, mirroring Phase 5: entry enumeration as Evidence — names, sizes, file-vs-directory kind — with the four-outcome error model extended for the archive case (Section 4), deterministic ordering (Section 3), and the traversal-boundary policy (Section 5) all resolved as concrete implementation, not left abstract.
3. **Explicitly excluded from both phases:** reading or interpreting the content of any individual entry (that is XML inspection's or a future capability's scope, not this one's — the same content/structure line `PROPOSAL_FILESYSTEM_COLLECTION.md` already drew for the filesystem case).

---

# 10. Expected Documentation Updates

If this proposal is approved at the architectural level (mirroring the two-step pattern already used for Filesystem Collection — architecture approval, then a separate implementation-readiness governance decision):

- `EvidenceCollection.md` — likely amendment to record archive-specific Collection Outcomes (Section 4) and a Traversal Boundary policy (Section 5) for the archive case, the same way Phase 5's Symbolic Link Policy and Collection Atomicity were recorded as amendments following `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own approval.
- `GOVERNANCE.md` — likely one or more new Governance Register items, parallel in shape to GOV-009/GOV-010 but scoped to the archive case (see Section 12).
- `CrateRoadmap.md` — `modiq-collection`'s maturity entry and the new external dependency, once chosen, recorded.
- A new ADR is plausible if the Technical Director judges the Collector-dispatch question (Section 2, Section 13) rises to an architectural decision — this proposal does not draft one.

---

# 11. Anticipated Governance Impact

- Very likely: new Governance Register item(s) for the archive-specific Collection Outcomes / Unsupported Input question (Section 4) and the archive traversal boundary policy (Section 5), following the GOV-009/GOV-010 precedent.
- Possible, contingent on the Technical Director's judgment: a governance item, or an ADR, addressing whether a second real collector is the concrete case that justifies a Collector-dispatch mechanism (Section 2) — this proposal explicitly does not pre-judge that question.
- Not anticipated: any change to GOV-004's resolution, GOV-008's status, ADR-0008, ADR-0009, or ADR-0010 — nothing in this proposal touches the engine orchestration boundary those items govern.

---

# 12. Engineering Risks

- **First adversarial input surface, first external dependency.** Both are named plainly in Sections 6 and 8 as real, not hypothetical, risks — this is qualitatively different engineering territory than anything Filesystem Collection faced.
- **Scope creep risk.** "ZIP traversal" can silently expand toward "the entire Assessment Framework" if entry-content inspection is not held to the strict structural-observation-only line Section 9 draws, the same risk `ROADMAP_REVIEW_2026.md` already named for XML inspection specifically.
- **Collector-dispatch deferral risk.** Building a second concrete collector without resolving how it's selected (Section 2, Section 13) risks the same kind of implicit, undocumented pattern GOV-004's evaluation found repeated three times before anyone examined it. This proposal does not resolve it, but names the risk of deferring it a fourth time without comment.
- **Determinism risk specific to the format.** ZIP's central-directory-vs-physical-order ambiguity (Section 3) is a new category of determinism hazard Filesystem Collection did not have to solve; getting it wrong would not surface as a test failure unless a test specifically constructs an archive with divergent orderings.

---

# 13. Open Questions Requiring Technical Director Review

- How would `AssessmentService`, `EvidenceCollector`, or some other component decide that a given `AssessmentInput` should be routed to archive-aware inspection rather than (or in addition to) the existing filesystem discovery? No such routing exists today, and this proposal does not design one.
- Is a second real, concrete collector — once ZIP traversal exists — the concrete case that finally justifies a Collector-dispatch abstraction, or should a third case be awaited, consistent with this platform's twice-applied "capability before abstraction" discipline?
- Is a malformed/corrupt archive the same architectural category as Unsupported Input, or does it warrant its own distinct outcome?
- How should duplicate entry names within a single archive be categorized?
- What resource limits, if any, should bound archive inspection, and should they be a Collection Error Model concern (a new outcome category) or an implementation detail beneath the model's current four outcomes?
- Which archive-parsing dependency to take is not evaluated by this proposal; the Technical Director may wish this decision, and its security implications, reviewed separately from this proposal's architectural approval.
- Should this proposal's approval be split, as Filesystem Collection's was, into an architecture-level approval followed by a separate implementation-readiness governance decision, or handled as one combined decision?

---

# 14. Confirmation of Scope

This proposal does not modify `EvidenceCollector`, does not modify `AssessmentService`, does not modify `EngineAPI.md`, does not create a new crate, does not introduce a trait or provider abstraction, and does not implement any ZIP support. It describes the shape of a future milestone for review, consistent with the architecture Platform Validation Phase 1 confirmed.
