# Engineering Release 1.0

| Property | Value |
|---|---|
| **Release** | 1.0 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 10 complete (Runtime Fixture Corpus Acquisition; Repository Closeout) |
| **Scope** | Capability Definition and a real, provenance-tracked evidentiary foundation for Runtime Log Interpretation. **Not** the capability's own implementation — no Rust source was modified this Sprint. |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.9.md` (Sprint 9) |
| **Governing ADRs** | None new |
| **Governing Plan** | `SPRINT10_CAPABILITY_DEFINITION.md`, `SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md` |

---

## Why This Release Looks Different

Every Engineering Release since 0.4 has followed the same sixteen-section structure, built around a Sprint that changed code: Testing Growth, Crate Maturity Review, and Technical Debt Review all assume something in the workspace moved. Sprint 10 did not touch a single Rust source file — the root workspace test suite is unchanged at 210/210, Sandbox unchanged at 7/7. Forcing this Sprint's own record into that template would produce several sections reporting "unchanged" for no informative reason. This release instead follows the structure Sprint 10's own Closeout mission specified directly: objectives, work completed, the corpus acquired, documentation completed, engineering decisions, known limitations, an explicit deferral statement, and the logical next step. Nothing about this is a lowering of rigor — every claim below is still grounded in direct repository evidence, re-verified this session.

---

## 1. Sprint Objectives

Sprint 10 was authorized to determine precisely what capability the platform should gain for Runtime Log Interpretation, and — per `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11's own strengthened engineering requirement — to ensure no architectural decision about that capability would ever be made against an assumed, unverified shape of a real Farming Simulator runtime log. The Sprint's own charter was therefore twofold: define the capability, and acquire the real evidence any future architecture must be built against, before that architecture is designed.

## 2. Work Completed

- **Capability Definition** (`SPRINT10_CAPABILITY_DEFINITION.md`): scoped Runtime Log Interpretation to recognizing exactly one class of signal — that a bundled runtime log indicates a mod failed to load — grounded directly in `Vision.md`'s own named founding question, "why does it fail to load?" Explicitly out of scope: a general log-parsing framework, cross-Assessment correlation, and any Repair Recipe pairing. The session's own review strengthened its Section 11 precondition from an observation into a hard engineering requirement: architectural decisions must derive from real log evidence or authoritative documentation, never assumption.
- **Runtime Fixture Corpus design and construction** (`SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md`): a new, permanent, top-level `fixtures/runtime-logs/` directory — deliberately separate from `apps/sandbox/src-tauri/fixtures/`'s own, unrelated synthetic-fixture convention — with a consistent per-fixture metadata schema (`TEMPLATE.md`) covering provenance, consent, normalization, and known limitations.
- **All three initial fixtures captured, normalized, and integrated as real evidence** — see Section 3.
- **Two real corpus-documentation gaps found during acquisition, formalized before the fixture that exposed them was integrated** — Installation State versus Savegame State, and Warning Categorization — see Section 5.
- **A Runtime Log Normalization policy formalized**, requiring deterministic, precisely-specified, strictly substitutive handling of personally identifying or machine-specific content — see Section 5.

## 3. Runtime Corpus Acquired

| Fixture | Content | Result |
|---|---|---|
| `clean-base-game` | Global mods directory verified empty by direct action (physically removed and replaced) before capture | Zero mod-related content in 1,448 lines; zero `Error` lines; 5 Base-game warnings, none mod-related — the platform's first real "nothing is wrong" baseline |
| `single-compatible-mod` | Global mods directory configured to contain exactly one real, third-party vehicle mod (`FS25_2011_Silverado_2500_Short_Bed`, no Lua scripts) | Mod discovered, enumerated, loaded, and used successfully; zero `Error` lines; 5 Base-game warnings plus 2 real, mod-attributable Fixture warnings (a console-size advisory on the mod's own oversized geometry file) — the negative control |
| `single-incompatible-mod` | Global mods directory configured to contain exactly one real, third-party vehicle mod (`FS25_DodgeChallengerHellcat`) | Mod discovered and enumerated, then rejected: `Error: Unsupported mod description version in mod FS25_DodgeChallengerHellcat` — the mod's own declared `descVersion` (107) exceeds the runtime's recognized `ModDesc Version` (106). No `Load mod:` line for this mod exists anywhere in the file. Failure stage, precisely evidenced: **modDesc validation**, before registration or asset loading. This directly corroborates the acquisition team's own independent observation that the mod never appeared in the Start Game → Mods selection list. |

No mod archive was stored in the repository at any point, for any fixture — only the runtime logs themselves, and factual metadata independently verified from each archive's own contents (`modDesc.xml`, package structure, Lua presence), per Sprint 10's own explicit policy.

## 4. Documentation Completed

- `fixtures/README.md` — the top-level corpus directory's own orientation: why it exists, how it differs from `apps/sandbox/src-tauri/fixtures/`, what belongs and what does not.
- `fixtures/runtime-logs/README.md` — the corpus's own living policy reference: Repository Language (distinguishing engineering sample files, runtime logs, and platform Evidence), Runtime Log Normalization, Warning Categorization, Installation State versus Savegame State, structure, and the fixture-addition checklist.
- `fixtures/runtime-logs/TEMPLATE.md` — the canonical per-fixture metadata schema.
- Three fixture `README.md` files, each fully populated with real, evidence-derived metadata — no field left as a placeholder.
- `docs/engineering/SPRINT10_CAPABILITY_DEFINITION.md`, `docs/engineering/SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md` — the full planning and acquisition record, revised in place five times as real evidence arrived, each revision explained rather than silently applied.
- Repository Closeout synchronization: `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, `CrateRoadmap.md`, `docs/README.md` — all updated to reflect Sprint 10's actual, scoped completion.

## 5. Engineering Decisions

- **Fixtures live at `fixtures/`, not `testdata/`.** "Fixture" is this repository's own pre-existing, code-referenced term (`apps/sandbox/src-tauri/fixtures/`); introducing a second term for the identical concept would fragment terminology rather than clarify it.
- **`Redaction Applied` was renamed to `Normalization Applied`**, not duplicated alongside it — redaction is the concrete first instance of a broader "fixtures are deterministic engineering artifacts" principle, and two fields describing the same fact would itself be the kind of schema conflation this corpus's own Installation-State finding already taught against.
- **Normalization must be deterministic and precisely specified** — the exact pattern matched, the exact placeholder, the exact count — sufficient that anyone re-applying it to the same raw source reproduces the fixture byte for byte. Verified directly for all three fixtures (exact byte deltas: +18, +36, +18, each fully accounted for by the substitution alone).
- **Warning Categorization is attribution-based (`Base-game` / `Fixture` / `Fixture-affecting`), not severity-based.** Chief Architect direction corrected an initial "benign" framing specifically because attribution is a verifiable fact (does the same line appear in a mod-free capture?) while severity is a judgment call — consistent with this project's evidence-first discipline.
- **The mod archive itself is never stored in the repository.** Only the runtime log it produced, and factual metadata independently verified from its own contents, are recorded — real third-party content is inspected, never redistributed.
- **A documentation gap was treated as a hard stop, twice** (Installation State vs. Savegame State; Warning Categorization) — both formalized as corpus policy before the fixture that exposed them was accepted as `Captured`, not worked around informally.

## 6. Known Limitations

- Three fixtures, one platform (macOS), one map, one game version. Windows and Linux captures of the same three scenarios remain uncaptured.
- `single-incompatible-mod` establishes exactly one failure class (a declared-`descVersion` mismatch). It does not establish that all mod-load failures share this signature — broader signature coverage is explicitly deferred, named future work.
- `modded-map-only` (does the recognized signal generalize across a different Assessment Subject content type?) and `real-world-mod-profile` (a large, realistic multi-mod profile) remain named, deferred fixture candidates, not built this Sprint.
- The Warning Categorization taxonomy has been exercised against three real fixtures' worth of warnings; whether it remains sufficient once a fourth, structurally different fixture is captured is not yet known.

## 7. Parser Implementation Is Intentionally Deferred

**No Collector, no Rule, and no activation of `EvidenceCategory::RuntimeLogs` were implemented this Sprint, and none was in scope.** This is a deliberate, explicit boundary, not an incomplete Sprint: `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11 required that acquiring and validating real evidence precede any implementation-oriented architectural decision, and this Sprint's entire charter was completing that requirement — not building against it. Runtime Log Interpretation's own Architectural Resolution remains future work, to begin from the three real fixtures this Sprint produced, once additional corpus growth (further platforms, a second failure class, or the deferred `modded-map-only` fixture) is judged sufficient by whichever future session takes that question up.

## 8. Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged) |
| Root workspace tests | 210 (unchanged from Engineering Release 0.9) |
| Sandbox tests | 7 (unchanged) |
| Governance items | 13 total — 8 Resolved, 5 Open (unchanged) |
| Documentation Release | 2.1, Frozen (unchanged) |
| Engineering Methodology Version | 1.0 (unchanged) |
| Real fixtures acquired | 3 of 3 initial corpus targets |
| Mod archives stored in repository | 0 (by explicit policy) |

## 9. Next Sprint

The logical continuation is Runtime Log Interpretation's own Architectural Resolution — deciding how a Collector reaches this corpus's now-real evidence and how a Rule interprets it — grounded in three captured fixtures rather than assumption, exactly as Sprint 10 existed to enable. Whether that Sprint should also expand the corpus first (a second platform, `modded-map-only`, or broader failure-signature coverage) or proceed directly to Architectural Resolution against the current three fixtures is a capability-scoping question for that Sprint's own Capability Definition to ask — not decided or assumed here.

---

**Sprint 10 is complete for its own scope. The repository is ready for Chief Architect approval and push.**
