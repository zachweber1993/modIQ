# Engineering Release 2.1

| Property | Value |
|---|---|
| **Release** | 2.1 |
| **Documentation Release** | 2.1 (Frozen — unchanged this capability; no specification amendment) |
| **Milestone** | C2 complete (Declared Dependency Interpretation) — the Rule Engine gains its fifth Rule, `DeclaredDependencyDuplicationRule`, asking a question of a mod's own declared-dependency content for the first time |
| **Scope** | The work `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §3 authorizes: one new Rule in `modiq-rules`, dispatched from `RuleEngine::evaluate` as a fifth, fixed-order Rule — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_2.0.md` (C1) |
| **Governing ADRs** | None new |
| **Governing Documents** | `docs/engineering/CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_IMPLEMENTATION_PLAN_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_IMPLEMENTATION_REPORT_DECLARED_DEPENDENCY_INTERPRETATION.md` |

---

## 1. Executive Summary

C2 closed the gap its own Capability Definition named directly: `XmlCollector` has produced `XmlInspection` Evidence for every declared `<dependency>` element since Sprint 7, but no Rule ever asked a question of its content — it was reachable only through `EvidencePresenceRule`'s undifferentiated "any Evidence exists" Finding, indistinguishable there from any other Evidence item in the same Assessment. `DeclaredDependencyDuplicationRule` is now the platform's fifth Rule: it detects a declared dependency name repeated more than once within one manifest and produces a Finding specifically about that fact.

This is the second post-foundation capability, and the first to reach a real, previously-untested architectural question rather than apply already-settled precedent by construction (C1's own basis). `SPRINT12_ARCHITECTURAL_RESOLUTION.md` had named, and left open, whether two Rules sharing one already-interpreted `EvidenceCategory` (`XmlInspection`) composes safely under GOV-012 alone. C2 is the concrete instance that question required: its own dedicated Architecture Evaluation and Architectural Resolution interpreted GOV-012 as already governing the dispatch question, and adopted a new, narrowly-scoped Architectural Constraint — Rule Conclusion Non-Contradiction — as the answer to the part GOV-012's text did not already settle. Implementation then directly demonstrated that constraint against real code, not merely by textual argument.

No Planning defect occurred during C2, unlike C1. The Implementation Plan's own wording did surface two drafting ambiguities during a Technical Director review of Phase 2 — whether a Plan-directed test rename counts as "altering" an existing test, and whether Evidence cardinality for a multi-item Rule should be stated explicitly — both resolved without reopening architecture and recorded as Lessons Learned in `C2_IMPLEMENTATION_REPORT_DECLARED_DEPENDENCY_INTERPRETATION.md`, not as Planning defects.

---

## 2. Authorization and Planning Summary

C2 followed a five-document lineage before implementation began, longer than C1's own three-document path because C2 reached a genuinely untested architectural question C1 did not: Capability Definition (`7c7faf8`) → Architecture Evaluation (`2cf8ba8`) → Architectural Resolution (`849eed6`) → Implementation Authorization (`4f546fb`) → Implementation Plan (`208a755`). Each was independently re-verified against current repository source rather than trusted from the document preceding it, per this repository's own standing discipline.

The Architecture Evaluation determined that `RuleEngine::evaluate`'s dispatch mechanism contains no concept of category ownership to enforce a cardinality limit with — a second `XmlInspection`-interpreting Rule introduces no structural risk. The Architectural Resolution then adopted two decisions: **Decision 1**, that GOV-012's own general text already governs shared-category dispatch, requiring no amendment; and **Decision 2**, the Rule Conclusion Non-Contradiction Constraint — when two Rules independently interpret the same `EvidenceCategory`, no single Evidence item may support mutually contradictory specific conclusions from both, though non-contradictory overlap is not itself barred. Decision 2 constrains Rule *authorship*, not dispatch, and introduces no new runtime mechanism.

The Implementation Plan decided the one design question the Authorization deliberately left open — what specific judgment the new Rule asks — selecting duplicate-name detection within one manifest as the only single-mod-scoped, evidence-groundable judgment reachable from what `XmlCollector` already produces, and split implementation into three phases: Rule construction (standalone, untested by dispatch), dispatch wiring with a direct Non-Contradiction demonstration, and final reverification.

---

## 3. Implementation Summary

Three phases, executed in sequence, each independently gated and independently reviewed — extending C1's own per-phase Repository Validation Review practice, which C1's release recorded as an observation rather than a mandatory stage:

1. **Phase 1 — Rule Construction, Standalone and Unit-Tested** (`e5d931b`). `DeclaredDependencyDuplicationRule` introduced in `crates/modiq-rules/src/rules/declared_dependency_duplication_rule.rs`: filters `XmlInspection` Evidence for the `"modDesc.xml declares dependency: "` prefix, determines which declared names recur via a two-pass algorithm that never relies on `HashMap` iteration order for the resulting Finding's own `evidence_ids` or summary text, and produces one `Warning`-severity, `Structure`-dimension Finding referencing every occurrence of every duplicated name, paired with an inline-authored Recommendation. Declared in `rules/mod.rs`; not yet dispatched. Seven new tests.
2. **Phase 2 — Dispatch Wiring and Non-Contradiction Demonstration** (`ce21006`). One new `use` import and one new, fifth dispatch line in `RuleEngine::evaluate`, appended after `RuntimeLoadFailureRule`'s, preserving the four existing Rules' declaration order. A dedicated test calls `VersionCompatibilityRule::evaluate` and `DeclaredDependencyDuplicationRule::evaluate` independently against one shared Evidence set and asserts zero overlap in their respective `evidence_ids` — a direct, code-level demonstration of Decision 2, not a textual argument. Three new tests plus one Plan-directed rename-and-extension of an existing dispatch-composition test.
3. **Phase 3 — Final Reverification.** Zero files changed; root workspace, `apps/sandbox/src-tauri`, and `apps/console`'s `npm run build` all reconfirmed clean; a full-span `git diff --stat` confirmed only the three files Plan §4 named were touched across the entire capability.

No change to `AssessmentService`'s public entry points, `RuleEngine::evaluate`'s own signature, `XmlCollector`, `EvidenceCategory`'s closed set, or any `modiq-*` crate other than `modiq-rules`, at any phase.

---

## 4. Repository Impact

| Area | Change |
|---|---|
| `modiq-rules` (`crates/modiq-rules/src/rules/`) | `declared_dependency_duplication_rule.rs` (new, 302 lines / 302 insertions); `mod.rs` (+2 insertions, additive `pub mod`/`pub use`); `engine.rs` (+121/−12 — one new import, one new dispatch line, one doc-comment sentence, four new/extended tests) |
| `modiq-runtime`, `modiq-collection`, `modiq-versioning`, `modiq-report`, `modiq-engine`, `modiq-storage`, `modiq-cli`, `modiq-knowledge` | **Unmodified** — confirmed via empty diff across the full capability span (`208a755`..`ce21006`) |
| `apps/console`, `apps/sandbox` | **Unmodified** — confirmed via empty diff |
| `AssessmentService` public entry points | Unaffected |
| Every `Cargo.toml` / `Cargo.lock`, `package.json` / `package-lock.json` | **Unmodified** — zero new dependency edge, confirmed via empty diff |
| Governance Register | Unaffected |
| ADRs | None new |
| New documents | `docs/engineering/CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_IMPLEMENTATION_PLAN_DECLARED_DEPENDENCY_INTERPRETATION.md`, `docs/engineering/C2_IMPLEMENTATION_REPORT_DECLARED_DEPENDENCY_INTERPRETATION.md`, this release document |

Total source impact across the full capability (`git diff --stat 208a755 ce21006`): 3 files changed, 425 insertions, 12 deletions.

---

## 5. Validation Status

```
Root workspace (default-members, 9 crates):
cargo fmt --check       → clean
cargo check --workspace → clean
cargo test              → 271 → 281 passing (modiq-rules: 36 → 46; every other default-member crate unchanged)

Full workspace (--workspace, 10 members):
cargo test --workspace  → 278 → 288 passing

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check → clean
cargo check       → clean
cargo test        → 9/9 passing, unaffected

apps/console (TypeScript):
npx tsc       → clean
npm run build → clean (tsc && vite build), unaffected — no apps/console file touched by C2

Dependency-edge check (all Cargo.toml/Cargo.lock, package.json/package-lock.json): zero matches
Dispatch-shape check: RuleEngine::evaluate confirmed, by direct inspection, to remain five sequential
`if let Some(outcome) = ... { outcomes.push(outcome); }` statements — no trait, registry, or dispatch table
Non-Contradiction check: DeclaredDependencyDuplicationRule's and VersionCompatibilityRule's content-shape
filters confirmed, by direct string comparison, to diverge at their 24th character — no Evidence item can
satisfy both
```

All figures re-verified fresh at this release's own drafting, independent of phase-report or Implementation-Report figures.

---

## 6. Engineering Observations

Recorded as evidence only — no process change, new governance mechanism, or Governance Register item follows from any observation below.

**Observation 1 — a genuinely untested architectural question, named three Sprints earlier, was resolved by a real instance rather than in the abstract.** `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §8 stated plainly that no historical decision had yet tested two Rules sharing one already-interpreted `EvidenceCategory`, and cautioned against resolving it through prospective certainty-manufacturing. C2's own Architectural Resolution treated C2 as exactly the forcing function that caveat required, adopting the Rule Conclusion Non-Contradiction Constraint only once a real second Rule existed to design against, and Phase 2's own test then checked that Constraint against real code rather than assuming it. This is the second consecutive capability (after C1's own closeout-predicted-follow-on) where a previously-named, deliberately-deferred repository question was taken up on real evidence rather than left further unresolved.

**Observation 2 — two Plan-drafting ambiguities were found and handled without reopening architecture or the approved Plan.** During Phase 2, a Technical Director review identified that Plan §6's own instruction to produce a renamed, extended dispatch-composition test sat in tension with Plan §8's completion criterion that "no existing test is altered, only extended," and that Plan §6's prose left a new Rule's own required Evidence cardinality (two items, not one) to be inferred rather than stated. Both were resolved by following the Plan's more specific instruction, disclosed rather than silently decided, and independently reviewed across multiple Technical-Director-style passes — implementation-session review history, not committed repository artifacts. Neither is a Planning defect in the sense C1's own Phase 2/3 boundary was: neither made any stated verification gate unsatisfiable, and neither required reconciling the committed Plan itself. Both are recorded as Lessons Learned in `C2_IMPLEMENTATION_REPORT_DECLARED_DEPENDENCY_INTERPRETATION.md` for future Plan drafting, not adopted as a repository convention on this single instance.

**Observation 3 — a documentation-currency gap, consistent with pre-existing repository pattern, not introduced by C2.** `declared_dependency_duplication_rule.rs`'s own module doc comment still states the Rule is "Not yet reachable from `RuleEngine::evaluate`," accurate when written in Phase 1 but stale since Phase 2's dispatch wiring — and no phase of the committed Plan authorized touching that file again to correct it. The identical phrasing already existed, uncorrected, in `structural_duplication_rule.rs` and `runtime_load_failure_rule.rs` since Sprint 5 and Sprint 11. C2 is a third instance of an existing pattern, not a new one.

---

## Outstanding Limitations / Reserved Responsibilities

Named, not resolved, exactly as `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` §6–§7 and `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §5 already name them — none of the three is created or newly recommended by this release; each is carried forward from committed history, confirmed unchanged as of this document's own drafting:

- **A `RuleEngine.md` amendment recording Decision 2** (the Rule Conclusion Non-Contradiction Constraint) as architecture text, mirroring `RuleEngine.md`'s own existing Initiative 1 AC-2/AC-4 citation pattern — recommended by the Architectural Resolution, not performed by it or by any phase of implementation. Confirmed unchanged: `RuleEngine.md` carries no reference to C2 or Decision 2.
- **A GOV-012 cross-reference, in `GOVERNANCE.md`, to the Architectural Resolution** — recommended as future housekeeping, not performed. Confirmed unchanged: `GOVERNANCE.md`'s GOV-012 entry carries no such reference.
- **`SPRINT12.md`'s and `SPRINT12_ARCHITECTURAL_RESOLUTION.md`'s own Historical Validation tables** — the Architectural Resolution's Decision 3 named a future, small update citing whatever Rule eventually implements C2 as the confirming (or correcting) fifth data point, deferred until a real, shipped instance exists. C2 is now that instance; the table update itself remains unperformed. Confirmed unchanged: neither document references C2.

---

## 7. Technical Director Assessment

**C2 is the first post-foundation capability to reach and resolve a real, previously-open architectural question, rather than apply an already-settled principle by direct construction.** C1's own basis was `FrontendArchitecture.md`'s Consumer-Owned State, adopted and already exercised without reinterpretation at Sprint 23; C2 instead required its own Architecture Evaluation and Architectural Resolution because `SPRINT12_ARCHITECTURAL_RESOLUTION.md` had explicitly left the "second Rule, same category" case untested. That resolution — GOV-012 already governs dispatch; a new, narrow Rule Conclusion Non-Contradiction Constraint governs authorship — was then directly exercised, not merely cited, by Phase 2's own dispatch wiring and its dedicated non-contradiction test.

**Every phase was independently reviewed, adversarially, more than once in Phase 2's case** — a Repository Validation Review after Phase 1, another after Phase 2, a Commit Readiness Verification before each commit, and the Implementation Report itself underwent a full Review → Reconciliation → Post-Reconciliation Verification cycle before being committed. No engineering, architectural, repository-consistency, or governance-consistency defect survived any of these passes. The only findings that did survive were cosmetic (a misattributed per-file insertion count, an imprecise review-pass citation, a self-referential "repository clean" claim) — each corrected, each independently reverified afterward.

**Risk: low.** No Planning defect occurred. No architectural document was reopened. The new Rule's own severity and dimension assignments were explicitly qualified, in the Implementation Plan itself, as design-time proposals consistent with `DataModel.md`'s Finding Severity discipline rather than evidence-compelled conclusions — the correct honesty level for a judgment this repository's own governing documents deliberately left open.

---

## 8. Final Release Recommendation

**C2 is complete.** All three phases implemented, verified, and independently reviewed at every stage; every Authorization exclusion remains absent (no cross-mod resolution, no `DependencyResolution` activation, no `XmlCollector` change, no consumer-transport change, no GOV-013 reopening); the Rule Conclusion Non-Contradiction Constraint was directly demonstrated, not assumed; no Planning defect occurred.

**Recommend:** Chief Architect final approval of this release.

---

## 9. Repository Status

C2 is complete. Root workspace `cargo fmt --check`, `cargo check --workspace`, and `cargo test` are clean at 281/281 (271 → 281, `modiq-rules` only); the full workspace is clean at 288/288 (278 → 288); Sandbox remains clean at 9/9; `apps/console`'s `npx tsc` and `npm run build` are both clean and unaffected. No Governance Register item or ADR resulted from this capability. Implementation is complete on `feature/runtime-implementation` (`e5d931b`, `ce21006`, `35a5040`), 18 commits ahead of origin and not yet pushed as of this document's own drafting. The repository is ready for its next engineering objective, not yet scoped by this document — expected to be a Capability Definition (or, if C3's own identity requires re-confirmation, a return to `CAPABILITY_PORTFOLIO_ASSESSMENT.md`) for the next capability.

---

## 10. Repository Timeline

```
Engineering Release 2.0 — C1 complete
        ↓
Capability Definition C2 (7c7faf8)
        ↓
Architecture Evaluation C2 (2cf8ba8) — the "second Rule, same category" question, left open since Sprint 12
        ↓
Architectural Resolution C2 (849eed6) — GOV-012 interpreted as already governing dispatch;
Rule Conclusion Non-Contradiction adopted
        ↓
C2 Implementation Authorization (4f546fb)
        ↓
C2 Implementation Plan (208a755)
        ↓
C2 Phase 1 — Rule Construction, Standalone and Unit-Tested (e5d931b)
        ↓
C2 Phase 2 — Dispatch Wiring and Non-Contradiction Demonstration (ce21006)
        ↓
C2 Phase 3 — Final Reverification (verification-only, no commit)
        ↓
C2 Implementation Report (35a5040)
        ↓
Engineering Release 2.1 — C2 complete
```
