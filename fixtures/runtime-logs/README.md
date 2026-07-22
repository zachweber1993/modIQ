# Runtime Log Fixtures

Real, captured Farming Simulator runtime log fixtures — the engineering corpus Runtime Log Interpretation (Sprint 10 and beyond) depends on for Architectural Resolution, implementation, and deterministic testing.

Full rationale for this structure, the metadata schema, the initial fixture set, and the standards below lives in `docs/engineering/SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md`. This document is the practical, standing reference for adding or consuming a fixture; that one is the decision record for why it looks this way.

---

## Why this exists

This platform's own Evidence-Based Engineering discipline applies to how its architecture gets decided, not only to what it concludes about a mod (`SPRINT10_CAPABILITY_DEFINITION.md`, Section 11). No architectural or implementation decision for Runtime Log Interpretation may assume the structure, wording, stability, or formatting of a Farming Simulator runtime log. This directory exists to hold the real runtime log samples those decisions must be derived from instead.

**No fixture in this corpus is synthetic, placeholder, or invented.** A directory with a `README.md` and no log file means exactly what it says: the fixture is planned and its expected shape is documented, but the real log has not yet been captured or acquired. Nothing should ever be added here to "fill the gap" in the meantime.

---

## Repository Language

Three distinct things are easy to blur together when talking about this corpus, and this repository is deliberate about keeping them separate:

- **Engineering sample files** — the general category this corpus belongs to: real, external, provenance-tracked artifacts checked in to ground an engineering decision in fact rather than assumption. Runtime logs are the first kind this platform has needed; a future corpus for another kind (real Lua scripts, real asset files) would be a sibling of this one, not a subdirectory of it.
- **Runtime logs** — the actual content this corpus holds: raw text Farming Simulator itself produced, unmodified except for the documented normalization described below. A runtime log, on its own, is not a platform concept — it is exactly what it appears to be, nothing more, until something inside the platform inspects it.
- **Platform Evidence** — `DataModel.md`'s own, formal Runtime Domain concept: "objective information collected during an Assessment," produced exclusively by a Collector from an Assessment Input. A raw runtime log sitting in this directory is not Evidence and does not become Evidence merely by being stored here. It becomes Evidence only once a real Collector — not yet built, not this session's concern — reads it and produces a structured `Evidence` item from it, the same way a `modDesc.xml` file on disk is not Evidence until `XmlCollector` actually inspects it.

This distinction is not pedantic. Calling a raw log "Evidence" before any Collector exists would misstate what this corpus actually is: reference material for engineering decisions, not a Runtime Domain artifact.

---

## Runtime Log Normalization

**Why normalization exists.** Every fixture in this corpus is committed to permanent, shared version control — read by every future session, contributor, and CI run, indefinitely. A raw capture, exactly as it left one specific machine, is not automatically fit for that: it can carry information particular to the machine or person who captured it (a username embedded in a filesystem path, for instance) that has nothing to do with what Farming Simulator itself is communicating. Normalization is the deliberate, minimal, fully documented act of removing that information before a capture becomes a fixture — first performed during `clean-base-game`'s own integration, formalized here so it no longer depends on whichever contributor happens to remember it.

**Runtime fixtures are deterministic engineering artifacts, not raw archival copies.** This corpus does not exist to preserve a forensic copy of one person's disk contents. It exists to give every future engineering decision a stable, real artifact that reads identically on every machine, indefinitely — the same standard this project's Real-I/O Testing Discipline already holds every other fixture to. A capture that still carries one specific person's username has not yet met that standard; normalization is what gets it there.

**What must be normalized.** Personally identifying or machine-specific information — most concretely, a username embedded in a filesystem path (e.g., a `Mod Directory` line rooted in a user's own home or Library folder) — must be replaced with a standardized, fixed placeholder before a fixture is committed. The current standard placeholder is `REDACTED_USER`; every fixture that needs one uses the same token, so normalized fixtures remain directly comparable to one another rather than each inventing its own convention.

**Normalization must be deterministic.** The same raw source log, normalized the same documented way, must always produce the same normalized fixture, byte for byte — regardless of who performs the substitution, when, or how many times. This rules out ad hoc, judgment-based redaction (a person deciding line-by-line what "looks sensitive," which can vary between attempts or between people) in favor of a fixed, mechanical substitution: a specific matched value replaced by a specific placeholder, a specific number of times. A fixture's own `Normalization Applied` field must describe the substitution precisely enough — the exact pattern matched (a path prefix, a line reference), the exact placeholder used, the exact count — that anyone re-applying it to the same raw source would arrive at an identical result, not merely a general description of what kind of thing was removed.

This is not a claim that different real captures should look alike. Two genuinely different sessions will have different timestamps, different durations, different content, and normalization must never paper over that — determinism describes the *procedure* applied to one specific input, not a convergence between fixtures. A normalization step that made two different raw logs resemble each other more than they actually do would itself violate "must never alter runtime semantics," below.

**What normalization must never do.** Normalization is strictly substitutive, never interpretive. It must never alter **parser-relevant content** (anything a future Collector would actually read to extract a fact), **runtime semantics** (what genuinely happened during the captured session), **ordering** (the sequence lines appear in), **line counts**, or **formatting** (encoding, line-ending convention, whitespace) — beyond the exact, documented substitution itself. A normalized fixture's byte-length difference from its original capture should be fully and exactly explained by the substitution alone (a token of known length replacing another, a known number of times); anything else found in that diff is not normalization, it is an undocumented change and must not be committed silently.

**Every normalization must be recorded, not merely performed.** A fixture's own `README.md` (`TEMPLATE.md`'s `Normalization Applied` field) must state exactly what was changed, the placeholder used, and how many times — or explicitly state that nothing required it. The same discipline this corpus already applies to every other fact about a fixture: a stated conclusion, never a silent omission.

---

## Warning Categorization

**Discovered directly during acquisition, formalized before it depended on per-fixture judgment:** a captured log's own presence of the literal word "Warning" is not, by itself, informative — this corpus already established (Section: Runtime Log Normalization's sibling finding, first observed in `clean-base-game`) that Farming Simulator's base game emits genuine `Warning`-level lines with no mod installed at all. Once a fixture's own mod can *also* produce warnings, a single "is this benign?" judgment call is not enough — it invites exactly the kind of inconsistent, contributor-dependent classification this corpus's other policies (Normalization, Installed Mods vs. Savegame Mod State) already exist to rule out. Every warning a captured fixture's log contains must be classified into exactly one of three categories, **by attribution — an observable fact — not by an interpretive judgment of severity:**

- **Base-game warning.** The warning's exact text also appears in `clean-base-game`'s own captured log — a log with the global mods directory independently verified empty. This is a direct, checkable cross-reference, not an assumption about what "looks like" base-game content: if the identical line is present in a mod-free capture, it originates from the base game, full stop.
- **Fixture warning.** The warning is attributable to the fixture's own mod — it names or references the mod's own file, asset, or content directly — but does not indicate a compatibility problem, a script error, or a load failure. The referenced content still loads and functions; the session still completes normally. This is a real, mod-caused warning, and a future Rule must not treat it as a failure signal merely because it is mod-attributable.
- **Fixture-affecting warning.** The warning, or the condition it reveals, calls into question whether this log still validly represents its fixture's own documented Purpose and Expected Observable Behavior. None has been found in any fixture captured to date; if one is, the fixture's own validity — not merely its Known Limitations — must be re-evaluated before the fixture is considered `Captured`.

These are attribution categories, not a severity scale — deliberately. "Base-game" and "Fixture" both describe *where a warning comes from*, verifiable against real evidence (another fixture's own content, or the mod's own declared files); only "Fixture-affecting" asks a genuinely interpretive question, and it is scoped narrowly, to the fixture's own validity, not to how alarming a warning sounds. A fixture's own `README.md` must classify every warning it contains this way, in its Known Limitations field.

---

## Installation state versus savegame state

**Discovered directly during acquisition, not assumed:** creating a new savegame does not produce a "clean" runtime log. A Farming Simulator runtime log enumerates installed mods from the game's own global mods directory, regardless of which savegame is active and regardless of whether that savegame itself uses any of them. A fresh savegame with mods still present in the global mods directory does **not** yield a mod-free log — the two are independent facts, and this corpus's own metadata schema (`TEMPLATE.md`) now records them as two separate fields, **Installed Mods (global)** and **Savegame Mod State**, rather than one conflated "Enabled Mods" field.

This is a corpus-wide acquisition discipline, not a fact specific to one fixture: **every fixture's own "clean" or "single-mod" claim must be verified against the global mods directory's actual contents at capture time, never inferred from savegame freshness alone.** A fixture whose documentation states "one mod" but was captured without checking that the mods directory contained *only* that one mod is not a valid single-mod fixture — it may be silently enumerating others alongside it.

Whether this same installation-state/savegame-state distinction matters to modIQ's own eventual Evidence model (e.g., whether a future Collector needs to represent "installed" and "used" as two different facts) is an open question for Architectural Resolution, not decided or assumed here. This corpus's own job is only to capture and document the real log content accurately enough that question can be answered from evidence when it is asked.

---

## Structure

One directory per fixture, at this level, named descriptively in kebab-case (`clean-base-game`, not `fixture-1` or `01-clean`). Each fixture directory contains:

- Its own `README.md` — the fixture's metadata, using the schema in `TEMPLATE.md`.
- Its own raw log file(s), once captured — stored verbatim, subject only to the documented normalization recorded in that fixture's own `README.md` (see "Runtime Log Normalization," above), never edited afterward beyond that.

No shared or cross-fixture data lives at this level beyond `README.md` (this file) and `TEMPLATE.md`. If a genuine cross-cutting need emerges later (e.g., many fixtures sharing an identical mod list), that is a future forcing function to evaluate then — not something to build speculatively now.

---

## Adding a new fixture

1. **Before capturing anything, verify the global mods directory's actual contents directly** — do not infer them from savegame state, and do not assume a new savegame "clears" anything. Record exactly what was checked.
2. Copy `TEMPLATE.md` into a new, descriptively-named directory as that directory's own `README.md`.
3. Fill in every field, including both **Installed Mods (global)** and **Savegame Mod State** separately. Do not leave **Source**, **Consent / Licensing**, or **Normalization Applied** blank.
4. **Normalize personally identifying or machine-specific information before the file is ever committed** (see "Runtime Log Normalization," above). Git history is not a safe place to fix this afterward — a mistake caught after a push is a real incident, not a follow-up task.
5. Commit the fixture's raw log file and its `README.md` together, in the same commit.
6. **Never edit a captured fixture's raw log content in place.** If a correction, a different capture, or a re-capture against a newer game version is needed, add a new fixture directory and mark the old one's `Status` as `Superseded`, naming its replacement directly in its own `README.md`. This mirrors this project's own standing convention that historical records are superseded, never silently rewritten.

---

## Platform independence

Farming Simulator's own log output is not guaranteed to be identical across Windows, macOS, and Linux. This corpus's own structure makes no assumption either way — the `Platform` field on every fixture is load-bearing, and fixtures from more than one platform are expected to coexist here as they become available, not to be treated as interchangeable.
