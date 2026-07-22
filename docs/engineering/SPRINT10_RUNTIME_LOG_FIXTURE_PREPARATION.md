# Sprint 10 Runtime Log Fixture Preparation

| Property | Value |
|---|---|
| **Document** | SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md — renamed from `SPRINT10_RUNTIME_LOG_FIXTURE_PLAN.md` during Repository Stabilization, to match the session's own name (this project's own terminology-consistency discipline, most recently exercised at Sprint 7's Engineering Workflow Consolidation) |
| **Project** | modIQ |
| **Purpose** | Design and prepare the permanent repository structure for Runtime Log engineering fixtures, satisfying `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11's strengthened precondition, before Architectural Resolution begins |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `b7cb4a6` |
| **Status** | **Fixture-preparation session only. No implementation, no Collector, no Rule, no Architectural Resolution.** Repository structure and documentation created; no log content, real or synthetic, added. Revised once, during Repository Stabilization (acquisition order corrected, Section 4; terminology reviewed, Executive Summary of that session). Awaiting Chief Architect approval before Architectural Resolution. |

---

# 0. Repository Verification

| Check | Result |
|---|---|
| Working tree | `git status` — clean, except the untracked planning documents from prior sessions |
| Current branch | `feature/runtime-implementation`, in sync with `origin` |
| Latest commit | `b7cb4a6` — "docs: complete Sprint 9 repository closeout" |
| Existing fixture convention | Confirmed by direct inspection: the only prior fixture convention in this repository is `apps/sandbox/src-tauri/fixtures/` (`sample-assessment-input/`, `sample-archive-input.zip`) — trivial, engineer-authored content exercising the Sandbox's own real-I/O paths, referenced via `CARGO_MANIFEST_DIR` string literals in `apps/sandbox/src-tauri/src/lib.rs`. No cross-crate, provenance-tracked, real-world-data corpus has existed in this repository before this work. |
| Real log sample availability | Reconfirmed: no runtime log sample, fixture, or format description exists anywhere in this repository, consistent with `SPRINT10_CAPABILITY_DEFINITION.md`'s own finding. |

---

# 1. Executive Summary

`SPRINT10_CAPABILITY_DEFINITION.md`, Section 11 established, as an engineering requirement rather than an observation, that no architectural decision for Runtime Log Interpretation may assume the structure, wording, stability, or formatting of a Farming Simulator runtime log — and that acquiring and validating representative logs is the first engineering activity that requirement demands. This work builds the repository structure that acquisition work will fill: a dedicated, provenance-tracked fixture corpus (`fixtures/runtime-logs/`), a consistent per-fixture metadata schema (`TEMPLATE.md`), and three initial fixture directories — `clean-base-game`, `single-compatible-mod`, `single-incompatible-mod` — each fully documented and explicitly marked `Awaiting capture`, with no log content of any kind, real or synthetic, present.

This is a structural and documentation session only. It creates a place for real runtime log samples to live and a discipline for how they must be recorded; it does not, and must not, manufacture that content itself.

---

# 2. Repository Design

**Location: `fixtures/runtime-logs/`, a new top-level directory sibling to `crates/`, `apps/`, and `docs/`.**

This placement was chosen over crate-internal alternatives (e.g., nesting under `crates/modiq-collection/`) deliberately, to avoid presuming which crate will eventually consume this corpus — a question that belongs to Architectural Resolution, not this work. A top-level, crate-independent location keeps the fixture corpus available to whichever future Collector, Rule, or dedicated Boundary-Proving investigation needs it, mirroring how `apps/sandbox/src-tauri/fixtures/` already sits alongside code rather than buried inside a `src/` tree.

Evaluated against each named design criterion:

- **Scales to many future fixtures.** One directory per fixture, flat under `fixtures/runtime-logs/`, named descriptively rather than numerically. Adding a fixture never requires renaming, renumbering, or restructuring an existing one.
- **Preserves provenance.** Every fixture's own `README.md` (Section 3) records who captured it, how, when, under what consent, and what (if anything) was redacted — co-located with the content it describes, not in a separate index that could drift out of sync.
- **Separates fixture data from documentation.** The entire corpus lives outside `docs/`, which remains reserved for specifications, governance, ADRs, and Sprint process records. A fixture's own metadata travels with its data, not with the specification tree — satisfying the instruction directly, rather than merely keeping raw log bytes out of Markdown files.
- **Supports deterministic testing.** Fixtures are ordinary, version-controlled files, addressable by a stable, repository-relative path — the same shape `apps/sandbox/src-tauri/fixtures/` already relies on via `CARGO_MANIFEST_DIR`. Any future test can read a fixture and get byte-identical content on every run, on every machine, in CI. The exact mechanism a future Collector or test uses to locate a fixture is an Architectural Resolution / Implementation decision, not fixed here.
- **Remains platform-independent where possible.** The structure itself makes no assumption about which operating system a log was captured on — each fixture's own `Platform` field is load-bearing (Section 3), and fixtures from more than one platform are expected to coexist, not be treated as interchangeable, since Farming Simulator's own log output is not guaranteed identical across Windows, macOS, and Linux.

**No generalized, multi-category fixture taxonomy (e.g., `fixtures/<category>/`) was introduced beyond `runtime-logs/` itself.** Only Runtime Log Interpretation currently has a real-world-data forcing function; a second category (real Lua scripts, real asset files) would be the concrete forcing function needed to generalize this structure further, not something to anticipate now, consistent with this project's own "capability before abstraction" discipline. `fixtures/README.md` (added during Repository Stabilization) names this expectation explicitly for whichever future session introduces a second corpus.

---

# 3. Fixture Documentation

**Format chosen: a per-fixture Markdown `README.md`, using this repository's own established Property/Value table convention** (identical in spirit to every specification, ADR, and Sprint document already in this repository) — not a structured machine-readable format (YAML/TOML/JSON). This was a deliberate choice, not an oversight: nothing in this platform's existing test discipline reads fixture metadata programmatically at test time (a future test hardcodes its own expectations in Rust, the same way every existing fixture-driven test in this repository already does); the metadata's job is to inform the human engineer designing or reviewing that test, not to be parsed by it. Introducing a structured schema now, with no consumer that needs one, would be exactly the kind of speculative extensibility this project's own Architectural Principles have repeatedly rejected elsewhere (`SPRINT9_ARCHITECTURAL_RESOLUTION.md`'s own Question 2, most recently).

**Required fields, and why each is load-bearing** (full schema in `fixtures/runtime-logs/TEMPLATE.md`):

| Field | Why it is required |
|---|---|
| Fixture ID, Status | Identifies the fixture and states plainly whether it is real content or a documented placeholder awaiting capture — the single most important field for preventing a planned-but-empty fixture from being mistaken for a real one. |
| Farming Simulator Version, Platform, Map, Enabled Mods | The complete, exact conditions under which the log was produced — without these, a captured log cannot be reproduced, compared, or trusted to isolate the one variable it is meant to isolate. |
| Purpose | Forces the fixture's reason for existing to be stated as a falsifiable claim, not inferred from its name alone. |
| Expected Observable Behavior | States what the log should show, in advance of (or immediately following) capture — the concrete target a future Rule's test assertions are written against. |
| Source, Consent / Licensing | The two fields this platform's own culture would otherwise be most tempted to skip under time pressure. A real runtime log is third-party-adjacent data by nature (it may describe a real person's install); this repository must never store one without being able to state plainly who provided it and on what basis. |
| Captured | A date, for the same reproducibility reason as the version/platform/map/mod fields. |
| Redaction Applied | Explicit, not implied. "None" is an acceptable value, but it must be a stated conclusion, not a silent omission — the field's absence is treated the same as an unredacted fixture until proven otherwise. |
| Format Notes | Encoding and line-ending details a future byte-level test will otherwise have to rediscover the hard way. |
| Known Limitations | Prevents a single fixture from being over-generalized into a claim it cannot support — the same discipline `VersionCompatibilityRule` already applies to what a single declared-version mismatch can and cannot conclude. |
| File(s) | A direct pointer from metadata to content, since the two are stored as siblings, not merged. |

---

# 4. Initial Fixture Set

**The mission's own five-item progression was evaluated twice — once during the original fixture-preparation session, and once more during Repository Stabilization, which corrected the acquisition order. Both revisions are recorded here as the current, superseding shape; the original three-fixture selection stands, only the sequence changed.**

## Recommended, created — acquisition order

1. **`clean-base-game`** — unchanged from the original progression's first position. Establishes ground truth for "nothing is wrong"; a precondition for trusting any future Rule's silence.
2. **`single-compatible-mod`** — **captured before `single-incompatible-mod`, not after.** The original fixture-preparation session had reasoned about *value* (which fixture matters most) and promoted `single-incompatible-mod` to second on that basis. Repository Stabilization reconsidered this on a different, more rigorous basis: *interpretive safety*. If the first real mod-related log this platform ever examines is the failure case, with no compatible-mod baseline yet in hand, there is a real risk of misattributing ordinary, unfamiliar-but-benign mod-loading noise to the failure signature — precisely because nothing yet establishes what *normal* mod-loading log output looks like. Capturing the compatible-mod fixture first gives any future reading of the incompatible-mod log a contrastive baseline already in hand, the same discipline a controlled comparison requires: establish the control before examining the treatment, not after. This is judged the stronger argument and is adopted here.
3. **`single-incompatible-mod`** — still, by content, **the single most important fixture in this initial corpus** — the real, concrete shape of a mod-failed-to-load signature is the one fact Sprint 10's entire capability exists to recognize, and no other fixture substitutes for it. What changed is only its position in the *acquisition sequence*, not its importance: it is examined third, deliberately, once a clean baseline and a compatible-mod baseline both already exist to interpret it against.

## Deprioritized, not created

4. **`modded-map-only`** — the original progression's third item. **Recommended for deferral, not inclusion in the initial corpus**, unchanged from the original fixture-preparation session's own reasoning: `SPRINT10_CAPABILITY_DEFINITION.md`, Section 7 scopes this Sprint to recognizing *one* class of signal (a mod failed to load); a modded-map fixture tests whether that signal generalizes across a different Assessment Subject content type, a legitimate but secondary question this Sprint's own Capability Definition did not scope to answer. Recommended as the natural first addition to the corpus *after* `single-incompatible-mod` is captured and a real signature is in hand to test for generalization against.

## Correctly deferred, unchanged

5. **`real-world-mod-profile`** — already named as future work in the original progression, unrevised. Presupposes the signal already exists and is recognized in the minimal case first. No directory is created for it.

---

# 5. Engineering Standards

Reviewed during Repository Stabilization against the same four pillars named at their original introduction — **all four confirmed appropriate, unchanged.** No speculative versioning, schema, or automation was found to have crept in, and none is added here.

- **Naming.** Descriptive, kebab-case, one directory per fixture (`clean-base-game`, not `fixture-01` or `01-clean-base-game`). Numeric prefixes were deliberately avoided: they encode a progression that will not scale cleanly as the corpus grows, and Section 4 above already shows the "obvious" progression order needed revision twice — a naming scheme should not make that kind of reordering harder later.
- **Immutability.** A captured fixture's raw log content is never edited in place, without exception. A correction, a redaction found necessary after the fact, or a re-capture against a newer game version is always a *new* fixture directory; the old one's `Status` is updated to `Superseded`, naming its replacement directly. This mirrors this project's own standing convention that historical records (ADRs, Engineering Releases, handoff snapshots) are superseded, never silently rewritten.
- **Versioning.** No version suffix is applied to a fixture's directory name by default — the `Farming Simulator Version` field in its own `README.md` is the authoritative record. A version suffix should only be introduced once more than one game version's worth of fixtures genuinely coexist — not anticipated speculatively now.
- **Provenance.** No fixture may be added without its `Source` and `Consent / Licensing` fields stated plainly and specifically — "captured internally" or "found online" are not acceptable values on their own; the exact capture method and, where the content did not originate from modIQ's own controlled capture, the specific permission obtained, must be recorded.
- **Redaction of personal information.** Mandatory, and must occur *before* a fixture is ever committed to version control — git history is not a safe place to correct a redaction mistake after the fact. A real Farming Simulator log may contain a Windows username embedded in a file path, a Steam identifier, or other locally-identifying detail; every fixture's `README.md` must state explicitly what was redacted and how (a fixed, consistent placeholder token, not an ad hoc one per fixture), or explicitly state that nothing required redaction.
- **Future expansion.** The structure and schema are expected to scale by addition alone — new fixture directories, no schema change — for the foreseeable growth this Sprint anticipates (more failure classes, more platforms, eventually the deferred `modded-map-only` and `real-world-mod-profile` fixtures). A shared or cross-referenced metadata mechanism should only be introduced once a real, repeated duplication forcing function appears — not built into the schema preemptively.

---

# 6. What Was Created

- `fixtures/runtime-logs/README.md` — corpus-level documentation: purpose, structure, how to add a fixture, platform-independence note, terminology (Repository Language, added during Repository Stabilization).
- `fixtures/runtime-logs/TEMPLATE.md` — the canonical, blank per-fixture metadata schema.
- `fixtures/runtime-logs/clean-base-game/README.md` — fully documented, `Status: Awaiting capture`, no log file.
- `fixtures/runtime-logs/single-compatible-mod/README.md` — fully documented, `Status: Awaiting capture`, no log file.
- `fixtures/runtime-logs/single-incompatible-mod/README.md` — fully documented, `Status: Awaiting capture`, no log file.
- `fixtures/README.md` (added during Repository Stabilization) — the top-level corpus directory's own README, explaining why `fixtures/` exists as distinct from `apps/sandbox/src-tauri/fixtures/`, and how future engineering corpora beyond `runtime-logs/` should be organized.

**No log content — real, synthetic, or placeholder — was added anywhere.** No directory was created for `modded-map-only` or `real-world-mod-profile`. No repository code, Collector, Rule, or Architectural Resolution was performed at any point across either session.

---

# 7. Recommendation

**Ready for Chief Architect approval as the repository's prepared state for Runtime Log Interpretation.** The next engineering activity — per `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11's own requirement — is acquiring and validating real Farming Simulator runtime logs against the three `Awaiting capture` fixtures, in the order Section 4 now recommends: `clean-base-game` first, `single-compatible-mod` second (to establish a compatible-mod baseline before the failure case is examined), and `single-incompatible-mod` third. Architectural Resolution should not proceed on any assumption about log structure, wording, stability, or formatting until at least the third fixture is real.

---

Awaiting Chief Architect approval before Architectural Resolution. No implementation, Collector, Rule, governance item, or ADR has been created across either session.
