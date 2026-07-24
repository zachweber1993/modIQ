# GOV-015 Architecture Evaluation — Role of ADR-0003 in Describing AssessmentReport Ownership

| Property | Value |
|---|---|
| **Document** | GOV015_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Governance item addressed** | GOV-015 (`docs/engineering/GOVERNANCE.md`) — Open since Sprint 16 (Governance Initiation) |
| **Purpose** | Evaluate whether ADR-0003's description of `AssessmentReport` ownership still accurately reflects the platform's architecture, and whether governance action is required |
| **Origin** | Chief Architect authorization, Sprint 17 |
| **Status** | **Architecture Evaluation complete; Architectural Resolution complete (Section 6) — GOV-015 Resolved. No ADR created, no ADR proposal created, no implementation authorized.** |

---

## 1. Architectural Question

Restated exactly from GOV-015's Question field in `GOVERNANCE.md`:

> Does ADR-0003's inclusion of `AssessmentReport` as an Assessment-owned entity still accurately describe the platform's architecture? If not, is the correct reconciliation an in-place ADR-0003 amendment (recorded explicitly, per the precedent already established for `DataModel.md`'s Sprint 5 amendment and `EvidenceCollection.md`'s four amendments) or a new ADR that supersedes ADR-0003's specific claim without disturbing its still-valid core decision (`Assessment` as the Runtime domain's sole aggregate root, which this item does not question)? Does the correction additionally require reconciling `PROJECT_HANDOFF_v1.1.md` or any other living document found to repeat ADR-0003's original framing?

**A note before proceeding, not a restatement of the question:** this question's own text, as recorded, asserts a specific precedent for the "in-place amendment" option — a claim this evaluation checked directly rather than accepted at face value. Section 4 reports what that check found.

---

## 2. Evidence Reviewed

Every artifact this Sprint's own scope named, plus source verification performed directly:

- **`docs/engineering/GOVERNANCE.md`** — GOV-015's own entry (Title, Status, Raised, Description, Question, Resolution); the Reporting Crate Boundary Rule; the full Governance Register (confirming no other item bears on this question); the Documentation Authority ordering (Vision > Product Specification > Architecture > Data Model > Principles > **ADRs** > **Governance** > Engineering Release Documents > Engineering Log > Source Code).
- **`docs/engineering/PROPOSAL_GOV-015.md`** — the approved Governance Initiation proposal, re-read in full to identify which of its own claims this evaluation should verify rather than assume.
- **`docs/adrs/0003-assessment-aggregate-root.md`** — read in full, including its Date field (2026-07-16) and its full Decision, Consequences, and Status sections.
- **`docs/adrs/0007-runtime-entity-design-pattern.md`** — re-read in full, specifically its "Aggregate Root Pattern," "Entity Identity," and "Constructor Validation" sections, which define, precisely and technically, what "ownership" means for a Runtime entity in this repository.
- **`docs/adrs/README.md`** — read in full. Its "Engineering Notes" section states repository policy directly: *"Accepted ADRs should not be modified to reflect new decisions. If an architectural decision changes, a new ADR should be created that supersedes or amends the previous decision while preserving the historical record."* It also records, as a live example of this policy being applied, that ADR-0002's own known internal numbering defect ("ADR-0002" mistitles itself "ADR-0001" in its own body text) has been deliberately left uncorrected, "since accepted ADRs are not to be modified per this document's own Engineering Notes."
- **`docs/architecture/Architecture.md`** — the System Overview diagram; the Assessment Lifecycle diagram; the Platform Boundaries section ("the architecture intentionally separates... Assessment execution... Reporting... Persistence").
- **`docs/architecture/DataModel.md`** — re-read in full, specifically the Domain Overview ("Supporting entities provide context, evidence, conclusions, recommendations, **and reporting**"), the Assessment Report section, and the Entity Relationships diagram ("Assessment ... generates one Assessment Report").
- **`docs/engineering/GOV002_ARCHITECTURE_EVALUATION.md`** — Finding 3 and §7 Option C, the origin of GOV-015 — re-read specifically to identify which of its claims were verified at the time versus asserted by inference.
- **`docs/governance/CHANGELOG.md`** — Sprint 1's own entry ("Implemented immutable Assessment Report snapshot generation (`modiq-report::AssessmentReport`)") and `docs/implementation/CrateRoadmap.md`'s own revision history (1.0.0–1.4.0, dated 2026-07-16 through 2026-07-18), checked specifically to establish *when* `modiq-report` became a distinct crate relative to ADR-0003's own date.
- **`crates/modiq-runtime/src/assessment/assessment.rs`, `assessment_status.rs`, `evidence.rs`, `finding.rs`, `recommendation.rs`, `subject.rs`, `context.rs`** — re-confirmed directly: no field, method, or dependency in `Assessment` (or any type it owns) touches `AssessmentReport` or `modiq-report`.
- **`crates/modiq-engine/src/engine/assessment_service.rs`** — re-confirmed the exact call site: `AssessmentReport::generate(&assessment)`, external to `Assessment` itself.
- **`crates/modiq-report/README.md`** — read for completeness; found to still describe four scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) retired at Sprint 6. This is a separate, pre-existing staleness, unrelated to GOV-015's own question and not further investigated here — noted only so it is not mistaken for evidence this evaluation relied on.

---

## 3. Findings

### 3.1 Does ADR-0003 accurately describe the current architecture?

**Partially.** Of ADR-0003's six named Assessment-owned entities, five are fully accurate, confirmed directly: `Assessment Context`, `Assessment Subject`, `Evidence`, `Findings`, and `Recommendations` are all held as fields inside `Assessment`, and the latter three are mutated only through `Assessment`'s own methods, exactly as ADR-0003 and ADR-0007 both describe. The sixth, `Assessment Report`, is not: `Assessment` holds no field, method, or dependency referencing it anywhere; it is generated externally, by a different crate, from `Assessment`'s already-public getters. **ADR-0003 does not accurately describe `AssessmentReport`'s current relationship to `Assessment`.**

### 3.2 Has `AssessmentReport` ownership changed?

**This is where this evaluation departs from the prior evaluation's own framing, and the departure matters.** `GOV002_ARCHITECTURE_EVALUATION.md`'s Finding 3 characterized this as ADR-0003 having "reflected the platform's single-crate-era design at the time ADR-0003 was written," implying the architecture *changed after* ADR-0003 and left it stale. Checked directly against the repository's own timeline, **this is not what happened:**

- ADR-0003 is dated **2026-07-16**.
- `docs/implementation/CrateRoadmap.md`'s own revision history records Sprint 0's completion — "all workspace crates scaffolded" — at **1.1.0, also 2026-07-16**, and `modiq-report` advancing with real `AssessmentReport` generation content at **1.3.0, 2026-07-18**, two days later.
- `docs/governance/CHANGELOG.md`'s own Sprint 1 entry independently confirms this: "Implemented immutable Assessment Report snapshot generation (`modiq-report::AssessmentReport`)" — naming the crate-qualified type directly, at Sprint 1, essentially concurrent with ADR-0003's own date.

**There was no single-crate era.** `modiq-report` existed as its own crate, separate from `modiq-runtime`, from the platform's original nine-crate scaffold — the same day ADR-0003 was accepted, not after it. `AssessmentReport` ownership has not "changed" since ADR-0003 in the sense of the architecture evolving away from what the ADR once correctly described. **The imprecision was present when ADR-0003 was written.**

### 3.3 Is the discrepancy architectural, historical, or editorial?

Given Finding 3.2, this is better characterized as **editorial imprecision in ADR-0003's own drafting, with a narrow architectural consequence** — not "historical drift," and not a defect requiring an architectural redesign.

The likely mechanism, checked directly: ADR-0003 (2026-07-16) used "owned" as a loose, example-driven term for six things associated with an Assessment's execution, without a precise technical definition — none existed yet. **ADR-0007 (Sprint 2, "authoritative as of Engineering Release 0.2") established that precise definition one ADR later**: a Runtime entity is "owned" when it receives a stable identity generated by a single consistent mechanism, is mutated only through `Assessment`'s own aggregate methods, and never mutates itself or exists independently. By this later, more precise standard, `AssessmentReport` plainly does not qualify — it has no identity assigned by `Assessment`, no aggregate-mutation path, and is not itself a Runtime entity construction ADR-0007's pattern was written to describe. **ADR-0003's own example list was never checked against ADR-0007's own more precise definition, established one ADR after it.** This is the actual gap: not architecture changing out from under documentation, but a later, more precise internal standard never being back-applied to an earlier, looser list.

This still has genuine architectural weight, not merely cosmetic: ADR-0003 is a formally Accepted decision record, actively cited (Finding 3.4), and the imprecision concerns a boundary question (what does "Runtime-owned" mean, and does `AssessmentReport` cross it) that this project's own Reporting Crate Boundary Rule and Platform Boundaries section both already answer correctly. The correction is narrow, but it is a correction to Accepted architecture, not a typo.

### 3.4 Does `PROJECT_HANDOFF_v1.1.md` repeat ADR-0003, or independently establish the same claim?

**Neither, precisely — a distinction worth being exact about.** `PROJECT_HANDOFF_v1.1.md`'s current text reads: *"Runtime Domain (`DataModel.md`) — information that exists because an Assessment is executed: `Assessment` (the sole aggregate root, ADR-0003), `Evidence`, `Finding`, `Recommendation`, `AssessmentReport`. Fully real, fully implemented."* Read precisely, this sentence attributes "sole aggregate root" to ADR-0003 specifically — it does not attribute "`AssessmentReport` is Assessment-owned" to ADR-0003. That broader claim (`AssessmentReport` belongs conceptually to the Runtime Domain) traces instead to **`DataModel.md`'s own Domain Overview and Entity Relationships diagram**, which independently states Assessment "generates one Assessment Report" and lists reporting among the runtime domain's "supporting entities." `DataModel.md` outranks both `GOVERNANCE.md` and ADR-0003 in the Documentation Authority ordering (Finding, Section 2).

This matters directly for scope: `DataModel.md`'s own conceptual claim — that Assessment Report belongs to the runtime domain as something Assessment's execution *generates* — is not obviously wrong or in conflict with the Reporting Crate Boundary Rule. "Generates" and "is implemented in a separate crate from" are not incompatible statements, the same way Evidence Collection *produces* Evidence for an Assessment without `modiq-collection`'s code living inside `modiq-runtime`. **The specific word that creates the actual conflict is ADR-0003's "owned," not `DataModel.md`'s "generates."** This narrows the likely correction considerably: `DataModel.md` and `PROJECT_HANDOFF_v1.1.md`'s conceptual framing do not, on this evidence, need to change — only ADR-0003's specific ownership example list does.

---

## 4. Architectural Options

Presented without a recommendation, per this Sprint's own scope.

**Option A — Leave ADR-0003 unchanged.** Its Consequences section and Status remain historically accurate to what was decided; a reader who understands ADR-0007's later, more precise definition can already reconcile the two documents unassisted. *Disadvantage:* the imprecision is not hypothetical — it has already propagated into `PROJECT_HANDOFF_v1.1.md`'s current text (Finding 3.4) and misled this project's own prior evaluation (`GOV002_ARCHITECTURE_EVALUATION.md`'s "single-crate-era" characterization, corrected in Finding 3.2 above) into a materially incorrect historical narrative. Leaving it unchanged does not stop that propagation.

**Option B — Amend ADR-0003 in place.** This is the option `PROPOSAL_GOV-015.md`'s own Question text named as precedented, citing `DataModel.md`'s Sprint 5 amendment and `EvidenceCollection.md`'s four amendments. **This evaluation found that precedent does not transfer.** Those two documents are Architecture-tier technical specifications, governed by `DocumentationRelease.md`'s own explicit in-place-amendment allowance. ADRs are governed by a different, separately stated policy (`docs/adrs/README.md`, Engineering Notes): *"Accepted ADRs should not be modified to reflect new decisions."* That policy has already been applied once, directly on point — ADR-0002's own acknowledged internal numbering defect was deliberately left uncorrected specifically because "accepted ADRs are not to be modified per this document's own Engineering Notes." Amending ADR-0003 in place would be the first departure from that stated policy in this repository's history, not an application of an existing precedent.

**Option C — Create a new ADR that supersedes ADR-0003's specific claim.** This is the mechanism `docs/adrs/README.md` itself explicitly prescribes: *"If an architectural decision changes, a new ADR should be created that supersedes or amends the previous decision while preserving the historical record."* No ADR has yet been superseded in this repository's history — this would be a first — but it is the *documented* mechanism, not an undocumented one, unlike Option B. It would need to preserve ADR-0003's still-valid core decision (`Assessment` as aggregate root, Evidence/Findings/Recommendations/Context/Subject genuinely owned) while correcting only the `AssessmentReport` example — a narrow scope for a new ADR, proportionate to a narrow finding.

**Option D — Resolve at the Governance Register level only, without any ADR-level document change.** GOV-015's own resolution could simply state the correct interpretation (`AssessmentReport` is generated by, not owned by, `Assessment`) without touching `docs/adrs/` at all. *This option conflicts with the Documentation Authority ordering itself (Finding, Section 2):* ADRs rank above Governance in Normative Authority. A Governance Register resolution alone would be a lower-authority document attempting to override a higher-authority one's specific claim — the same "silent reinterpretation" this project's own standing discipline (`CHIEF_ARCHITECT_HANDOFF_v1.1.md` §3) holds should never happen to an ADR. This option is available mechanically but appears to conflict with a principle this repository has stated about itself.

---

## 5. Preliminary Conclusion

The evidence establishes, with direct confirmation rather than inference:

- ADR-0003 does not accurately describe `AssessmentReport`'s current relationship to `Assessment` — five of its six named entities remain accurate; `AssessmentReport` does not.
- **This is not the architecture having evolved past ADR-0003.** `modiq-report` existed as a separate crate concurrent with ADR-0003's own writing, confirmed by the repository's own dated revision history. The prior evaluation's "single-crate-era" framing does not survive this check and should not be relied upon going forward.
- The more precise, better-evidenced account is that ADR-0007 — accepted one ADR later — established a technical definition of "ownership" that ADR-0003's own earlier, looser example list was never checked against. The gap is a real but narrow editorial imprecision in an Accepted architectural document, not a case of implementation drifting away from documented intent.
- `PROJECT_HANDOFF_v1.1.md`'s current text traces its claim primarily to `DataModel.md`'s own independent, higher-authority conceptual framing (Assessment "generates" a Report), not to a simple repetition of ADR-0003's "owned" language. This suggests the eventual correction can likely be scoped narrowly to ADR-0003 itself, without necessarily requiring `DataModel.md` or `PROJECT_HANDOFF_v1.1.md`'s own conceptual claims to change — though that determination belongs to Architectural Resolution, not this document.
- Of the four mechanisms available, this repository's own explicit, written ADR policy (`docs/adrs/README.md`) rules out in-place amendment as a departure from stated practice, and names superseding-ADR as the documented (if not yet exercised) mechanism. A Governance-Register-only fix would conflict with the Documentation Authority ordering's own ranking of ADRs above Governance. Which of these the Chief Architect weighs as correct — and whether "leave unchanged" is in fact acceptable given the propagation already observed — is an Architectural Resolution decision, not made here.

**No architectural decision has been made. GOV-015 remains Open.** This evaluation's responsibility ends here, awaiting Chief Architect review before Architectural Resolution begins.

---

## 6. Architectural Resolution

**Resolution decision: GOV-015 is Resolved.**

### 6.1 The Four Determinations

**Is ADR-0003 an architectural inconsistency or an editorial imprecision?** Editorial imprecision, not an architectural inconsistency. The evaluation's own timeline check (§3.2) is decisive: `modiq-report` existed as a crate separate from `modiq-runtime` concurrent with ADR-0003's own date (2026-07-16), not after it. The actual architecture — the Reporting Crate Boundary Rule, `Architecture.md`'s Platform Boundaries, and the implementation itself — has never been internally inconsistent; all three agree with each other and always have. What is imprecise is ADR-0003's own word choice: it used "owned" for six items without the technical precision to distinguish `AssessmentReport` from the five items that genuinely fit that word.

**Does ADR-0007 supersede ADR-0003's terminology by definition?** Not formally, but yes in substantive effect. ADR-0007 never declares itself a supersession of ADR-0003 — its own "Relationship to Other Specifications" section lists ADR-0003 among documents its decision "is reflected in," presenting itself as consistent with, not replacing, ADR-0003. But ADR-0007 establishes the only precise technical definition of "ownership" this repository has for Runtime entities (stable identity assigned by `Assessment`, mutation only through `Assessment`'s own methods), and by that definition `AssessmentReport` does not qualify. ADR-0007 supersedes the *term* in effect, without having formally superseded the *document* — which is precisely why a governance action is still needed: the substantive answer already exists in ADR-0007, but nothing has yet made that answer the recorded, authoritative one for ADR-0003's own list.

**Does the repository's ADR governance require a superseding ADR rather than modification of ADR-0003?** Yes, if ADR-0003 is to be corrected at all. `docs/adrs/README.md`'s own Engineering Notes state directly that Accepted ADRs should not be modified, and that a new ADR should be created to supersede or amend one instead — a policy already applied once, directly on point, when ADR-0002's own acknowledged numbering defect was deliberately left uncorrected in place for this exact stated reason. In-place amendment of ADR-0003 would be a first departure from stated policy, not an application of precedent.

**Is any remaining discrepancy architectural or purely documentary?** Purely documentary. No behavior, boundary, or ownership relationship needs to change — `Assessment` continues exactly as it operates today, and `modiq-report` continues generating `AssessmentReport` externally exactly as it does today. What would change, if the recommended follow-up work below is later authorized and completed, is which document states that relationship correctly.

### 6.2 Supporting Evidence

Drawn entirely from Sections 2–5 above, re-verified where load-bearing: ADR-0003's Date field against `CrateRoadmap.md`'s revision history and `CHANGELOG.md`'s own Sprint 1 entry (establishing the crate separation is concurrent with, not subsequent to, ADR-0003); direct source confirmation that `Assessment` holds no field, method, or dependency touching `AssessmentReport` anywhere in `modiq-runtime`; `docs/adrs/README.md`'s Engineering Notes and the ADR-0002 numbering-defect precedent; and `PROJECT_HANDOFF_v1.1.md`'s current text (re-confirmed unchanged), read precisely as citing ADR-0003 only for "sole aggregate root," not for the `AssessmentReport` inclusion, which traces instead to `DataModel.md`'s own independent "generates" framing.

### 6.3 Disposition of Every Finding

| Finding | Disposition |
|---|---|
| §3.1 — ADR-0003 partially inaccurate (5 of 6 entities correct) | Confirmed, accepted as this resolution's own basis. |
| §3.2 — "Single-crate-era" framing (`GOV002_ARCHITECTURE_EVALUATION.md`) was incorrect | Confirmed. This correction stands; any future document referencing this history should cite the corrected, concurrent-not-sequential account. |
| §3.3 — Editorial imprecision surfaced by ADR-0007's later precision, not historical drift | Confirmed; this resolution's own answer to Determination 1. |
| §3.4 — `PROJECT_HANDOFF_v1.1.md` traces to `DataModel.md`, not a repetition of ADR-0003; likely does not itself need to change | Confirmed as the working assumption, not fully closed — see Recommended Follow-up Work, Item 2. |
| §4 Option A (leave unchanged) | Not selected — the imprecision has already propagated once and leaving it unchanged does not stop further propagation. |
| §4 Option B (amend ADR-0003 in place) | Not selected, per Determination 3. |
| §4 Option C (new superseding ADR) | **Selected as the correct mechanism**, per Determination 3. Not created or authorized by this resolution. |
| §4 Option D (Governance-Register-only fix) | Not selected — conflicts with the Documentation Authority ordering (ADRs rank above Governance). |

### 6.4 Recommended Follow-up Work (Described Only — Not Authorized)

1. **Prepare a proposal for a superseding ADR** (`PROPOSAL_ADR-0011.md`, or the next available ADR number) addressing ADR-0003's specific inclusion of `AssessmentReport`, while explicitly preserving ADR-0003's still-valid architectural decisions (`Assessment` as the sole aggregate root; Context, Subject, Evidence, Findings, and Recommendations remain unchanged). Per the repository's established governance discipline, the proposal is not itself an authoritative architectural decision — it exists solely for Chief Architect review. Upon approval, the proposal becomes the basis for the superseding ADR, which may then proceed through the repository's normal reconciliation process. The intended governance lifecycle: `GOV-015 Resolution → PROPOSAL_ADR-0011.md → Chief Architect Approval → ADR-0011 → Repository Reconciliation`. Preparation of the proposal, creation of the superseding ADR, and any resulting documentation reconciliation each require their own explicit authorization under the repository's stage-gated workflow. This Resolution recommends that workflow only; it does not authorize or perform any part of it.
2. **Re-check `PROJECT_HANDOFF_v1.1.md`'s own citation once the superseding ADR's actual text exists** — Finding 3.4's disposition is a working assumption, not a final determination; a new ADR number in the Register may warrant a citation update even if the underlying claim doesn't change.
3. **No other document was identified as requiring correction** — `DataModel.md`, `Architecture.md`, and `GOVERNANCE.md`'s Reporting Crate Boundary Rule are all already accurate.

**GOV-015 is marked Resolved.** No ADR was modified or created, no ADR proposal was created, no other repository documentation was changed as part of reaching this resolution, and no implementation was authorized.
