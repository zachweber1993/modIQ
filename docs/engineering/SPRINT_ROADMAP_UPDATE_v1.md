# Sprint Roadmap Update v1

| Property | Value |
|---|---|
| **Document** | SPRINT_ROADMAP_UPDATE_v1.md |
| **Project** | modIQ |
| **Purpose** | Incorporate the resolved Runtime Log Interpretation workflow decision into the Sprint roadmap; recommend formal Sprint 9 and Sprint 10 commitments |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `ad29fd6` (Sprint 8 formally closed); 205/205 tests, working tree otherwise clean |
| **Inputs** | `POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, Chief Architect's product workflow decision (this session) |
| **Status** | Roadmap maintenance and planning only. No implementation, no code, no documentation modification, no governance work, no ADR. Awaiting Chief Architect authorization to begin Sprint 9. |

---

# 1. Roadmap Update Summary

The Chief Architect has resolved the one open question the Architecture Evaluation surfaced: Runtime Log Interpretation will support a single, bundled Assessment submission — the log is Evidence-bearing content reached by a new Collector against the *same* Assessment Input every structural Collector already inspects, directly mirroring `XmlCollector`'s own Sprint 7 introduction. No second Assessment Input, no standalone Runtime Log Assessment, no cross-Assessment correlation. This closes every architectural question the prior evaluation left open.

**This resolution changes Runtime Log Interpretation's own risk and confidence rating. It does not change the roadmap's ordering.** Sprint 9 remains Repair Guidance; Sprint 10 is now formally committed to Runtime Log Interpretation, upgraded from a tentative candidate to a de-risked, high-confidence Sprint. Sections 3–6 explain why the resolution moves confidence without moving rank.

---

# 2. Resolved Planning Decisions

Recorded as inputs to this roadmap update, not re-litigated:

- **Workflow:** Runtime Log Interpretation is Option A from `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md` — a new Collector against the existing, single Assessment Input, the same architectural shape as `XmlCollector`. Options B (second Assessment Input) and C (standalone Runtime Log Assessment) are both closed; neither is planned.
- **Correlation:** No cross-Assessment correlation mechanism is required or planned. A single Assessment, given a bundled submission, can already correlate log-derived Evidence with manifest-derived Evidence in one Rule, since `RuleEngine::evaluate` already consumes a flat, provenance-blind `&[Evidence]` slice — this was true before this decision and required no new mechanism to become usable.
- **Architectural uncertainty:** Resolved. The Architecture Evaluation's own comparative table already rated Option A "Low complexity" and "High repository consistency" before this decision — the decision selects among already-evaluated options rather than requiring new architectural work to evaluate a previously-unexamined shape.
- **Out of scope for this update:** the full capability inventory (`POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Sections 2–4) and every other candidate's own ranking are unchanged and not re-evaluated here, per instruction.

---

# 3. Updated Capability Rankings

Only the two candidates affected by this session's decision are re-examined; the remainder of the Capability Prioritization Study's own ranking (Lua Static Analysis, Dependency Resolution, Asset Validation, a second Version Profile, Report Improvements, Performance Observations) is carried forward unchanged.

| Rank | Capability | Change since the Prioritization Study |
|---|---|---|
| 1 | **Repair Guidance** (paired with minimum `modiq-knowledge`) | **Unchanged.** Nothing in this session's decision touches `modiq-knowledge`, `RepairRecipeReference`, or Rule Recommendation quality. Its case rests on smallest architectural footprint and platform-wide leverage — properties independent of Runtime Log Interpretation's own workflow question. |
| 2 | **Runtime Log Interpretation** | **Confidence raised; rank unchanged.** Previously ranked second specifically because it "carries a genuinely unprecedented architectural question... that Repair Guidance does not" (`SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, Section 6). That question is now closed. It is no longer an open-question candidate — it is a scoped, low-risk, precedented capability awaiting its own Capability Definition. |
| 3+ | *(unchanged — not re-evaluated this session)* | Lua Static Analysis, Dependency Resolution, a second Version Profile, Asset Validation, Report Improvements, Performance Observations retain their prior order and reasoning in full. |

**Why rank 1 and 2 do not swap.** The reason Repair Guidance outranked Runtime Log Interpretation was never solely the latter's architectural risk — it rested on two independent properties untouched by today's decision: Repair Guidance needs no new Collector at all (it activates `RepairRecipeReference`, dormant since Sprint 2, via one existing Rule), and its leverage compounds across *every* Rule's output, present and future. Runtime Log Interpretation, even fully de-risked, still requires a new Collector and a new Rule — a larger absolute engineering footprint than Repair Guidance's own minimum-viable slice, and its leverage — while real, especially once it can correlate against manifest Evidence in the same Assessment — is a strong new detection capability rather than a platform-wide quality improvement. Removing risk from Runtime Log Interpretation makes it a much stronger *second* choice; it does not add the properties that made Repair Guidance first.

---

# 4. Sprint 9 Commitment

**Sprint 9 should now be formally committed to Repair Guidance.**

- **Why this capability should begin next.** It directly deepens the most weakly-served of `Vision.md`'s own three founding Assessment questions — "what can I do next?" — for every Assessment the platform already produces, not only future ones. It has the smallest architectural footprint of any ranked candidate: no new Collector, no new `EvidenceCategory`, no change to `AssessmentService`'s public entry points.
- **Expected repository impact.** `modiq-knowledge` advances from L1 (pure scaffold, unimplemented since Sprint 0) to real content for the first time in the platform's history. At least one existing Rule's `Recommendation` gains a real `Some(RepairRecipeReference)` in place of `None`. One new dependency edge expected (`modiq-rules` → `modiq-knowledge`), a direct parallel to Sprint 8's own `modiq-rules` → `modiq-versioning` edge.
- **Expected product impact.** Players and Creators receiving a `Warning` Finding today (declared-version mismatch, structural duplication) receive generic boilerplate text; this capability replaces that with real, specific repair guidance — the direct realization of `Vision.md`'s belief that "software should educate its users rather than simply produce results."
- **Architectural readiness: High.** `RepairRecipeReference` already exists, already tested, already wired through `Recommendation::new`'s constructor since Sprint 2 — this activates dormant, already-designed scaffolding rather than building new architectural surface, the same shape Sprint 8 proved out for `modiq-versioning`.
- **Engineering readiness: High.** This is architecturally the least novel candidate in the inventory — "give a zero-implementation scaffold crate its first minimum-viable real content, consumed by exactly one existing Rule" is precisely Sprint 8's own template, applied to a different crate.
- **Implementation risk: Low–Medium.** The named risk remains scope creep — building out `KnowledgeModel.md`'s full conceptual model (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) all at once, rather than the single minimum-viable `RepairRecipe` type this recommendation scopes. Sprint 9's own Capability Definition should name this explicitly out of scope, mirroring Sprint 8's own "one hardcoded profile, not exhaustive version knowledge" discipline.

**This confirms, and does not change, the recommendation already made in `POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Section 6.**

---

# 5. Sprint 10 Commitment

**Runtime Log Interpretation is now sufficiently de-risked to become the formally planned Sprint 10 capability.**

The bar for treating a future Sprint as committable, rather than tentative, is that its architecture and workflow no longer contain an open question requiring its own resolution before implementation can be responsibly scoped. Before this session, Runtime Log Interpretation failed that bar — `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`'s own Recommendation stated it "warrants its own Capability Definition before any Architecture Evaluation of implementation mechanics" specifically because the workflow question was open. That question is now closed by explicit Chief Architect decision: bundled submission, Option A, no correlation mechanism needed.

What remains before Sprint 10 begins is ordinary Capability Definition scoping (which specific log error signatures to recognize first, matching the same "smallest real slice" discipline every prior Collector has followed) — not an open architectural question. This is the same category of remaining work every other committed Sprint has had at this stage (Sprint 8's own Capability Definition still had to pick a specific `descVersion` value and a minimum profile shape, despite its own architecture already being resolved in principle) — normal Sprint-entry scoping, not roadmap-level uncertainty.

**Sprint 10 is therefore committed in principle, contingent only on Sprint 9's own successful closeout**, consistent with this project's standing practice of not authorizing a future Sprint's implementation before the current one is complete.

---

# 6. Roadmap Confidence Assessment

**What uncertainty was removed.** The Architecture Evaluation identified a single load-bearing uncertainty: whether Runtime Log Interpretation required expanding the Assessment model to accept more than one Assessment Input, or whether it could be built as a Collector against the existing, single-input model. That question — not a broader architectural risk, but this one specific, well-defined fork — is what the Chief Architect's decision resolves.

**Why the ranking remains unchanged.** The uncertainty that was removed bore on Runtime Log Interpretation's *risk*, not on Repair Guidance's *case*. Repair Guidance's rank-one position was justified by its footprint and leverage relative to *every* candidate in the inventory, not specifically by comparison to Runtime Log Interpretation's own risk level. Removing risk from one competitor does not add footprint-reduction or leverage-breadth to another — the two properties are independent, and only the latter determined Repair Guidance's rank.

**Why confidence increased.** Before this session, any roadmap statement about Sprint 10 would have required a hedge — "Runtime Log Interpretation, pending resolution of its own architectural question." That hedge is now removable. A capability whose implementation shape is already known to be low-complexity and precedented (per the Architecture Evaluation's own comparative table) can be committed to a specific Sprint slot with the same confidence any other de-risked, evidence-grounded capability receives — exactly the standard this project has applied to every Sprint since Sprint 3's original Evidence Collection boundary work.

---

# 7. Chief Architect Questions

No architectural questions genuinely remain from this session's own work — the Architecture Evaluation's four prior questions are each closed by the decisions recorded in Section 2 above (workflow, correlation, and standalone-Subject treatment were all directly answered; the fourth, whether this needed its own Capability Definition, is answered by this session itself having performed the equivalent scoping decision). One minor, process-level question is worth naming rather than assuming:

1. Should Sprint 10's own Capability Definition (naming the specific minimum-viable log signatures to recognize first) begin only after Sprint 9's own Repository Closeout, or may it be prepared in parallel once Sprint 9 implementation is authorized — mirroring how Sprint 8's own Capability Definition referenced Sprint 7's still-fresh closing recommendation? This is a scheduling preference, not an architectural question, and does not block Sprint 9 authorization either way.

---

Awaiting Chief Architect authorization to begin Sprint 9. No implementation, documentation change, governance item, or ADR has been made this session.
