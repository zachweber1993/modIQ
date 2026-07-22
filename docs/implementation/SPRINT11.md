# Sprint 11

> **Runtime Log Ingestion Foundation — Runtime Evidence Processing Architecture**

---

| Property | Value |
|----------|-------|
| **Document** | SPRINT11.md |
| **Version** | 1.0.0 |
| **Status** | Proposed — planning only, awaiting Chief Architect review before Sprint 11 begins |
| **Project** | modIQ |
| **Documentation Release** | 2.1 (unchanged; this document does not amend it) |
| **Owner** | Zach Weber |
| **Created** | 2026-07-22 |
| **Last Updated** | 2026-07-22 |

---

# Specification Authority

Authority:

- Vision.md
- ProductSpecification.md
- Architecture.md
- DataModel.md
- KnowledgeModel.md
- RuleEngine.md
- EvidenceCollection.md
- GOVERNANCE.md
- PROJECT_HANDOFF_v1.0.md, Section 5 (the canonical Sprint lifecycle)

Frozen planning inputs this Sprint builds on, not reopened:

- `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md` and `SPRINT_ROADMAP_UPDATE_v1.md` — the already-resolved Runtime Log Interpretation workflow decision (bundled submission, no second Assessment Input, no cross-Assessment correlation).
- `SPRINT10_CAPABILITY_DEFINITION.md` — the capability's own scope (recognizing one class of signal: a mod failed to load) and its strengthened evidentiary precondition.
- `SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md` and `ENGINEERING_RELEASE_1.0.md` — the real fixture corpus this Sprint is grounded in.

---

# Sprint Objective

**Sprint 11 defines the Runtime Evidence Processing Architecture: the architectural model governing how a real runtime observation becomes an interpretable platform outcome — grounded in the three real runtime log fixtures Sprint 10 acquired — without implementing code.**

This is the single capability Sprint 11 delivers: an approved, evidence-grounded architecture, not a working feature. Where a runtime observation enters the platform's existing Assessment pipeline, how it is structurally represented, how it is interpreted, and what platform stage acts on that interpretation are all questions this Sprint resolves architecturally — none is assumed in advance as a specific implementation construct. It is the smallest meaningful advancement beyond Sprint 10 available to this project — smaller than an implementation Sprint, but the only step that can responsibly precede one, since Sprint 10's own charter was acquiring evidence specifically so this architecture would not have to be defined on assumption (`SPRINT10_CAPABILITY_DEFINITION.md`, Section 11). Sprint 11 does not reopen Sprint 9's own already-resolved workflow decision, and it does not build against its own conclusions in the same Sprint — that remains a distinct, later, separately authorized Implementation Sprint.

---

# Background

Sprint 10 was authorized on the explicit basis that no architectural or implementation decision for Runtime Log Interpretation may assume the structure, wording, stability, or formatting of a real Farming Simulator runtime log. It closed having acquired exactly that evidence: three real, captured, normalized fixtures (`clean-base-game`, `single-compatible-mod`, `single-incompatible-mod`), each fully documented, together spanning a mod-free baseline, a successful third-party mod load, and one real, fully evidenced failure (a declared-`descVersion` mismatch, rejected at modDesc validation with the exact engine message `Error: Unsupported mod description version in mod <name>`).

That evidence is the direct enabling prerequisite for Sprint 11, not a side effect of it. Before Sprint 10, any architectural decision about how runtime observations are processed would have been made against an assumed log shape — precisely the failure mode `SPRINT10_CAPABILITY_DEFINITION.md`'s own strengthened precondition exists to prevent. Sprint 11 is the first point in this project's history where that architecture can be defined responsibly, because real evidence, not assumption, now exists to define it against.

Sprint 10 intentionally deferred parser implementation. That decision remains unchanged for Sprint 11 as well: this Sprint produces a reviewed architecture, not Rust code. Implementation is the Sprint that follows Sprint 11's own successful architectural definition, not part of it.

---

# Scope

## Included

- Reviewing all three real fixtures as the grounding evidence for every architectural decision below — no decision may be made without citing at least one fixture or an established repository precedent.
- **Architecturally defining where and how a runtime observation enters the platform's existing Assessment pipeline** — resolving, not presuming, whether this is realized within the existing Evidence Collection subsystem (e.g., as a new Collector, consistent with `XmlCollector`'s own Sprint 7 composition precedent) and how it composes into `AssessmentService::execute_from_assessment_input`, consistent with the already-resolved bundled-submission workflow (`SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, Option A).
- **Architecturally defining what shape `EvidenceCategory::RuntimeLogs`'s produced content takes** — a factual, observation-derived description of the recognized signal, mirroring `XmlCollector`'s own Content Extraction discipline (report the fact, never the interpretation).
- **Architecturally defining how this evidence is interpreted into a platform outcome** — including whether that interpretation is realized as a new Rule, its position within `RuleEngine::evaluate`'s fixed declaration order (GOV-012), and what `FindingSeverity` applies — a real architectural question, since Farming Simulator's own log calls this an "Error" but that is not automatically this platform's own Finding Severity vocabulary. Resolved by this Sprint's own reasoning, not assumed in advance.
- **Producing the Runtime Interpretation Decision Matrix** (see Deliverables) — the architectural contract mapping each recognized class of runtime observation to its interpretation, confidence level, and intended future Rule or processing stage, which future implementation must conform to rather than invent independently.
- Naming, explicitly, whether any additional fixture is needed before Implementation can responsibly begin, or whether the current three are sufficient — a conclusion, not an assumption.
- **Sprint 11 consumes the existing Sprint 10 runtime corpus as it stands.** Reviewing, citing, and reasoning from the three existing fixtures is in scope; acquiring new ones is not (see Out of Scope).
- Producing a Sprint 11 Architectural Resolution document recording every decision above, its rationale, its repository precedent, and the alternatives considered and rejected — the same standard every prior Architectural Resolution in this project has met.

## Not Included

- Writing any runtime evidence processing implementation (e.g., a Collector or Rule) or tests for it.
- Any change to `AssessmentService`'s public entry points, unless this Sprint's own architectural findings demonstrate a genuine, evidence-grounded need — and if so, that need is named and reasoned about, not assumed in advance.
- Any change to `RuleEngine::evaluate`'s existing parameter shape beyond what dispatching one additional interpretation stage requires, mirroring exactly how `VersionCompatibilityRule`'s own addition was handled at Sprint 8 — decided during this Sprint's own architectural work, not pre-decided here.

## Out of Scope

- **Any Rust implementation** — processing code (whether realized as a Collector, a Rule, or otherwise) or tests for it. Explicitly deferred to a future Implementation Sprint.
- **Reopening the Runtime Log Interpretation workflow decision** (second Assessment Input, cross-Assessment correlation, a standalone Runtime Log Assessment Subject) — closed by Sprint 9's own Architecture Evaluation and Chief Architect decision; not reconsidered here.
- **Acquiring additional runtime fixtures.** Sprint 11 consumes the existing Sprint 10 runtime corpus (`clean-base-game`, `single-compatible-mod`, `single-incompatible-mod`) as its complete evidentiary basis. No additional fixture is to be acquired during this Sprint unless a documented architectural deficiency is discovered that cannot be resolved using the existing corpus — and if one is found, it is named and reasoned about in the resolution itself, not acted on silently. `modded-map-only`, `real-world-mod-profile`, and non-macOS platform captures remain named, deferred candidates; this Sprint may recommend one as an Implementation precondition, but does not acquire it.
- **Broader log-signature coverage** beyond the one class of signal Sprint 10 already scoped (`SPRINT10_CAPABILITY_DEFINITION.md`, Section 7).
- **Any Governance Register item or ADR**, unless this Sprint's own findings demonstrate a genuine cross-platform architectural decision extending beyond this capability — the same bar already applied at Sprint 9's own Architectural Resolution, not lowered here.
- **Any Knowledge Domain pairing** (a Repair Recipe for a runtime-log Finding) — a named future extension, not this Sprint's concern.

---

# Engineering Goals

- Produce an approved, evidence-grounded Runtime Evidence Processing Architecture, with every decision traceable to real fixture evidence or an established repository pattern — no decision grounded in assumption.
- Produce a deterministic, explicit interpretation contract (the Runtime Interpretation Decision Matrix) that future implementation must conform to, rather than invent interpretation behavior independently.
- Preserve the Evidence Collection / Rule Engine boundary exactly as specified (`EvidenceCollection.md`'s Collector Contract: observation is reported, never judged in the same step) — whatever structure this architecture resolves to, the stage that reports a runtime fact and the stage that interprets it remain distinct, never reversed.
- Introduce no new abstraction (a dispatch table, a plugin mechanism, a general processing-stage trait) without a proven forcing function, consistent with this project's "capability before abstraction" discipline, applied without exception since Sprint 4.
- Preserve `AssessmentService`'s two public entry points and `RuleEngine::evaluate`'s existing dispatch shape unless the evidence itself demonstrates otherwise — the same additive-only outcome Sprint 8 and Sprint 9 both actually produced.
- Preserve deterministic behavior: any recognition mechanism this architecture defines must be exercisable identically against the same fixture every time, and the design must state plainly how that determinism will be verified once implemented.
- Maintain platform-first architecture: the architecture must not assume Farming Simulator's log format is stable indefinitely, and must name what happens to the design if a future capture (a different platform, a future game patch) contradicts it — the fixture corpus's own immutability-and-superseding policy already provides the mechanism; this Sprint's job is to confirm the architecture relies on that mechanism rather than silently assuming stability.

---

# Deliverables

- **A Sprint 11 Architectural Resolution document** — recording the Runtime Evidence Processing Architecture this Sprint defines: where and how a runtime observation enters the platform's existing Assessment pipeline; `EvidenceCategory::RuntimeLogs`'s content shape; how that evidence is interpreted, its position relative to the existing fixed Rule dispatch order (GOV-012), and its assigned `FindingSeverity`; any new dependency edge anticipated; and whether the current fixture corpus is sufficient for Implementation or whether a specific additional fixture should be named as a precondition. Every decision is resolved by this Sprint's own architectural reasoning, not assumed from a specific implementation construct in advance.
- **The Runtime Interpretation Decision Matrix** — a deterministic architectural specification, not parser logic and not implementation, defining how future runtime observations will be interpreted. For each recognized class of runtime observation, the matrix states: the observation itself, its architectural interpretation, a stated confidence level, and the intended future Rule or processing stage responsible for acting on it. The matrix is the implementation contract for future development — implementation must conform to it; it does not invent interpretation behavior independently.
- **No crate changes, no interfaces, no tests, no new fixtures, no APIs, no examples.** Sprint 11's own scope is architectural decision-making; the only artifacts its scope justifies are the decision record and the interpretation contract above.

---

# Success Criteria

- A Sprint 11 Architectural Resolution document and a Runtime Interpretation Decision Matrix both exist, reviewed and approved by the Chief Architect, together answering every question named in this Sprint's own Scope section — each answer citing at least one of the three real fixtures or a named, existing repository precedent.
- The Decision Matrix covers every runtime observation class this Sprint's own evidence supports, each entry stating an architectural interpretation, a confidence level, and an intended future Rule or processing stage — no entry left as a placeholder.
- Zero Rust source file is modified; `cargo test --workspace` remains at 210/210, Sandbox at 7/7 — unchanged, confirmed directly, not assumed.
- No existing architectural boundary (the Evidence Collection / Rule Engine separation, `AssessmentService`'s two public entry points, GOV-012's fixed Rule dispatch order) is redesigned; any proposed extension is additive.
- No Governance Register item or ADR is created unless this Sprint's own findings explicitly meet the existing bar for one — and if either is created, it states the specific finding that justified it.
- No additional runtime fixture is acquired during this Sprint, unless a documented architectural deficiency requiring one is explicitly named and reasoned about in the resolution itself.

---

# Risks

- **Single-sample design risk.** Only one failure class has real evidence (`single-incompatible-mod`'s own declared-`descVersion` mismatch). A design built too tightly around that one signature's literal text could fail to generalize to a different failure class later. *Mitigation:* the resolution should design the recognition mechanism against the general shape of the evidence (a discrete, `Error:`-prefixed engine message naming a specific mod) rather than hardcoding the one literal string, and must name this explicitly as a known limitation — the same discipline that made `VersionCompatibilityRule` an accepted minimum-viable design at Sprint 8, not a premature generalization.
- **Platform coverage gap.** All three fixtures were captured on macOS; nothing is yet known about whether Farming Simulator's own log format differs on Windows or Linux. *Mitigation:* name this as a residual risk the resolution cannot close by design alone — no architectural decision substitutes for a missing fixture — and flag whether Implementation should be gated on acquiring at least one additional platform's capture first.
- **Premature schema commitment.** Deciding `EvidenceCategory::RuntimeLogs`'s content shape too generally, before a second failure class exists to test it against, risks the same speculative-abstraction pattern this project has already corrected twice (Sprint 9's Question 2 conflation; the Warning Categorization gap found during Sprint 10 acquisition). *Mitigation:* scope the resolution to the minimum shape the one recognized signal actually needs, mirroring `VersionProfile::fs25()`'s and `RepairRecipe::version_compatibility_declared_version_mismatch()`'s own "one real value, no generalization mechanism yet" precedent.
- **Scope drift into implementation.** A design that feels sufficiently obvious can invite writing code before it is reviewed. *Mitigation:* this Sprint's own Success Criteria explicitly exclude any code artifact; Implementation Authorization is a distinct, later, separately authorized stage, exactly as this project's canonical workflow already requires (`PROJECT_HANDOFF_v1.0.md`, Section 5).

---

# Dependencies

All already satisfied by prior releases; none invented for this Sprint:

- `EvidenceCategory::RuntimeLogs` — real, defined since before Sprint 1, unused, confirmed directly this session.
- Three real, captured, fully documented fixtures — `fixtures/runtime-logs/clean-base-game/`, `single-compatible-mod/`, `single-incompatible-mod/` (Sprint 10; `ENGINEERING_RELEASE_1.0.md`).
- The already-resolved Runtime Log Interpretation workflow decision — bundled submission, same Assessment Input, no correlation mechanism (`SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, `SPRINT_ROADMAP_UPDATE_v1.md`).
- Established repository precedents this resolution is expected to reason against: the Collector Contract (`EvidenceCollection.md`); `XmlCollector`'s Content Extraction discipline (Sprint 7, extended Sprint 8); GOV-012's fixed Rule dispatch order; ADR-0007's Opaque Runtime References pattern; and the "one named, domain-authored minimum-viable value" precedent (`VersionProfile::fs25()`, `RepairRecipe::version_compatibility_declared_version_mismatch()`).
- The fixture corpus's own standing policies — Runtime Log Normalization, Warning Categorization, Installation State versus Savegame State (`fixtures/runtime-logs/README.md`) — governing how any future fixture work must continue to be handled, even though Sprint 11 itself adds no new fixture.

---

# Deferred Work

- **Runtime Log Interpretation's own Implementation** — Collector code, Rule code, and their tests — the Sprint immediately following Sprint 11's own successful Architectural Resolution, not begun here.
- **Fixture corpus expansion** — `modded-map-only`, `real-world-mod-profile`, and any non-macOS platform capture — named, real, deferred candidates; Architectural Resolution may recommend one as an Implementation precondition without acquiring it itself.
- **Broader log-signature coverage** beyond the one class of signal already scoped.
- **Any Knowledge Domain pairing** (a Repair Recipe for a runtime-log Finding) — named at Sprint 10 as a natural future extension, still not taken up.

---

# Recommended Sprint Sequence

Sprint 11 must precede Runtime Log Interpretation's own Implementation Sprint for the same reason Capability Definition has always preceded Architectural Resolution, and Architectural Resolution has always preceded Implementation, throughout this project's history (`PROJECT_HANDOFF_v1.0.md`, Section 5). Building a Collector or Rule before their shape has been decided and reviewed risks the "implementation-led architecture" failure mode this project's own Decision Framework exists to prevent (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, Section 6). Sprint 10 already demonstrated, directly, the cost of skipping evidence-gathering — the `clean-base-game` fixture had to be reclassified once, after an assumption about savegame freshness proved wrong. Sprint 11 exists to avoid the analogous cost of skipping design review before code is written against real evidence that, for the first time, actually exists.

---

# Technical Director Assessment

**Sprint sizing:** Small and well-bounded — a design-only Sprint, deliberately as narrow as Sprint 10's own precedent-breaking, evidence-only shape. This project's own history already establishes that a Sprint producing no code can still be a complete, well-scoped Sprint; Sprint 11 follows that same shape one stage further along the lifecycle.

**Architectural alignment:** High. Every question this Sprint must answer has a directly analogous, already-proven precedent elsewhere in this repository — `XmlCollector`'s Content Extraction discipline, `VersionCompatibilityRule`'s Warning-severity precedent, GOV-012's fixed dispatch order, and the "one named, domain-authored minimum-viable value" pattern proven twice (Sprint 8, Sprint 9). This is the least novel architectural territory Runtime Log Interpretation could occupy, now that Sprint 9 closed its workflow question and Sprint 10 closed its evidentiary one.

**Risk:** Low. No code change is possible within this Sprint's own scope, so no regression risk exists; the residual risks (single-sample design, platform coverage, premature schema commitment) are all named above with a concrete mitigation, not merely acknowledged.

**Expected repository impact:** One new planning and decision document, and one architectural interpretation contract (the Decision Matrix). No crate, dependency, test, or fixture change. Repository maturity metrics (nine crates, 210 root tests, 7 Sandbox tests) are expected to remain exactly as recorded at Engineering Release 1.0 through the end of this Sprint.

---

# Document Status

**Current Version:** 1.0.0

**Status:** Proposed. Awaiting Chief Architect review before Sprint 11 begins. No implementation, ADR, or repository file beyond this document has been produced or modified.
