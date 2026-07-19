# Technical Director Engineering Handoff v2.0

**Engineering Release 0.2 · Sandbox Phase 2 · Sprint 3 Phase 1**

| Property | Value |
|----------|-------|
| **Document** | TECHNICAL_DIRECTOR_HANDOFF_v2.0.md |
| **Project** | modIQ |
| **Purpose** | Canonical engineering handoff — the starting point for the next engineering session |
| **Last Updated** | 2026-07-19 |
| **Branch** | `feature/runtime-implementation` |

---

## Executive Summary

modIQ is a platform for producing **deterministic, evidence-based, explainable assessments** of Farming Simulator mods (and related subjects — mod collections, savegames, maps, and future assessment targets). Its purpose is not to generate an opaque quality score but to answer three questions for every Assessment: does the mod work, why was this conclusion reached, and what can be done to improve it. Education and transparency are treated as first-class product goals, not secondary features.

The project's architectural philosophy is documentation-first and deliberately conservative:

- **Deterministic over automated.** Rule evaluation is authoritative and reproducible; AI may assist exploration in the future but never replaces deterministic reasoning (ADR-0005).
- **Evidence-based and explainable.** Every conclusion must be traceable to the Evidence that supports it (RuleEngine.md, Glossary.md).
- **Platform-first.** The core architecture is independent of any single Farming Simulator release; version-specific behavior is isolated into Version Profiles (ADR-0004), not yet integrated.
- **Documentation precedes implementation.** Architecture is frozen through a formal Documentation Release process before engineering work begins (GOVERNANCE.md, DocumentationRelease.md), and implementation is expected to raise conflicts with frozen specifications rather than resolve them silently.
- **Crate boundaries are load-bearing.** Runtime, Rule Engine, Reporting, Knowledge, and Engine orchestration are separate crates with narrow, enforced responsibilities (GOVERNANCE.md).

**Where the project stands today:** Documentation Release 2.0 is complete and frozen. Engineering Release v0.1.0-alpha (Sprint 1) delivered the first working, content-free pipeline. Engineering Release 0.2 (Sprint 2) gave the Runtime Domain's core entities real content, identity, and validation, and made `Assessment` responsible for resolving the relationships between them. A developer sandbox application (`apps/sandbox`) now exists as the platform's first real, non-test consumer. Sprint 3 Phase 1 just proved that sandbox can drive the complete, real pipeline — Evidence through Rule Engine through Assessment Report — with zero new domain logic. **Sprint 3 Phase 1's implementation is complete and verified, but is not yet committed to git and has not yet received explicit review sign-off in conversation — confirm this before proceeding.**

---

## Current Repository State

**`apps/`** — one application, `apps/sandbox`: a Tauri 2 + React 19 + TypeScript + Vite + TailwindCSS v4 + shadcn/ui developer sandbox. It is its own independent Cargo workspace (see Sandbox Architecture below) and is not a member of the root modIQ workspace. It currently has one Tauri command, `create_assessment`, which executes the real Assessment pipeline and returns a structured DTO. 3 Rust unit tests live in `apps/sandbox/src-tauri/src/lib.rs`.

**`crates/`** — eight crates, unchanged in count and boundaries since Sprint 0:

| Crate | Maturity | State |
|---|---|---|
| `modiq-runtime` | L3, heavily tested (84 tests) | Assessment aggregate; Evidence, Finding, Recommendation with real fields, identity, and validation; relationship resolution |
| `modiq-rules` | L3 for one rule | Deterministic `RuleEngine`; consumes Evidence, produces Finding + Recommendation with real content |
| `modiq-report` | L3 | `AssessmentReport` read-only snapshot |
| `modiq-engine` | L3 for `AssessmentService` | Orchestrates the full pipeline; four other EngineAPI services remain stubs |
| `modiq-knowledge` | L1, scaffold | Not connected to anything |
| `modiq-versioning` | L1, scaffold | Not connected to anything |
| `modiq-cli` | L1, scaffold | Not wired to `modiq-engine` |
| `modiq-common` | L1, scaffold | Unused; purpose undecided (GOV-003) |

**`docs/`** — constitutional (Frozen), architecture/technical (Frozen), implementation (mixed — some Frozen, some living), governance (living), engineering (living — logs, releases, handoff docs, governance register), ADRs (historical, Accepted, seven total), releases (new directory, holds `ENGINEERING_RELEASE_0.2.md`). Full inventory in Documentation Status below.

**Current branch:** `feature/runtime-implementation`.

**Current Engineering Release:** 0.2, tagged `v0.2.0-alpha` (verified: this tag points at the `docs(release): freeze Engineering Release 0.2` commit).

**Current Documentation Release:** 2.0, Complete. No frozen specification has changed since — Sprint 2 and Sprint 3 Phase 1 both required zero Documentation Release activity.

**Current Sprint status:** Sprint 3, Phase 1 (of an unbounded number of phases). Phase 1 objective — prove a real application can drive the existing pipeline end-to-end — is implemented and verified (see Repository Health). **Not yet committed to git; not yet explicitly approved in this conversation.**

**Sandbox status:** Phase 1 (ping/IPC bootstrap) and Phase 2 (empty-Assessment DTO) both approved and committed (commit `e62559b`). The Sprint 3 Phase 1 work substantially rewrote Sandbox Phase 2's `create_assessment` command in place — the sandbox now runs the real pipeline rather than an empty Assessment. There is no separately-tracked "Sandbox Phase 3"; the sandbox and Sprint 3 tracks converged at this point, and future sandbox work should be planned as Sprint 3 phases going forward.

---

## Platform Architecture

```
                 modiq-cli          apps/sandbox (Tauri)
                      │                    │
                      └─────────┬──────────┘
                                 ▼
                           modiq-engine
          ┌─────────┼──────────┬──────────┬──────────┐
          ▼         ▼          ▼          ▼          ▼
   modiq-runtime  modiq-knowledge  modiq-rules  modiq-versioning  modiq-report
          │                                 │
          └───────────────┬─────────────────┘
                          ▼
                    modiq-common
```

This is the full, intended hierarchy (per `CrateRoadmap.md`/`DependencyMap.md`). The edges actually declared today are a subset: `modiq-rules → modiq-runtime`, `modiq-report → modiq-runtime`, `modiq-engine → {modiq-runtime, modiq-rules, modiq-report}`, and now `apps/sandbox → {modiq-runtime, modiq-report, modiq-engine}` (a direct path dependency from its own, separate Cargo workspace — not a workspace member). `modiq-cli → modiq-engine`, `modiq-engine → {modiq-knowledge, modiq-versioning}`, and any edge into `modiq-common` remain undeclared, matching documented reality rather than aspiration.

**Ownership boundaries** (GOVERNANCE.md, enforced and verified throughout, never violated):

- **Runtime** (`modiq-runtime`) — owns `Assessment` and all state generated during its execution: identity, lifecycle, Evidence, Finding, Recommendation collections. Never evaluates rules, never generates reports, never owns Knowledge, never orchestrates.
- **Rule Engine** (`modiq-rules`) — owns deterministic evaluation. Consumes Evidence (and, eventually, Knowledge); produces Findings and Recommendations. Never mutates `Assessment` directly, never creates Evidence.
- **Knowledge** (`modiq-knowledge`) — will own reusable engineering knowledge (Rules, Repair Recipes, Best Practices, Engine Behaviors, Compatibility Patterns, Known Issues), independent of any Assessment. Currently scaffolded only.
- **Reporting** (`modiq-report`) — owns `AssessmentReport`, an immutable snapshot. Performs no analysis; reflects state that already exists.
- **Engine** (`modiq-engine`) — owns orchestration and execution flow. `AssessmentService::execute` composes Runtime, Rule Engine, and Reporting into one callable operation. Never owns runtime state, never implements business rules, never generates reports itself.
- **CLI** (`modiq-cli`, planned) — will own user-facing command invocation, wired to `modiq-engine`. Never business logic.
- **Sandbox** (`apps/sandbox`) — a thin, non-reusable application. Consumes the platform exactly as any future application (including the eventual CLI) would: through public APIs only, via Tauri commands. Owns zero domain logic.

**Dependency direction** is strictly downward: `modiq-runtime` is the leaf (depends on nothing internal). No circular dependency has ever existed in this codebase.

---

## Runtime Domain Model

**Assessment** — the aggregate root (ADR-0003, ADR-0007). Owns identity (`AssessmentId`), subject (`AssessmentSubject`, still content-free), context (`AssessmentContext`, still content-free), a four-state lifecycle (`Created → CollectingEvidence → EvaluatingRules → Completed`), and its Evidence/Finding/Recommendation collections. Every mutation method (`add_evidence`, `add_finding`, `add_recommendation`) follows the same shape: check completion, check lifecycle state, check data precondition, then mutate — no exceptions, no variation. Assessment is also now responsible for **relationship resolution**: `evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation` resolve cross-entity references, since only the aggregate holds both sides of any relationship.

**Evidence** — `id` (`EvidenceId`), `category` (`EvidenceCategory`: XmlInspection, LuaAnalysis, RuntimeLogs, AssetValidation, DependencyResolution, PerformanceObservations, FileStructureAnalysis), `description` (validated non-empty), `location` (optional, validated non-empty when present). Constructed via `Evidence::new` / `Evidence::with_location`, returning `Result<_, EvidenceError>`.

**Finding** — `id` (`FindingId`), `severity` (`FindingSeverity`: Error, Warning, Informational, BestPractice), `description` (validated non-empty), `evidence_ids` (`Vec<EvidenceId>`, **currently permitted to be empty**), `rule_reference` (`RuleReference` — an opaque Runtime-owned value identifying the producing Rule, never a `modiq-knowledge` type). Constructed via `Finding::new`, returning `Result<_, FindingError>`.

**Recommendation** — `id` (`RecommendationId`), `action` (validated non-empty), `finding_ids` (`Vec<FindingId>`, **currently permitted to be empty**), `repair_recipe_reference` (optional, opaque `RepairRecipeReference`). Constructed via `Recommendation::new`, returning `Result<_, RecommendationError>`.

**Aggregate ownership:** all three collections are mutated only through `Assessment`'s own methods; no child entity ever mutates itself, another entity, or the aggregate.

**Entity identity:** `AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId` all share one mechanism — a process-local, monotonic `AtomicU64` counter, generated once at construction, never derived from content. No UUIDs anywhere (deliberate: determinism, zero new dependencies, no persistence yet to require global uniqueness — see ADR-0007). Equality for every Runtime entity is **identity-based**: two independently constructed values with identical content are not equal.

**Relationship resolution:** permissive by deliberate design. An unresolvable `EvidenceId`/`FindingId` reference is silently omitted from resolution results, never rejected. This is not an oversight — it is the direct consequence of two invariants that are identified but intentionally **not enforced**, pending governance approval:

- **GOV-005** — should a Finding be required to reference at least one Evidence item?
- **GOV-006** — should INV-005 be refined so a Recommendation must reference *specific, existing* Finding(s), not merely coexist with *some* Finding?

**Current invariants:** INV-001 through INV-012 (`RuntimeInvariants.md`) — lifecycle sequencing, evidence/finding immutability post-phase, aggregate-exclusive mutation — all implemented and tested. GOV-005 and GOV-006 remain open in the Governance Register.

---

## Execution Pipeline

```
Assessment Subject
      │
      ▼
Evidence  ←── constructed by the caller (today: sandbox, using a deterministic
      │        bootstrap value; in the future: real inspection of the subject)
      ▼
Rule Engine  (consumes Evidence, produces Findings/Recommendations — never Evidence)
      │
      ▼
Findings
      │
      ▼
Recommendations
      │
      ▼
Assessment Report  (AssessmentReport::generate — read-only snapshot)
      │
      ▼
Sandbox DTO  (AssessmentSummary + nested Evidence/Finding/Recommendation entries)
      │
      ▼
React UI
```

### Why Evidence is not produced by the Rule Engine

This was the central architectural question of Sprint 3 Phase 1, raised before any code was written. The original phase framing asked the Rule Engine to construct and attach Evidence itself. A full architectural review (constitutional/architecture/technical docs plus current source) found this would directly reverse a Frozen specification:

- `RuleEngine.md` states explicitly: "The Rule Engine consumes runtime entities... **Evidence**. The Rule Engine produces: **Findings, Recommendations**, Assessment Report content." Evidence is listed only as consumed.
- `Glossary.md` defines a Rule as something that "examines **one specific aspect of a mod**" — a Rule Engine self-check does not examine the mod at all, so it does not fit the definition of a Rule.
- `Architecture.md`'s Assessment Lifecycle diagram positions **Evidence Collection as its own distinct conceptual stage**, separate from Rule Engine (selection) and Finding Generation (evaluation) — implying Evidence generation was always meant to be an independent responsibility, not a Rule Engine one.

The review's conclusion, accepted before implementation began: Evidence generation (from real mod inspection — XML/Lua/file analysis) is a genuine, currently **unowned** architectural gap — not a defect in the frozen docs, and not something to improvise into an existing crate. No Documentation or Governance changes were required, because the correct fix was to *not* do the thing that would have caused drift. Sprint 3 Phase 1 instead proved the existing, already-correct pipeline using Evidence supplied as ordinary external input (exactly as `modiq-engine`'s own integration tests have always done) — from a real application instead of test code. A Finding and Recommendation are produced as a normal consequence (`RuleEngine::evaluate`'s existing, unchanged behavior for non-empty Evidence), not suppressed.

---

## Sandbox Architecture

**Stack:** Tauri 2 (Rust backend, native webview), React 19 + TypeScript 5.8 + Vite 7, TailwindCSS v4 (via `@tailwindcss/vite`, no config file needed), shadcn/ui (`radix` base, `nova` preset).

**IPC:** React and Rust communicate exclusively through Tauri commands (`invoke` / `#[tauri::command]`). No HTTP server, no REST API, no networking of any kind.

**Boundary discipline:** `Assessment` (the aggregate) is never exposed over IPC. The current command, `create_assessment`, builds one deterministic `Evidence` item, executes it through `AssessmentService::execute` (the same orchestration entry point `modiq-engine`'s own integration tests use), receives an `AssessmentReport` — the platform's own existing read-only snapshot type — and maps *that* into a serializable DTO (`AssessmentSummary`, with nested `EvidenceEntry`/`FindingEntry`/`RecommendationEntry` lists). The DTO, not any Runtime type, is the actual API contract crossing into JavaScript.

**Workspace isolation:** `apps/sandbox/src-tauri/Cargo.toml` declares an empty `[workspace]` table, making it its own independent Cargo workspace root rather than a member of the root modIQ workspace. This is deliberate: it keeps `cargo check --workspace` / `cargo test --workspace` at the repo root scoped to the eight platform crates only, so Tauri's large GUI dependency tree never enters that verification ritual.

**Current capabilities:** launches; links and exercises `modiq-runtime`, `modiq-rules` (transitively), `modiq-report`, and `modiq-engine` for real; displays Assessment ID, Evidence/Finding/Recommendation counts, and simple lists of each.

**Current limitations:** no persistence, no application state beyond a single `useState` in React, no routing, no file dialogs, no ZIP/XML/Lua parsing, no Knowledge or Version Profile integration, default shadcn/Tailwind styling only, and identifiers currently render as Rust's derived `Debug` output (e.g. `"EvidenceId(3)"` — see Technical Debt).

**Why a visualization tool, not a production application:** this has been an explicit constraint since Sandbox Phase 1 — the sandbox exists so engineers can observe real Runtime and Rule Engine behavior during development, and deliberately has none of the input capabilities (mod file loading, ZIP handling) a real product would require. Every phase so far has explicitly excluded production UI, dashboards, routing, and persistence.

---

## Documentation Status

**Documentation Release:** 2.0, Complete. Constitutional, architecture, and technical layers are Frozen. Execution-layer `Sprint0.md`/`Sprint1.md` are also Frozen.

**Engineering Releases:** v0.1.0-alpha (Sprint 1, tag `v0.1.0-alpha`) → 0.2 (Sprint 2, tag `v0.2.0-alpha`, record at `docs/releases/ENGINEERING_RELEASE_0.2.md`). Sprint 3 Phase 1 has not yet been folded into a new Engineering Release.

**ADR inventory** (all Accepted; ADRs are historical and are not to be modified once accepted, per `docs/adrs/README.md`):

| ADR | Title | Note |
|---|---|---|
| 0001 | Foundation Freeze | — |
| 0002 | Domain Model Boundaries | Internally mislabels itself "ADR-0001" in its own body text — a known, unfixed defect, flagged in `docs/adrs/README.md` |
| 0003 | Assessment Aggregate Root | — |
| 0004 | Platform-First Version Profiles | — |
| 0005 | Deterministic Assessment Engine | — |
| 0006 | Documentation Release 1.0 Freeze | References git tag `v0.1.0-docs`, which was never created; corrected elsewhere (README.md, PROJECT_STATUS.md) but not in the ADR itself |
| 0007 | Runtime Entity Design Pattern | New at Engineering Release 0.2; records the aggregate/identity/value-object/opaque-reference/validation/equality/resolution/governance-control/determinism pattern |

**Governance status:** six open Governance Register items in `GOVERNANCE.md`, none resolved:

- **GOV-001** — Assessment Report generation timing (pending Documentation Release 1.1)
- **GOV-002** — Runtime invariant reconciliation with `RuntimeInvariants.md` (pending Documentation Release 1.1)
- **GOV-003** — role of `modiq-common` (pending)
- **GOV-004** — Engine service granularity (pending)
- **GOV-005** — new Finding→Evidence invariant (pending Documentation Release 2.1)
- **GOV-006** — INV-005 refinement for Recommendation→Finding (pending Documentation Release 2.1)

**Known deferred decisions:** Rule abstraction / multiple Rules; Knowledge Domain integration; Version Profile integration; CLI wiring; advanced Reporting; persistence; `modiq-common`'s purpose; Engine service expansion; and — newly identified by the Sprint 3 architectural review — a real Evidence Collection subsystem, which has no owning crate, service, or ADR yet.

**Frozen vs. living, explicitly:** Constitutional, architecture/technical, and `Sprint0.md`/`Sprint1.md` = Frozen, do not edit. Governance documents (`GOVERNANCE.md`, `PROJECT_STATUS.md`, `CHANGELOG.md`, `CrateRoadmap.md`, `ENGINEERING_LOG.md`, `DocumentationRelease.md`) = explicitly living, per `PROJECT_STATUS.md`'s own text. Engineering Release documents and `HANDOFF_SPRINT1.md` = historical, point-in-time snapshots — never rewritten to reflect later state; superseded by new documents instead. ADRs = historical and Accepted; corrections belong in a new ADR or explicit errata, not an edit to the original.

**Newly observed while preparing this handoff, not yet corrected:** `docs/README.md`'s "Current Documentation Status" section still says "Implementation work continues under Sprint 1" — stale in the same way `PROJECT_STATUS.md`/`CHANGELOG.md` were stale before Engineering Release 0.2 (and that correction did not touch `docs/README.md`). `docs/00-Governance.md` is a shorter, older reading-order index than `docs/README.md` and does not mention Documentation Release 2.0 at all — likely vestigial, not yet reconciled or removed. Neither has been corrected as part of this handoff (no documentation changes were made preparing it, per instruction) — flagging both for the next session.

---

## Repository Health

**Workspace status:** root workspace (8 crates) clean; `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all green. The sandbox is a separate, independently-green workspace.

**Test count:** 97 tests in the root workspace (84 `modiq-runtime`, 4 `modiq-engine` unit, 3 `modiq-engine` integration, 3 `modiq-report`, 3 `modiq-rules`) — up from 55 at Engineering Release v0.1.0-alpha. The sandbox has 3 additional Rust tests of its own, not counted in the 97.

**Verification rituals:** at the repo root — `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`. Inside `apps/sandbox/src-tauri` — `cargo fmt`, `cargo check`, `cargo test`. For the frontend — `npm run build` (`tsc` + `vite build`). For the whole app — a live `npm run tauri dev` launch check (background process, confirmed running via `ps`, then cleanly terminated).

**Build status:** zero warnings across both workspaces as of the end of Sprint 3 Phase 1.

**Engineering quality markers maintained throughout:** every rejection path asserts state was left unchanged, not just that an error returned; identity-based equality is explicitly tested (not assumed) for every Runtime entity; permissive relationship resolution is explicitly tested (dangling references produce empty results, never errors); determinism tests compare content, never incidental identity.

---

## Technical Debt

Only genuine, previously-identified debt — nothing speculative:

1. **No public formatting/serialization for Runtime identity and enum types.** `AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId`, `EvidenceCategory`, and `FindingSeverity` have no `Display` or `Serialize` implementation. Every external consumer (currently, only the sandbox) is forced to use the derived `Debug` representation (e.g. `"EvidenceId(3)"`) to show or serialize an identifier. Flagged twice now (Sandbox Phase 2, Sprint 3 Phase 1); not fixed because it requires touching `modiq-runtime`, which was out of scope for sandbox-only work. Worth a deliberate decision, not urgent.
2. **`modiq-cli` remains unwired.** True since Sprint 1. The sandbox and test code remain the only real entry points into the pipeline.
3. **`modiq-common` remains unused** (GOV-003) — an empty, purposeless workspace member.
4. **Two unfixed ADR document defects** — the ADR-0002 internal mislabeling and the ADR-0006 stale tag reference (both described above). No repair mechanism has been decided (new ADR vs. errata vs. leave as historical record).
5. **Git tag hygiene.** `v0.2.0` and `v0.3.0` predate `v0.1.0-alpha` chronologically (they tag Sprint 0 completion and Documentation Release 2.0 respectively) despite sorting "later" by semver. Unresolved; no action taken since it was first flagged.
6. **`docs/README.md` and `docs/00-Governance.md` are stale/vestigial** (see Documentation Status) — newly observed, not yet corrected.

---

## Engineering Philosophy

Principles consistently followed from Sprint 0 through Sprint 3, with how each showed up concretely:

- **Platform-first** — Version Profiles deferred rather than baked into core logic (ADR-0004).
- **Evidence-based** — Evidence is defined as objective, opinion-free fact; Findings/Recommendations always carry traceable references back to it.
- **Deterministic** — identity generated via a monotonic counter, never randomness; every "determinism" test compares content, never incidental identity (ADR-0005).
- **Explainable** — `evidence_ids`, `rule_reference`, `finding_ids` fields exist specifically to preserve traceability, even before any consumer uses them yet.
- **Aggregate ownership** — `Assessment` is the sole mutator and sole relationship-resolver; no exceptions were made anywhere, including under sandbox/application pressure.
- **Minimal abstractions** — no Rule trait exists despite three implementation phases that could have justified one; `RuleReference`/`RepairRecipeReference` are plain opaque strings, not elaborate types.
- **Architecture before implementation** — the Sprint 3 Phase 1 pause-and-review (this handoff's centerpiece example) is the clearest instance: implementation stopped entirely until the ownership question was resolved on paper.
- **Documentation-driven engineering** — `SPRINT2_IMPLEMENTATION_PLAN.md` served as an approved blueprint before Sprint 2 code was written; Engineering Release documents serve as freeze points.
- **Incremental implementation** — Evidence, Finding, Recommendation, and Assessment aggregate integration were four separate phases, each independently reviewed before the next began.
- **No speculative generalization** — GOV-005/GOV-006 were left permissive rather than guessed at; no Evidence Collection subsystem was invented to fill a gap that isn't blocking anything yet.
- **Stable public APIs** — `Assessment`'s core method signatures are unchanged since Sprint 1, even as the types flowing through them gained real content.
- **Thin applications** — the sandbox contains zero domain logic; every phase's implementation was checked against this specifically.

---

## Immediate Next Steps

1. **Confirm Sprint 3 Phase 1's status explicitly.** The implementation is complete and verified (fmt/check/test/build/launch all clean), but as of this handoff it is **uncommitted** (`git status` shows four modified files in `apps/sandbox/`) and had not yet received an explicit "approved" from the Technical Director in conversation. Resolve this before writing any new code.
2. **Commit Sprint 3 Phase 1's work**, once confirmed, with a commit message describing what changed (Cargo dependencies, `lib.rs` rewrite, `App.tsx` update) — do not silently fold it into a future commit.
3. **Do not start Sprint 3 Phase 2 by guessing its scope.** The natural fork ahead is between two different kinds of work: (a) resolving GOV-005/GOV-006 through the governance process now that Finding/Recommendation reference fields are actually being exercised by a real application, or (b) evaluating whether a second, genuinely distinct Rule now justifies introducing a Rule abstraction — a fork Sprint 1 and Sprint 2's own documentation predicted would arrive once Evidence carried real content. Sprint 3 Phase 2 should begin with a short architecture/governance conversation on this fork, the same way Phase 1 did, rather than an assumed technical direction.

**Recommended next milestone: Sprint 3 Phase 2**, scoped by whichever fork above the Technical Director selects — not prescribed further here, per instruction not to speculate beyond the near future.

---

## Longer-Term Roadmap

Kept intentionally brief — none of this is scheduled, only sequenced conceptually:

Evidence Collection (real ZIP/XML/Lua inspection of the Assessment Subject) → Rule abstraction and multiple Rules → Knowledge Domain integration → Version Profile integration → CLI wiring → advanced Reporting/Explainability output → persistence → resolution of the six open Governance Register items → eventual production-grade application (whether an evolved Sandbox or a genuinely separate product).

---

## Guidance for the Next Technical Director

Evaluate every new engineering request against the same questions this handoff's own centerpiece episode (Sprint 3 Phase 1's original framing) was resolved with:

- **Does this conflict with a Frozen specification?** If a request would change what a documented subsystem consumes, produces, or owns, stop and check the actual text of `RuleEngine.md`/`DataModel.md`/`Architecture.md`/`GOVERNANCE.md` before writing code — don't assume the request is self-evidently correct just because it's plausible-sounding.
- **Does this cross a crate boundary?** Runtime stores, Rule Engine evaluates, Reporting renders, Engine orchestrates, Knowledge and Versioning stay independent until deliberately integrated. A request that quietly blurs one of these is a request to flag, not to implement.
- **Is documentation staying honest?** `PROJECT_STATUS.md` and `CHANGELOG.md` sat stale for two full sprints before Engineering Release 0.2 caught it, and `docs/README.md` still is stale as of this handoff. Update governance documents at the close of each phase, not in a batch months later.
- **Is this abstraction earned yet?** No Rule trait, no Evidence Collection subsystem, no Knowledge integration should be built because it seems likely to be needed soon — only when a second concrete case makes the need unavoidable.
- **When in doubt, stop and ask.** Every genuinely ambiguous decision in this project's history so far was resolved by pausing and asking rather than picking a plausible interpretation — that discipline is the reason the architecture has stayed coherent across three sprints of engineering work.

---

## Session Resume Checklist

Before writing code:

- [ ] Read this handoff completely.
- [ ] Read PROJECT_STATUS.md.
- [ ] Read ENGINEERING_RELEASE_0.2.md.
- [ ] Read DocumentationRelease.md.
- [ ] Review open ADRs.
- [ ] Run:
  - cargo check --workspace
  - cargo test --workspace
- [ ] Confirm Sandbox launches.
- [ ] Continue with Sprint 3 Phase 2 only after architecture review.
