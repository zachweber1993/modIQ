# Proposal: GOV-011 Question 2 — Duplicate-Entry Detection Representation

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md |
| **Stage** | Proposal (Architecture Review) |
| **Prepared by** | Engineering, for Technical Director review |
| **Governance item addressed** | GOV-011, Question 2 — representation mechanism only (`PROPOSAL_GOV-011.md`, Future Implementation Scope: "How 'duplicate central-directory records were detected' becomes an observable fact is undetermined") |
| **Sprint context** | Sprint 4 Phase 3C preparation, per `SPRINT4_IMPLEMENTATION_PLAN.md` Phase 3 |
| **Status** | Presented for Technical Director review only. **No Rust file has been modified. No `Cargo.toml`/`Cargo.lock` change. No `EvidenceCategory`, `CollectionError`, `ArchiveEntry`, or `ArchiveEvidenceBuilder` change has been made.** Nothing here is authorized for implementation until approved. |

---

## Scope

GOV-011's policy for duplicate entry names (Option (b), Technical Director-approved) is settled: no silent last-write-wins, no fabricated per-entry Evidence for an entry the collection mechanism cannot actually observe, and the existence of duplication must itself become an observable fact of collection. What that policy does **not** settle — and what this document addresses — is *how* "duplication was detected" is represented as a concrete Rust value that eventually reaches the rest of the platform.

This document is scoped to representation only. It does not address:
- **Detection mechanism** — how the eventual code determines duplication exists at all. Phase 2 found the `zip` crate's ordinary enumeration API cannot see this (below); an eventual detection mechanism is separate, real design work, not decided here.
- **Traversal-boundary filtering** or **resource limits** — GOV-011's other Phase 3C items, unaffected by this question.
- **`AssessmentService` routing** — Phase 3D, out of scope here.

## Constraints This Proposal Must Satisfy

Drawn directly from `EvidenceCollection.md` and the current codebase, not invented for this document:

1. **Not a failure.** `EvidenceCollection.md`, Duplicate Archive Entry Policy: "This is not itself a failure: an archive containing duplicate entry names is still a valid, reachable, supported location, and collection of it still succeeds." `CollectionError` (`crates/modiq-collection/src/collection/collection_error.rs`) has exactly two variants, `Inaccessible` and `Unsupported`, both documented as "Collection aborts." Neither describes a completed, successful collection that also observed an anomaly. Whatever represents this fact must not be modeled as a new failure/abort outcome — that would misstate what actually happened.
2. **One producer→consumer channel.** `EvidenceCollection.md` (Which failures belong to Collection) is explicit that the Evidence Collection / Rule Engine boundary is a producer-consumer relationship this specification exists to preserve, and the Duplicate Archive Entry Policy paragraph itself frames the fix in the same terms: Collection "reports what it can factually establish about the subject... rather than presenting a silently incomplete result as if it were complete" — the same evidence-first discipline applied everywhere else in this spec. Today, `Evidence` (`modiq-runtime`) is the only channel by which Collection communicates observable facts to the Rule Engine. `AssessmentService` (`crates/modiq-engine`) passes a `Vec<Evidence>` through; nothing else crosses that boundary.
3. **No fabricated per-entry Evidence for the unobservable entry.** Confirmed directly from Phase 2's evidence (`PROPOSAL_GOV-011.md`, Evidence Incorporated): against a fixture with two identically-named entries, `archive.len()` reported 1, and the earlier entry was not reachable through the `zip` crate's ordinary enumeration API at all. `ArchiveReader::entries()` as it exists today (`crates/modiq-collection/src/collection/archive_reader.rs`) iterates `0..self.archive.len()` — it will silently see only the surviving entry, with no indication a second one ever existed. `ArchiveEvidenceBuilder::build()` will, unmodified, produce one `Evidence` item for that surviving entry, correctly, and no item at all for the other — which is the silent-incompleteness case GOV-011 forbids presenting as if it were complete.
4. **Determinism.** Platform-wide requirement, applied without exception elsewhere (`ArchiveReader::entries()`'s explicit sort is the most recent example). Any representation that carries more than a boolean — e.g., a list of affected names — must be produced in a stable, deterministic order.
5. **`modiq-runtime` is the leaf, currently unchanged by Sprint 4.** The v2.0 handoff notes explicitly that Phase 3B needed no `modiq-runtime` change and that this crate is treated as stable/L3. A representation option that touches `EvidenceCategory` is a change to that leaf and should be weighed accordingly, not treated as free.

## Options

### Option A — Reuse `EvidenceCategory::FileStructureAnalysis`

Emit one additional `Evidence` item, alongside the per-entry items `ArchiveEvidenceBuilder` already produces, using the existing category: e.g. `Evidence::new(EvidenceCategory::FileStructureAnalysis, "N duplicate entry name(s) detected during archive collection.")`.

- **Fit with constraints 1–3:** satisfies them — it is not a failure outcome, it flows through the existing Evidence channel, and it does not fabricate an item for the unobservable entry specifically.
- **Architecture fit:** zero `modiq-runtime` change; smallest possible footprint, confined entirely to `modiq-collection`.
- **Concern:** `FileStructureAnalysis` is a category of facts *about the mod's discovered structure* — one item per real file or directory, which is exactly what `ArchiveEvidenceBuilder`'s own tests currently assert (`build_produces_one_evidence_item_per_entry_in_order`, one item per entry, category uniform across all of them). A duplicate-detection item is a fact about *collection's own observability*, not about the mod's structure. I confirmed the platform's one real Rule (`RuleEngine::evaluate`, `crates/modiq-rules/src/rules/engine.rs`) does not currently filter or count by category — it treats all Evidence uniformly today, so this would not misbehave *yet*. But any future Rule reasoning over `FileStructureAnalysis` by count or by iterating expecting one item per real file (an entry-count check, a "no Lua files present" scan) would silently misinterpret this meta-item as a spurious file entry. That failure mode is latent, not hypothetical to guard against — this category is precisely the kind of thing a future Rule is expected to consume structurally.

### Option B — New, narrowly-scoped `EvidenceCategory` variant

Add a new variant (naming not fixed here — e.g. `CollectionIntegrity`) to `modiq-runtime`'s `EvidenceCategory`, and emit the same kind of item as Option A, but tagged with the new category instead of `FileStructureAnalysis`.

- **Fit with constraints 1–3:** same as Option A — not a failure, flows through the same Evidence channel, no fabrication.
- **Fit with constraint 4:** if the eventual detection mechanism can recover affected names (open question, see below), they can be listed in the description or `location` field in sorted order, same discipline as `ArchiveReader::entries()`.
- **Advantage over Option A:** keeps "fact about the mod's content" and "fact about collection's own process" as two distinct, honestly-labeled categories, rather than one category silently carrying two different meanings. This is consistent with the Archive-Specific Outcomes precedent in `EvidenceCollection.md`, which extended the *outcome* model by reusing existing categories only where the new case is genuinely the same kind of thing (malformed archive = Unsupported Input, not a new outcome) — duplication is arguably not the same kind of thing as a discovered file, so it does not get the same "reuse without a new variant" justification Phase 3B's reuse of `FileStructureAnalysis` had (a directory or file inside an archive genuinely is the same kind of fact as one on disk).
- **Cost:** this is a `modiq-runtime` change — the one crate every phase of Sprint 4 so far has left untouched, and one the v2.0 handoff calls out as stable/L3. Per this project's architecture-authority rule, a change to `modiq-runtime`'s public domain model is exactly the kind of decision Engineering should surface rather than make unilaterally, even though the change itself (`match` exhaustiveness aside) is additive and small.

### Option C — Carry the fact outside the Evidence stream entirely

Introduce a new return shape from the archive collector (e.g., a struct wrapping `Vec<Evidence>` plus a separate `duplicate_entries_detected` field), consumed by `AssessmentService` or another orchestration layer rather than by the Rule Engine via `Evidence`.

- **Fit with constraint 2:** fails it directly. This creates a second, out-of-band channel parallel to Evidence, which is exactly what `EvidenceCollection.md` says this specification exists to prevent — a collection-observed fact that never reaches the Rule Engine at all unless some other layer specially threads it through. It would also move the "silently incomplete result presented as complete" problem from the entry level to the Collection→Engine boundary rather than resolving it.
- **Fit with GOV-008:** to be consumed anywhere, this would require a new signature or return type on `AssessmentService`'s public API — exactly what GOV-008 (AssessmentService Public API Evolution) deliberately deferred, and what the v2.0 handoff states plainly should not be touched "until directed." Choosing this option risks quietly reopening GOV-008 as a side effect of a Question 2 representation choice, which is a materially larger architectural footprint than this question should require.
- Not recommended, included for completeness of the option space.

## Recommendation

**Option B** — a new, narrowly-scoped `EvidenceCategory` variant, carried through the existing Evidence channel.

Reasoning: Options A and C both fail one of the two constraints that matter most here. Option A stays inside the Evidence channel but conflates two different kinds of fact under one category, which is a real (if latent) correctness risk for any future Rule that reasons over `FileStructureAnalysis` structurally. Option C avoids that conflation but exits the Evidence channel entirely, which both violates the producer-consumer boundary `EvidenceCollection.md` protects and risks entangling this decision with GOV-008. Option B is the only one of the three that keeps the fact inside the one established channel *and* keeps it honestly labeled as a distinct kind of fact. Its cost — a small, additive `modiq-runtime` change — is real but bounded, and is a decision for the Technical Director rather than one Engineering should make by choosing Option A to avoid raising it.

## Open Questions for the Technical Director

- Is a new `EvidenceCategory` variant authorized, given `modiq-runtime` has been unchanged through all of Sprint 4 so far? If not, Engineering's fallback recommendation is Option A, with the conflation risk above accepted and documented rather than resolved.
- Naming/severity: this proposal does not fix a variant name or wording — that is implementation detail once the representation question itself is decided.
- Detection-mechanism dependency: Phase 2 showed the `zip` crate's ordinary API cannot expose the unobservable entry itself, but did not establish whether a lower-level scan (e.g., raw central-directory record parsing, the same class of technique Phase 2 used for cross-verification via `unzip -l`/`zipfile`/byte-signature counting) can recover the *duplicated name(s)*, as opposed to only a count or boolean. This proposal recommends designing the representation to accommodate a list of names if achievable (richer, still deterministic) while remaining valid if only a count is recoverable — but the detection mechanism itself is separate work, not decided here, per GOV-011's own separation of Question 2's policy from its representation and detection mechanisms.

## What This Proposal Does Not Do

No Rust file was modified. No `Cargo.toml` or `Cargo.lock` was modified. No `EvidenceCategory` variant was added. No `CollectionError` variant was added. No detection mechanism was chosen or implemented. No archive collection behavior changed. `AssessmentService` was not touched. `docs/architecture/EvidenceCollection.md` and `docs/engineering/GOVERNANCE.md` are not modified by this document — any resulting documentation update follows Technical Director approval, per this project's established proposal-then-record discipline.
