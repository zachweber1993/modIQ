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
- **Runtime logs** — the actual content this corpus holds: raw, unmodified (redaction excepted) text Farming Simulator itself produced. A runtime log, on its own, is not a platform concept — it is exactly what it appears to be, nothing more, until something inside the platform inspects it.
- **Platform Evidence** — `DataModel.md`'s own, formal Runtime Domain concept: "objective information collected during an Assessment," produced exclusively by a Collector from an Assessment Input. A raw runtime log sitting in this directory is not Evidence and does not become Evidence merely by being stored here. It becomes Evidence only once a real Collector — not yet built, not this session's concern — reads it and produces a structured `Evidence` item from it, the same way a `modDesc.xml` file on disk is not Evidence until `XmlCollector` actually inspects it.

This distinction is not pedantic. Calling a raw log "Evidence" before any Collector exists would misstate what this corpus actually is: reference material for engineering decisions, not a Runtime Domain artifact.

---

## Structure

One directory per fixture, at this level, named descriptively in kebab-case (`clean-base-game`, not `fixture-1` or `01-clean`). Each fixture directory contains:

- Its own `README.md` — the fixture's metadata, using the schema in `TEMPLATE.md`.
- Its own raw log file(s), once captured — stored verbatim (redacted only as documented in that fixture's own `README.md`), never edited afterward.

No shared or cross-fixture data lives at this level beyond `README.md` (this file) and `TEMPLATE.md`. If a genuine cross-cutting need emerges later (e.g., many fixtures sharing an identical mod list), that is a future forcing function to evaluate then — not something to build speculatively now.

---

## Adding a new fixture

1. Copy `TEMPLATE.md` into a new, descriptively-named directory as that directory's own `README.md`.
2. Fill in every field. Do not leave **Source**, **Consent / Licensing**, or **Redaction Applied** blank.
3. **Redact personally identifying information before the file is ever committed.** Git history is not a safe place to fix this afterward — a redaction mistake caught after a push is a real incident, not a follow-up task.
4. Commit the fixture's raw log file and its `README.md` together, in the same commit.
5. **Never edit a captured fixture's raw log content in place.** If a correction, a different capture, or a re-capture against a newer game version is needed, add a new fixture directory and mark the old one's `Status` as `Superseded`, naming its replacement directly in its own `README.md`. This mirrors this project's own standing convention that historical records are superseded, never silently rewritten.

---

## Platform independence

Farming Simulator's own log output is not guaranteed to be identical across Windows, macOS, and Linux. This corpus's own structure makes no assumption either way — the `Platform` field on every fixture is load-bearing, and fixtures from more than one platform are expected to coexist here as they become available, not to be treated as interchangeable.
