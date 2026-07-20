# Proposal: Platform Validation Review

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_PLATFORM_VALIDATION_REVIEW.md |
| **Stage** | Review / Proposal (Architecture Review) |
| **Prepared by** | Engineering, for Technical Director review |
| **Status** | Design-only. No Rust code, no governance changes, no documentation changes outside this file, no ADRs, no implementation. |
| **Repository state reviewed** | `dc8af22` (Engineering Release 0.3, Sprint 3 complete, working tree clean) |
| **Scope** | Classify architectural assumptions as Validated, Requiring Refinement, or Retirement Candidate. Recommend a sequencing for resolving them. Decide nothing. |

---

# 1. Why This Review, and Why Now

Every prior architectural decision in this platform was evaluated against one of two things: reasoning in the abstract, or a synthetic case built specifically to prove a boundary without real consequences (the Sprint 1 pipeline, the Phase 4 synthetic collector). Sprint 3 is the first body of work that stress-tested the architecture against three genuinely different kinds of pressure at once, on real production code rather than a proof:

- **A new subsystem's introduction.** Evidence Collection did not exist when Documentation Release 1.0 froze the platform. Its arrival tested whether the dependency-direction rules and crate-boundary discipline actually hold when a new dependency edge is added, not just when they're followed by the code that was designed for them from day one.
- **Real, external, non-deterministic infrastructure.** Every prior phase's Evidence was synthetic. Phase 5's filesystem collector is the platform's first code that has to produce deterministic output from a source — a real filesystem — that offers no determinism guarantee of its own. This is the first real test of whether "determinism over automation" survives contact with the outside world.
- **A content-level invariant added under governance, after the fact.** GOV-005/006's cardinality enforcement tested whether `RuntimeInvariants.md` and the Governance Register's Level 1–4 process could absorb a new invariant into an already-shipped aggregate without redesigning it.

Three Engineering Releases and nine implementation phases now exist as evidence. This review reads that evidence plainly — it does not generate new evidence, and it does not decide anything. Its purpose is to state, for each assumption this platform was built on, whether the evidence available at `dc8af22` supports it, complicates it, or has quietly stopped supporting it at all — before Evidence Collection expands beyond the filesystem and adds still more surface area on top of assumptions nobody has re-examined since Documentation Release 1.0.

---

# 2. Method

This review is a reading exercise, not an experiment. For each assumption evaluated:

- The originating specification or ADR is cited.
- The real implementation evidence is cited — specific crates, specific tests, specific governance items — as it exists in the repository at `dc8af22`, not as described in any prior summary.
- No new code was written, run, or modified to produce this review. No governance item's status was changed. Where this review references test counts or usage patterns, they were confirmed directly against source at `dc8af22` (see Section 6 for what was specifically checked).

Classification is three-valued:

- **Validated** — the assumption has been tested by real, non-synthetic implementation pressure, more than once, and held without modification.
- **Requires Refinement** — the assumption is not wrong, but real evidence has surfaced a tension, a gap, or a growing cost that the original specification did not anticipate. A decision is needed; this review does not make it.
- **Retirement Candidate** — the assumption, or the scaffolding built to support it, has accumulated enough evidence of non-use or divergence from reality that continuing to carry it should itself be questioned. This review does not retire anything — retiring or reinterpreting a Frozen specification is a Documentation Release decision, and removing shipped scaffolding is, at minimum, a Level 3 change.

---

# 3. Validated Architectural Assumptions

| Assumption | Origin | Evidence | Confidence |
|---|---|---|---|
| Assessment is the sole aggregate root and sole mutator of Runtime state | ADR-0003, INV-006/007/009 | Held across all nine phases, including Evidence Collection's introduction: `EvidenceCollector::collect` returns `Vec<Evidence>` to its caller and never touches `Assessment`. No child entity or external crate mutates Assessment-owned state directly, anywhere in the codebase. | High |
| Dependency direction is strictly downward, `modiq-runtime` as the leaf | Architecture.md, CrateRoadmap.md | `modiq-collection`'s new edge (`modiq-engine → modiq-collection → modiq-runtime`) was verified by inspection, not assumed, to match ADR-0008 exactly — including confirming the Sandbox reaches `modiq-collection` only transitively through `modiq-engine`. Zero violations across five phases and a new crate. | High |
| Determinism is judged by content, not incidental identity | ADR-0005, ADR-0007 | Survived its first real test against genuinely non-deterministic infrastructure: filesystem directory listings carry no OS-level ordering guarantee, and Phase 5 had to impose its own (lexicographic, per-level sort) to preserve the principle. The principle did not have to change — the implementation had to work harder to satisfy it, which is a stronger validation than a synthetic pass. | High |
| Runtime Entity Design Pattern (ADR-0007) generalizes correctly | ADR-0007 | `AssessmentInput` (a `modiq-collection` value type, not a Runtime entity) still correctly borrowed the pattern's judgment — constructor validation, no identity-based equality for a value type — without ADR-0007 needing amendment to cover a type outside its own crate. The pattern reasoned correctly about a case it wasn't written for. | High |
| The Rule Engine / Evidence Collection producer-consumer boundary | RuleEngine.md, EvidenceCollection.md, ADR-0008 | First drawn in Sprint 3 Phase 1, before Evidence Collection existed as a subsystem. Survived four subsequent phases of adjacent work, including the subsystem's entire design and implementation, with zero modification to the boundary itself. | High |
| Proposal-first workflow (Architecture → Governance → Documentation → Implementation → Verification) | This project's working discipline, not a single ADR | Exercised three separate times this Sprint (Evidence Collection Boundary, GOV-005/006, Filesystem Collection) with zero post-implementation architectural rework in any case. This is no longer a claimed process — it has a three-for-three track record on real work. | High |
| "A capability should justify an abstraction, not the reverse" | Stated engineering principle, tested at the Rule trait and Collector trait questions | Declined twice under real, not hypothetical, pressure to build both — once when Evidence Collection was designed, again when Phase 5 shipped a second real I/O path with no trait behind it. Both times the smaller, concrete implementation shipped with no loss of correctness. | High |
| Abstract governance resolution, concurrently with the first real case | Implicit pattern, first used for GOV-005/006 | Used again, successfully, for GOV-009/010: both were deliberately left open through Phase 4's synthetic collector and resolved only once Phase 5's real filesystem I/O gave them something concrete to be designed against. The four-outcome Collection Error Model, designed this way, needed no revision once real I/O code had to implement it. This is now a twice-validated methodology, not a one-off. | High |
| Collection Atomicity requires no dedicated mechanism | GOV-010 resolution, `EvidenceCollection.md` | Specified in the abstract before any code existed to satisfy it. Fell directly out of ordinary `Result`/`?` propagation in `execute_from_assessment_input` — no rollback logic, no transaction abstraction, anywhere. | High |

---

# 4. Assumptions Requiring Refinement

## 4.1 EngineAPI.md's five-service model vs. observed composition

**Origin:** `EngineAPI.md` (Frozen, Documentation Release 1.0) specifies five conceptual services — Assessment, Knowledge, Rule Evaluation, Reporting, Version Profile — cooperating "through well-defined responsibilities rather than shared ownership," with a Service Relationships diagram showing `AssessmentService` invoking `RuleEvaluationService`, which invokes `KnowledgeService`, feeding `ReportingService`, contextualized by `VersionProfileService`.

**Evidence:** Three real subsystems now exist — Rule Engine, Reporting, Evidence Collection — and `AssessmentService` composes all three by direct instantiation (`RuleEngine`, `AssessmentReport::generate`, `EvidenceCollector`), never through `KnowledgeService`, `ReportingService`, `RuleEvaluationService`, or `VersionProfileService`. Those four stub services remain empty unit structs with zero call sites anywhere in the workspace, across the platform's entire history.

**Why this isn't a retirement call:** `EngineAPI.md` is a Frozen conceptual specification, and its capability-oriented framing may still be correct at the level it was written for — it explicitly disclaims defining "REST endpoints... SDK interfaces... programming language constructs." It's possible direct instantiation today is simply the correct *implementation* of a still-correct *conceptual* service boundary, and the stub structs were premature scaffolding rather than a wrong design. Three independent data points choosing the same alternative is real evidence, but it doesn't by itself say which side is wrong.

**This is GOV-004.** Governance status: open. What's new since it was raised: it now has three real data points instead of zero.

## 4.2 AssessmentService's two parallel entry points

**Origin:** ADR-0009, GOV-008. `execute` and `execute_from_assessment_input` coexist specifically to avoid deciding whether `AssessmentService`'s public API should evolve, rather than resolving it as a side effect of unrelated work.

**Evidence:** The pattern has shipped twice cleanly (Phase 4's `execute_from_descriptor`, Phase 5's renamed `execute_from_assessment_input`) with `execute` itself untouched since Sprint 1. The Lead Engineer Handoff's own prior guidance named the threshold explicitly: "a third parallel entry point would be the signal that routing around it has stopped working." The next real collector is the most likely candidate to be exactly that third case.

**Why this isn't decided here:** Whether the next collector needs a third entry point, an evolved `execute_from_assessment_input`, or something else entirely depends on what that collector's input shape turns out to be — which this review, by design, does not scope.

**This is GOV-008.** Governance status: open, and the platform's own prior guidance already named the condition under which it stops being deferrable.

## 4.3 Referential integrity for Finding/Recommendation references

**Origin:** GOV-005/006 resolved cardinality only (`Finding` must reference ≥1 Evidence item, `Recommendation` must reference ≥1 Finding item); referential integrity — whether a referenced ID actually resolves to something present in the same Assessment — was deliberately left open, explicitly pending "a second real Evidence producer."

**Evidence:** Phase 5 is arguably exactly that producer. `ENGINEERING_RELEASE_0.3.md`'s own Technical Debt Review already flags this as "newly ripe, not yet acted on." Independently, `RuntimeInvariants.md` still states plainly, for both INV-013 and INV-014, that "whether each referenced Id resolves to \[an item\] actually present within the same Assessment is a separate, unenforced question."

**Why this isn't decided here:** Enforcing referential integrity would be a Level 3 (Behavioral) change under `GOVERNANCE.md`, requiring its own governance approval before any enforcement code is written — exactly the same category GOV-005/006 themselves were.

## 4.4 The opaque Knowledge Domain reference pattern is structurally proven, not functionally proven

**Origin:** ADR-0007 introduced `RuleReference` and `RepairRecipeReference` as opaque wrappers, explicitly described in the ADR's own Consequences as "placeholders for a future Knowledge Domain integration that has not yet begun."

**Evidence:** Confirmed directly against source at `dc8af22`: every construction of `RuleReference` or `RepairRecipeReference` anywhere in the workspace — production code and tests alike — uses a hardcoded string literal (`"evidence-presence-rule"`, `"sample-rule"`, `"sample-repair-recipe"`). Nothing has ever resolved one against real Knowledge Domain content, because `modiq-knowledge` has had zero implementation since Sprint 0. The *shape* of the pattern (opaque reference, held not resolved, preserving the Knowledge Domain boundary from ADR-0001) is well-exercised. Whether that shape is actually sufficient once a real `Rule` type with real content exists behind it is untested by any real case.

**Why this isn't decided here:** No forcing function exists yet — this observation doesn't argue for building Knowledge Domain integration now, only for naming clearly that "the reference pattern works" and "the reference pattern will work for real Knowledge Domain integration" are two different claims, and only the first one currently has evidence behind it.

## 4.5 Scoped, temporary architectural decisions are accumulating

**Origin:** None single — a pattern across GOV-009's resolution, GOV-010's resolution, and Phase 5's own design.

**Evidence:** Phase 5 alone introduced two decisions explicitly scoped as temporary: Collection Atomicity ("an intentional Phase 5 scope decision, not a permanent platform limitation... a future collector or phase may revisit it") and the Symbolic Link Policy ("a Phase 5 architectural boundary, not a permanent one"). Both are correctly documented as scoped rather than permanent. Neither has an owner or a trigger condition recorded anywhere for when it should actually be revisited.

**Why this isn't decided here:** This isn't a claim that either decision is wrong for now. It's a process observation: the platform now has at least two (and will likely accumulate more, as ZIP/XML collectors each bring their own scoped decisions) architectural choices whose stated lifespan is "until revisited," with nothing tracking whether that revisit ever happens. `GOVERNANCE.md` tracks open questions; it does not currently track answered-for-now questions with a stated expiration condition.

## 4.6 Testing discipline is proven at the unit level; its overall sufficiency is not yet re-examined

**Origin:** No single specification — an accumulated claim across every Engineering Release that the platform's testing discipline is sound.

**Evidence:** 115 tests, zero flaky, zero ignored, and a real, disciplined determinism-testing convention followed without exception — this part is not in question. Two structural gaps, both independently confirmed in `ENGINEERING_RELEASE_0.3.md` and `ROADMAP_REVIEW_2026.md` before this review and reconfirmed here: no session in the platform's history has ever visually verified the Sandbox's actual UI, and adversarial/malformed-input coverage exists only for what Phase 5 specifically needed (symlinks, permission denial) — nothing has yet tested a genuinely adversarial *file*, because nothing has parsed file content yet.

**Why this isn't decided here:** The gap is well-understood and already named three times across three documents; what hasn't happened is a decision about whether it needs to close before or concurrently with the next collector, given that a second real collector is very likely to be the first code in the platform's history to parse untrusted binary content.

## 4.7 The Sandbox's "no input capability" constraint has done its job and may now be worth revisiting

**Origin:** Sandbox design principle, reaffirmed at Sandbox Phase 1 and again explicitly in Phase 5's scope.

**Evidence:** The constraint has held under real pressure twice (Phase 1's real pipeline, Phase 5's real filesystem collector) using a fixed, checked-in fixture path instead of a file picker. `ROADMAP_REVIEW_2026.md` already named this same constraint "the single biggest blocker standing between the platform and demonstrating real Evidence Collection through its own developer tool" — before Phase 5 shipped, and the observation was not revisited once Phase 5 landed.

**Why this isn't decided here:** Adding real input capability to the Sandbox is explicitly a UI-scoped decision, outside this review's boundary and outside every proposal that has touched Evidence Collection so far. Named here only because the tension is real and has now gone unaddressed across two consecutive phases where it was directly relevant.

---

# 5. Retirement Candidates

## 5.1 `modiq-common`

**Evidence:** Confirmed directly against source at `dc8af22`: the entire crate is three files, each containing a single module-level doc comment and nothing else (`error.rs`: `//! Shared error types.`, `id.rs`: `//! Shared identifier types.`, `prelude.rs`: `//! Common imports.`). Two full Engineering Releases and five Sprint 3 phases have produced zero code that imports from it, zero types that needed a shared home, and GOV-003 has had no forcing function arrive in nine phases of engineering.

**What retirement would mean:** Not deletion by default — GOV-003's own standing guidance ("resolve only when two crates naturally need to share a type") remains sound and this review does not argue against it. What's worth naming plainly is that `modiq-common` is now the scaffold crate with the single weakest evidence, of any of the four, that it belongs in the roadmap in its current form — weaker than `modiq-knowledge`, `modiq-versioning`, or `modiq-cli`, each of which at least has a Frozen specification describing real future content. `modiq-common` has no such specification; it has never had one.

**Why this proposal does not retire it:** Removing a workspace crate, even an empty one, is at minimum a Level 2–3 change under `GOVERNANCE.md` and would need its own governance record. This review's role is to say the evidence for keeping it as-is is now the weakest in the platform, not to act on that finding.

## 5.2 The unused `EngineAPI` stub services and `modiq-rules` stub submodules

**Evidence:** As detailed in 4.1, above — this is the same underlying finding as GOV-004, viewed from the scaffolding side rather than the architecture side. `modiq-rules`'s four stub submodules (`RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability`) mirror the same pattern: they exist, unused, alongside a `RuleEngine::evaluate` that implements selection, evaluation, and traceability inline.

**What retirement would mean:** If GOV-004 resolves toward formally endorsing direct composition as the standing pattern (rather than deciding the stub services are simply not yet wired up), the stub services and stub submodules themselves become deletion candidates — dead code kept alive by a service-oriented API design nothing currently implements against.

**Why this proposal does not retire it:** This is downstream of GOV-004, not a separate decision. Listed here only so that GOV-004's resolution, whichever direction it goes, is understood to have this deletion as a likely direct consequence rather than a separately-litigated follow-up.

---

# 6. What Was Specifically Checked

For traceability: this review's evidence claims were confirmed directly against the repository at `dc8af22`, specifically —

- `cargo test --workspace` (root) and the equivalent inside `apps/sandbox/src-tauri`, both passing, matching the counts recorded in `ENGINEERING_RELEASE_0.3.md`.
- Direct source inspection of `crates/modiq-common/src/*.rs` (all three files, in full).
- A workspace-wide search for `RuleReference` and `RepairRecipeReference` construction sites, confirming every call site uses a literal string.
- A workspace-wide search confirming no call site exists anywhere for `KnowledgeService`, `ReportingService`, `RuleEvaluationService`, or `VersionProfileService` outside their own definitions.
- Direct reading of `EngineAPI.md`, `KnowledgeModel.md`, `VersionProfile.md`, `DataModel.md`, `RuleEngine.md`, `RuntimeInvariants.md`, ADR-0001, ADR-0002, and ADR-0007 in full, cross-referenced against `GOVERNANCE.md`'s current item statuses.

No governance item's status was changed by this process. No file outside this one was modified.

---

# 7. Recommended Sequencing

This review recommends an order for resolving Section 4's items, not the resolutions themselves:

1. **GOV-004 first.** It has the most evidence, the clearest three-data-point pattern, and — per 5.2 — its resolution determines whether a separate cleanup pass is even needed. Nothing else in this review depends on it, but its answer (formally endorse direct composition vs. wire something real into the stub services) shapes how the platform should treat scaffolding going forward, including `modiq-common`'s eventual disposition.
2. **GOV-008, before or alongside scoping the next collector.** The platform's own prior guidance already named the trigger condition, and the next collector is likely to hit it directly — resolving this after a third entry point already exists would repeat the exact pattern GOV-008 was designed to avoid.
3. **The referential-integrity follow-up (4.3), on its own track.** It doesn't block collector work and isn't blocked by it; it can be scoped as its own small governance item whenever convenient.
4. **4.5 (tracking scoped/temporary decisions) as a process question, not an architecture question.** This may be as simple as a new section in `GOVERNANCE.md` or `ENGINEERING_LOG.md` for "time-boxed decisions and their revisit trigger" — worth a short conversation, not a proposal of its own.
5. **4.6 (testing sufficiency) resolved concretely, not abstractly, once the next collector is scoped** — consistent with this platform's own twice-validated pattern (Section 3) of resolving this kind of question against a real case rather than in the abstract.
6. **4.4, 4.7, and Section 5 remain named but unscheduled.** None has a forcing function yet. Recording them here is this review's contribution; deciding when one arrives is not.

---

# 8. Explicitly Out of Scope

Per instruction, this review does not scope, design, or begin the next collector, does not touch CLI wiring, and does not resolve any Governance Register item. Its only output is the classification above and the sequencing recommendation in Section 7. Every item in Sections 4 and 5 remains exactly as open, unresolved, and undecided after this document as before it — this review's job was to make the evidence legible, not to spend it.
