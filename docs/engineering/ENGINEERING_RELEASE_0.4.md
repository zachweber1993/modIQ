# Engineering Release 0.4

| Property | Value |
|----------|-------|
| **Release** | 0.4 |
| **Documentation Release** | 2.1 (Frozen — Evidence Collection subsystem boundary; amended three times since freeze, see Documentation Completed) |
| **Milestone** | Sprint 4 complete (Phases 1–3D, plus Closeout) |
| **Scope** | GOV-011 (Archive Collection Model) resolution; the platform's second real Evidence Collector, archive-based, from structural enumeration through full policy enforcement and explicit `AssessmentService` routing; repository reconciliation and Sandbox archive validation at Closeout |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.3.md` (Sprint 3) |
| **Governing ADRs** | None. Whether the explicit-routing decision (Phase 3D) warranted a standalone ADR was named as an open question at Sprint 4's own outset (`SPRINT4_IMPLEMENTATION_PLAN.md`, Documentation Updates Expected); the Technical Director has since reviewed and concluded explicit routing remains an implementation decision, not a standalone architectural principle — no ADR follows |
| **Governing Proposals** | `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`, `PROPOSAL_GOV-011.md`, `PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md` |
| **Governing Plan** | `SPRINT4_IMPLEMENTATION_PLAN.md` |

---

## Executive Summary

Sprint 3 (Engineering Release 0.3) closed with the platform's first real Evidence Collector — filesystem discovery — and a named, waiting next step: a second real collector, most likely ZIP traversal. Sprint 4 delivered exactly that. GOV-011 (Archive Collection Model) was resolved in full across two review rounds, informed by an empirical boundary-proving investigation of the `zip` crate conducted entirely outside the workspace before any production code was written. Four implementation phases then built, in order, a deterministic archive-structure reader, an Evidence-generating transformation over it, a real Collector enforcing all three of GOV-011's remaining policies (resource limits, the Archive Traversal Boundary Policy, and the Duplicate Archive Entry Policy), and finally explicit routing wiring that Collector into `AssessmentService` — completing the platform's first end-to-end archive assessment path. No collector-dispatch abstraction was introduced at any point, continuing this platform's now four-times-applied "capability before abstraction" discipline (the Rule trait, Collector dispatch itself at GOV-004, and now this Sprint's own routing decision, confirmed rather than merely repeated).

This release is a consolidation, following the same charter Engineering Release 0.3 set: it records what four implementation phases and a Closeout already established, reconciles documentation that had fallen behind them — a recurring pattern this release names plainly rather than treats as new — and validates the completed capability through the Sandbox application before recommending what comes next.

**One structural finding from Closeout is worth stating up front, distinct from any single accomplishment:** `docs/governance/PROJECT_STATUS.md` and `docs/governance/CHANGELOG.md` — the two documents this repository designates as its living, always-current status record — had not been touched since before Sprint 4 began, despite four completed implementation phases and a fully resolved governance item in between. This is the same staleness pattern Engineering Release 0.3 flagged as "recurring... unless something changes about when these documents get touched." It recurred. See Sprint 4 Retrospective and Lessons Learned, below, for what this suggests rather than merely repeats.

---

## Scope of Sprint 4

### Delivered

- **Phase 1 — Governance Preparation.** Candidate answers to GOV-011's four questions drafted in concrete terms, ahead of empirical investigation.
- **Phase 2 — Boundary-Proving (Investigation).** The `zip` crate (v8.6.0) investigated against nine constructed fixtures entirely in a standalone project outside this repository: deterministic entry enumeration confirmed (explicit sort still required); malformed/non-archive input fails cleanly, no panic; entry count and compression ratio both recoverable from metadata alone (~1,270× faster than full decompression); `enclosed_name()` correctly rejects `..`-based traversal but *sanitizes* rather than rejects absolute-path entries; and, the one finding that changed Phase 1's own candidate — duplicate-named entries are only partially observable through the dependency's ordinary enumeration API, confirmed independently via three separate tools.
- **GOV-011 Resolved.** All four questions (malformed archives, duplicate entries, resource limits, traversal boundary) plus the Archive Metadata Policy resolved in full (`PROPOSAL_GOV-011.md`), incorporating Phase 2's evidence. `EvidenceCollection.md` amended.
- **Phase 3A — ZIP Archive Foundation.** `ArchiveReader`/`ArchiveEntry`/`ArchiveReadError`: opens a real archive, deterministically enumerates its structure (explicit sort imposed, not inherited), fails cleanly for malformed input. `zip` v8.6.0 added to `modiq-collection` — the platform's first archive-parsing dependency, and its second external dependency overall.
- **Phase 3B — Archive Evidence Generation.** `ArchiveEvidenceBuilder`: transforms `ArchiveEntry` values into real `Evidence`, reusing `EvidenceCategory::FileStructureAnalysis` unchanged — confirmed, not merely assumed, that no `modiq-runtime` change was needed for this step.
- **Phase 3C — Duplicate Detection, Traversal Filtering, Resource Limits.** `ArchiveCollector` assembled Phases 3A/3B with GOV-011's three remaining policies into one real Collector. Preceded by a dedicated Architecture Review (`PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md`) evaluating three representation options for duplicate-entry detection; approved with a naming refinement — `EvidenceCategory::StructuralDuplication`, named for the observation's semantic class rather than the collection mechanism, the platform's first new `EvidenceCategory` variant since Sprint 2. Duplicate detection required stepping outside the central directory entirely: the dependency indexes records by name, so a sequential local-file-header walk is the only way to observe a name collision the central directory itself discards while parsing.
- **Phase 3D — AssessmentService Archive Routing.** `ArchiveCollector` wired into `execute_from_assessment_input` via one explicit, inline, case-insensitive `.zip`-suffix check — no dispatcher, registry, trait, or common supertype. `execute` and the filesystem `EvidenceCollector` path both untouched.
- **Closeout — Repository Reconciliation, Sandbox Validation, Retrospective.** Documentation audited against actual repository state (this document and its companion Closeout report are among the results); a checked-in archive fixture and dedicated tests added to the Sandbox's own workspace, exercising `AssessmentService`'s archive-routing path through the exact production entry point `create_assessment` uses, with a regression guard confirming the pre-existing filesystem path is unaffected.

### Deliberately Not Delivered

- Nested archive traversal (an archive within an archive), or any archive format other than ZIP.
- XML, Lua, manifest, or dependency-analysis collectors — still the next capabilities in sequence, none started.
- Resolution of GOV-001, GOV-002, GOV-003, or GOV-008.
- A standalone ADR for the explicit-routing decision — named as a Technical-Director confirmation point at Sprint 4's outset; reviewed and declined at Closeout (explicit routing remains an implementation decision, not a standalone architectural principle).
- Calibration of resource-limit numeric thresholds (50,000 entries; 10,000:1 compression ratio) against production data — chosen with headroom above Phase 2's measured baseline, not against real-world archive telemetry that does not yet exist.
- A visual, GUI-level exercise of the Sandbox application — Closeout's Sandbox Archive Validation ran the same real production code path (`AssessmentService`, `ArchiveCollector`, the real fixture) the Tauri command itself uses, via the Sandbox's own `cargo test`, not by launching and interacting with the actual desktop window. This is the same limitation Engineering Release 0.3 already named ("no session has ever visually verified the Sandbox's actual UI") — still true after this release.

---

## Major Architectural Accomplishments

- **A second concrete Collector validated, rather than merely repeated, the "capability before abstraction" principle GOV-004 first confirmed.** `SPRINT4_IMPLEMENTATION_PLAN.md`'s own Technical Director Decisions state it plainly: "one existing collector and one newly approved collector is insufficient evidence to justify a routing abstraction." Four phases later, that decision held exactly as stated — `EvidenceCollection.md`'s Collector Contract section, which had left Collector composition an explicitly open question since Documentation Release 2.1, was amended at Phase 3D to record it resolved: one inline check, no registry, no trait.
- **The proposal-first workflow held for a fourth and fifth time.** `PROPOSAL_GOV-011.md` (the governance resolution) and `PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md` (the Question 2 representation choice) were both written and approved before their corresponding implementation began. Neither required a return trip to architecture once implementation started.
- **An architecture review changed a design decision before any code existed to make changing it expensive.** Phase 2's finding that duplicate entries are only partially observable invalidated Phase 1's original candidate (one Evidence item per duplicately-named entry) *before* Phase 3 implementation began, not after. The proposal-then-implement sequencing this platform has followed since Sprint 3 caught this at its cheapest possible point.
- **The Evidence channel absorbed a genuinely new kind of fact without a new mechanism.** `StructuralDuplication` is a fact about collection's own observability, not about the assessment subject's structure — a categorically different kind of thing than every other Evidence category to date — and it still fit inside the existing `Evidence`/`EvidenceCategory` model with one additive variant, not a parallel reporting channel. The Architecture Review that reached this conclusion explicitly rejected the alternative (an out-of-band field on a new return type) specifically because it would have reopened GOV-008 as a side effect.

---

## Major Implementation Accomplishments

- Real, deterministic archive-structure enumeration: entries sorted explicitly (the dependency's own central-directory order confirmed unreliable, not assumed), file-vs-directory kind and uncompressed size recorded, no content ever decompressed for structural purposes.
- `modiq-collection` grew from 12 tests (Engineering Release 0.3) to 43: every GOV-011-defined outcome — resource-limit violation (both entry-count and compression-ratio forms), each traversal-violation shape (relative escape, Unix absolute, Windows drive-qualified), duplicate detection (present and correctly absent), malformed/inaccessible archives, and determinism — has its own direct test, mirroring the outcome-per-test discipline Sprint 3 Phase 5 established for the filesystem case.
- Resource-limit and duplicate-detection test fixtures needed hand-built, byte-level archive construction (`write_raw_archive`) rather than the `zip` crate's own `ZipWriter`, once `ZipWriter` itself was found to reject duplicate filenames at write time — a constraint that did not exist as a testable obstacle during Phase 2's own external investigation, discovered only once Phase 3C's own tests needed it.
- `AssessmentService` gained its first behavioral branch since Sprint 3 Phase 5 without changing either public entry point's signature: `is_archive_location` is a private, pure, directly-unit-tested function: 7 new `modiq-engine` tests (9 → 16), covering the routing decision itself and a full real-I/O archive assessment through the pipeline.
- The Sandbox now exercises both real collectors from its own workspace: the pre-existing filesystem fixture, unmodified, and a new checked-in archive fixture, both driven through the identical `AssessmentService::execute_from_assessment_input` entry point `create_assessment` uses in production — 3 new Sandbox tests (3 → 6), added at Closeout.

---

## Governance Completed

| Item | Resolution | Phase |
|---|---|---|
| GOV-011 | Archive Collection Model resolved in full: malformed/resource-limit-violating archives are Unsupported Input (no new outcome); duplicate entries recorded as an observable fact, never fabricated per-entry; traversal boundary normalizes and skips invalid entries (including absolute paths, independent of dependency sanitization) without aborting the archive; Archive Metadata Policy excludes timestamps/permissions/comments from Evidence. | Phase 1/2, resolution session |

**Still open:** GOV-001 (Report generation timing), GOV-002 (Runtime invariant reconciliation), GOV-003 (role of `modiq-common`), GOV-008 (AssessmentService API evolution). None were touched this Sprint; see Technical Debt Review.

No new Governance Register item was opened this Sprint — GOV-011 was already created (candidate) at Sprint 4's own Phase 1. Total Governance Register size: 11 items, 7 Resolved (GOV-004, 005, 006, 007, 009, 010, 011), 4 Open (GOV-001, 002, 003, 008).

**One governance-adjacent question was explicitly deferred to Sprint 4's own close, per its own implementation plan, and is now resolved:** whether the explicit-routing decision warrants a standalone ADR, mirroring ADR-0010's treatment of GOV-004. Raised for Technical Director confirmation in the Closeout report rather than decided by Engineering, per this project's standing division of authority — the Technical Director has reviewed it and concluded explicit routing remains an implementation decision, not a standalone architectural principle. No ADR follows. `SPRINT4_IMPLEMENTATION_PLAN.md`'s Documentation Updates Expected section, and its Completion Checklist, are updated to record this outcome.

---

## Documentation Completed

- **`EvidenceCollection.md`** amended three times this Sprint (v1.2.0 → v1.4.0): GOV-011's archive-specific Collection Outcomes and both remaining policies (Phase 1/2 resolution); all three implemented, not merely resolved, and the `StructuralDuplication` representation choice recorded (Phase 3C); Collector Contract's composition question marked resolved (Phase 3D). One version-table inconsistency (top metadata table one revision behind the document's own Document Status footer) found and corrected in the course of the Phase 3D amendment, not left in place.
- **Three Architecture Review proposals** — `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`, `PROPOSAL_GOV-011.md`, `PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md` — retained as historical records of the design work that preceded implementation, per this project's proposal convention.
- **`ENGINEERING_LOG.md`** — eight new entries this Sprint (Phase 1, Phase 1 continued, Phase 2, GOV-011 Resolved, Phase 3A, Phase 3B, Phase 3C, Phase 3D), each following the established Status/Affected Crates/Affected Documents/Notes structure. A ninth, for this Closeout itself, is added alongside this document.
- **`CrateRoadmap.md`** — five new revision-history entries (1.10.0 through 1.14.0 span GOV-004 implementation through Phase 3D); `modiq-collection`'s Implementation Status row updated to reflect both real collectors' reachability; a new "Sprint 4 — Complete" narrative section added at Closeout, matching the existing Sprint 1/2/3 pattern.
- **Living-document reconciliation at Closeout** — `docs/governance/PROJECT_STATUS.md` and `docs/governance/CHANGELOG.md`, both found stale at Sprint 3's own frozen state (Platform Validation Phase 1) despite an entire Sprint's worth of work since, corrected: new milestone/phase/focus fields, a new `## Sprint 4 — Complete` narrative section in `PROJECT_STATUS.md`, and a new `# [Sprint 4]` entry in `CHANGELOG.md` mirroring Sprint 3's own Added/Deferred/Released structure. `docs/README.md`'s one stale Engineering Release cross-reference corrected to point to this document. `docs/governance/ROADMAP.md` and `docs/governance/EngineeringGuide.md` were found stale as well, but since Sprint 0/1 — predating Sprint 4 entirely and out of this Closeout's named scope — and were not touched; see Repository Reconciliation Summary.
- **`SPRINT4_IMPLEMENTATION_PLAN.md`** — Status field and full Completion Checklist reconciled against verified repository state at Closeout, not carried over from any phase's own self-report. The standalone-ADR item was left explicitly unchecked pending Technical Director review; the Technical Director has since declined it (explicit routing remains an implementation decision), and the checklist is updated to reflect a fully closed Sprint 4.

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release 0.3 | 112 | 3 |
| End of Sprint 4 Phase 3A | 120 | 3 |
| End of Sprint 4 Phase 3B | 126 | 3 |
| End of Sprint 4 Phase 3C | 143 | 3 |
| End of Sprint 4 Phase 3D | 150 | 3 |
| End of Sprint 4 Closeout (this release) | **150** | **6** |

By crate, at this release: `modiq-runtime` 82, `modiq-collection` 43, `modiq-engine` 16 unit + 3 integration, `modiq-report` 3, `modiq-rules` 3. Zero tests are ignored or flaky anywhere in the workspace, verified by direct execution while preparing this release, not carried over from any phase's own report.

**A second genuinely new testing discipline entered the suite this Sprint**, matching the pattern Engineering Release 0.3 named for Sprint 3 Phase 5's OS-level fixtures: hand-built, byte-level archive construction, required once `ZipWriter`'s own duplicate-filename rejection made realistic fixtures for GOV-011's adversarial cases (duplicate names, path traversal, fabricated resource-limit metadata) impossible to build through the library's own writer API.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, unchanged in count since Engineering Release 0.3. The `modiq-rules` `Cargo.toml` inconsistency flagged in 0.3 (`version`/`edition` declared literally rather than via `.workspace = true`) is still present, still not worth a dedicated fix on its own. |
| Crate boundaries | **Stable** | Zero violations across five phases plus Closeout, including the introduction of a second real Collector and a new `EvidenceCategory` variant. |
| Dependency direction | **Stable** | Strictly downward throughout; `modiq-collection`'s new `zip` dependency edge verified to introduce no new crate-to-crate edge — `modiq-engine`'s dev-only `zip` dependency (added at Phase 3D, for test fixtures) does not participate in the production dependency graph. |
| Documentation consistency | **Improved, but the underlying pattern is unresolved.** Engineering Release 0.3 named this "a recurring pattern, not an incident" and predicted it would recur "unless something changes about when these documents get touched." It recurred exactly as predicted — `PROJECT_STATUS.md`/`CHANGELOG.md` went stale again, caught again only at a dedicated Closeout. The correction this release performs is real; the structural cause is not yet addressed. See Lessons Learned. |
| Testing maturity | **Improved.** Sprint 3's own noted gap — no adversarial-input testing beyond what that phase specifically needed — was substantially closed this Sprint: GOV-011's archive work is the platform's first genuinely adversarial input surface, and it now has direct, dedicated test coverage for every defined adversarial case. The Sandbox-UI verification gap 0.3 named remains open, unchanged — see Deliberately Not Delivered. |
| Engineering workflow | **Stable.** The proposal → governance → documentation → implementation → verification cycle was exercised successfully across GOV-011's resolution and the Architecture Review for its Question 2 representation, with zero post-implementation architectural rework required in either case. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-runtime` | L3, 82 tests, unchanged this Sprint | `Display`/`Serialize` for identity/enum types (flagged repeatedly, still open); referential integrity for Finding/Recommendation references | High — untouched by four phases of neighboring collector work, zero surprises | Low — stable, revisit only when a concrete forcing function arrives |
| `modiq-engine` | L3, 16 unit + 3 integration tests (up from 12) | Resolve GOV-008 when implementation evidence justifies it; no other known gap | High — the routing addition needed no signature change to either public entry point | Low — stable |
| `modiq-collection` | L2, 43 tests (up from 12) | XML, Lua, manifest, and dependency-analysis collectors; GOV-011's provisional numeric thresholds await production calibration | High — two real, independent, production-reachable collectors now proven | **High** — this is still where the platform's actual value proposition continues to be built |
| `modiq-rules` | L3 for exactly one Rule, 3 tests, unchanged since Engineering Release 0.3 | A second Rule, if and when a concrete case justifies one (still deliberately deferred) | High for the one Rule that exists — evidence-count-driven, unaffected by a second Evidence source | Low — deliberately deferred |
| `modiq-report` | L3, 3 tests, unchanged since Sprint 2 | GOV-001 (report generation timing) still bears directly on this crate | High for what exists | Medium — unchanged from 0.3's own assessment |
| `modiq-knowledge` | L1, pure scaffold, unchanged since Sprint 0 | Everything | Low — untested against any real usage | Low — correctly sequenced after Rule abstraction, itself deferred |
| `modiq-versioning` | L1, pure scaffold, unchanged since Sprint 0 | Everything | Low — untested against any real need | Low — no forcing function exists yet |
| `modiq-cli` | L1, pure scaffold, unchanged since Sprint 0 | Wiring `AssessCommand` to `modiq-engine`, mirroring the Sandbox's now-thrice-proven thin-client pattern (filesystem, then archive, both through the identical entry point) | High — the pattern is more de-risked than at 0.3, not less | Medium — genuinely low-risk, independent of any Collection work |
| `modiq-common` | L1, empty stub files, unchanged since Sprint 0 | Undecided | Low — zero evidence, after ten phases across four Sprints, that this crate is needed | Low — do not invent a use for it |

**Four crates — `modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-common` — have now received zero implementation across five Sprints.** This Sprint changes nothing about that assessment; Engineering Release 0.3's reasoning for leaving them deferred (the same "second concrete case" discipline applied elsewhere) still holds and is not repeated in full here.

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- GOV-008 remains unresolved; the additive-entry-point pattern (`execute` / `execute_from_assessment_input`) is unchanged and untouched a second consecutive Sprint, per explicit Technical Director direction each time.
- No dispatch abstraction for either Rule selection or Collector selection — evaluated and declined again this Sprint, the third and fourth applications respectively of the same standing principle.
- Resource-limit numeric thresholds remain provisional by design (GOV-011's own resolution text), not a gap to close before production calibration data exists.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- XML inspection, Lua inspection, manifest analysis, dependency analysis — named in sequence by `PROPOSAL_FILESYSTEM_COLLECTION.md`'s Future Evolution section, still not started.
- Nested archive traversal, non-ZIP archive formats.
- Rule abstraction and multi-rule dispatch.
- Knowledge Domain integration, Version Profile integration, CLI wiring, persistent assessment storage.

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — flagged at Sandbox Phase 2, Sprint 3 Phase 1, Engineering Release 0.3, and again here: **this is now the fifth consecutive release record to note it without scheduling it.**
- No session has ever visually verified the Sandbox's actual UI — this release's own Sandbox Archive Validation exercised the real production code path via `cargo test`, not the actual desktop window, for the same reason Engineering Release 0.3 gave: every check across every phase to date has been Rust-only.
- Git tag hygiene: `v0.2.0` and `v0.3.0` predate `v0.1.0-alpha` chronologically (pre-existing, flagged previously); Engineering Release 0.3 could not be tagged `v0.3.0` as a result. **`v0.4.0` does not currently exist as a tag** — unlike 0.3, this release could be tagged cleanly if the Technical Director wishes; this document does not create that tag itself.
- The `modiq-rules` `Cargo.toml` `version`/`edition` inconsistency flagged in Engineering Release 0.3 is still present, unfixed, two releases later.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- `docs/governance/ROADMAP.md` and `docs/governance/EngineeringGuide.md` reconciliation — stale since Sprint 0/1, never addressed by any release record to date, this one included.
- `CrateRoadmap.md`'s "Exit Criteria" section has no Sprint 3 or Sprint 4 entry — a gap that predates this Sprint (Sprint 3 never received one either) and was not created asymmetrically for Sprint 4 alone during this Closeout.

---

## Sprint 4 Retrospective

**What went well.** The proposal-first workflow held for a fourth and fifth consecutive time, across a governance resolution (GOV-011) and a narrower Architecture Review (duplicate-entry representation) — neither required a return trip to architecture once implementation began. The "capability before abstraction" principle, confirmed in the abstract at GOV-004, was tested under real, concrete pressure with a second real Collector actually in hand — and held, producing one inline conditional instead of a dispatch mechanism. GOV-011's four questions, resolved substantially in the abstract during Phase 1, needed exactly one substantive revision once real investigation began (Question 2), and that revision was caught by design-before-code sequencing at its cheapest possible point.

**What surprised us.** The `zip` crate's own duplicate-filename rejection at write time — not present as an obstacle during Phase 2's external investigation, discovered only once Phase 3C's tests needed to construct exactly the fixtures Phase 2 had used. The central directory's name-indexed internal structure (`IndexMap<Box<[u8]>, ZipFileData>`) was more thoroughly characterized this Sprint than Phase 2's original investigation had needed to go, once Phase 3C's implementation had to build production code around the limitation rather than merely observe it. And, less pleasantly but not actually surprising given precedent: `PROJECT_STATUS.md`/`CHANGELOG.md` staleness recurred exactly as Engineering Release 0.3 predicted it would.

**Architectural decisions validated by implementation.** GOV-004's "capability before abstraction" principle, confirmed for Rule/Collector dispatch in the abstract at Platform Validation Phase 1, was validated a second time with an actual second Collector in hand — the harder, more concrete test of the same claim. `EvidenceCollection.md`'s Collector Contract, which had explicitly left composition "an open implementation question" pending a second concrete Collector, was resolved exactly the way the document's own framing anticipated: a capability justified the decision, not the reverse.

**Areas where the architecture proved stronger than expected.** The Evidence channel absorbed a categorically new kind of fact — `StructuralDuplication`, a fact about collection's own observability rather than about the assessment subject — without needing a second reporting channel. The Architecture Review that reached this conclusion explicitly evaluated and rejected the out-of-band alternative specifically because it would have reopened GOV-008, showing the GOV-008 boundary actively shaping a decision in an unrelated area, not merely sitting unresolved.

**Areas requiring future attention.** GOV-008 has now gone two consecutive Sprints untouched with the same two-entry-point stopgap in place. Documentation staleness is now confirmed recurring, not merely predicted to recur — Lessons Learned, below, treats this as a pattern requiring a structural response, not another flag. The standalone-ADR question for explicit routing, raised at Sprint close exactly where `SPRINT4_IMPLEMENTATION_PLAN.md` said it would need a decision, has since been resolved by the Technical Director: no ADR, explicit routing remains an implementation decision.

---

## Remaining Risks

- **Documentation staleness is now a confirmed, repeating pattern across two consecutive Sprints**, not a one-time incident — the single most important repository-process risk this release identifies, ahead of any technical risk.
- **GOV-004 and GOV-008 aging without resolution.** GOV-004 is now resolved (this was true at 0.3's writing but is worth restating as no longer a risk); GOV-008 remains, now aging across two full Sprints since Platform Validation Phase 1 last reviewed it.
- **The next real collector's own adversarial surface is unknown until scoped.** GOV-011's archive work closed the platform's first genuinely adversarial input surface; whichever collector comes next (XML or Lua inspection, most likely, per `PROPOSAL_FILESYSTEM_COLLECTION.md`'s sequencing) will need its own dedicated threat analysis — this Sprint's archive-specific findings do not transfer automatically.
- **Resource-limit thresholds are unvalidated against real Farming Simulator mod archives.** Chosen with headroom above Phase 2's synthetic measurements, not against production telemetry that does not exist yet.

---

## Lessons Learned

- **Flagging documentation staleness without a dedicated reconciliation task does not fix it — this is now demonstrated twice, not asserted once.** Engineering Release 0.3 said exactly this after Sprint 3; it recurred, unchanged, through Sprint 4. The evidence now supports a structural response rather than a repeated flag: a documentation-sync step embedded at Sprint close (which this Closeout performed) is not the same as one embedded per-phase, and per-phase staleness (Sprint 4's own governance/status documents falling behind mid-Sprint, corrected only now) suggests even a Sprint-close-only reconciliation is not the earliest point this could be caught.
- **Byte-level test fixture construction, once needed for one adversarial case, generalizes cleanly to the rest of the same adversarial surface.** `write_raw_archive`, built to route around `ZipWriter`'s duplicate-filename rejection, was reused directly for traversal and resource-limit fixtures needing similarly precise, otherwise-unconstructible byte layouts — this capability should be reused, not reinvented, by whichever future collector next needs adversarial fixtures.
- **A narrow, dedicated Architecture Review — scoped to one representation question, not a full governance resolution — is a lighter-weight tool than a full `PROPOSAL_*` document and worked well for exactly the size of decision GOV-011's Question 2 turned out to be.** Worth reaching for deliberately the next time an implementation-mechanism question (as opposed to a policy question) needs Technical Director input.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged since Engineering Release 0.3) |
| Governance items | 11 total — 7 Resolved (GOV-004, 005, 006, 007, 009, 010, 011), 4 Open (GOV-001, 002, 003, 008) |
| Documentation Release | 2.1, Frozen (amended three times this Sprint within `EvidenceCollection.md`, per this project's established frozen-document-amendment pattern) |
| Engineering Release | 0.4 (this document) — not yet tagged; `v0.4.0` is available (no collision), unlike `v0.3.0` |
| Root workspace tests | 150 (up from 112 at Engineering Release 0.3) |
| Sandbox tests | 6 (up from 3) |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0–3; Engineering Releases v0.1.0-alpha, 0.2, 0.3; Platform Validation Phase 1; Sprint 4 Phases 1–3D; Sprint 4 Closeout |
| Implementation readiness | Architecturally and functionally ready for the next capability. Working tree state as of this document's writing: see the accompanying Closeout report's Repository Reconciliation Summary. |

---

## Repository Timeline

```
Documentation Release 1.0
        ↓
Sprint 0
        ↓
Sprint 1
        ↓
Engineering Release v0.1.0-alpha
        ↓
Sprint 2
        ↓
Engineering Release 0.2
        ↓
Sprint 3 (Phases 1–5)
        ↓
Documentation Release 2.1
        ↓
Engineering Release 0.3
        ↓
Platform Validation Phase 1 — GOV-004 resolved; GOV-008 deferred
        ↓
Sprint 4 Phase 1/2 — GOV-011 resolved
        ↓
Sprint 4 Phase 3A — ArchiveReader
        ↓
Sprint 4 Phase 3B — ArchiveEvidenceBuilder
        ↓
Sprint 4 Phase 3C — ArchiveCollector; EvidenceCategory::StructuralDuplication
        ↓
Sprint 4 Phase 3D — AssessmentService archive routing
        ↓
Sprint 4 Closeout — repository reconciliation; Sandbox archive validation
        ↓
Engineering Release 0.4
        ↓
Sprint 5 (not yet scoped)
```

---

## Recommendation

**Sprint 5 is ready to be scoped.** This release's own findings support that conclusion on the same basis Engineering Release 0.3 used: the architecture held under a second real Collector's worth of pressure, no boundary was crossed, the full workspace and Sandbox test suites are green, and the one outstanding process risk (documentation staleness) is a repository-process concern, not a blocker to further implementation — it did not prevent Sprint 4 from completing, and this release's own reconciliation demonstrates it is correctable, if not yet structurally prevented.

**What Sprint 5 should contain is not decided here.** `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own Future Evolution sequencing names XML inspection as a plausible next collector; CLI wiring remains an independent, low-risk parallel track this release's own Crate Maturity Review rates as more de-risked than ever, now that the Sandbox's thin-client pattern has proven out against two collectors instead of one. Neither is scoped here — consistent with this release's own charter, and with Engineering Release 0.3's before it, a release record recommends a direction, not a plan.

**One item should be resolved before or very early in Sprint 5, not carried forward silently a third time:** whether documentation staleness gets a structural fix — a per-phase or per-commit reconciliation habit, not merely a per-Sprint-close one — rather than a sixth consecutive release record repeating the same finding. The standalone-ADR question is no longer open: the Technical Director has confirmed explicit routing remains an implementation decision, closing the one item Repository Reconciliation had flagged as genuinely undecided.
