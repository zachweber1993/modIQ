# GOV-012 and FindingSeverity Preparation

| Property | Value |
|----------|-------|
| **Document** | GOV-012_AND_FINDINGSEVERITY_PREPARATION.md |
| **Stage** | Governance/Specification Drafting — prepared per Technical Director direction, for final review before insertion into `GOVERNANCE.md` and `DataModel.md` |
| **Sprint context** | Sprint 5 Phase 1 (Design Preparation), per `SPRINT5_IMPLEMENTATION_PLAN.md` |
| **Status** | GOV-012 approved; the FindingSeverity definitions and GOV-013 (opened by Technical Director review of those definitions) recorded below. **No `GOVERNANCE.md` or `DataModel.md` edit has been made. No Rust code has been written.** Phase 2 of `SPRINT5_IMPLEMENTATION_PLAN.md` does not begin until GOV-012's and GOV-013's formal entries and the `FindingSeverity` definitions are reviewed and inserted. |

---

## Part 1 — GOV-012 Formal Entry (for insertion into `GOVERNANCE.md`)

The Technical Director has already approved the substance of all three questions below (relayed directly, not drafted as options by Engineering). This section formalizes that decision into the same entry format `GOVERNANCE.md`'s Governance Register already uses for GOV-009 through GOV-011, for a final review pass before the Register itself is amended.

Ready to insert verbatim as `## GOV-012`, immediately following `## GOV-011`:

> Title
>
> Rule Evaluation Model
>
> Status
>
> Resolved
>
> Raised
>
> Sprint 5 Phase 1 (Assessment Intelligence Layer — Design Preparation)
>
> Description
>
> Since Sprint 1, `modiq-rules::RuleEngine` has evaluated exactly one Rule, applied unconditionally to all Evidence regardless of category. `SPRINT5_IMPLEMENTATION_PLAN.md` requires a second, category-specific Rule, which raises the same class of architectural question GOV-009/010/011 resolved for Evidence Collection, applied here to Rule Evaluation for the first time: how `RuleEngine::evaluate` represents more than one Rule's outcome, in what order, and whether Rules interact with or suppress one another.
>
> Question
>
> 1. Does `RuleEngine::evaluate` return a single aggregated outcome, or one outcome per matching Rule?
> 2. When more than one Rule matches, in what order are the resulting Findings and Recommendations produced?
> 3. When more than one Rule matches the same Evidence, does a more specific Rule suppress a more general one, or do both fire independently?
>
> Resolution
>
> Approved by Technical Director in its entirety, following `SPRINT5_IMPLEMENTATION_PLAN.md`'s Design Questions 1–3. All three questions resolved:
>
> **Question 1 (return shape):** `RuleEngine::evaluate` returns `Vec<RuleOutcome>` — zero, one, or several `(Finding, Recommendation)` pairs, one per matching Rule. This requires no change to `Finding`'s or `Recommendation`'s existing one-Rule-per-Finding shape (`Finding.rule_reference` remains a single `RuleReference`), and keeps each Rule's Finding independently traceable to exactly the Rule and Evidence that produced it, per `RuleEngine.md`'s Traceability Management responsibility.
>
> **Question 2 (ordering):** Rules are evaluated, and their outcomes produced, in a fixed, explicit declaration order internal to `RuleEngine` — the order Rules are listed in `evaluate`'s own dispatch logic — never an order derived from Evidence's own arrival sequence. This mirrors the same discipline `ArchiveReader::entries()` and `EvidenceCollector`'s directory traversal already apply: explicit order imposed by the producer, never inherited from an unordered or incidentally-ordered source.
>
> **Question 3 (composition):** Rules compose independently; no suppression model exists. Every Rule is evaluated against whichever Evidence it applies to, regardless of whether another Rule also matches related or overlapping Evidence. An Assessment whose Evidence matches both the existing generic Rule and a new category-specific Rule produces both Rules' Findings and Recommendations, not one at the expense of the other. This avoids inventing a precedence/suppression mechanism this Sprint's own evidence does not yet justify — only two concrete Rules exist even after Sprint 5 — consistent with `RuleEngine.md`'s description of Rule Selection as determining "which Rules are applicable," plural and independent, not a single winner-takes-all choice.
>
> **Implementation-mechanism questions remaining:** the exact internal dispatch structure of `RuleEngine::evaluate` (a `match` over category, a sequence of `if let` checks, or another shape) is an implementation detail, not fixed by this resolution, provided no trait, registry, factory, or plugin mechanism is introduced — `GOVERNANCE.md`'s Crate Boundary Rules already states inline fulfillment as `RuleEngine`'s approved pattern (ADR-0010, GOV-004), and this resolution does not revisit that.
>
> Full definition recorded in `SPRINT5_IMPLEMENTATION_PLAN.md`, Design Questions 1–3.

---

## Part 2 — `FindingSeverity` Semantic Definitions (candidate amendment to `DataModel.md`)

### Why this is needed now, not invented for this document

`FindingSeverity` (`Error`, `Warning`, `Informational`, `BestPractice`) has existed in `modiq-runtime` since Sprint 2, as "a direct translation of Glossary.md's... Finding definition" (`ENGINEERING_RELEASE_0.2.md`). Glossary.md names the same four categories ("Findings may represent: Errors, Warnings, Informational observations, Best Practice recommendations") but does not define what distinguishes one from another. `DataModel.md`'s own Finding section is equally silent on severity. In three Sprints of use, exactly one variant (`Informational`) has ever been assigned, by the one Rule that has ever existed. Sprint 5's second Rule is the first Rule that must actually *choose* among the four — which is not possible to do consistently, or explain to a user, without a stated definition of what each one means.

**These definitions are provisionally accepted for Sprint 5, not confirmed as permanently correct.** Drafting them surfaced a real modeling tension — `BestPractice` does not sit on the same ordered "how urgent is this" scale `Error`/`Warning`/`Informational` do; it answers a different question, what *kind* of observation a Finding represents, independent of urgency. The Technical Director has reviewed this and declined to change `FindingSeverity`'s shape based on two Rules alone. The definitions below are written to be usable as-is for Sprint 5's actual Rules, while the conflation itself is tracked separately, open, as GOV-013 (Part 3, below) — revisited once more concrete Rules exist to evaluate it against, not resolved here.

### Candidate definitions

Drafted for Technical Director review, not yet inserted anywhere. Proposed home: a new subsection under `DataModel.md`'s existing `## Finding` section (§192–198), titled `### Finding Severity`, amended in place per this project's established frozen-document-amendment discipline (the same pattern `EvidenceCollection.md` has used across its own three amendments).

> ### Finding Severity
>
> A Finding's severity (`Error`, `Warning`, `Informational`, `BestPractice`) classifies the nature of its conclusion, not merely its importance. Each Rule must assign the severity its Evidence actually supports — consistent with the Rule Engine's Evidence-Based execution principle (`RuleEngine.md`), a Rule must never assign a severity stronger than what its Evidence conclusively establishes.
>
> **Error.** The assessed mod will not function correctly, or a required expectation is violated, as conclusively established by Evidence alone. Reserved for conclusions a user should treat as blocking. A Rule must not assign `Error` for a suspected, probable, or environment-dependent problem — only for a defect Evidence directly proves.
>
> **Warning.** A genuine, Evidence-supported concern about the mod's reliability, correctness, or trustworthiness, where Evidence does not conclusively establish the mod is broken. The mod may still function; the condition is one a careful user should not ignore. Appropriate when a condition's real-world consequence is uncertain or depends on factors this platform's own Evidence cannot fully resolve.
>
> **Informational.** A neutral, factual observation about the mod's structure or content, carrying no implication that anything is wrong or suboptimal. Appropriate when a Rule's role is to report what was found, not to judge it.
>
> **BestPractice.** The mod does, or does not, follow a convention associated with quality, maintainability, or compatibility, where deviating from the convention does not itself indicate the mod is broken or untrustworthy. Distinguishes advisory, convention-level guidance from `Warning`'s genuine reliability concern.
>
> These four categories are not a single linear ranking. `Error` is the most severe in the sense that it alone indicates conclusive breakage. `Warning` and `BestPractice` are not ordered relative to each other — one flags an uncertain reliability concern, the other an advisory convention deviation — and `Informational` carries no judgment at all.

### Applying the definitions: Sprint 5's own second Rule

Per these definitions, a Rule evaluating `EvidenceCategory::StructuralDuplication` (GOV-011) should assign **`Warning`** — not `Error`. GOV-011's own policy text establishes that collection "could not fully resolve every entry," a genuine reliability concern (which physical entry a later reader actually extracts is not guaranteed by the archive format itself), but does not establish that the mod is definitively broken — a well-formed archive containing duplicate names is not, by itself, proof of malfunction. This is stated here as the concrete application these definitions were drafted to support, not as a claim already approved independently of the definitions themselves.

---

## Part 3 — GOV-013 (for insertion into `GOVERNANCE.md`)

Raised directly by Technical Director review of Part 2's own definitions, not by implementation surfacing it independently — recorded here as a Governance Register item because it is exactly the shape `GOVERNANCE.md` itself defines one for: "an architectural question discovered during engineering," to "remain open until resolved."

Ready to insert verbatim as `## GOV-013`, immediately following `## GOV-012`:

> Title
>
> FindingSeverity Severity/Kind Conflation
>
> Status
>
> Open
>
> Raised
>
> Sprint 5 Phase 1 (Assessment Intelligence Layer — Design Preparation), during Technical Director review of the `FindingSeverity` semantic definitions (Part 2)
>
> Description
>
> `FindingSeverity` (`Error`, `Warning`, `Informational`, `BestPractice`) has existed in `modiq-runtime` since Sprint 2 but was exercised by only one variant, `Informational`, until Sprint 5's second Rule required a real choice among all four. Drafting semantic definitions for each variant (Part 2) surfaced that `BestPractice` does not sit on the same axis the other three do. `Error`/`Warning`/`Informational` answer one question — how urgent is this? — on a single ordered scale. `BestPractice` answers a different question — what kind of observation is this? — independent of urgency. A real best-practice deviation could independently be more or less urgent than another, and nothing in the current model can express that, since `BestPractice` and `Warning` are mutually exclusive values of the same field.
>
> Question
>
> Does this conflation warrant splitting `FindingSeverity` into two independent concepts — an ordered severity scale, and a separate, orthogonal classification of Finding kind — or is the existing four-variant model adequate as actually used in practice?
>
> Resolution
>
> Not resolved. Technical Director decision, recorded here rather than acted on: `FindingSeverity` remains unchanged for Sprint 5. The current model is provisionally accepted, not confirmed permanently correct. This item stays Open, to be revisited once the Rule Engine has multiple concrete Rules operating in practice and this question can be evaluated against real implementation evidence — not decided from two Rules alone. This is the same evidence-based resolution discipline GOV-004 and GOV-011 both already applied: a concrete forcing function should justify a model change, not the reverse.

---

## What This Document Does Not Do

No `GOVERNANCE.md` edit was made — Part 1's (GOV-012) and Part 3's (GOV-013) entries are drafted, ready for insertion, not inserted. No `DataModel.md` edit was made — Part 2's definitions are drafted, ready for insertion, not inserted. No `FindingSeverity` restructuring was performed or proposed as a decision — GOV-013 records an open question, not a resolution. No Rust file was modified. No `modiq-rules` Rule was implemented. Sprint 5 Phase 2 remains unauthorized until all three parts of this document are reviewed and their corresponding documents amended.
