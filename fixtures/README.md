# Engineering Sample Corpora

This directory holds real, external, provenance-tracked engineering sample data — content this platform did not author, captured or acquired from the real world specifically to ground an engineering decision in fact rather than assumption.

Full rationale for why this directory exists in this shape lives in `docs/engineering/SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md`. This file is the short, standing orientation for anyone encountering the directory itself.

---

## Why this exists, and how it differs from `apps/sandbox/src-tauri/fixtures/`

This repository already has one fixture directory, `apps/sandbox/src-tauri/fixtures/`, and the two are not the same kind of thing:

- **`apps/sandbox/src-tauri/fixtures/`** holds small, trivial, engineer-authored content (`sample.txt`, a hand-built ZIP) that exists purely to exercise the Sandbox's own real-I/O code paths. Nothing in it claims to represent real-world data; it is synthetic by design, and that is entirely appropriate for what it is used for.
- **`fixtures/` (this directory)** holds real-world artifacts this platform did not create — content whose actual, unpredictable shape is the entire reason it is being stored, because no specification or assumption can substitute for it. This is why it carries a heavier documentation burden (provenance, consent, redaction) than a synthetic fixture ever needs to.

If new engineering work needs a small, disposable, synthetic file to exercise a code path, it belongs alongside the code that uses it (as `apps/sandbox/src-tauri/fixtures/` already demonstrates) — not here. This directory is reserved for real, external, provenance-bearing material only.

---

## What belongs here

- Real captured artifacts whose exact structure, wording, or formatting is not something this project controls or can safely assume — Farming Simulator runtime logs are the first example.
- Each kind of artifact gets its own top-level subdirectory (`runtime-logs/` today), self-contained: its own `README.md`, its own metadata schema, its own per-fixture documentation.

## What does not belong here

- Synthetic or invented content of any kind, standing in for real data "for now." A documented, empty `Awaiting capture` fixture is correct; a placeholder file with made-up content is not.
- Engineer-authored test data with no real-world provenance — that belongs with the code it tests, per the existing `apps/sandbox/src-tauri/fixtures/` precedent.

---

## How future corpora should be organized

Only one corpus exists today: `runtime-logs/`. If a second real-world-data need arises in the future (real Lua scripts for static analysis, real asset files for asset validation), it should become its own sibling subdirectory here, following `runtime-logs/`'s own shape exactly: a corpus-level `README.md`, a `TEMPLATE.md` for its own metadata schema, and one directory per fixture. No shared taxonomy, common schema, or cross-corpus abstraction should be introduced preemptively — `runtime-logs/` is the first instance of this pattern, not evidence a general one is needed yet. That question is a forcing function for whichever future session introduces a genuine second corpus, not something to anticipate here.
