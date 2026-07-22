# Sprint 8 Implementation Authorization

| Property | Value |
|---|---|
| **Document** | SPRINT8_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Validate the approved Sprint 8 architectural decisions against current repository evidence, produce an implementation roadmap, and recommend (not grant) implementation authorization |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `61905aa`, working tree clean except three untracked Sprint 8 planning documents |
| **Status** | Validation and planning only. No code changed, no documentation changed, no commits, no branch, no Governance Register item, no ADR. Chief Architect Decisions 1–6 are treated as authoritative throughout and are not altered anywhere below. Awaiting explicit Chief Architect implementation authorization. |

---

# 1. Repository Validation

Re-verified directly this session, not carried forward from `SPRINT8_ARCHITECTURAL_RESOLUTION.md`.

- `git status`: branch `feature/runtime-implementation`, up to date with `origin`. Only the three Sprint 8 planning documents (`SPRINT8_INITIALIZATION_REPORT.md`, `SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `SPRINT8_ARCHITECTURAL_RESOLUTION.md`) are untracked. `git diff --stat HEAD` shows zero modifications to any tracked file. **No production code or existing documentation has changed since the Architectural Resolution session.**
- `cargo test --workspace`: **187/187 passing**, identical distribution to every prior record (`modiq-runtime` 82, `modiq-collection` 56, `modiq-engine` 18 + 3 integration, `modiq-rules` 15, `modiq-cli` 10, `modiq-report` 3, `modiq-knowledge`/`modiq-versioning`/`modiq-common` 0 each).
- `crates/modiq-runtime/Cargo.toml`: `[dependencies]` contains only `thiserror` (external). **`modiq-runtime` has zero dependencies on any other workspace crate — confirmed directly, and not previously stated this precisely in the Architectural Resolution.** This fact is central to Section 2's validation of Decision 1, below.
- `crates/modiq-runtime/src/assessment/rule_reference.rs` and `repair_recipe_reference.rs`: both confirmed, by direct re-inspection, to be simple opaque wrappers (`struct RuleReference(String)`, `struct RepairRecipeReference(String)`) defined entirely within `modiq-runtime`, requiring no dependency on `modiq-knowledge` (the domain each identifies). This is the established, working precedent for how `Assessment`-owned entities reference another domain's concepts without depending on that domain's real types (`ADR-0007`: Opaque Runtime References).
- `crates/modiq-versioning`, `crates/modiq-engine`, `crates/modiq-rules` `Cargo.toml`s: unchanged since the prior session's inspection — `modiq-versioning` has zero dependencies; neither `modiq-engine` nor `modiq-rules` depends on it.

No repository drift has occurred between the Architectural Resolution session and this one. All validation below is against the current, unchanged repository state.

---

# 2. Validation of Chief Architect Decisions

For each decision: confirmation against repository evidence, and any newly-identified implementation consequence. **No decision is altered. Where this validation surfaces something not fully captured at Architectural Resolution, it is named as a refinement for implementation planning, not a challenge to the decision itself.**

## Decision 1 — Version Profile Ownership: APPROVED, no conflict, one refinement

Repository evidence fully supports this decision (`ADR-0003`'s ownership list omitting Version Profile; `DataModel.md`'s explicit ownership exception) exactly as validated at Architectural Resolution.

**Refinement identified this session:** Decision 1's wording — *"a first-class entity owned by Assessment"* — is compatible with two different implementation shapes, and the difference matters structurally:

- **(a) `Assessment` holds the real `modiq_versioning::VersionProfile` type directly.** This requires `modiq-runtime` to gain its first-ever dependency on another workspace crate — breaking a zero-workspace-dependency leaf status confirmed unbroken since Sprint 0 (Repository Validation, above) — and departs from `ADR-0007`'s own established Opaque Runtime References pattern, which exists specifically for this situation: a Runtime entity needing to refer to something owned by another domain.
- **(b) `Assessment` holds a new, `modiq-runtime`-local opaque `VersionProfileReference` type**, structurally identical to `RuleReference`/`RepairRecipeReference` (a `String`-wrapping identifier, defined and owned entirely within `modiq-runtime`). This satisfies Decision 1's wording exactly — Version Profile is still a first-class, Assessment-owned relationship, distinct from Context — while requiring zero new dependency on `modiq-runtime`'s part, and following the platform's only existing precedent for this exact category of relationship without exception.

**Recommendation for implementation:** (b). This is not a reopening of Decision 1 — both readings satisfy it — but a Lead-Engineer-level representational choice of the same kind GOV-011 and GOV-012 both left open explicitly (*"the exact Rust representation... is an implementation detail, not fixed by this resolution, provided implementation faithfully realizes the policy"*). Choosing (b) also directly serves Decision 5's own instruction (*"only dependency relationships required by Sprint 8 shall be introduced"*): under (b), `modiq-runtime` needs no new dependency at all, and the real `VersionProfile` value lives only where it is actually evaluated (`modiq-engine`, `modiq-rules`) — the minimal footprint consistent with every prior Sprint's own "smallest real slice" discipline.

## Decision 2 — Version Extraction: APPROVED, no conflict

Fully confirmed. `XmlCollector` already parses the manifest into a `roxmltree::Document`; reading the root element's `descVersion` attribute is a direct, additive extension of `inspect()`, requires no new dependency, and stays strictly observational — `XmlCollector` reports the declared value as a fact; it does not compare it against anything, exactly as Decision 2 requires ("No compatibility interpretation shall occur within the Collector").

## Decision 3 — Version-aware Evaluation: APPROVED, no conflict, dependency direction confirmed

Fully confirmed, and this session's validation sharpens *why* `modiq-rules` needs the real `VersionProfile` type while `modiq-runtime` (per Decision 1's recommended shape) does not: the Rule Engine's job is to *evaluate* — it needs the real profile's actual query capability (e.g., "does this profile recognize `descVersion` 93?") to produce a Finding. `Assessment`'s own field, by contrast, only needs to *record which profile was used*, a traceability concern `VersionProfileReference` alone satisfies, exactly mirroring how `Finding` needs only `RuleReference` (an identifier) while the real evaluation logic lives entirely in `modiq-rules`, never in `modiq-runtime`.

## Decision 4 — Assessment Construction: APPROVED, no conflict with repository evidence; scope boundary clarified

Repository evidence does not contradict characterizing this as normal domain evolution — that characterization is a legitimate Chief Architect policy call, not a factual claim repository evidence could confirm or deny, and this session does not relitigate it.

**What this session clarifies, without altering the decision:** Decision 4 addresses `Assessment::new`'s own constructor specifically. It does not, by its own text, extend to `AssessmentService::execute`/`execute_from_assessment_input`'s public signatures — a materially different category under `GOVERNANCE.md`'s own Public API Policy (*"Public APIs are considered contracts. Breaking changes require governance approval"*). `Assessment::new` has never been designated a public API contract in that sense; `AssessmentService`'s two entry points explicitly have been, repeatedly, across GOV-008's five-Sprint deferral. Read together with Decision 6 (no Governance Register work this Sprint), this has a direct, non-conflicting implication: **evolving `AssessmentService`'s existing two entry points' signatures directly would be a breaking change requiring governance approval under `GOVERNANCE.md`'s own Public API Policy — approval Decision 6 does not authorize seeking this Sprint.** The two decisions are fully consistent, not in tension, provided implementation follows the same additive-entry-point pattern already established when `execute_from_assessment_input` was introduced alongside `execute` without replacing it. See Implementation Roadmap, Phase 4, below.

## Decision 5 — Crate Dependencies: APPROVED, no conflict; dependency set is leaner than previously estimated

Fully confirmed and, per Decision 1's refinement above, more precisely scoped than `SPRINT8_ARCHITECTURAL_RESOLUTION.md` estimated: only `modiq-engine → modiq-versioning` and `modiq-rules → modiq-versioning` are required (both directly justified — orchestration needs the real profile to receive it from the caller and forward it to the Rule Engine; the Rule Engine needs it to evaluate). `modiq-versioning → modiq-common` is not added, exactly as recommended and now confirmed. `modiq-runtime → modiq-versioning` is **not** required under the recommended opaque-reference shape (Decision 1) — a dependency this session's own deeper validation finds unnecessary, where the Architectural Resolution had not yet ruled it out.

## Decision 6 — Governance: APPROVED, no conflict; directly precedented

No conflict with repository evidence. This decision has direct precedent, not merely permission: Sprint 7's own Collector Composition Model was deliberately **not** opened as a Governance Register item, by explicit Chief Architect decision, specifically because *"Sprint 7 represents the first implementation evidence, not the final governance evidence"* (`ENGINEERING_RELEASE_0.7.md`, Governance Completed). Sprint 8's own Architectural Resolution session (this Sprint's equivalent of a Sprint-specific Decision Record, mirroring `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` Section 14's role for Sprint 7) already serves the function a Governance Register item would otherwise serve, without creating a permanent Register entry ahead of real implementation evidence. Decision 6 is this project's second application of the same, now-recurring pattern, not a departure from prior discipline.

**One consequence of Decision 6 worth stating plainly:** the `modiq-versioning` Crate Boundary Rule gap (`GOVERNANCE.md`) will remain unaddressed this Sprint, by explicit decision — a known, deliberately-accepted gap, not an oversight.

---

# 3. Implementation Roadmap

Ordered for dependency correctness and independent phase authorization, per this project's own standing practice. Each phase is expected to be separately validated (`cargo fmt`/`check`/`test`, both workspaces) before the next begins. Focused on sequence, not internal design detail.

## Phase 1 — `modiq-versioning`: minimum domain content

Give `GameVersion` and `VersionProfile` real fields for the first time since Sprint 0: a game-version identifier and a small, explicit set of recognized `descVersion` values, plus a query method (e.g., `supports`). No new dependency — `modiq-versioning` remains a true leaf. Independently testable; nothing yet consumes it.

## Phase 2 — `modiq-runtime`: opaque reference and `Assessment` construction

Add `VersionProfileReference` (opaque, `modiq-runtime`-local, mirroring `RuleReference`/`RepairRecipeReference` exactly). Evolve `Assessment::new` to accept it as a third parameter (Decision 4). Update every existing call site within `modiq-runtime`'s own test suite. No new dependency.

## Phase 3 — `modiq-collection`: declared-version extraction

Extend `XmlCollector::inspect` to extract `descVersion` as a new, purely factual `XmlInspection` Evidence item (found/absent/non-numeric-or-malformed-attribute cases), mirroring Sprint 7 Phase 3's exact Content Extraction discipline. No new dependency. Independently testable ahead of any Engine or Rule wiring.

## Phase 4 — `modiq-engine`: wiring

Add the `modiq-versioning` dependency. `AssessmentService` receives the real `VersionProfile` from its caller, derives the `VersionProfileReference` to pass into `Assessment::new`, and forwards the real `VersionProfile` to `RuleEngine::evaluate` (Phase 5). Per Decision 4's clarified scope (Section 2, above): **`execute` and `execute_from_assessment_input`'s existing signatures are not modified.** A new, additive entry point is introduced instead, mirroring exactly how `execute_from_assessment_input` was itself added alongside `execute` without replacing it — preserving both existing entry points as stable contracts and requiring no Public API Policy governance approval this Sprint, consistent with Decision 6.

## Phase 5 — `modiq-rules`: version-compatibility Rule

Add the `modiq-versioning` dependency. `RuleEngine::evaluate` gains a new parameter carrying the active `VersionProfile` (or an `Option` of one). A new Rule, following `StructuralDuplicationRule`'s exact shape, evaluates declared-version Evidence against it, producing a Finding — `Warning`, not `Error`, per `DataModel.md`'s Evidence-Based severity discipline (declared-version mismatch is a genuine concern, not conclusive proof of breakage).

## Phase 6 — `modiq-cli` / `apps/sandbox`: application-layer wiring

Add the `modiq-versioning` dependency to both. Each constructs the single, hardcoded FS25 `VersionProfile` and supplies it to the new entry point introduced in Phase 4 — mirroring exactly how both already construct `AssessmentSubject`/`AssessmentContext` today.

## Phase 7 — Closeout

Full-workspace validation (both workspaces, zero warnings), the standard living-document reconciliation (`PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`) that every prior Sprint's Closeout performs — distinct from, and not precluded by, Decision 6's specific Governance Register/ADR/Crate-Boundary exclusion — and an Engineering Release record produced at or near this Closeout, explicitly correcting the two-Sprint-running late-production pattern `ENGINEERING_RELEASE_0.7.md` itself named as a risk not to repeat a third time.

---

# 4. Crate Implementation Order

Strict dependency-respecting order, confirmed against the current, unchanged dependency graph:

1. **`modiq-versioning`** (Phase 1) — no prerequisites; true leaf.
2. **`modiq-runtime`** (Phase 2) — no new prerequisite; already a leaf, unaffected by Phase 1.
3. **`modiq-collection`** (Phase 3) — depends only on `modiq-runtime` (unchanged); can proceed in parallel with Phase 2, since it doesn't touch `Assessment` construction.
4. **`modiq-engine`** (Phase 4) — requires Phases 1 and 2 complete (needs both the real `VersionProfile` type and `Assessment`'s new constructor shape); also requires Phase 3's Evidence to exist for meaningful end-to-end testing, though not for compilation.
5. **`modiq-rules`** (Phase 5) — requires Phase 1 complete (needs the real `VersionProfile` type); can proceed in parallel with Phase 4, converging at Phase 4's own `RuleEngine::evaluate` call site.
6. **`modiq-cli` / `apps/sandbox`** (Phase 6) — requires Phase 4's new entry point to exist.
7. **Closeout** (Phase 7) — requires all preceding phases green in both workspaces.

`modiq-report` requires no phase of its own — it already depends on `modiq-runtime` and needs no new dependency to reflect `Assessment::version_profile_reference()` (or equivalent) in an `AssessmentReport`, exactly as it reflects every other Assessment field today, without analysis.

---

# 5. Risk Assessment

## Architectural risks

- **The Decision 1 implementation-shape refinement (Section 2) must be settled before Phase 2 begins**, not discovered mid-implementation — choosing shape (a) instead of the recommended (b) changes Phase 2's crate-dependency footprint and reopens the "does `modiq-runtime` gain a new dependency" question Decision 5 explicitly wants minimized.
- **The additive-entry-point approach (Phase 4) must actually remain additive.** The risk is subtle: it is easy, under implementation pressure, to "simplify" by changing `execute`/`execute_from_assessment_input` directly instead of adding a third path, which would silently require the governance approval Decision 6 does not authorize this Sprint. This should be an explicit Architectural Conformance Review checkpoint (Phase 4's own review), not assumed correct because tests pass.

## Engineering risks

- **First-ever cross-crate consumption of `modiq-versioning`.** No prior Sprint has wired this crate into anything; Phases 4 and 5 are simultaneously this crate's first real consumers, a genuinely novel integration surface compared to Sprint 7's "third Collector of an already-proven shape."
- **`RuleEngine::evaluate`'s signature change (Phase 5)** touches every existing caller of that method — verified this session to be exactly one production call site (`AssessmentService::execute`) plus its own unit tests — a small, well-contained blast radius, but real.

## Testing risks

- **Determinism of the new Finding** must be verified directly (per this project's own standing lesson from Sprint 5 Phase 5: an untested determinism claim is not a verified one) — specifically, that the same declared version against the same profile always produces the same severity and description, and that combining this Rule's outcome with the two existing Rules' outcomes preserves the already-established fixed dispatch order (GOV-012).
- **Severity-discipline testing**: an explicit test asserting the new Rule never assigns `Error` for an unrecognized-but-not-conclusively-broken declared version is worth writing deliberately, not left to incidental coverage, given `DataModel.md`'s explicit warning that a Rule "must never assign a severity stronger than what its Evidence conclusively establishes."

## Migration risks

Every existing call site of `Assessment::new` (all of `modiq-runtime`'s own test suite) and of `AssessmentService::execute`/`execute_from_assessment_input` used in tests requires updating for the new parameter — verified this session as a real, non-trivial but entirely mechanical footprint across `modiq-runtime`, `modiq-engine`, `modiq-cli`, and `apps/sandbox`'s own test suites. No behavioral risk, but real review-time cost.

## Backward compatibility considerations

Under the recommended additive-entry-point approach (Phase 4), `AssessmentService`'s two existing public entry points remain fully backward compatible — no existing consumer of `execute`/`execute_from_assessment_input` requires any change to keep working exactly as it does today, mirroring the zero-migration-required precedent `execute_from_assessment_input`'s own introduction already set. `Assessment::new` itself is not backward compatible under Decision 4 (by explicit Chief Architect decision) — this is confined to `modiq-runtime`'s own internal callers and `modiq-engine`'s single construction site, not exposed to any external consumer directly.

## Mitigation summary

Resolve the Decision 1 implementation-shape refinement explicitly before Phase 2 (recommended: opaque reference). Treat the additive-entry-point requirement as an explicit Phase 4 Architectural Conformance Review item, not an assumption. Write the determinism and severity-discipline tests as first-class, named test cases in Phase 5, not incidental coverage.

---

# 6. Builder Pattern Evaluation

**Comparing constructor evolution against a Builder pattern for `Assessment` construction, as Decision 4 requires.**

## The case for a Builder

`Assessment::new` will carry three required parameters after this Sprint (`subject`, `context`, and, per Decision 1, an opaque `VersionProfileReference`). A Builder could, in principle, offer more readable call sites for a growing parameter list, and support incremental, staged construction.

## The case against, evaluated against repository evidence

- **Every Runtime entity constructed so far already handles this many (or more) parameters with a plain, fallible constructor, and none has ever needed a Builder.** `Finding::new` takes four parameters (`severity`, `description`, `evidence_ids`, `rule_reference`); `Recommendation::new` takes three, including an `Option<RepairRecipeReference>` — direct, already-working precedent that optional-field handling and multi-parameter construction do not, on their own, justify departing from `ADR-0007`'s single-constructor pattern.
- **All three of `Assessment::new`'s parameters remain required, not optional.** A Builder's core value — avoiding a combinatorial explosion of constructor overloads for optional-field combinations — does not apply when every field is supplied together, every time, exactly as `subject`/`context` already are today.
- **`Assessment::new` performs no cross-field validation today**, and this capability introduces none. A Builder's other common justification — staged, incremental validation across fields before a final, fallible build step — has no forcing function here; `Evidence`/`Finding`/`Recommendation`'s own fallible constructors already demonstrate this platform's established mechanism for validation-at-construction, without a Builder.
- **Introducing a Builder for `Assessment` alone, while every other Runtime entity uses a plain constructor**, would itself be an inconsistency — the same class of unforced, one-off pattern deviation `ADR-0007` exists to prevent (*"a new entity can be reviewed against an established pattern rather than requiring its own design conversation"*).
- **This project's own "capability before abstraction" discipline applies directly**: a Builder is additional construction-flexibility machinery. Nothing in this capability's actual requirements — three required fields, no staged validation, no optional-combination explosion — demonstrates a need for that flexibility. Building it now would be the same speculative-scaffolding pattern this project has rejected at least seven times already (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, Section 8).

## Recommendation

**Retain constructor-based creation.** `Assessment::new` should evolve directly to accept the third parameter, following the exact shape its own two-parameter form and every other Runtime entity's constructor already use. A Builder pattern is evaluated and explicitly not recommended: it would solve problems this capability does not have (parameter growth into optional territory, cross-field staged validation), while introducing a construction-pattern inconsistency this platform's own entity design discipline has never had to tolerate before. Should a future capability introduce genuinely optional `Assessment` construction inputs — a real, concrete case, not a hypothetical one — that would be the evidence-based forcing function to revisit this question, exactly per this project's own standing evidentiary discipline (GOV-004, GOV-013).

---

# 7. Testing Strategy

Unchanged in discipline from every prior Sprint: real I/O, real fixtures, no mocks; `cargo fmt`/`check`/`test`, both workspaces, zero warnings, continuously through implementation.

**Required tests, by phase:**

- **Phase 1 (`modiq-versioning`):** its first tests ever — `VersionProfile::supports` (or equivalent) against recognized and unrecognized `descVersion` values.
- **Phase 2 (`modiq-runtime`):** `VersionProfileReference` construction/identifier-preservation tests, mirroring `RuleReference`'s own single test exactly; `Assessment::new`/`Assessment::version_profile_reference()` tests, mirroring the existing `subject()`/`context()` coverage pattern.
- **Phase 3 (`modiq-collection`):** real fixtures with a recognized `descVersion`, an unrecognized one, and none at all (mirroring the existing found/absent/malformed-manifest fixture shape), for both the directory and archive cases `XmlCollector` already supports.
- **Phase 4 (`modiq-engine`):** a full-pipeline test proving the new, additive entry point produces the expected `Assessment`/Report state; an explicit test confirming the two existing entry points (`execute`, `execute_from_assessment_input`) are unaffected — a regression guard, not an assumption.
- **Phase 5 (`modiq-rules`):** the new Rule's own unit tests, including the severity-discipline test named in Risk Assessment (never `Error` for an unresolved version mismatch); a determinism test proving repeated evaluation with the same declared version and profile produces identical output, per Sprint 5 Phase 5's own standing lesson.
- **Phase 6 (`modiq-cli` / `apps/sandbox`):** existing test suites updated for the new call-site shape; at least one new test in each confirming the version-compatibility Finding reaches the real, end-to-end consumer.
- **Phase 7:** full-workspace and Sandbox-workspace validation, zero warnings, as the closing gate.

---

# 8. Sprint Readiness

**READY FOR IMPLEMENTATION.**

All six Chief Architect Decisions are validated against current repository evidence with no factual conflicts. This session's own validation surfaced two implementation-shape refinements (Decision 1's ownership representation; Decision 4's scope boundary against `AssessmentService`'s own public entry points) — both are resolved here with clear, well-precedented recommendations (an opaque `VersionProfileReference`, mirroring `RuleReference`/`RepairRecipeReference`; a new additive `AssessmentService` entry point, mirroring `execute_from_assessment_input`'s own introduction), consistent with this project's own standing practice of leaving representational detail to implementation once policy is resolved (GOV-011, GOV-012). Neither refinement reopens or conflicts with any of the six approved decisions; neither requires further Architecture Evaluation before Phase 1 can begin.

No blocking issue remains. The Implementation Roadmap (Section 3) is phased for independent authorization and validation exactly as every prior Sprint has been.

---

# 9. Implementation Authorization Recommendation

This session recommends that Sprint 8 implementation be authorized, beginning with Phase 1 (`modiq-versioning` minimum domain content), proceeding through Phases 2–7 as sequenced in Section 3, each independently validated before the next begins.

This recommendation is not itself an authorization. Per this project's own Engineering Workflow (`PROJECT_HANDOFF_v1.0.md`, Section 5), Implementation Authorization is a distinct, explicit Chief Architect act — confirmed here as the next and only remaining step before Sprint 8's Implementation stage may begin.

---

Awaiting explicit Chief Architect implementation authorization. No code, documentation, Governance Register item, or ADR has been created or modified during this session.
