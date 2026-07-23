# INV-001 — Lua Analysis Capability Investigation

| Property | Value |
|---|---|
| **Document** | INV-001_LUA_ANALYSIS_CAPABILITY.md |
| **Project** | modIQ |
| **Purpose** | Evidence-acquisition investigation into whether a real, bounded, deterministic Lua Analysis capability exists for modIQ to recognize — the mandatory precondition to any future Architecture Evaluation, per this project's own evidence-before-architecture discipline |
| **Origin** | Follows CAP-001 (Lua Analysis Capability Proposal), whose recommendation — evidence insufficient for Architecture Evaluation, further investigation required — was accepted by the Chief Architect. CAP-001 itself was conversational scoping only and is not a separate repository artifact; its accepted conclusion is what authorized this investigation. |
| **Status** | **Complete. Recommendation A selected (evidence remains insufficient; further investigation required). Architecture Evaluation has NOT been authorized. Sprint Planning has NOT begun.** See "Investigation Status," below, for the complete, explicit statement. |
| **Governing Follow-up** | GOV-014 (Lua Fixture Acquisition Governance) — opened as a direct consequence of this investigation's own findings; see `GOVERNANCE.md`. |

---

# Investigation Status

Recorded explicitly, not left implicit, per this project's own standing discipline of stating a decision to wait as a decision, not an absence of one:

- ✅ **Investigation completed.**
- ✅ **Recommendation A selected** — evidence remains insufficient; further investigation required.
- ⛔ **Architecture Evaluation has NOT been authorized.**
- ⛔ **Sprint Planning has NOT begun.**
- ⏳ **Additional evidence is required before Architecture Evaluation may proceed** — specifically, human-performed acquisition of real Farming Simulator mod Lua scripts, gated on GOV-014's own resolution (provenance, licensing, storage, and acquisition governance for this new kind of fixture) before any such acquisition may begin.

This investigation produced no Rust source, no test, no fixture, no ADR, and (directly) no Governance Register item — the one Governance Register item it did lead to (GOV-014) is a distinct, explicit governance action, recorded separately and reasoned about on its own terms, not silently bundled into this document's own scope.

---

The findings below are preserved as originally produced, with only header/status formatting brought into repository convention — no substantive rewording of the investigation's own reasoning, evidence, or recommendation.

---

## 1. Real-World Lua Landscape

**Not directly observed this session, and stated honestly rather than inferred.** No real Farming Simulator mod's Lua script has been examined by this investigation. What *is* confirmed real: Farming Simulator's own publisher, GIANTS Software, maintains a public, no-login-required documentation portal (GDN — GIANTS Developer Network, `gdn.giants-software.com/documentation.php`) with a dedicated "Scripting API Reference" (LuaDoc) for FS25, FS22, and FS19 separately — confirming a real, defined, versioned Lua API surface exists for mods to target. This establishes that FS25 modding *has* a documented scripting surface (vehicles, placeables, specializations, and similar systems are the kind of thing such a reference would cover, per the GDN's own section structure) — but this investigation did not parse that reference's actual contents, and doing so would tell us what the API *permits*, not how mods actually *use* or *misuse* it. Those are different questions; only real samples answer the second one.

**Source:** [GDN Documentation](https://gdn.giants-software.com/documentation.php)

## 2. Representative Sample Collection

**Not obtained. Not obtainable within this investigation, and the reason is structural, not a lack of effort.** Acquiring real, representative Lua scripts requires the same precondition Sprint 10's own runtime-log fixtures required: a human with a real, licensed Farming Simulator installation, real installed third-party mods, and this project's own explicit consent/provenance discipline (a mod's own archive is never stored in this repository — only independently-verified factual metadata about it, per Sprint 10's standing policy). This is a human-performed acquisition activity. No amount of research substitutes for it, and attempting to download third-party mod archives from a modding site during this investigation would violate the exact consent-and-provenance standard Sprint 10 established and this project has never relaxed. **This is the single largest gap this investigation leaves open** — see Section 9.

## 3. Defect Landscape

Reported as two separate tiers, deliberately not blended: verified general facts about Lua as a language (real, checkable, sourced), versus what would need real samples to establish (not claimed here).

**Verified, language-level facts (not FS-specific, not fabricated):**
- Lua has a formally defined grammar; a parser either accepts or rejects a given source file — **syntax validity is an objective, binary, language-level fact**, independent of any specific mod or game.
- Lua is dynamically typed *and* dynamically scoped: a global is created by ordinary assignment, with no "declare before use" requirement anywhere in the standard language. This is a real, well-documented property of Lua itself, not an assumption — and it directly undercuts "undefined globals" as a clean target: what looks like an undefined global in a naive static read may be a deliberately dynamic, entirely valid pattern.

**Not established this session, and not claimed:** whether "invalid API usage," "deprecated function usage," "callback issues," "registration problems," or "nil dereferences" actually occur in real FS25 mods, at what frequency, or in what form. These require either real samples (to know what actually goes wrong) or the GDN LuaDoc's own structured content cross-referenced against real usage (to know what's valid) — neither exists yet.

## 4. Deterministic Recognition Assessment

| Candidate | Objectively recognizable? | Deterministically reproducible? | Explainable with evidence? | Separable from subjective style? | Verdict |
|---|---|---|---|---|---|
| **Syntax validity** | Yes — parser accepts/rejects | Yes — same source, same parser version, same result, always | Yes — the parser's own error location | Yes — grammar, not opinion | **Passes all four** |
| Undefined globals | No, not reliably | No — depends on dynamic patterns a static read cannot distinguish from valid ones | Only partially | No — flagging a legitimate dynamic-global pattern as wrong is a judgment, not a fact | **Fails** |
| Invalid/deprecated API usage | Plausible, *if* cross-referenced against a real, versioned API surface | Not yet demonstrated — requires structured LuaDoc content and real usage samples, neither in hand | Would be, if built | Yes, in principle | **Unresolved, not yet feasible** |
| Callback/registration issues, nil dereferences | Generally requires dataflow or runtime behavior, not static structure alone | No | No | No — these are typically judgment calls about intent | **Very likely outside what a static Collector can ever cleanly do** |

**Only one candidate — syntax validity — passes every test today, without requiring evidence this investigation could not obtain.**

## 5. Collector Boundary Analysis

**Pure fact, safely within the Collector Contract:** "this file does or does not parse as valid Lua [version], per parser X" — structurally identical to `XmlCollector`'s own "is `modDesc.xml` well-formed" check (Sprint 7), which this platform has already proven a Collector may state without crossing into judgment.

**Interpretive judgment, which a Collector must never make:** "this variable is probably meant to be local," "this callback pattern is likely wrong," "this is poorly engineered." Every defect class beyond syntax validity risks landing here, to varying degrees — "invalid API usage" least so (it can, in principle, remain a fact — "this identifier does not appear in the cited API reference" — *if* the reference itself is treated as ground truth rather than interpreted), "callback issues" and "nil dereferences" most so, since detecting them at all typically requires inferring *intent*, not just structure.

## 6. Recognition Target Selection

**The smallest viable deterministic target is Lua syntax validity — not "Lua Analysis" as a category.** This mirrors this project's own repeated "smallest real slice" discipline exactly (`VersionProfile::fs25()`'s one hardcoded value; `RepairRecipe`'s one authored recipe; `RuntimeLoadFailureRule`'s one recognized signature) and is architecturally the closest analog to `XmlCollector`'s own well-formedness check, not to a general static-analysis engine.

**Stated honestly, not oversold:** this target is narrower than the product vision "detect implementation issues" implies. It would catch a mod whose Lua script is outright broken (a real, if modest, value — and one deliverable *before* a player ever attempts to run the mod, unlike Sprint 11's own runtime-only detection). It would not catch the deeper engineering-quality concerns `ProductSpecification.md`'s own Creator objective names. Whether that narrower value is still worth pursuing is a judgment for the Recommendation, not something this section can resolve on its own.

## 7. Dependency Investigation

Real findings, sourced during this investigation:

- **`full_moon`** (crates.io/docs.rs) — a lossless Lua 5.1/5.2/5.3/5.4/Luau parser written in Rust, MPL-2.0 licensed, version 2.2.0 (published 2026-07-09), ~1.12M all-time downloads, ~279K recent downloads — actively maintained by a real, ongoing project (also the parser behind `StyLua`, a real, shipping Lua formatter). Critically: **it is parse-only — it builds an AST, it does not execute anything.** This is the same safety property `roxmltree` was chosen for at Sprint 7 (no DTD/entity expansion, read-only) — inspecting untrusted, community-submitted content without ever running it.
- **`mlua` / `rlua`** — Lua *bindings*, not parsers: they embed a real Lua interpreter to *execute* Lua code from Rust. `rlua` is now explicitly deprecated in favor of `mlua`. **Both are architecturally inappropriate for this capability regardless of maturity** — executing untrusted, community-submitted script content to analyze it would be a severe, unnecessary security risk this platform has no comparable precedent for taking, and no static-analysis need requires it.

**No recommendation to adopt `full_moon` is made here** — only that it is, today, the sole candidate this investigation's research found that fits the safety requirement this platform has already applied once (`roxmltree`). Adoption remains a future Architecture Evaluation's own decision, contingent on real samples existing to test it against.

**Sources:** [full_moon on docs.rs](https://docs.rs/full_moon/latest/full_moon/), [full_moon on crates.io](https://crates.io/crates/full_moon), [mlua on crates.io](https://crates.io/crates/mlua/)

## 8. Risk Assessment

- **Technical:** Low, specifically for the syntax-validity target — `full_moon` is mature, safe, and does exactly one well-bounded thing. Risk rises sharply for any broader target, both technically (needing to safely extract and structurally compare against a live API reference) and in the dependency itself (none evaluated here would be appropriate for anything beyond parsing).
- **Architectural:** Real, for any target beyond syntax validity — genuine risk of eroding the Collector/Rule observational boundary this platform has never once breached.
- **Product:** Real and unresolved — the one target with a clean deterministic story (syntax validity) may be too narrow to meaningfully answer "detect implementation issues"; a genuinely valuable target may not exist within this platform's own boundary constraints at all.
- **Maintenance:** Low but real — this would be the platform's first Lua-specific dependency, a new surface, though `full_moon`'s own real download/maintenance activity is reassuring evidence, not a guarantee.
- **False-positive risk:** High for anything beyond syntax validity — Lua's own dynamic-scoping nature means naive checks for "undefined globals" or similar would very likely flag legitimate, idiomatic code.
- **False-negative risk:** High for syntax validity specifically, relative to the *broader* product goal — most real-world "this mod behaves badly" cases are almost certainly syntactically valid Lua that is simply wrong in ways only execution or deep semantic analysis reveals, which this narrower target would never catch.
- **A further risk, surfaced by cross-referencing this platform's own existing work:** a Lua syntax error severe enough to matter would very likely also prevent the mod from loading at runtime — meaning a future `RuntimeLogCollector`-observed failure (Sprint 11) might already surface a closely related fact, *after* the mod is run. A pre-runtime syntax-validity Collector's own incremental value is answering the same class of question earlier, not answering a question nothing else can ever answer — a real, meaningful product distinction, but a narrower one than it may first appear.

## 9. Evidence Gaps

**Resolved this investigation:**
- A safe, mature, parse-only Lua parsing dependency exists (`full_moon`); execution-capable alternatives (`mlua`/`rlua`) are correctly ruled out for this purpose.
- FS25's own official scripting API documentation exists and is publicly reachable without authentication.
- Exactly one candidate defect class — syntax validity — passes every determinism/boundary test without requiring evidence this investigation could not obtain.

**Unresolved, and separated explicitly from what was resolved:**
- **No real FS25 mod Lua script has ever been examined by this project.** This remains the load-bearing gap; it requires human-performed acquisition, mirroring Sprint 10 exactly, and nothing in this investigation substitutes for it.
- Whether syntax errors occur with any meaningful frequency in real, released mods is unknown — it is possible this deterministically-clean target is also a rare one in practice.
- Whether the syntax-validity target's own incremental value (Section 8's runtime-overlap finding) is strong enough to justify building it at all, once weighed against Sprint 11's own already-existing runtime detection.
- Whether any recognition target beyond syntax validity can ever be made to fit this platform's own Collector/Rule boundary — genuinely open, not merely unstarted.

## 10. Recommendation

**A. Evidence remains insufficient. Further investigation required.**

This investigation made real, concrete progress — it eliminated an entire class of unsafe dependency (Lua-executing bindings), identified the one dependency shape that fits this platform's own safety discipline, and narrowed "Lua Analysis" to the one candidate target (syntax validity) that survives contact with Lua's own dynamic-language properties and this platform's Collector boundary. But the investigation's own central question — "what specific Lua problem can modIQ objectively recognize" — is not yet fully answerable, because the one artifact that would answer it, real Farming Simulator mod Lua scripts, does not exist anywhere in this project and cannot be produced without a human performing the acquisition.

**The concrete next step:** human-performed acquisition of a small number of real, licensed, third-party FS25 mod Lua scripts — mirroring Sprint 10's own fixture-acquisition precedent — to establish (a) whether syntax errors occur in practice at all, and (b) whether the resulting capability's value survives the runtime-overlap question named in Section 8, before any Architecture Evaluation begins. **This acquisition itself must not begin before GOV-014 (Lua Fixture Acquisition Governance) is resolved** — provenance, licensing, storage, and acquisition-governance questions specific to committing real third-party source code (a materially different artifact from a captured runtime log) have not yet been answered, and Sprint 10's own runtime-log policies do not automatically transfer to this different kind of fixture.

This is Recommendation A, not B. No Architecture Evaluation or Sprint Planning follows from this document alone.

---

# Document Status

**Current Version:** 1.0.0

**Status:** Complete. Recommendation A. Committed to the repository as the permanent record of this investigation, superseding its own prior conversational-only existence. Architecture Evaluation and Sprint Planning both remain unauthorized pending the evidence named in Sections 2, 9, and 10, and pending GOV-014's own resolution.
