# Proposal: Evidence Collection Boundary

**Stage:** Proposal (first stage of the Architectural Review Process, `GOVERNANCE.md`)

**Prepared by:** Engineering, for Technical Director review

**Status:** Design exercise only. No implementation has been attempted. No Runtime, Rule Engine, or Sandbox code has been touched. No documentation other than this proposal has been modified.

---

## Context

Sprint 3 Phase 1's own architectural review identified Evidence Collection — real inspection of an Assessment Subject's actual files — as "a genuine, currently unowned architectural gap": no crate, service, or ADR currently claims it. Today, every Evidence value in the system is synthetic: `modiq-rules::RuleEngine` only ever *consumes* Evidence (per `RuleEngine.md`, correctly, and unchanged by this proposal), and `apps/sandbox` constructs one deterministic, hardcoded Evidence item as a stand-in. This proposal answers the ownership and boundary questions the Technical Director requested, so that concrete collectors (ZIP, XML, Lua, manifest, dependency inspection) have a stable foundation to be implemented against later, rather than each reopening the same architectural questions.

Nothing here proposes implementing any collector. This is the boundary the collectors will eventually sit behind.

---

## 1. Ownership

### Which crate owns Evidence Collection?

**Recommendation: a new crate**, positioned as a peer to `modiq-rules` and `modiq-report` in `CrateRoadmap.md`'s dependency hierarchy (depends on `modiq-runtime` only; nothing depends on it except `modiq-engine`). A working name — `modiq-collection` — is used throughout this proposal for reference only; naming is a decision for whoever authors the resulting ADR, not this proposal.

Options considered and rejected:

- **Inside `modiq-runtime`.** Rejected: `GOVERNANCE.md`'s Crate Boundary Rules state Runtime "must never... perform orchestration," and reading real files (filesystem/ZIP/XML/Lua parsing) is exactly the kind of specialized, fallible, side-effecting logic Runtime has never owned — Runtime owns *state and identity*, not external inspection. It would also tie Runtime's dependency graph to parsing libraries for no architectural reason.
- **Inside `modiq-engine`.** Rejected: Engine's charter is orchestration and composition — `GOVERNANCE.md` states it "must never... implement business rules." Inspecting mod files and classifying the result into an `EvidenceCategory` *is* business logic, of the same specialized kind `modiq-rules` was already kept separate from Engine for. Folding it into Engine would make Engine a grab-bag, contradicting the precedent that kept Rule evaluation in its own crate.
- **Inside `modiq-knowledge`.** Rejected on domain grounds: `Architecture.md`'s Domain Separation principle distinguishes runtime assessment data (exists only within one Assessment) from reusable engineering knowledge (exists independently of any Assessment). Evidence is runtime data, generated fresh per Assessment Subject — it has nothing to do with the Knowledge Domain's reusable-knowledge charter.
- **A new crate.** Matches the precedent already set by `modiq-rules` and `modiq-report`: each is a narrow, single-responsibility crate depending only on `modiq-runtime`, composed by `modiq-engine`. Evidence Collection is a distinct responsibility from all four existing content crates (Runtime state, Rule evaluation, Reporting, orchestration) and fits the same shape.

### Which component is responsible for orchestrating collection?

**The Engine** (`modiq-engine`, specifically wherever `AssessmentService::execute` — or a sibling service — already lives), not the Runtime `Assessment` aggregate itself. This mirrors the precedent already established and already reviewed for Rule execution: `AssessmentService::execute` invokes `RuleEngine::evaluate` and then calls `Assessment::add_finding`/`add_recommendation` — the `Assessment` aggregate never reaches out and invokes the Rule Engine itself. Evidence Collection should follow the identical shape: the Engine invokes the collector(s), receives Evidence values back, and calls the Assessment's existing `add_evidence` for each — the aggregate remains a passive receiver of state, never an active orchestrator, consistent with `GOVERNANCE.md`'s Runtime boundary ("must never... perform orchestration") and with `DataModel.md`'s Immutability/Assessment-Centric principles.

(`DataModel.md`'s definition of Assessment — "The Assessment coordinates evidence collection, rule execution, finding generation..." — reads at the conceptual/vocabulary level, not the crate-implementation level; the existing, already-reviewed implementation already resolves this by having Engine orchestrate while Runtime's `Assessment` type owns only the resulting state. This proposal is consistent with that existing resolution, not a departure from it.)

---

## 2. Pipeline

Proposed end-to-end flow, extending (not replacing) the already-implemented and already-reviewed pipeline:

```
Application input (a path or input descriptor)
        │
        ▼
   Engine (modiq-engine)
        │
        ▼
Evidence Collection  ←── new: one or more Collectors inspect
        │                the input descriptor and return Evidence
        ▼
   Assessment.add_evidence (existing, unchanged — Runtime remains
        │                    sole mutator, INV-006/007/009)
        ▼
   Rule Engine (modiq-rules)  ←── unchanged, still only ever
        │                          consumes Evidence, never produces it
        ▼
   Findings / Recommendations (existing, unchanged)
        │
        ▼
   Assessment Report (modiq-report, existing, unchanged)
        │
        ▼
   Application output (Sandbox DTO, CLI output, etc.)
```

This matches `DataModel.md`'s Runtime Lifecycle ordering (Evidence Collected, then Findings Produced) and the pipeline already implemented and reviewed in Sprint 3 Phase 1 — it inserts one real step (Collection) in place of the sandbox's current hardcoded bootstrap Evidence, without reordering anything downstream.

**A documentation inconsistency worth flagging, not resolving here:** `Architecture.md`'s own Assessment Lifecycle diagram places "Evidence Collection" *after* "Rule Engine" and "Knowledge Base" in its stage ordering, which conflicts with `DataModel.md`'s Runtime Lifecycle (Evidence Collected precedes Findings Produced) and with the actual, already-reviewed implementation. This predates this proposal — it isn't introduced by it — and should be logged as a documentation defect for a future Documentation Release, in the same spirit as the already-tracked ADR-0002/ADR-0006 defects, rather than silently reinterpreted here.

---

## 3. EvidenceCollector Boundary

**In scope for a Collector:**

- Given an input descriptor, deterministically inspect it and return zero or more Evidence values.
- Assign each Evidence value an accurate `EvidenceCategory`, `description`, and (where applicable) `location`.
- Report its own failure to collect (see Lifecycle, below) through an explicit channel.

**Explicitly out of scope for a Collector:**

- **Evaluating Evidence or producing Findings/Recommendations.** This is the Rule Engine's exclusive responsibility (`RuleEngine.md`: "consumes... Evidence," "produces... Findings, Recommendations"). A Collector that evaluated its own output would collapse the exact producer/consumer boundary Sprint 3 Phase 1 already fought to preserve.
- **Mutating `Assessment` directly.** A Collector returns Evidence values; only the Engine, calling `Assessment::add_evidence`, adds them. Runtime remains the sole mutator (INV-006/007/009).
- **Forming opinions or subjective judgments about what it finds.** `DataModel.md`: "Evidence never represents opinion." A Collector observes; it does not conclude.
- **Knowing about Rules, Knowledge, or Recommendations.** A Collector's dependency should point only at `modiq-runtime` (for `Evidence`/`EvidenceCategory`), the same shape as `modiq-rules`'s own dependency. No awareness of `modiq-rules` or `modiq-knowledge` in either direction.
- **Acquiring the input descriptor itself** (file dialogs, user prompts, upload handling). That is an application-layer concern (Sandbox, CLI) — see Sandbox Integration, below.
- **Persistence of any kind.** No Storage Layer role.
- **Version-specific interpretation** beyond what a future Version Profile might supply as context — not foreclosed, not implemented here.

---

## 4. Collector Lifecycle

- **Invocation.** The Engine invokes one or more Collectors with an input descriptor (and likely Assessment Context) as part of orchestrating an Assessment, before Rule Engine evaluation begins — the same position `RuleEngine::evaluate` already occupies in today's `AssessmentService::execute`, just one step earlier.
- **Return.** A Collector returns a deterministic, ordered set of Evidence values (or an equivalent success result) to its caller. It does not touch `Assessment` itself.
- **Failure representation.** Unlike Rule evaluation — a pure function over already-valid, in-memory Evidence — Collection touches the outside world (missing files, malformed archives, permission errors) and can genuinely fail in ways Rule evaluation structurally cannot. This needs its own explicit, typed failure channel, distinct from "the input legitimately contained nothing to collect" (a valid empty result, not an error — mirroring `RuleEngine::evaluate`'s existing `None`-for-empty-evidence pattern). Whether a collection failure aborts the Assessment outright, is recorded as evidence of its own kind, or surfaces as a distinct caller-visible error is an open design question this proposal raises but does not answer.
- **Determinism preservation.** Given identical input bytes, a Collector must produce an identical Evidence set every time — the same principle already governing Rule Engine output and Runtime identity generation (ADR-0005). This has a concrete implication for later implementation: any concrete Collector must impose a stable, explicit ordering over whatever it traverses (e.g., sorted file paths within a ZIP), since most underlying I/O and archive APIs do not guarantee stable iteration order on their own. That is an implementation detail for later, but the *requirement* belongs in the boundary definition now.

---

## 5. Future Extensibility

The boundary above — input descriptor in, Evidence out, no side responsibilities — is deliberately narrow so that concrete collectors (ZIP traversal, XML inspection, Lua inspection, manifest inspection, dependency inspection, and eventually Version-Profile-aware interpretation) can each be added as independent, additive units of work behind it, without any of them touching `modiq-rules`, `modiq-runtime`, `modiq-report`, or each other. This is the same extensibility posture `Architecture.md` already states as a principle ("platform should evolve through extension rather than modification... introducing additional Rules... implementing Extension interfaces") applied to a second axis (collectors, not rules).

Consistent with the Technical Director's stated principle this session — *a capability should justify an abstraction, not the other way around* — this proposal deliberately does **not** prescribe how multiple concrete collectors would be dispatched internally (a trait? an enum? a fixed sequence inside one function, the same way `RuleEngine::evaluate` currently holds one rule inline?). That question should be answered once a second real collector exists to test it against, exactly as the Technical Director's Rule Engine decision this session established for `modiq-rules`. The boundary only needs to guarantee that answering it later doesn't require moving Evidence, Rule Engine, or Assessment code — and it does guarantee that, because none of those depend on how collection internally dispatches.

Version Profile awareness (`Architecture.md`'s Version Isolation principle) fits this boundary without modification: a future Version Profile can inform *which* collectors run or how they interpret content via Assessment Context, without changing the Collector boundary itself.

---

## 6. Sandbox Integration — re-evaluating the "Sandbox conflict"

The conflict flagged in the prior proposal assumed Evidence Collection would require the Sandbox itself to gain file-loading/ZIP-handling *logic*. Re-examined against the boundary above, that assumption doesn't hold: **the Sandbox does not need to perform Collection — it only needs to supply an input descriptor** (e.g., a filesystem path) that it passes to the Engine, exactly as it already passes nothing today and gets a hardcoded Evidence item back. Concretely:

- Obtaining a path via a native OS file/folder picker (e.g., Tauri's dialog plugin) is a thin IPC-adjacent capability, not business logic — the Sandbox would still perform zero ZIP parsing, zero XML parsing, zero interpretation of the mod's contents. All of that stays inside the new Collection crate, invoked through the Engine, exactly like every other capability the Sandbox already exercises through `AssessmentService::execute`.
- This is a smaller, more precisely-scoped capability than "file loading, ZIP handling" as originally excluded — it is "supplies a path string," not "parses a mod."

This does not eliminate the need for a decision, though: it is still a real, if narrow, expansion of the Sandbox's documented capabilities (`Sandbox Architecture`'s "Current limitations" explicitly lists "no file dialogs" as a deliberate exclusion maintained since Sandbox Phase 1). Recommend treating "should Sandbox gain a native path-picker to supply Collection's input descriptor" as its own small, explicit decision — likely a Level 2/3 change to `Sandbox Architecture`'s documented scope — separate from, and much smaller than, the Level 4 Evidence Collection ownership decision itself.

---

## 7. Public API Impact

Named here for visibility; none of this should be implemented against this proposal:

- A new public boundary type (a `Collector`/`EvidenceCollector` concept, exact shape undecided) in the new crate.
- **`AssessmentService::execute` (or equivalent) gains a new input parameter** — today it takes no subject-location input at all, since the Sandbox constructs Evidence itself ad hoc before calling it. This is a breaking signature change to an existing Engine-facing public API, requiring governance approval under `GOVERNANCE.md`'s Public API Policy.
- A new, distinct error/result type for collection failure, separate from `AssessmentError`/`FindingError`/`RecommendationError`/etc. — shape undecided.
- Possibly new content on `AssessmentSubject` and/or `AssessmentContext` (both currently content-free "marker" types) to carry the input descriptor conceptually — an open modeling question, not decided here.
- Possibly a small Sandbox-facing Tauri command surface change if the path-picker capability (Section 6) is approved separately.

---

## 8. Governance Impact

- **This is a Level 4 (Architectural) change** under `GOVERNANCE.md` ("new crates... ownership changes... major public API redesign" — all three apply). It requires an ADR, governance approval, and a Documentation Release before any implementation begins — not merely a Governance Register entry, and not the lighter Level 3 process GOV-005/GOV-006 went through.
- **A new ADR is needed**, recording: the Evidence Collection ownership decision (new crate vs. alternatives, per Section 1), its boundary (Section 3), its lifecycle contract (Section 4), and its relationship to `Assessment`/Engine/Rule Engine — following the shape already established by ADR-0003 (Assessment Aggregate Root) and ADR-0007 (Runtime Entity Design Pattern).
- **`Architecture.md` likely needs Documentation Release attention**: Evidence Collection does not appear as a named Core Platform Component in the System Overview diagram (only as a lifecycle stage), and that lifecycle stage's position is inconsistent with `DataModel.md` and current implementation (Section 2). Flagging for the next Documentation Release, not resolving here — Architecture.md is Frozen.
- **New Governance Register items to open**, not resolve, here:
  - Evidence Collection crate ownership/naming (formalizing Section 1's recommendation).
  - Collection failure representation (Section 4's open question).
  - Sandbox path-descriptor capability (Section 6) — smaller and likely resolvable independently of the others.
- **`RuntimeInvariants.md` likely needs no new invariant for the Runtime side** — Evidence Collection changes *where Evidence comes from*, not *what Evidence is* or how it's added to an Assessment; INV-002/INV-003 (Evidence may only be added before rule evaluation begins; becomes immutable once evaluation starts) already govern that, unchanged. Worth confirming explicitly in the eventual ADR rather than assuming.
- Worth a cross-reference (not a merge) to GOV-005/GOV-006's still-open referential-integrity follow-up: once Evidence Collection is a second real producer of Evidence (today only the Sandbox's synthetic bootstrap value exists), that open question stops being hypothetical.

---

## Summary for Technical Director Review

| Question | Answer proposed here |
|---|---|
| Owning crate | New crate (working name `modiq-collection`), peer to `modiq-rules`/`modiq-report` |
| Orchestrator | `modiq-engine`, extending the existing `AssessmentService::execute` pattern |
| Collector responsibility | Input descriptor → deterministic Evidence values, nothing else |
| Dispatch mechanism for multiple collectors | Deliberately undecided — to be answered once a second concrete collector exists |
| Sandbox impact | Thin path-descriptor capability only, not file-parsing logic — smaller than originally assumed, but still a real, separately-approvable scope change |
| Governance level | Level 4 — ADR + governance approval + Documentation Release required before implementation |

No implementation is proposed or should follow from this document without separate Technical Director approval of the ADR it would produce.
