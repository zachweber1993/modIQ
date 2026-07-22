# Sprint 9 Runtime Log Architecture Evaluation

| Property | Value |
|---|---|
| **Document** | SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Purpose** | Reduce uncertainty about whether Runtime Log Interpretation requires the platform's first true multi-input Assessment — evaluation only, no recommendation to implement |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `ad29fd6` (Sprint 8 formally closed) |
| **Status** | Architecture evaluation only. No code, documentation, governance, or ADR change. The question is evaluated, not resolved — per explicit instruction, this document reduces uncertainty rather than eliminating it. |

---

# 1. Current Assessment Model

Verified directly against source and frozen specification this session.

## Assessment Subject

`DataModel.md`: *"An Assessment Subject represents the artifact being evaluated... Each Assessment evaluates exactly one Assessment Subject."* Examples given: individual Farming Simulator mods, mod collections, savegames, maps, future supported assessment targets (`Glossary.md` adds vehicle packs, placeable packs). **"Exactly one" is stated without qualification** — this is a Runtime Domain-level constraint (`DataModel.md`, governed by `ADR-0003`), not merely an Evidence Collection convention. In current code, `AssessmentSubject` is a content-free unit struct — real content has never been implemented for it in any Sprint to date; every Assessment constructed so far uses the bare `AssessmentSubject` value regardless of what is actually being assessed.

## Assessment Input

Defined by `EvidenceCollection.md` (GOV-009), not `DataModel.md` — Assessment Input is an Evidence Collection-subsystem concept, application-supplied, consumed by Collectors, never created or reinterpreted by Collection itself. Verified directly against `crates/modiq-collection/src/collection/assessment_input.rs`: `AssessmentInput` wraps exactly one field, `value: String`. `AssessmentService::execute_from_assessment_input`'s own signature takes exactly one `input: impl Into<String>` parameter. There is no plural, no collection, no list anywhere in this type's history.

## Evidence

`DataModel.md`: *"Evidence represents objective information collected during an Assessment... Every Finding should be traceable to supporting Evidence. Evidence never represents opinion."* Currently produced by three Collectors (`EvidenceCollector`, `ArchiveCollector`, `XmlCollector`), all operating against the same single Assessment Input per Assessment.

## Findings

Produced by the Rule Engine from Evidence, never by Evidence Collection. Three real Rules exist (`EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule`), each consuming the full, flat `&[Evidence]` slice with no awareness of which Collector — or, by extension, which Assessment Input — produced any given item.

## Report

`AssessmentReport::generate` — a pure, read-only snapshot of one `Assessment`'s state (id, status, evidence, findings, recommendations). No field on `Assessment` or `AssessmentReport` references another Assessment. **No cross-Assessment correlation mechanism exists anywhere in the repository** — confirmed by direct inspection of `Assessment`'s full field list (`id`, `subject`, `context`, `version_profile`, `status`, `evidence`, `findings`, `recommendations`): nothing resembling a "related Assessment," "parent Assessment," or "session" identifier exists.

---

# 2. Assessment Input Analysis

## How many Assessment Inputs currently exist per Assessment?

**Exactly one, structurally, not merely by convention.** This is enforced at three independent levels, found by direct inspection, not inferred from a single source:

1. **Type level:** `AssessmentInput { value: String }` — a single scalar field, not a collection.
2. **API level:** `AssessmentService::execute_from_assessment_input(&self, subject, context, input: impl Into<String>)` — one `input` parameter.
3. **Specification level:** `EvidenceCollection.md`'s Collector Contract: *"A Collector receives: An Assessment Input (or the portion of one relevant to its kind of inspection)... A Collector receives nothing else."* Every reference to Assessment Input throughout `EvidenceCollection.md` uses the singular, indefinite article ("an Assessment Input") — never a plural form.

## What constitutes an Assessment Input today?

A stable reference to a filesystem object (file, directory, or `.zip` archive) at the moment collection begins — the mod's own package, submitted by the application layer. Sprint 7's Multi-Source Evidence Collection (`XmlCollector` running alongside a structural Collector) did **not** change this: both Collectors receive the *same* single Assessment Input independently; this is multiple Collectors against one input, not multiple inputs.

## Where the "exactly one" assumption is explicit versus load-bearing by omission

- **Explicit:** `DataModel.md`'s "each Assessment evaluates exactly one Assessment Subject" is a direct, unqualified textual constraint.
- **Load-bearing by omission, not by explicit prohibition:** GOV-009's own resolution is more cautious than `DataModel.md`'s Subject constraint — it explicitly states *"Future input types... are intentionally out of scope and explicitly deferred; this resolution does not claim to be Assessment Input's final shape for every future collector."* The specification itself has already flagged that Assessment Input's singular shape may not generalize — without saying how it should change. This is a meaningfully different kind of uncertainty than the Subject constraint: one is a stated rule, the other is a stated *absence of a rule* for the multi-input case.

---

# 3. Runtime Log Architectural Analysis

Evaluated against each candidate category the mission names, without recommending among them.

## As Evidence

A strong textual fit for *what a log's content actually is*: `DataModel.md`'s "objective information collected during an Assessment" describes a runtime log's content (recorded facts about a load/play attempt) reasonably well. But Evidence is *produced by Collectors from an Assessment Input* — this framing only resolves the *content* question, not *how the log's location reaches a Collector in the first place*, which is exactly the open question this session investigates.

## As Assessment Input

Plausible only under a specific, currently-unstated product assumption: that the log is supplied *as part of the same submission* as the mod (e.g., bundled into the same directory or archive). Under that assumption, a new Collector could locate a well-known log filename at the existing Assessment Input's own root — structurally identical to how `XmlCollector` already locates `modDesc.xml` at the same root every structural Collector also inspects. Under the *alternative* assumption — that a user assesses a mod first, then separately supplies a log afterward, expecting the two to be correlated — this framing does not hold at all: there would be two temporally separate submissions, and `AssessmentInput`'s current shape has no notion of "additional input arriving later against an already-existing Assessment."

## As Assessment Subject

`DataModel.md`'s own Assessment Subject examples are all *installable or creatable game artifacts a user possesses on an ongoing basis* — mods, mod collections, savegames, maps, vehicle/placeable packs. A runtime log is architecturally different in kind: it is forensic material describing a single *event* (an attempt to load or run something), not a persistent artifact analogous to the others. Nothing in `DataModel.md` or `Glossary.md` rules this out explicitly — "future assessment targets supported by the platform" is deliberately open-ended — but no existing example resembles it, and treating a log as its own Assessment Subject would mean *assessing the log itself*, as a complete, self-contained Assessment, rather than assessing "the mod, using the log as supporting material."

## As Assessment Context

`DataModel.md`'s Assessment Context examples — *"Version Profile, Assessment timestamp, Assessment configuration, Platform capabilities, Execution environment"* — describe lightweight, largely-tag-like descriptors of the environment an Assessment ran in. *"Execution environment"* is a striking near-match in wording. But every one of Context's actual examples is a small, discrete fact (a timestamp, a version label, a capability flag) consulted for context, not raw content requiring its own deterministic parsing and fact-extraction the way a manifest or archive does. A real runtime log — potentially containing stack traces, specific error codes, and multi-line diagnostic output — needs the same kind of dedicated, Collector-grade inspection `XmlCollector` already performs on `modDesc.xml`, which is a poor fit for Context's own established, lightweight role. `AssessmentContext` also remains, as of Sprint 8, a completely content-free unit struct — untouched even when Sprint 8 gave `Assessment` its own new Version Profile relationship (put directly on `Assessment`, not into `Context` — Sprint 8's own Decision 1).

**No single category is a clean, unambiguous fit.** This is itself the most important finding of this section: the difficulty is not that Runtime Log Interpretation lacks a plausible architectural home — it has at least two plausible ones (Evidence-via-bundled-input, or its own standalone Assessment Subject) — but that *which one is correct depends on a product/workflow question this repository does not yet answer*: is a runtime log assessed *together with* the mod that produced it, or *on its own*?

---

# 4. Architectural Options

Evaluated strictly against repository evidence; no new abstraction proposed beyond what the mission's own illustrative options and the analysis above already surface.

## Option A — Runtime Log as another Collector against the same Assessment Input

A new Collector locates a well-known log filename (or a log explicitly bundled alongside the mod) at the same Assessment Input's root, mirroring `XmlCollector`'s exact precedent for `modDesc.xml`. Requires the product assumption that the log is supplied *together with* the mod, in the same submission.

## Option B — Runtime Log as a second Assessment Input

`AssessmentService` (or a new entry point) accepts two distinct inputs — the mod and the log — for the same Assessment. No precedent exists for this anywhere in the repository; every Collector composition decision to date (Sprint 4's archive-vs-filesystem routing, Sprint 7's multi-Collector composition) has been multiple *Collectors* against one *input*, never multiple inputs.

## Option C — Runtime Log as its own Assessment Subject (a standalone Assessment)

A user submits *only* a runtime log; the platform assesses the log itself as the Assessment Subject, using a new Collector that parses log content directly, producing an Assessment and Report about *the log*, not about any mod. Fits `DataModel.md`'s "exactly one Assessment Subject" constraint without exception — the Subject is simply the log, a new example added to the same open-ended list "savegames," "maps," etc. already populate. Does not, by itself, correlate that log-Assessment with any separately-run mod-Assessment — no such correlation mechanism exists (Section 1).

## Option D — Runtime Log as Assessment Context

Evaluated in Section 3 and found a poor fit — included for completeness, not because repository evidence favors it. Context's own established examples are lightweight descriptors, not content requiring dedicated Collector-grade parsing.

## No option requiring a wholly new abstraction was found necessary

Every option above is expressible using concepts the repository already defines (Collector, Assessment Input, Assessment Subject, Assessment Context) — the uncertainty is about *which existing concept applies*, and about an unstated product assumption, not about needing to invent a new one.

---

# 5. Comparative Evaluation

| | **Option A** (Collector, same input) | **Option B** (second Assessment Input) | **Option C** (log as its own Subject) | **Option D** (Assessment Context) |
|---|---|---|---|---|
| **Architecture impact** | None — extends the already-proven Content Extraction / multi-Collector-composition pattern (Sprint 7) | New — no precedent for more than one input per Assessment anywhere in the repository | None — uses the Assessment Subject concept exactly as already specified, just a new example of it | Would require `AssessmentContext` to gain real content for the first time (Sprint 8 deliberately did not do this even for Version Profile) |
| **`AssessmentService` impact** | None — same composition shape as `XmlCollector`'s own introduction | Requires a new parameter or entry point accepting a second input; direct GOV-008-adjacent Public API Policy question, the same one Sprint 8 deliberately avoided reopening | None — same two existing entry points, just given a log path instead of a mod path | Would require passing log content into `Assessment::new` somehow — no existing path for this |
| **Evidence impact** | A new `EvidenceCategory::RuntimeLogs`-producing Collector, same shape as every prior Collector | Same new Collector, but must now reconcile Evidence from two distinct inputs in one Assessment — a new determinism/ordering claim analogous to, but larger than, Sprint 7's multi-Collector-same-input claim | Same new Collector; Evidence describes the log alone, no mod-related Evidence coexists in the same Assessment | Context is not itself Evidence-producing; the log's own content would need to become Evidence some other way regardless, undermining this option on its own terms |
| **Rule Engine impact** | None beyond a new Rule to interpret `RuntimeLogs` Evidence, mirroring `VersionCompatibilityRule`'s own recent introduction | Same new Rule; Rule Engine itself is unaffected either way, since it already consumes a flat, provenance-blind `&[Evidence]` slice regardless of how many inputs produced it | Same new Rule; entirely within a self-contained Assessment | None directly — the unresolved Evidence-production question above still applies |
| **Data Model impact** | None | Arguably none to `Assessment`'s own shape (Evidence is still one flat list) but real ambiguity about whether `AssessmentInput`'s own singular shape needs to change to plural, and how atomicity (`EvidenceCollection.md`: Collection Atomicity) applies when *one* of two inputs fails | None — Subject remains singular, exactly as `DataModel.md` already requires | `AssessmentContext` gains its first real field |
| **Implementation complexity** | Low — directly analogous to `XmlCollector`'s own Sprint 7 introduction | Medium–High — the "second input" question has no precedent to copy from anywhere in this repository | Low — directly analogous to introducing any new Assessment Subject content, though `AssessmentSubject` itself has never been given real content in any Sprint to date | Medium — `AssessmentContext`'s first real content, a change Sprint 8 explicitly declined to make even under direct pressure to do so |
| **Repository consistency** | High — extends an already-twice-validated pattern (Sprint 7's Collector Composition, Sprint 8's Content Extraction) | Low — no existing precedent; would require its own dedicated Architecture Evaluation, the same weight Sprint 7 gave Collector Composition itself | High — fits `DataModel.md`'s existing, unmodified "exactly one Subject" constraint without exception | Medium — plausible by wording, but Context's own established role (lightweight descriptors) does not match a log's actual informational density |

**A first-order finding, not previously visible before this comparison:** Options A and C are both **low-complexity and high-consistency** — the difficulty in choosing between them is not architectural at all, but a product-workflow question this repository's specifications do not answer: *is the log meant to be assessed together with the mod that produced it, or on its own?* Option B is the only option that is unambiguously an architectural expansion, and it is only necessary if the intended workflow is "assess the mod now, supply its log later, and correlate the two" — a shape no part of this repository currently supports at all, since no cross-Assessment correlation mechanism exists (Section 1).

---

# 6. Engineering Risk Assessment

**Would Runtime Log Interpretation naturally fit the existing implementation lifecycle, or does it warrant its own Capability Definition and Architectural Resolution?**

**It warrants its own Capability Definition before any Architecture Evaluation of implementation mechanics** — but for a more specific reason than "the architecture is uncertain." The uncertainty found in this session is **not primarily architectural** (Options A and C are both low-risk, high-consistency, and fully expressible with concepts this repository already defines) — it is a **product/workflow question that Capability Definition, not Architecture Evaluation, is the correct stage to resolve**, per this project's own canonical workflow (`PROJECT_HANDOFF_v1.0.md`, Section 5: Capability Definition establishes *what capability the work should provide*, grounded in product specification, before any architecture is designed).

Concretely: whether a real player workflow is "submit mod and log together" or "submit mod, then separately submit a log, and see them correlated" is not answerable from `ProductSpecification.md` or `Vision.md` as currently written — both name "runtime log interpretation" as a capability in the abstract, without describing the submission workflow around it. Answering that question is Capability Definition's job. Once answered, the corresponding architecture option (A or C, most likely; B only if correlation is the actual product intent) follows with comparatively low implementation risk, based on this session's own comparative evaluation.

**Risk if this distinction is not made explicit:** implementation could begin against an assumed workflow (most likely Option A, the path of least resistance, mirroring `XmlCollector`) only to discover mid-implementation that the intended product experience actually requires correlating a separately-submitted log against an already-completed Assessment — precisely the "implementation-led architecture" failure mode this project's own Decision Framework (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, Section 6) exists to prevent, and precisely the kind of premature commitment Sprint 7's own Collector Composition study was convened specifically to avoid for a comparably shaped question.

---

# 7. Repository Impact

*(Informational only — no implementation is proposed or recommended.)*

Should Capability Definition determine Option A or C is the intended shape: new `EvidenceCategory::RuntimeLogs`-producing Collector in `modiq-collection`, a new Rule in `modiq-rules`, no change to `AssessmentService`'s public entry points, no change to `DataModel.md`'s Subject cardinality constraint, no new dependency edge required by parsing alone (plain text/regex matching, no exotic dependency anticipated, unlike Sprint 4's own `zip` crate evaluation).

Should Capability Definition determine Option B is actually required: a dedicated Architecture Evaluation of comparable weight to Sprint 7's Collector Composition study would be warranted before implementation — this would be the first repository-wide precedent for multi-input Assessments, touching `EvidenceCollection.md`'s own Assessment Input definition (an amendment, per this project's "frozen does not mean immutable, amendment recorded explicitly" discipline), Collection Atomicity's own meaning when more than one input exists, and very likely a new Governance Register item analogous to GOV-009's own original scope.

---

# 8. Recommendation

**This session does not recommend an architectural option, and does not recommend Sprint 9 implementation, per its own explicit charter.**

What this evaluation does establish, with reasonable confidence, grounded in the comparison above:

1. **The uncertainty the Capability Prioritization Study flagged is real, but narrower than "does this require architectural expansion."** Two of the four evaluated options (A and C) require no architectural expansion at all and are directly expressible using already-proven repository patterns. Only Option B is a genuine architectural expansion, and it is conditional on a specific, currently unanswered product-workflow question.
2. **The open question is a Capability Definition question, not an Architecture Evaluation question.** Before any further architecture work on this capability, the intended player workflow (bundled submission versus separate, correlated submissions) should be established — the same discipline this project already applies everywhere else (capability before abstraction; capability before architecture).
3. **Should Sprint 9 (or any future Sprint) take up Runtime Log Interpretation, its own Capability Definition should explicitly name this workflow question as a question to answer, not an assumption to make** — mirroring exactly how Sprint 7's own Capability Definition surfaced the Collector Composition question before any implementation began, rather than discovering it mid-Sprint.

---

# 9. Chief Architect Questions

*(Not answered here.)*

1. Is the intended Runtime Log Interpretation player workflow "submit the mod and its runtime log together, in one submission" (favoring Option A), "assess a runtime log entirely on its own, with no mod involved" (favoring Option C), or "assess a mod now, and correlate a separately-submitted log against that same Assessment later" (requiring Option B, and a cross-Assessment correlation mechanism this repository does not yet have in any form)?
2. If correlation between a mod's own Assessment and a separately-submitted log is a real, intended future product need — even if not Sprint 9's own scope — should that be named now as a distinct, deferred future capability (mirroring how Sprint 8 named "a second Version Profile" as a known future need without building it), so it isn't rediscovered as a surprise later?
3. Does `AssessmentSubject`'s own complete lack of real content (a content-free unit struct since Sprint 0, across eight Sprints) need to be resolved before Option C (log as its own Assessment Subject) could be implemented cleanly — or would this capability be an acceptable first forcing function for giving `AssessmentSubject` real content, the same way Sprint 8 gave `modiq-versioning` its own first real content?
4. Should this question be scoped as its own dedicated Capability Definition session before Sprint 9 planning proceeds further, mirroring the weight Sprint 7 gave Collector Composition — or folded into whichever Sprint eventually takes up Runtime Log Interpretation as one of that Sprint's own early phases?

---

Awaiting Chief Architect review. No implementation, documentation change, governance item, or ADR has been made this session.
