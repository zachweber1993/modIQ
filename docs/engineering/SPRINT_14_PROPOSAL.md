# Sprint 14 Proposal — Technical Director Analysis

| Property | Value |
|---|---|
| **Document** | SPRINT_14_PROPOSAL.md |
| **Project** | modIQ |
| **Purpose** | Survey the repository's current architectural state and identify the highest-value forcing function for Sprint 14. Proposal analysis only. |
| **Origin** | Chief Architect directive, following Sprint 13's Repository Closeout (Engineering Release 1.3, commit `6664724`) and two documentation refinements (commit `7366b02`). |
| **Status** | **Proposal only. No Architecture Evaluation, no Sprint Planning, no implementation, and no governance resolution has been performed. Nothing in this document decides anything — it recommends what should be decided next, and by whom.** |

---

## 1. Repository State Review

### 1.1 Subsystem activation status

| Subsystem | Crate | Level | Real content |
|---|---|---|---|
| Runtime Domain | `modiq-runtime` | L3 | Full — Assessment, Evidence, Finding, Recommendation, lifecycle |
| Evidence Collection | `modiq-collection` | L2 | Four real Collectors (filesystem, archive, XML, runtime logs) |
| Rule Engine | `modiq-rules` | L3 | Four real Rules, fixed dispatch order |
| Reporting | `modiq-report` | L3 | `AssessmentReport` only, by design |
| Engine | `modiq-engine` | L3 | `AssessmentService`, two stable public entry points |
| Knowledge Domain | `modiq-knowledge` | L2 | One `RepairRecipe`; six of seven Knowledge Model categories still unimplemented, deliberately |
| Version Profiles | `modiq-versioning` | L2 | One hardcoded `VersionProfile::fs25()`; `Capability`/`Compatibility` deliberately deferred |
| CLI | `modiq-cli` | L2 | Four commands (`assess`, `retrieve`, `help`, `version`) |
| **Storage** | `modiq-storage` | L2 | **New this Sprint (13)** — single-report write/read only |
| Shared platform types | `modiq-common` | L1 | Scaffolded, zero real content, zero consumers (verified below) |
| Extension Layer | — | **L0** | No crate exists at all |

### 1.2 Remaining dormant subsystems

- **Extension Layer** — the platform's only remaining fully-dormant System Overview subsystem now that Storage is real. No crate, no proposal, no named product pressure anywhere in the repository.
- **Knowledge Domain's six unimplemented categories** (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) — deliberately deferred since Sprint 9, no forcing function named since.
- **Version Profiles' `Capability`/`Compatibility`** — deliberately deferred since Sprint 8, no second Version Profile has ever existed to force the question.
- **Storage's own two named, undecided product-forcing-functions** (`INV-002` §4): cross-mod collection validation and MKB accumulation from real Assessments. Neither is presupposed by Sprint 13's minimum-viable slice.

### 1.3 Open Governance items (14 total, 8 Resolved, 6 Open)

| Item | Age (Sprints) | Subject | Status |
|---|---|---|---|
| **GOV-001** | **13** (raised Engineering Release v0.1.0-alpha) | Assessment Report Generation Timing | Open — the oldest unresolved item in the Register |
| GOV-002 | 13 | Runtime Invariant Reconciliation | Open |
| GOV-003 | 13 | Role of `modiq-common` | Open |
| GOV-008 | 9 | `AssessmentService` Public API Evolution | Open, deferred every Sprint since raised |
| GOV-013 | 8 | `FindingSeverity` Severity/Kind Conflation | Open by design |
| GOV-014 | 0 (raised post-Sprint-12) | Lua Fixture Acquisition Governance | Open — blocks Lua Analysis's own Architecture Evaluation |

### 1.4 Architectural dependencies and gaps

- **`modiq-common` has zero consumers.** Verified directly this session: `grep`ing every crate's `Cargo.toml` for `modiq-common` finds only the crate's own manifest — no other crate in the workspace depends on it. Its own `src/` contains four one-line stub files. Thirteen Sprints, zero forcing function, zero real content, zero consumer.
- **The `modiq-versioning` Crate Boundary Rules gap** in `GOVERNANCE.md`, named at Sprint 8, remains open six Sprints later — `modiq-storage` gained its own Crate Boundary Rule pair directly this Sprint; `modiq-versioning`'s own gap was not touched.
- **`modiq-engine` does not re-export `AssessmentReport`** — named at Sprint 6 with two data points (Sandbox, `modiq-cli`); no third consumer has appeared since, and remains below this project's own three-point convergent-evidence bar.

### 1.5 Technical debt created by previous decisions

- **`modiq-common`'s own thirteen-Sprint silence** is itself a form of accumulated evidence — the same logic this project already applied in the opposite direction for GOV-004 (three converging subsystems justified a decision) and for `modiq-report`'s Sprint 6 scaffold retirement (zero real use justified removal). GOV-003 has sat open the entire time without anyone testing that evidence directly.
- **A forcing function Sprint 13 itself created, previously latent:** `AssessCommand::run`'s own manual smoke test this session produced the literal output `Assessment AssessmentId(1) — status: EvaluatingRules` for a report that was then durably persisted to disk by `modiq-storage`. GOV-001 asks whether a report should represent an Assessment immediately before or after completion — while Assessments were ephemeral and reports were printed once and discarded, this question had low practical stakes. Now that `modiq-storage` durably persists whatever `AssessmentReport::generate` produces, **the report a user retrieves later, possibly long after the process that created it has exited, may permanently describe a non-terminal `EvaluatingRules` state, not a completed Assessment** — a materially different, higher-stakes situation than the one GOV-001 was raised against at Sprint 1.

### 1.6 Product value

- `ProductSpecification.md`'s own Assessment Engine responsibilities name Lua analysis explicitly as one of six — a real, named, unaddressed product gap, currently blocked entirely on GOV-014.
- Storage's own two undecided forcing-functions each map directly to a named target-user objective (Server Administrators' "validate complete mod collections"; Community Contributors' MKB accumulation) — but neither has evidence yet indicating which is the actual next problem to solve.
- Resolving GOV-001 has no external product visibility of its own, but it directly determines whether Storage's own product promise — "retrieve a report later and trust what it says" — is actually true.

### 1.7 Repository maturity

Ten workspace crates (up from nine at Sprint 12), 253 root tests, 9 Sandbox tests, zero warnings across both workspaces. Zero ADRs added since ADR-0010 (Sprint 6) — every capability since has applied or extended existing architecture, never required a new durable principle. Governance Register unchanged in count since Engineering Release 0.8 until GOV-014 (post-Sprint-12), now 14 items.

### 1.8 Risk

- Extending Storage a second time before either of its own two forcing-functions is resolved risks the same premature-abstraction failure mode this project has avoided since Sprint 0 — building toward a guessed answer rather than a evidenced one.
- Continuing to defer GOV-001 now carries a different character of risk than its prior twelve Sprints of dormancy: it is no longer a purely theoretical question sitting beside an ephemeral, discarded artifact. It sits beside a durable one.
- `modiq-common`'s continued existence with zero consumers is low-risk in itself, but every Sprint it remains unresolved is a Sprint in which a new crate could reflexively be pointed at it out of habit rather than evidence, the same failure mode `GOVERNANCE.md`'s own Crate Boundary Rules exist to prevent.

---

## 2. Candidate Sprint Objectives

### Candidate A — Resolve GOV-001 (Assessment Report Generation Timing)

**Problem statement:** Should `AssessmentReport::generate` be called immediately before Assessment completion, or after it? Open since Engineering Release v0.1.0-alpha (Sprint 1) — the oldest item in the Governance Register.

**Why now:** Sprint 13's own `modiq-storage` gave this question real, durable stakes it never had while reports were ephemeral. Directly observed this session: a manually smoke-tested report was persisted showing `status: EvaluatingRules`, not `Completed`. A capability whose entire purpose is "trust what you retrieve later" is weakened if what gets durably stored may not represent an Assessment's own final state.

**Architectural impact:** Confined to where `AssessmentReport::generate` is invoked relative to `Assessment::complete()` inside `AssessmentService`'s own two entry points — does not, on its own, require touching `modiq-collection`, `modiq-rules`, or `modiq-storage`'s own already-fixed representation.

**Dependencies:** None blocking. `AssessmentService`, `Assessment`, and `AssessmentReport` are all already real, tested, and stable.

**Risks:** Low technically; the real risk is scope creep — this is a narrow timing question, not an invitation to redesign the Assessment lifecycle or `AssessmentReport` itself.

**Expected repository changes:** Primarily governance and, depending on the Chief Architect's own resolution, a small, well-contained change to where one method call happens inside `AssessmentService`. No new crate, no new public entry point.

**Why it should become Sprint 14:** It is the oldest open item in the repository, it now has a genuine, freshly created, directly observed forcing function rather than a hypothetical one, and resolving it gives Storage's own product promise real meaning rather than leaving a known gap sitting beneath a capability just shipped.

---

### Candidate B — Resolve GOV-014 (Lua Fixture Acquisition Governance)

**Problem statement:** What provenance, licensing, storage, and acquisition-governance discipline must be established before any real Lua fixture may be acquired, or Lua Analysis's own Architecture Evaluation authorized?

**Why now:** `INV-001` has been complete and fully blocked on exactly this question since before Sprint 13 began; nothing has advanced it since. Lua analysis is a named, explicit `ProductSpecification.md` responsibility, currently the platform's most visible unaddressed detection gap.

**Architectural impact:** None directly — this is governance and documentation work, mirroring Sprint 10's own precedent (an entire Sprint scoped to policy and evidence work, zero Rust source touched).

**Dependencies:** None. Does not require human-performed fixture acquisition to begin — only the policy questions that must be answered *before* that acquisition can start.

**Risks:** Low. The main risk is treating this Sprint as an invitation to begin fixture acquisition itself before the policy is actually settled — Sprint 10's own fixture-corpus policies were each resolved before the fixture that needed them was accepted, never worked around informally.

**Expected repository changes:** `GOVERNANCE.md` (GOV-014 resolution), likely a new provenance/licensing policy document mirroring `fixtures/runtime-logs/README.md`'s own standing policies. No crate, test, or fixture.

**Why it should not (yet) become Sprint 14:** Real and valuable, but its payoff is more distant and partly outside engineering's own control — even a resolved GOV-014 only unblocks human fixture acquisition, itself a precondition to Architecture Evaluation, not implementation. GOV-001, by contrast, is fully resolvable and fully payoff-realizing within one Sprint, with no external dependency.

---

### Candidate C — Resolve GOV-003 (Role of `modiq-common`)

**Problem statement:** What is `modiq-common` for? Open since Sprint 0.

**Why now:** Thirteen Sprints of zero real content and zero consumers, confirmed directly this session, is itself convergent evidence — the same evidentiary standard this project already applied in the opposite direction for GOV-004 and for `modiq-report`'s own Sprint 6 retirement.

**Architectural impact:** If retired, minimal — no crate currently depends on it, confirmed directly. If retained, requires an actual stated reason, not continued silence.

**Dependencies:** None.

**Risks:** Very low either way — this decision has essentially no blast radius given the confirmed absence of consumers.

**Expected repository changes:** `GOVERNANCE.md` (GOV-003 resolved), workspace `Cargo.toml` (crate removed, if that is the resolution) or a stated, evidenced reason for retention recorded in its place.

**Why it should not (yet) become Sprint 14 on its own:** Genuinely worth resolving, but it is a repository-hygiene decision with no product stakes, thin enough to be worth bundling into whatever Sprint 14 actually becomes rather than serving as that Sprint's own headline objective.

---

### Candidate D — Storage's Second Increment (Capability Definition only)

**Problem statement:** Which of Storage's own two named, undecided product problems — cross-mod collection validation, or MKB accumulation from real Assessments — should shape Storage's next real content?

**Why now:** Both map directly to named `ProductSpecification.md` objectives, and Storage is freshly real.

**Architectural impact:** Potentially significant — a query capability or a `modiq-knowledge` relationship, depending on which is chosen; premature to assess further without a Capability Definition first.

**Dependencies:** `modiq-storage` exists and is stable; no other blocker.

**Risks:** This is the strongest candidate for the premature-abstraction trap this project has avoided since Sprint 0. Neither Version Profiles nor Knowledge Domain received a second increment in the Sprint immediately following their own first activation — both remain deliberately deferred, still, pending their own forcing functions. Choosing between Storage's two open questions today would be guessing, not evidence.

**Expected repository changes:** None yet, if scoped correctly — this candidate's honest next step is a Capability Definition, not implementation.

**Why it should not become Sprint 14:** No forcing function exists yet distinguishing which of the two problems is real and urgent. This is the same restraint this project applied to Version Profiles and Knowledge Domain after their own first activation — the correct move is to wait for evidence, not manufacture a decision to stay busy.

---

### Candidate E — Extension Layer Activation

**Problem statement:** Should the platform's other fully-dormant System Overview subsystem receive its first real content?

**Why it should not become Sprint 14:** No proposal, no named product pressure, and no historical precedent even suggesting a shape. Activating a dormant subsystem because it is dormant, rather than because evidence demands it, is precisely what this project's "capability before abstraction" discipline exists to prevent. Named here for completeness, not as a live candidate.

---

### Candidate F — Resolve GOV-008 (`AssessmentService` Public API Evolution)

**Problem statement:** Should `AssessmentService`'s two public entry points evolve?

**Why it should not become Sprint 14:** Open for nine Sprints, deferred every time for the same reason — no new evidence. Sprint 13 generated none either: Storage integrates entirely at the caller layer, never touching `AssessmentService`'s own signature. Continuing to defer, explicitly, remains the correct application of this project's own restraint — not a gap needing Sprint 14 to close.

---

## 3. Recommendation

**Sprint 14's objective should be resolving GOV-001 (Assessment Report Generation Timing).**

It is the single item in this review that satisfies every criterion at once: it is the oldest open question in the repository; it now has a concrete, freshly created, and directly observed forcing function — not a hypothetical one — supplied by Sprint 13's own work; it is fully resolvable within one Sprint with no external dependency; its architectural footprint is small and well-contained; and its product stakes are real, if quiet — a capability whose purpose is durable retrieval should not leave open, thirteen Sprints on, exactly which moment of an Assessment's life that durable record represents.

GOV-014 (Candidate B) and GOV-003 (Candidate C) are both genuine, low-risk, evidence-supported candidates worth the Chief Architect's attention soon, and GOV-003 in particular is cheap enough to consider bundling into Sprint 14 alongside GOV-001 if scope allows — but neither carries a forcing function as immediate or as directly tied to what this project just shipped as GOV-001's does.

---

## 4. Explicit Non-Scope

- No Architecture Evaluation has been performed for any candidate above.
- No Sprint Planning has been performed.
- No implementation is proposed or implied anywhere in this document.
- **GOV-001 itself has not been resolved here** — this document identifies why it should be resolved next and what makes it urgent now; it does not decide whether reports should be generated before or after completion. That decision belongs to the Chief Architect, informed by whatever Architecture Evaluation follows, not to this proposal.
- No other Governance Register item was resolved, opened, or altered in producing this analysis.
