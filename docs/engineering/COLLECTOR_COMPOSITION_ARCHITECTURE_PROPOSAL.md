# Collector Composition Architecture Proposal

| Property | Value |
|----------|-------|
| **Document** | COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md |
| **Project** | modIQ |
| **Purpose** | Architectural study — determine the correct model for multi-source Evidence collection, independent of any single Collector |
| **Prepared by** | Lead Engineer, on `feature/runtime-implementation` |
| **Repository baseline** | HEAD `af65bf0` |
| **Status** | **Architecture approved by the Chief Architect.** See the Chief Architect Decision Record, Section 14. Historical analysis below (Sections 1–13) is preserved unaltered as the record of how the recommendation was reached; the approved, final architecture is authoritative in Section 14 and wherever explicitly marked below. Still no code changed, no commits, no branch created — this document integrates the approved decision into the repository's planning record; it does not itself authorize implementation. |

---

# 1. Problem Statement

**How should an Assessment collect Evidence from multiple independent sources while preserving the platform's existing architectural principles?**

This is a general question, not an XML-specific one. XML inspection is simply the first concrete case that forces it: every Collector composition decision made so far (`EvidenceCollector` vs. `ArchiveCollector`) has been mutually exclusive — a location either ends in `.zip` or it doesn't, and exactly one Collector runs. XML inspection doesn't fit that shape. A mod directory can need filesystem discovery *and* manifest inspection simultaneously — these are complementary concerns, not alternative routings of the same input.

`EvidenceCollection.md`'s own Collector Contract anticipated this moment directly and declined to answer it in advance: *"Should a third concrete Collector ever exist, whether this inline-conditional shape still holds is a question for that Collector's own implementation evidence, not decided preemptively here."* This document is that question, answered from evidence rather than assumption.

---

# 2. Existing Architecture

Reviewed directly against the frozen specifications and the current tree, not from memory:

- **`Architecture.md`, Information Flow:** Input → Evidence → Rule Evaluation → Findings → Assessment Report → User Understanding — a single-direction pipeline. Nothing downstream feeds back upstream.
- **`Architecture.md`, Extensibility:** *"The platform should evolve through extension rather than modification. New capabilities should integrate by: introducing additional Rules, extending Evidence Collection with additional Collectors, extending the Knowledge Base, adding Version Profiles, implementing Extension interfaces."* Additional Collectors — plural, additive — is the named mechanism, stated at the highest architectural authority level below `Vision.md`/`Principles.md`.
- **`EvidenceCollection.md`, Collector Contract — Inputs:** *"A Collector receives an Assessment Input (or the portion of one relevant to its kind of inspection)... and Relevant Assessment Context, where applicable. A Collector receives nothing else. It does not receive Rules, Engineering Knowledge, or any other Assessment state."* Not "any other Collector's output" specifically — but "any other Assessment state" already covers it: Evidence, once produced, belongs to the Assessment aggregate (`DataModel.md`: Ownership), not to the Collector that produced it, and a Collector receiving another Collector's Evidence would be receiving Assessment state indirectly.
- **`EvidenceCollection.md`, Composition (resolved, Sprint 4 Phase 3D):** the current two-Collector routing — one direct, inline, deterministic check, no registry, no dispatcher, no trait or common supertype — explicitly scoped to "exactly two Collectors," with the shape's continuation for a third left open.
- **`EvidenceCollection.md`, Future Evolution:** *"structured-text inspection, script inspection, manifest inspection, or dependency inspection... can each be added as independent, additive work behind it, without any of them requiring a change to the Rule Engine, the Runtime Domain, or Reporting, and without requiring changes to each other."* "Without requiring changes to each other" is the operative phrase for this study.
- **`GOVERNANCE.md`, Engine boundary:** *"Owns: orchestration, execution flow, composition of platform services... Composition of platform services means direct composition of each subsystem's own real type — not an intermediate layer of engine-local service objects."* This is the boundary ADR-0010/GOV-004 established by retiring the old `EngineAPI` service-object layer — the single most load-bearing constraint on how any new orchestration logic in `modiq-engine` may be shaped.
- **`AssessmentService::execute_from_assessment_input`** (current code, `crates/modiq-engine/src/engine/assessment_service.rs`): constructs one `AssessmentInput`, routes to exactly one Collector via `is_archive_location`, collects its Evidence, delegates to `execute`. This entire method is the current "composition" logic, in full — nine lines, no abstraction.
- **`RuleEngine::evaluate`** (current code): consumes `&[Evidence]` — a flat slice, with no knowledge of which Collector produced any item. Rule dispatch is already, independently, multi-participant (`EvidencePresenceRule` then `StructuralDuplicationRule`, GOV-012) — Rules already face the "more than one thing evaluates the same input independently" shape this study is asking Collectors to adopt.

---

# 3. Architectural Constraints

Treated as authoritative, per instruction, unless implementation evidence demonstrates otherwise — none surfaced during this study that would justify relaxing any of them:

1. A Collector receives only an Assessment Input and Assessment Context — never another Collector's output, never other Assessment state.
2. A Collector's guarantees (determinism, factual accuracy, boundary respect) are evaluated per-Collector, independently — no Collector's correctness may depend on another Collector's success, timing, or presence.
3. `modiq-engine` composes real subsystem types directly — no engine-local service-object indirection layer, regardless of how many participants exist.
4. No dispatch abstraction (trait, registry, plugin mechanism) is justified by a case count that hasn't produced concrete implementation pressure demonstrating one is needed (`GOVERNANCE.md`, GOV-004, GOV-012 precedent, reaffirmed at least six times in this platform's history).
5. `AssessmentService`'s public entry points (`execute`, `execute_from_assessment_input`) are the approved boundary; GOV-008 remains deliberately unresolved. Nothing in this study requires touching either signature.

---

# 4. Option Analysis

## Option A — Single Collector Expansion — **REJECTED** (Chief Architect Decision Record, Section 14)

Extend `EvidenceCollector` to also perform XML inspection.

- **Architectural fit:** Poor. Directly contradicts `Architecture.md`'s own Extensibility principle, quoted above — that principle names "additional Collectors" as the mechanism, not "expanded Collectors." It also contradicts `EvidenceCollection.md`'s Future Evolution language that future capabilities should be addable "without requiring changes to each other" — Option A requires changing `EvidenceCollector` itself to add an unrelated concern.
- **Responsibility boundaries:** Weakened, not violated outright. `EvidenceCollector` today has one job (filesystem structural discovery). Adding XML content inspection to the same type conflates two genuinely independent inspection concerns — structural presence and semantic content — inside one unit, the same kind of conflation `FileStructureAnalysis` vs. `StructuralDuplication` was kept deliberately separate to avoid (`EvidenceCollection.md`: "a Rule reasoning over `FileStructureAnalysis` structurally... must not silently absorb" a different kind of fact).
- **Scalability:** Fails outright against the Future Capability Review (Section 4, below). Six named future capabilities (XML, Lua, Localization, Textures, Store assets, additional metadata) crammed into one ever-growing `EvidenceCollector` produces exactly the kind of monolith this platform's crate-per-responsibility discipline has never allowed anywhere else.
- **Maintenance:** Every future content-inspection capability becomes a modification to an existing, already-shipped, already-tested type — the highest-regression-risk shape available, compared to the other two options where new capability is additive.
- **Future extensibility:** Poor, for the same reason.

## Option B — Collector Composition (Collectors depend on each other) — **REJECTED** (Chief Architect Decision Record, Section 14)

Allow Collectors to invoke, consume, or depend upon other Collectors.

- **Contract compliance:** Violates Constraint 1 directly and explicitly — the Collector Contract's Inputs section is unambiguous that a Collector "receives nothing else" beyond its Assessment Input and Context.
- **Coupling:** The failure mode this option introduces is real, not theoretical: if a future `XmlCollector` depended on `EvidenceCollector`'s discovered-file output, then `EvidenceCollector`'s own internal traversal choices (ordering, what counts as "discovered," future changes to the Symbolic Link Policy) become load-bearing for a Collector that has no architectural relationship to filesystem traversal at all. A change to one Collector's internals could silently break another's correctness — the platform has never had this kind of coupling anywhere in Evidence Collection, by design.
- **Testing:** Each Collector currently ships with fully independent, real-I/O tests (`ArchiveCollector`'s tests construct real ZIP fixtures with zero dependency on `EvidenceCollector`, and vice versa). A dependency chain between Collectors would force a downstream Collector's tests to either mock or genuinely invoke an upstream Collector — the first place this platform's "real I/O, real fixtures, never mocks" discipline would come under direct pressure to compromise.
- **Scalability:** Fails. A dependency graph among two collectors is manageable; among six (this Sprint's own Future Capability Review list), an ad hoc dependency graph becomes exactly the kind of implicit, undocumented coupling this platform's crate-boundary discipline exists to prevent.
- **Long-term consequences:** This is the option most likely to eventually force a real redesign — once enough Collectors exist with inter-dependencies, someone will need to reconstruct "what actually needs to run, in what order" logic that a clean independent-Collector model never would have required in the first place.

**Option B is rejected on contract-compliance grounds alone** — every other evaluation criterion is secondary to the fact that it violates an already-frozen, already-precise architectural constraint.

## Option C — Collection Coordinator — **PRINCIPLE APPROVED; LITERAL COORDINATOR COMPONENT REJECTED for Sprint 7** (Chief Architect Decision Record, Section 14)

A separate orchestration component selects participating Collectors, executes them, aggregates Evidence, and returns a unified result. Collectors remain independent and never depend on each other.

- **Architectural fit:** Strong on principle — Collectors staying independent and never coupled is exactly what Constraints 1 and 2 require, and exactly what `Architecture.md`'s Extensibility principle anticipates ("additional Collectors," each self-contained).
- **Separation of concerns:** Clean — selection/execution/aggregation is a genuinely distinct responsibility from any single Collector's own inspection logic, and from Rule evaluation.
- **Scalability / future collector integration:** Strong — every named future capability (Section 5, below) can be added as one more independent participant, exactly matching "without requiring changes to each other."
- **Testing:** Clean — each Collector's own tests remain fully independent; a coordinator's own tests would cover aggregation behavior only (ordering, combining results from real Collectors), not re-test any individual Collector's own logic.
- **The one real risk, and it is a significant one:** taken literally — "introduce a separate orchestration component" — this is structurally the same shape as the `EngineAPI` service-object layer this platform already built once, found unused across three Engineering Releases, and retired (ADR-0010, GOV-004). `GOVERNANCE.md`'s Engine boundary rule exists specifically to prevent a new intermediate layer from being introduced for hypothetical future flexibility. A literal, distinct "Collection Coordinator" type, introduced now, for exactly two Collectors, would be built for a case that does not concretely exist yet — the same failure mode (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, Section 8: "Speculative extensibility") this platform's own history has repeatedly named and avoided.

**Option C's principle is correct. Its literal, named implementation ("a separate orchestration component") is premature for the evidence currently in hand.** This tension is resolved in the Recommendation, below, rather than glossed over.

---

# 5. Future Capability Review

Evaluated against every named future collector: XML, Lua scripts, Localization, Textures, Store assets, additional metadata sources.

| Collector | Option A (expand existing) | Option B (dependency) | Option C-principle (independent, composed) |
|---|---|---|---|
| XML (`modDesc.xml`) | Forces `EvidenceCollector` to grow a second, unrelated concern | No natural dependency on another Collector exists or is needed | Fits — independent content inspection |
| Lua scripts | Same monolith problem, worse (`EvidenceCollection.md` calls this "the platform's highest-risk future collector" — the last thing that risk should share a type with) | No natural dependency | Fits |
| Localization | Same | No natural dependency | Fits |
| Textures | Same | No natural dependency | Fits |
| Store assets | Same | No natural dependency | Fits |
| Additional metadata | Same | Might be *tempting* to depend on XML's own output (e.g., metadata referenced from `modDesc.xml`) — exactly the coupling Option B risks normalizing | Fits, provided each source is inspected independently against the shared Assessment Input, never against another Collector's result |

**No future collector reviewed produces a case where Option A or B scales cleanly; every one fits Option C's underlying principle.** This table is the concrete evidence the "capability before abstraction" test asks for before adopting a shape — not a hypothetical future, but six named, product-specified capabilities (`ProductSpecification.md`: XML validation, Lua analysis, Dependency inspection, Asset verification are named Assessment Engine responsibilities today) that all point the same direction.

---

# 6. Assessment Pipeline Effects

```
Assessment
   ↓
Collection   ← the only stage any option changes
   ↓
Evidence
   ↓
Rules
   ↓
AssessmentReport
```

**Evidence:** unchanged under every option. `Evidence`'s shape (`category`, `description`, `location`) already accommodates any Collector's output; no option requires a Runtime change.

**Rules:** unchanged under every option. `RuleEngine::evaluate` already consumes a flat `&[Evidence]` slice with no awareness of provenance — it cannot tell, and does not need to tell, which Collector produced any item. Whichever composition model is adopted, Rules see the same shape they see today.

**AssessmentReport:** unchanged under every option, for the same reason it was unchanged when `StructuralDuplication` was introduced — new Evidence flows through the existing `evidence()`/`findings()`/`recommendations()` lists.

**Does any public API require change?** No. Not under Option A, B, or C. `AssessmentService::execute` and `execute_from_assessment_input` keep their exact current signatures under every option studied — the difference between options is entirely in what `execute_from_assessment_input`'s *body* does internally, never in its public contract. This holds even for Option C's literal coordinator shape: a coordinator, if introduced, would be called *by* `AssessmentService`, not replace or change its public surface. Per instruction, this document does not propose redesigning the public API — none of the three options gave cause to.

---

# 7. Evidence Philosophy

**Confirming the stated principle: Collectors observe. Rules evaluate. Evidence records facts. Findings express meaning. Recommendations express action.**

This remains correct under Option A and Option C — neither touches the Observe/Evaluate boundary; both keep Collectors producing only Evidence, never Findings.

**Option B is the one alternative that puts this principle under real strain**, though not by directly violating it. If Collector B's own observations depend on Collector A having already run and produced a particular result, Collector B is no longer purely observing the Assessment Subject — it is partly trusting another component's prior interpretation of what to look at next. That is a subtler failure than "a Collector evaluates Evidence" (which none of the three options do), but it erodes the same discipline the stated principle protects: that each observation stands on its own, independently verifiable against the real subject, not against another component's prior conclusions. This is an additional, independent reason (beyond the direct Contract violation in Section 4) to reject Option B.

---

# 8. Recommended Architecture

**Adopt Option C's principle — Collectors remain fully independent, never depend on or consume each other's output, and are composed by `modiq-engine` — implemented, for the current evidence, as direct inline composition inside `AssessmentService::execute_from_assessment_input`, not as a new, separately-named orchestration component.**

This is not a fourth option; it is Option C with its implementation shape corrected to match this platform's own "capability before abstraction" discipline, exactly as that discipline has been applied to every prior composition question (Collector routing itself, Rule dispatch, the original `EngineAPI` retirement). Concretely:

- The existing two-Collector routing (filesystem vs. archive, mutually exclusive, chosen by input shape) is preserved exactly as it is today — this recommendation does not touch it.
- A second, independent axis is added alongside it: content Collectors (starting with `XmlCollector`) are invoked directly, by name, against the same Assessment Input — not selected by a mutual-exclusivity check, since their applicability is about *content presence*, not *input shape*. Each independently reports Empty Collection when its own concern isn't present, exactly as the existing Collection Outcomes model already allows.
- `modiq-engine` combines every invoked Collector's Evidence into the single list already passed to `execute` — the same aggregation `execute_from_assessment_input` already performs for its one Collector today, extended to more than one.
- No new type, trait, registry, or module is introduced for this. The composition logic is a handful of direct, explicit calls — the same shape `is_archive_location`'s existing routing already takes, just no longer mutually exclusive.

**A concrete, named threshold for revisiting this:** extraction into a dedicated coordination component becomes justified when either (a) a third *additional* content Collector is added beyond `XmlCollector` (making inline composition's line count and conditional complexity genuinely unwieldy — not merely longer), or (b) applicability logic for any Collector becomes more complex than a straightforward "always invoke, let it self-report emptiness" or simple shape-based check. Until either condition produces real implementation evidence, building the coordinator now would be exactly the "speculative extensibility" this platform's own history (`EngineAPI`, three Engineering Releases of non-use) has already paid the cost of once.

---

# 9. Consequences

- **`modiq-engine`** gains a second, independent Collector invocation inside `execute_from_assessment_input`'s existing body — no new public surface, no new type.
- **`modiq-collection`** gains new Collectors additively, exactly as `Architecture.md`'s Extensibility principle prescribes — no existing Collector is modified to accommodate a new one.
- **Every future content Collector** named in Section 5 has a clear, evidenced template to follow, without needing this question re-litigated per Collector.
- **The Collector Contract's own "Composition" section in `EvidenceCollection.md`** needs an amendment recording that composition is no longer necessarily mutually exclusive — the frozen document currently only describes the two-Collector, either/or case, and its own text explicitly left this open for exactly this moment.
- **A new determinism claim exists for the first time**: the *combined* Evidence list's ordering across more than one Collector. This needs its own direct test once implemented (Sprint 5 Phase 5's own lesson: a determinism claim not directly tested is not verified) — not covered by any existing test, since no prior Assessment has ever combined output from two Collectors.

---

# 10. Required Repository Changes

Recorded here as what *would* be needed to adopt this recommendation — none performed in this session:

- **`EvidenceCollection.md` amendment** (following its own established amendment convention — four prior amendments on record): extend the "Composition" section to record that Collector composition may be non-exclusive, with independent Collectors aggregated rather than selected between.
- **A new Governance Register item**, analogous in shape to GOV-011 (Archive Collection Model) and GOV-012 (Rule Evaluation Model) — a "Collector Composition Model" resolution, formally recording: independent invocation, no inter-Collector dependency, inline composition (not a dedicated component) as the current approved shape, and the named threshold above for when that shape should be revisited.
- **No ADR change is recommended by this study.** This recommendation applies existing principles (`Architecture.md`'s Extensibility, `GOVERNANCE.md`'s Engine boundary, "capability before abstraction") rather than establishing a new durable one — the same reasoning `CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 4 gives for why GOV-012/GOV-013 didn't need ADRs of their own. This is a recommendation, not a decision — see Chief Architect Review Questions, below.

---

# 11. Migration Strategy

**None required.** This recommendation is purely additive: the existing filesystem-vs-archive routing is preserved exactly as it stands today, unmodified. The new composition axis (content Collectors) is added alongside it, not in place of it. No existing behavior changes, no existing test should need to change, and no existing consumer (`apps/sandbox`, `modiq-cli`) requires any change to keep working exactly as it does today, right up until a content Collector is actually wired in.

---

# 12. Risks

- **The inline-composition recommendation could itself be read as under-designing the problem** — it explicitly declines to build the more "complete-looking" coordinator now. This is a deliberate application of this platform's own repeated precedent, not an oversight; named here so it's a visible trade-off, not a silent one.
- **The named extraction threshold (Section 8) is a judgment call**, not a formula. "Genuinely unwieldy" is not as crisp as a numeric line count — worth the Chief Architect's own view on whether a more precise trigger should be recorded now, before it's needed under real pressure.
- **If XML inspection's own implementation evidence contradicts this study** (for example, if applicability turns out to need more than "always invoke, let it self-report emptiness"), that evidence should be reported and this recommendation revisited — not silently worked around during implementation.

---

# 13. Chief Architect Review Questions

Original questions, with resolution status recorded against each rather than rewritten — per the instruction to record decisions, not alter historical analysis:

1. **Does the recommended architecture — independent Collectors, composed inline inside `AssessmentService`, no new coordinator type — match your own intent, or did "Collection Coordinator" in the original framing mean something more structural than this study concluded the current evidence supports?** — **RESOLVED.** Confirmed as recommended. See Section 14.
2. **Should a new Governance Register item be opened for this now, or held until XML inspection's own implementation produces concrete evidence to record alongside the resolution?** — **Not resolved by this review.** No Governance Register item has been created; timing remains open.
3. **Is "no ADR needed" the correct call, or does this rise to the level of a durable principle worth recording permanently?** — **Not resolved by this review.** No ADR has been created; the question remains open.
4. **Is the named extraction threshold the right trigger, or should a different, more precise condition be recorded instead?** — **RESOLVED, refined.** A more precise, five-condition threshold is now approved. See Section 14.
5. **Does this study's scope match your intent, or should XML-specific questions be folded into this same review round?** — **Scope confirmed as originally structured.** XML-specific questions (absence-as-evidence, Rule Engine timing, dependency selection) remain tracked separately in `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, not resolved here.

---

# 14. Chief Architect Decision Record

**This is a Sprint-specific decision record, not a repository ADR.**

**Approved architecture:** Collection is a phase of an Assessment, not a standalone architectural component. Multiple independent Collectors may participate in a single Assessment; Collectors remain independent and never invoke, consume, or depend upon one another (Option B remains rejected on this basis). `AssessmentService` **temporarily** owns collection orchestration — determining which Collectors participate and aggregating their Evidence — implemented as direct, inline composition. **No `CollectionCoordinator` component, service, or abstraction is introduced during Sprint 7** (Option C's literal form remains rejected for now; its underlying principle — independence, no coupling, engine-composed — is approved and is what "temporarily owns" describes).

**Rationale:** This is the same "capability before abstraction" discipline this platform has applied at least six times before (GOV-004, GOV-012, the original `EngineAPI` retirement) — an abstraction is not built until concrete implementation evidence demonstrates it is needed, regardless of how architecturally tidy it would look built in advance.

**Extraction threshold, finalized (refines Section 8's original formulation):** orchestration extraction into a dedicated component should be reconsidered only when implementation evidence demonstrates one or more of:

- three or more content Collectors typically participate in an Assessment
- Collector applicability becomes significantly complex
- Collector execution order becomes configurable
- parallel collection becomes desirable
- multiple Assessment entry points require identical orchestration logic

Until one of these is demonstrated by real implementation pressure, not anticipated in the abstract, `AssessmentService` remains the orchestration boundary.

**Evidence philosophy:** unchanged, reconfirmed. Collectors observe; Evidence records facts; Rules evaluate Evidence; Findings express meaning; Recommendations express action. Sprint 7 must preserve this separation — nothing in the approved architecture puts any pressure on it (Section 7's analysis holds without modification).

**Implementation implications:** `AssessmentService::execute_from_assessment_input` gains a second, independent Collector invocation, composed inline — no new type, trait, or module. XML inspection is **the first implementation of Multi-Source Evidence Collection**, not a special case of its own — the architecture is general; XML is simply where it is first exercised. No change to `AssessmentService`'s public entry points. No change to `RuleEngine::evaluate`'s input shape. No change to `AssessmentReport`.

---

Architecture approved. Awaiting Chief Architect authorization for Sprint 7 implementation, per `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`'s own remaining open questions (Rule Engine timing, dependency selection, absence-as-evidence). No implementation has begun.
