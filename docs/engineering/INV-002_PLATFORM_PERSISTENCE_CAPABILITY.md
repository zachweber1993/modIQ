# INV-002 — Platform Persistence Capability Investigation

| Property | Value |
|---|---|
| **Document** | INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md |
| **Project** | modIQ |
| **Purpose** | Determine whether the repository's documented persistence gap constitutes a new capability requiring formal introduction into the platform architecture — the mandatory precondition to any future Capability Definition or Architecture Evaluation, per this project's own evidence-before-architecture discipline |
| **Origin** | Directed by the Chief Architect, following the Capability Identity governance clarification (commit `77e3adc`) and a discussion of next-highest-value platform investments. Unlike INV-001, this investigation was **not** preceded by a committed capability proposal document (no "CAP-002" exists) — the Observation and Evidence phases below are this candidate's first committed repository artifact. |
| **Status** | **Complete. See "Investigation Status," below, for the full explicit statement. Capability Definition has NOT been authorized. Architecture Evaluation has NOT been authorized. Sprint Planning has NOT begun. No crate, API, database, or serialization format is proposed anywhere in this document.** |
| **Governing Follow-up** | None opened directly by this document. A candidate governance question is identified in Section 4 and left for Chief Architect decision — not opened unilaterally, consistent with GOV-014's own precedent. |

---

# Investigation Status

Recorded explicitly, per this project's standing discipline of stating a decision to wait as a decision, not an absence of one:

- ✅ **Observation completed** (Section 1) — repository state only, no proposed solution.
- ✅ **Evidence collected** (Section 2) — repository sources only, facts separated from interpretation.
- ✅ **Capability Identity evaluated** (Section 3) — against all three axes and the Introduction test, without presupposing the outcome.
- ✅ **Recommendation produced** (Section 4).
- ⛔ **Capability Definition has NOT been authorized.**
- ⛔ **Architecture Evaluation has NOT been authorized.**
- ⛔ **Sprint Planning has NOT begun.**
- ⛔ **No crate boundary, API, database, or serialization format is designed or recommended anywhere in this document.**

This investigation produced no Rust source, no test, no fixture, no ADR, and no Governance Register item. Section 4 identifies a candidate governance question but does not open it — opening a Governance Register item is a Chief Architect action, not an outcome this investigation performs on its own authority.

---

## 1. Observation

Repository state only. No solution, technology, crate, or storage mechanism is proposed in this section.

### 1.1 What is lost when an Assessment completes

An `Assessment` (`modiq-runtime`) moves through a sequential lifecycle enforced by `RuntimeInvariants.md`: `INV-010` (transitions occur sequentially), `INV-011`, and `INV-012` ("A Completed Assessment SHALL reject further lifecycle transitions"). Completion is a terminal state — the invariants define what a Completed Assessment must reject, not what happens to it afterward, because nothing happens to it afterward. The object exists only in the memory of the process that created it.

Confirmed directly against the two real consumers of `AssessmentService`:
- `modiq-cli`'s `AssessCommand::run` (`crates/modiq-cli/src/commands/assess.rs`) returns `(String, ExitCode)` — a formatted report string handed back to the caller for printing. No file is written in any non-test code path.
- `apps/sandbox/src-tauri/src/` contains exactly `lib.rs` and `main.rs` — no local storage, cache, or database file of any kind exists in the sandbox application's source.

The only file-system writes found anywhere in the workspace (`modiq-engine/src/engine/assessment_service.rs`, `modiq-cli/src/commands/assess.rs`) are inside `#[cfg(test)]` modules, constructing temporary `.zip` fixtures as **test input** — not writing an Assessment or Report as **output**. Nothing in the current implementation persists an `Assessment`, its `Evidence`, its `Finding`s, its `Recommendation`s, or its generated `AssessmentReport` past the lifetime of the process that produced them.

### 1.2 Documented workflows that presuppose persistence

`ProductSpecification.md`'s own "Assessment Workflow" (§ Assessment Workflow) lists seven steps, the last of which is: **"Knowledge is incorporated into the MKB where appropriate."** This step has no implemented counterpart today — nothing in the current pipeline takes a completed Assessment's output and feeds it into `modiq-knowledge`.

The same document's "modIQ Knowledge Base (MKB)" section states the MKB "stores... Historical Assessments" as one of six named content types (alongside Known Issues, Repair Recipes, Compatibility History, Dependency Relationships, Engineering Patterns). "Historical Assessments" names a concept — a durable record of past Assessments — that has no implementation counterpart anywhere in the repository today.

The **Server Administrators** target user's stated objectives include "Validate complete mod collections" and "Identify conflicting mods" (`ProductSpecification.md`, Target Users) — objectives that describe evaluating a set of mods together, which the current architecture does not describe any mechanism for beyond running one ephemeral Assessment per mod.

### 1.3 Architectural documents that already describe Storage/Persistence

Storage is not a hypothetical addition — it is already named, structurally, in the platform's own authoritative architecture:

- `Architecture.md`'s System Overview diagram depicts `Assessment Service` delegating to six subsystems: Evidence Collection, Rule Engine, Version Profiles, Knowledge Base, Reporting, and **Storage** — Storage has occupied this diagram since the document's own baseline.
- `Architecture.md` has a dedicated "Storage Layer" section: "The Storage Layer provides persistence services," with named responsibilities — "assessment persistence, knowledge storage, configuration, historical records, cached resources" — and the explicit statement "The architecture intentionally abstracts storage implementation."
- `Architecture.md`'s "Platform Boundaries" section separately lists "Persistence" as one of six things the architecture "intentionally separates" from the others (Assessment execution, Engineering knowledge, Version-specific behavior, Reporting, Persistence, User interfaces).
- ADR-0010 (Engine Orchestration Simplification) cites `Architecture.md`'s System Overview — "Evidence Collection, Rule Engine, Knowledge Base, Version Profiles, Reporting, and Storage as subsystems" — as authoritative evidence for its own reasoning, confirming Storage's standing as an equal, named peer to the other five subsystems, not an afterthought.
- Three separate specifications (`DataModel.md`, `RuleEngine.md`, `KnowledgeModel.md`) each explicitly exclude "storage technologies" / "persistence models" from their own scope, consistently deferring the concern to "their respective specifications" (`DataModel.md`) rather than disputing that it belongs somewhere.

### 1.4 Governance items and their actual relationship to persistence

Only one open Governance Register item mentions related territory, and the relationship is narrower than it may first appear:

- **GOV-001 (Assessment Report Generation Timing)** — "Assessment reports are currently generated before Assessment completion... Should reports represent the Assessment immediately before completion or after completion?" This is a question about **when**, within the lifecycle, a report snapshot is taken — it is well-defined and fully resolvable whether or not Assessments are ever persisted. It is adjacent to persistence (a persisted-report design would need to know which snapshot to store) but is not, on its own text, a persistence question. Stated honestly rather than inferred: no Governance Register item currently asks whether or how an Assessment should be persisted at all.
- No other open item (GOV-002, GOV-003, GOV-008, GOV-013, GOV-014) references storage, persistence, or historical records in its own text.

### 1.5 Architectural limitations arising from process-locality

- No Assessment can be compared against a prior Assessment of the same mod (e.g., "did this mod's compatibility improve after the creator's last update?") — every run starts from zero history.
- No Assessment can inform a later, different Assessment's Knowledge Base content — `ProductSpecification.md`'s own Assessment Workflow step 7 is specified and unreachable.
- No mechanism exists for the Server Administrator's "complete mod collections" objective beyond invoking the CLI or sandbox once per mod and manually retaining each printed report outside the platform.
- `modiq-knowledge`'s only real content (`RepairRecipe::version_compatibility_declared_version_mismatch()`, Sprint 9) was authored directly by an engineer, not derived from any Assessment the platform ever ran — consistent with there being no path for Assessment output to reach the Knowledge Base at all.

---

## 2. Evidence

Repository sources only, with facts kept separate from interpretation — the same discipline INV-001 §3 applied to Lua.

### Product requirements (fact — direct quotation)

- `Vision.md`: "We believe that preserving knowledge is as important as generating it." Also: "Historical knowledge preservation" is listed as one of the platform's core capabilities, alongside Automated assessment, Evidence-based analysis, the MAF, the MKB, Explainable Assessment Reports, and Structured recommendations.
- `Principles.md`, "Knowledge Preservation": "Validated knowledge should accumulate over time. As new engine behaviors, best practices, and implementation patterns are discovered, they should strengthen the modIQ Knowledge Base **rather than remaining isolated within individual assessments**." (emphasis reflects the document's own point, not added interpretation)
- `ProductSpecification.md`: MKB "stores... Historical Assessments"; Assessment Workflow step 7, "Knowledge is incorporated into the MKB where appropriate."

### Existing architectural commitments (fact — direct quotation)

- `Architecture.md` names Storage as a peer subsystem to Evidence Collection, Rule Engine, Version Profiles, Knowledge Base, and Reporting, with its own dedicated section and named responsibilities (§1.3, above).
- ADR-0010 treats Storage's presence in the System Overview as settled, citable evidence, not an open question.

### Documented user workflows (fact — direct quotation, with interpretation flagged separately)

- **Fact:** Server Administrators' stated objectives include "Validate complete mod collections" and "Identify conflicting mods."
- **Interpretation, flagged as such:** these objectives are *suggestive* of a need to track more than one mod's Assessment at a time, but the current wording does not itself specify whether that requires durable storage versus a single multi-input Assessment run. This investigation does not resolve which is true — see Section 4's evidence gap.

### Governance implications (fact)

- GOV-001 is open and adjacent, but — stated plainly, not glossed over — its own text is about report-generation timing, not about whether reports or Assessments are persisted. Treating GOV-001 as "the persistence governance item" would overstate what it actually says.
- No Governance Register item currently names persistence as its own subject.

### Current implementation state (fact, verified directly against source)

- No `Storage` crate exists in the nine-crate workspace.
- No persistence-capable dependency (a database, embedded key-value store, or ORM) exists in any `Cargo.toml` in the repository. `serde`/`serde_json` are present but used for existing interop, not for writing Assessment state anywhere in non-test code.
- `INV-012` makes `Completed` a terminal Assessment state with no further transition — confirmed directly in `RuntimeInvariants.md`.
- `apps/sandbox`, "the only real end-to-end consumer of the platform" (Project Handoff §2), has no local storage of any kind in its own source.

### Interpretation, stated as such and not blended with the above

The four architectural documents in §1.3 do not merely permit persistence — they already assume it will exist, structurally, as a peer to the five subsystems that do. This is different in kind from a capability like Lua Analysis, which has no existing architectural placeholder anywhere and would need one invented from nothing. Storage's placeholder already exists; only its content does not.

---

## 3. Capability Identity Evaluation

Per the Chief Architect's explicit instruction, this section does not presuppose Capability Introduction. Each criterion is walked through on its own terms, and the procedure is allowed to produce whatever answer the evidence supports — including "this procedure does not cleanly classify this candidate at all," which is the finding this section arrives at.

### A scope observation, checked before applying the axes

`SPRINT12_ARCHITECTURAL_RESOLUTION.md` derives the three axes and the Introduction test **exclusively from Collector/Rule historical decisions** (Sprint 3, 4, 4 Phase 3C, 7, 8, 9, 11) — every one of the seven concerns either a Collector producing a new kind of `Evidence` or a Rule making a new kind of judgment over it. `SPRINT12.md`'s own list of future candidates the procedure was built to anticipate — "a second runtime-log signature, Lua analysis, asset validation, dependency resolution, performance observations" — is named twice in that document, in nearly identical wording, and **never once includes Storage or persistence.** This is direct evidence that the procedure was not derived with a non-Collector/Rule subsystem in mind.

### Collection Axis

"Does producing the new fact require an inspection mechanism no existing Collector already uses?" — **This question does not apply to the candidate as stated.** Persistence does not produce a fact about an Assessment Subject's content at all; it is not an inspection activity. There is no "fact" here for a Collector to produce — the candidate operates on already-produced Runtime entities (`Assessment`, `Evidence`, `Finding`, `Recommendation`, `AssessmentReport`), after Evidence Collection and Rule Evaluation have already finished.

### Evidence Axis

"Does the resulting fact represent a kind of observation no existing `EvidenceCategory` covers?" — **Not applicable.** Persistence produces no `Evidence` and activates no `EvidenceCategory`. Nothing about "should an Assessment be stored" is an observation about a mod.

### Interpretation Axis

"Does interpreting the fact require a judgment no existing Rule already makes?" — **Not applicable.** Persistence makes no judgment about a mod's engineering quality; it produces no `Finding` and no `Recommendation`. There is no Rule-shaped question here to classify.

### Capability Introduction Test

"Does this require a new way for Collectors or Rules to relate to each other or to the engine, or only another instance of a relationship already proven?" — **The test's own wording is Collector/Rule-specific and does not name the kind of relationship persistence would introduce** (a durable record surviving process termination, consumed by a later, separate process invocation). Answering this test as literally worded produces no verdict, because the relationship in question is not a Collector-to-Collector or Rule-to-engine relationship at all.

### The closer historical precedent, found by looking past the three axes

Two prior capabilities match this candidate's actual shape far more closely than any Collector/Rule decision does: **Version Profiles** (`modiq-versioning`, dormant from Sprint 0, given its first real content — `VersionProfile::fs25()` — at Sprint 8) and **Knowledge Base** (`modiq-knowledge`, dormant from Sprint 0, given its first real content — `RepairRecipe` — at Sprint 9). Both are, like Storage, subsystems named in `Architecture.md`'s own System Overview from the beginning, left deliberately unimplemented for lack of a forcing function, and later activated with deliberately minimum-viable real content.

**This precedent already has a formal name in the repository.** `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 ("Cross-Cutting Assessment") explicitly named and applied **Architectural Activation** as a distinct classification, defined against three siblings — Capability implementation, Infrastructure expansion, and Platform evolution — and applied it to Version Profiles with this exact reasoning: *"This Sprint is the first to realize an already-specified-but-dormant architectural dimension, not to invent a new one or merely extend an already-live one."* Sprint 9 (`SPRINT9_ARCHITECTURAL_RESOLUTION.md`, `SPRINT9_CAPABILITY_DEFINITION.md`) followed the identical shape for Knowledge Base in plain language ("the same shape Sprint 8 proved out for `modiq-versioning`"), without re-citing the formal term. **Neither activation was run through a three-axis Capability Identity Classification** — that procedure did not exist until Sprint 12 — but both were run through this earlier, sibling classification instead. Storage's own candidate shape is a third instance of the same pattern, not an unclassified one.

### Conclusion of this section

The Capability Identity procedure, applied honestly rather than by analogy, **does not produce a classification for this candidate** — not because the candidate is too novel, and not because no repository precedent exists for it, but because it is not the kind of candidate that procedure was built to classify. Storage's activation is shaped like Version Profiles' and Knowledge Base's own prior activations — a dormant, already-specified subsystem receiving its first real content — not like a new Collector or Rule. Forcing an Enrichment/Expansion/Introduction verdict onto it would be answering a question the evidence does not support answering, the same failure mode this project's own adversarial-verification discipline exists to catch.

**Corrected finding:** this is not an absence-of-precedent gap. The repository already contains a named classification for exactly this shape of work — Architectural Activation (Sprint 8) — and Storage fits it the same way Version Profiles and Knowledge Base did. What this investigation has actually identified is narrower and more precise: **Sprint 12's Capability Identity procedure was derived and validated exclusively against Collector/Rule decisions (above) and was never reconciled against Sprint 8's own sibling classification**, even though Sprint 8 predates Sprint 12 and was available evidence at the time of its derivation. The open question this candidate surfaces is a governance reconciliation question — how two already-existing, differently-scoped taxonomies relate to each other — not a question of whether any applicable historical precedent exists at all.

---

## 4. Recommendation

**Does this Observation represent a valid architectural capability candidate?** Yes. The evidence in Sections 1–2 is concrete and repeated across independent sources: `Vision.md`'s own founding beliefs, `Principles.md`'s own named "Knowledge Preservation" principle, and `ProductSpecification.md`'s own Assessment Workflow and MKB content list all describe a persistence-dependent capability that has never been built. This is not a speculative or invented need — it is a documented specification gap, the same standing this project already granted Version Profiles and Knowledge Base before their own Sprint 8/9 activation.

**What architectural problem is actually being introduced?** Based on Section 3, it is not "does modIQ need a new Collector or Rule" — it is **"how does the already-specified, currently-dormant Storage Layer receive its first real, minimum-viable content,"** structurally the same question Sprint 8 and Sprint 9 each already answered once for a different dormant subsystem. That reframing matters: it changes what kind of Chief Architect review this candidate needs next.

**Should the repository proceed to Capability Definition?** Not yet — and, corrected from this investigation's own earlier reading, not because no applicable classification exists. A **governance reconciliation question** stands in the way, surfaced directly by Section 3: `PROJECT_HANDOFF_v1.1.md` §5 states a new capability "does not proceed to Capability Definition until it has been classified through the Capability Identity procedure." Storage cannot be classified by *that specific* procedure (Section 3) — but it is already classifiable under Architectural Activation, the repository's own Sprint 8 precedent for exactly this shape of work. The open question is therefore not "which reading of an ambiguous rule applies," but a narrower, factual one: **how are Sprint 8's Architectural Activation classification and Sprint 12's Capability Identity procedure intended to coexist**, given the latter never referenced or reconciled against the former. Only the Chief Architect can resolve that.

**Are any governance questions discovered that should be opened before Architecture Evaluation?** One, recommended for the Chief Architect's consideration rather than opened here — narrower than this investigation originally framed it: **how Sprint 8's Architectural Activation classification and Sprint 12's Capability Identity procedure are intended to coexist within the repository's governance model.** This is not specific to Storage — Extension Layer sits in the identical dormant state today, and any future subsystem-level activation would face the same reconciliation question — but it is a narrower question than "does any governing concept exist," since one already does.

**A second, separate evidence gap, disclosed rather than resolved:** Section 2 flagged that Server Administrators' "validate complete mod collections" objective is only *suggestive* of a persistence need, not conclusive — it is equally consistent with a single multi-mod Assessment run that never persists anything. This investigation cannot determine, from repository evidence alone, which specific product problem (single-mod history, cross-mod collection validation, or MKB accumulation from real Assessments) is the actual forcing function, and each would plausibly shape a different eventual design. Resolving this is Capability Definition's own job, once the procedural question above is settled — not something to be guessed at here.

**Summary recommendation:** This candidate is real and well-evidenced, and — corrected from this investigation's own earlier reading — is not without historical classification; it fits Architectural Activation, the repository's own Sprint 8 precedent. It remains blocked on a governance reconciliation question, not a missing-evidence question, unlike INV-001. No Capability Definition, Architecture Evaluation, or Sprint Planning should begin until the Chief Architect resolves how Sprint 8's Architectural Activation classification and Sprint 12's Capability Identity procedure are intended to coexist, and, separately, which of the several plausible persistence problems this platform intends to solve first.

---

# Document Status

**Current Version:** 1.1.0 (amended 2026-07-23 — Sections 3 and 4 revised to incorporate `SPRINT8_ARCHITECTURAL_RESOLUTION.md`'s own "Architectural Activation" classification, discovered after this investigation's original drafting. No evidence, finding, or conclusion regarding persistence, the Recommendation, implementation, Architecture Evaluation, or Sprint Planning was altered — only the diagnosis of what kind of gap blocks Capability Definition was corrected, from "no applicable classification exists" to "a governance reconciliation question exists between two already-existing taxonomies.")

**Status:** Complete. Committed to the repository as the permanent record of this investigation. Capability Definition and Architecture Evaluation both remain unauthorized pending Chief Architect resolution of the reconciliation question named in Section 4. No Sprint Planning has begun. No crate, API, database, or serialization technology has been proposed, named, or implied as a recommendation anywhere in this document.
