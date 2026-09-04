# Capability Definition — C15: Assessment & Report Identity

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md |
| **Project** | modIQ |
| **Type** | Capability Definition — defines the capability only. Not a Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or Implementation Plan. |
| **Origin** | `POST_C14_PRODUCT_CAPABILITY_ASSESSMENT` (chat-delivered analytical artifact, this engineering cycle, unpublished as a repository document) and the subsequent Governance Handoff, which validated its central conclusion without contradiction against current repository state: Assessment/Report Identity is a genuinely forced capability, not yet implementation-ready. |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `60ef3af` (C14 Repository Closeout), working tree clean |
| **Status** | **Draft. Awaiting review and explicit authorization before Process Determination begins.** No implementation, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or ADR is made by this document. |

---

## 1. Capability

**Assessment & Report Identity**: modIQ must be able to meaningfully identify, distinguish, and refer to an Assessment and its resulting Report — as a durable product concept, not merely as an artifact of whichever process happened to produce it.

This is a capability about *what modIQ can say about which Assessment/Report it means*, independent of any particular consumer's use of that meaning. It is not a statement about how identity is stored, computed, or displayed — those are downstream design questions this document does not answer (§7, §12).

---

## 2. Product Need

The Product Definition frames modIQ as an evidence-based assessment and diagnostic platform whose core question is "what can modIQ determine about this mod, what appears wrong, how confident can we be, and what should be done." Answering that question meaningfully — even once, let alone repeatedly across a real usage session — requires a user to be able to say *which* answer they are looking at. Today, the product can produce a real, trustworthy Report, but cannot durably say what that Report is *of*.

---

## 3. User Need

C14 — Operational Assessment Console gave modIQ its first real, continuous, multi-assessment interactive session:

```
Assessment A
  → Review
  → "Assess another mod"
  → Assessment B
  → Review
```

This was exercised for real, twice, in both directions, against two distinct Farming Simulator mods (`FS25_HH_HempProdSystem.zip` and `Hashy_01Silv.zip`), confirmed in `IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`. In that workflow, a real user has a concrete, immediate need to answer:

> **Which assessment/report am I looking at right now, and what mod does it represent?**

Today, the only thing distinguishing Assessment A's Review from Assessment B's Review is `Reviewing.tsx`'s `subjectLabel` prop — the literal filesystem path string the user selected — visible only while that Review is on screen, and gone the moment "Assess another mod" resets it. There is no point past that reset at which a user (or the product) can refer back to "the first one" by anything more durable than their own memory of what they clicked.

---

## 4. Current State

Traced directly against current source; existing mechanisms are described precisely rather than treated as absent.

- **`AssessmentId`, `EvidenceId`, `FindingId`** (`modiq-runtime`) — real, opaque, per-construction identifiers, each freshly assigned within a running process. `ReportKey`'s own doc comment states the operative constraint directly: `AssessmentId` "has no externally accessible representation and is not meaningful across process boundaries" — its generator restarts at 1 on every process invocation. These identifiers are real and correctly serve their own purpose (in-process referential integrity); they were never intended to, and do not, provide durable, human- or cross-process-meaningful identity.
- **`ReportKey`** (`modiq-storage`) — real, durable (persisted to disk, retrievable by key across process invocations via `ReportStore::retrieve`), but opaque: confirmed directly against `report_key.rs`, its value is `"{nanosecond-timestamp}-{sequence}"`, carrying no information about which mod, subject, or user intent produced it.
- **`AssessmentSubject`** (`modiq-runtime`) — a real type, `pub struct AssessmentSubject;`, confirmed directly against `subject.rs`. It carries zero fields today. `AssessmentContext` is identically a zero-field unit struct.
- **`PersistedAssessmentReport`** (`modiq-storage`) — confirmed directly against `persisted_report.rs`: carries `status`, `evidence`, `findings`, `recommendations`. No subject or identity field of any kind is persisted alongside a stored Report.
- **The console's `subjectLabel`** (`apps/console/src/regions/Workspace.tsx`) — a real, presentation-layer string: the literal path handed to the file-selection dialog, passed through unmodified to `Reviewing`. It is not a designed identity concept, is never persisted, and is discarded entirely by C14's own `resetToIntake`.
- **CLI persistence/history** (`modiq-cli`) — `assess` calls `ReportStore::store(&report)` after every successful run; `retrieve` reads a Report back by an already-known `ReportKey`; `history` aggregates `rule_reference`/`severity`/`ModHealthDimension` recurrence across every stored Report. This path is real, tested, and already exercises `modiq-storage`'s full persistence surface — for the CLI.
- **The console currently persists nothing.** Confirmed directly against `apps/console/src-tauri/src/assessment.rs`: no `modiq_storage` import exists in this file; `submit_assessment` never constructs a `ReportStore` or calls `.store()`. Every Assessment run through the actual console product exists only in memory for the life of that one Tauri command invocation.

**The precise gap, stated once, plainly:** modIQ has real identifiers at every layer that needs one for its own internal purpose, and a real, working persistence mechanism — but nothing anywhere in the system associates a Report with a durable, meaningful statement of *which mod* it concerns. This is not "no identity whatsoever"; it is the absence of *subject* identity specifically, at exactly the one place (`AssessmentSubject`) already structurally reserved for it and already found, twice, by prior governance work (§9), to be currently empty.

---

## 5. Desired Capability

Once this capability exists, modIQ should be able to meaningfully answer, for any Assessment or Report it has produced: *what is this an assessment of?* — durably enough that the answer survives past the moment of first display, and consistently enough that two Reports of the same mod can eventually be recognized as related, and two Reports of different mods can eventually be recognized as unrelated, wherever a downstream consumer chooses to make that comparison.

This is stated at the level of what becomes *possible*, not how it is achieved. It does not specify a data shape, a source of truth for the identifying information, a persistence mechanism, or a presentation. Whether the identity is derived from a filename, a declared manifest name, a user-supplied label, some combination, or something not yet considered, is explicitly not decided here (§12).

---

## 6. Capability Scope

- Establishing that Assessment/Report Identity is a real, named product concept modIQ's architecture should account for.
- Framing the central question a future Architecture Evaluation must answer: what constitutes sufficient, meaningful, durable identity for an Assessment/Report Subject.
- Cataloguing, precisely, every identity-adjacent mechanism that already exists (§4), so that future work extends or completes what is already there rather than inventing a parallel mechanism.
- Naming the concrete, evidenced forcing function (§11) and the governance preconditions any resolution must reconcile (§9).

---

## 7. Capability Non-Scope

Explicitly not decided, designed, or authorized by this document:

- Console assessment history, as a UI or as a feature.
- Same-mod trend tracking.
- Cross-mod comparison (C5).
- Any change to `ReportStore`, `ReportKey`, or `modiq-storage`'s persistence implementation.
- Any field, shape, or content added to `AssessmentSubject` or `AssessmentContext`.
- Any UI, navigation, or presentation design.
- Any API, DTO, or transport change.
- Any database or schema change.
- Any architectural decision of any kind — that is Architecture Evaluation's task, not this document's.
- Whether the console should begin persisting Assessments to `modiq-storage` — a plausible downstream consequence, not decided here.

These may become consequences or consumers of this capability once it is realized; none is authorized, scoped, or implied as in-progress by this document.

---

## 8. Acceptance Criteria

Stated at the product/behavioral level, not the implementation level:

- A user completing two consecutive Assessments in one console session can, at the moment each Report is presented, correctly state which mod it concerns — without relying solely on their own short-term memory of the order in which they acted.
- Two Reports produced from the same mod can, in principle, be recognized by the product as concerning the same subject; two Reports from different mods can be recognized as concerning different subjects. ("In principle" — this capability establishes that the product *can* support this recognition; it does not itself build a feature that performs the recognition, such as history or comparison.)
- No existing Assessment behavior, Rule outcome, Evidence content, or Report content is altered by the presence of this capability — identity is additive to what already exists, never a reinterpretation of it.
- The distinction between a *new, independent* Assessment (as C14's "Assess another mod" already produces) and *supplementing* an existing completed Assessment (Initiative 2's own unresolved question) remains intact and unaffected — this capability concerns identifying separate Assessments, not changing how many Assessments exist or how they relate to Runtime's own completion lifecycle.

---

## 9. Dependencies / Preconditions

Recorded as fixed, authoritative, and explicitly **not resolved by this document**:

- **GOV-002** (Runtime Invariant Reconciliation) — Resolved. Its own resolution previously treated `AssessmentSubject`'s zero-field state as requiring no action. Initiative 3's own Architecture Evaluation later found this disposition and Report Identity's own requirement in direct tension, without reopening GOV-002 itself.
- **Initiative 3, Item 6a** — Adopted-but-gated: a Subject-identity field on `AssessmentReport`/`AssessmentSubject` is Adopted in principle, but its own Architectural Resolution states plainly it "shall not be implemented until `AssessmentSubject` carries sufficient content" — a precondition this document does not satisfy and does not attempt to satisfy.
- **Initiative 2** (Reentrant Assessment Lifecycle) — remains unresolved ("Requires Governance Reconciliation"), and concerns exclusively whether one *already-completed* Assessment may later accept supplementary evidence so its *same* Report continues to evolve. This capability concerns identifying *separate, independent* Assessments (exactly the shape C14's "Assess another mod" already produces) and does not touch, imply, or depend on Initiative 2's own question in any respect.
- **Existing identity mechanisms** (§4) — `AssessmentId`/`EvidenceId`/`FindingId`, `ReportKey`, `AssessmentSubject`, `AssessmentContext`, and the console's own `subjectLabel` — are treated as the existing foundation any future resolution must build from or explicitly supersede, not as absent.
- **GOV-008** (AssessmentService Public API Evolution) — remains Open, unaffected. Nothing in this capability's own definition presumes or requires a particular resolution of it, though a future Architecture Evaluation may find the two questions intersect.

---

## 10. Downstream Consumers

Named for context only — **none is authorized, scoped, or implied as in-progress by this document**:

- Console assessment history (retrieval and review of previously completed Assessments).
- Same-mod trend tracking across repeated Assessments of one mod over time.
- Cross-mod comparison (C5).
- Richer Report presentation that surfaces subject identity directly (e.g., a Report header naming the mod it concerns).

Each of these would consume whatever this capability eventually establishes; none is a precondition for this capability to exist, and this capability's own completion does not obligate building any of them next.

---

## 11. Forcing Function

Grounded in C14's own real, verified operational evidence, not generic future vision:

`IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` §5 records a real, non-mock acceptance sequence — Hemp `.zip` → Assess another mod → fresh Intake → Hashy → Hashy Reviewing, and the reverse direction — the first time in this repository's history a human needed to distinguish two real, completed Assessments within one continuous product session, rather than across separate CLI process invocations (where nothing survives to need distinguishing) or within a unit test asserting identifier inequality (a determinism check with no user-facing stakes). This is the concrete instance that makes Assessment/Report Identity a demonstrated need rather than an architecturally interesting absence — the distinction the Post-C14 assessment and its governance handoff both drew explicitly, and this document does not weaken.

---

## 12. Open Questions

Explicitly identified as unresolved, and explicitly **not answered here** — reserved for Process Determination and, if it concludes one is required, Architecture Evaluation:

- What constitutes meaningful Assessment Subject identity — sourced from what, and containing what, in principle?
- What relationship should exist between Assessment identity and Report identity — are they the same concept at two lifecycle stages, or genuinely distinct?
- How should any new identity concept relate to `AssessmentId` (Runtime, process-local) and `ReportKey` (Storage, durable but opaque) — extend one of them, bridge them, or introduce something that relates to both without replacing either?
- What identity information, if any, is meaningfully durable and meaningful *across* a process boundary, given `AssessmentId`'s own confirmed non-durability?
- How should any resolution reconcile with GOV-002's prior disposition on `AssessmentSubject`?
- How should any resolution satisfy, or explicitly revisit, Initiative 3 Item 6a's own gating condition?

---

## 13. Explicit Boundary Notes

- **Identity is not merely a display label.** A capability satisfied by "show the filename somewhere" would not meet the standard this document sets — durable reference across the reset that currently discards `subjectLabel` is part of what "meaningful" requires (§5, §8). At the same time, this document does not conclude that satisfying that standard requires building persistence or history as part of this capability — whether identity is realized independently of, prior to, or jointly with a persistence change is exactly the kind of design question reserved for later stages (§12), not decided by asserting a scope boundary here.
- **This document does not misrepresent unrelated capabilities as related.** Savegame evidence, User Observation evidence, diagnostic workflow progression (Investigate/Diagnose/Remediate), Confidence, Evidence Explanation, and Knowledge-layer activation are not described as existing, in progress, or as dependencies of this capability — none is implicated by Assessment/Report Identity except where explicitly named above (§9, §10).

---

## 14. Next Required Repository Artifact

A **Process Determination for C15**, applying this repository's own established two-prong test (is there a genuinely open architectural design question or unsolved principle; are materially different architectural approaches being weighed) to the open questions named in §12 — not assumed to resolve in favor of Architecture Evaluation merely because this document expects it will, and not performed by this document.

---

## Status

**Draft. Awaiting review and explicit approval.** No Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or implementation has been performed. No file other than this one is created, modified, or implied as changed by this document. GOV-002, Initiative 2, Initiative 3 Item 6a, GOV-008, and every other named governance item remain exactly as they were before this document — none is resolved, reinterpreted, or amended here.
