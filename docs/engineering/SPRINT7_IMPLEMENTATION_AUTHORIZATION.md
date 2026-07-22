# Sprint 7 Implementation Authorization

| Property | Value |
|----------|-------|
| **Document** | SPRINT7_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Final pre-implementation authorization record for Sprint 7 — resolves the last open architectural item (XML parsing dependency) and confirms readiness |
| **Prepared by** | Lead Engineer, on `feature/runtime-implementation` |
| **Repository baseline** | HEAD `af65bf0` |
| **Related documents** | `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md` |
| **Status** | No code changed. No commits. No branch created. Awaiting Chief Architect implementation authorization. |

---

# Resolved Architectural Decisions

Recorded here as authoritative, per this session's authorization, cross-referencing the two prior documents rather than restating their own analysis:

- Collection is an Assessment phase, not a standalone component.
- `AssessmentService` performs collection orchestration, as direct inline composition.
- Collectors remain independent; no Collector consumes another Collector's output.
- No `CollectionCoordinator` is introduced.
- XML inspection is the first implementation of Multi-Source Evidence Collection.
- **A missing `modDesc.xml` SHALL be collected as Evidence** — resolves `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`'s absence-as-evidence question: an absence, not silent Empty Collection.
- **Sprint 7 SHALL introduce Evidence only** — resolves the Rule Engine scope question: no new Rule this Sprint, mirroring the Sprint 4 → 5 precedent (`StructuralDuplication` shipped a full Sprint before `StructuralDuplicationRule` existed).
- **Sprint 7 SHALL NOT introduce Rule Engine evaluation.**
- **No Governance Register item shall be opened at this time** — resolves the GOV-timing question left open in both prior documents.

With this session's dependency selection, every item either document left open is now resolved.

---

# XML Parsing Dependency Recommendation

## Repository review

Checked directly: nothing in this repository references any XML crate today (`crates/modiq-collection/Cargo.toml` has exactly three dependencies — `modiq-runtime`, `thiserror`, `zip` — no XML parser, no fixture, no prior discussion). This is a genuinely fresh decision, not a confirmation of something already implied.

## Candidates considered

| Crate | Shape | Fit for this task |
|---|---|---|
| **`roxmltree`** | Read-only DOM tree, parse-once-query-freely | Strong |
| **`quick-xml`** | Streaming, event-based (SAX-style) reader/writer, optional `serde` | Weak fit for this task, despite being the most widely used XML crate in the Rust ecosystem generally |
| **`quick-xml` + `serde`** | Typed deserialization into Rust structs | Rejected — see below |
| **`xml-rs`** | Older streaming (SAX-style) reader | Rejected — superseded in practice by `quick-xml` |
| **`minidom`**, **`xmltree`** | DOM-style, smaller ecosystems | Rejected — no compelling advantage over `roxmltree` for this use case, less widely used |

## Recommendation: `roxmltree`

**Why it fits this task specifically.** XML inspection's job, as scoped for Sprint 7, is bounded queries against a small, complete document: does `modDesc.xml` exist and parse as well-formed XML, which required elements/attributes are present or absent, what dependency declarations does it name. That is a *read, then ask questions of a tree* problem, not a *stream and transform* problem. `roxmltree` is a read-only DOM — parse once, then query with `.children()`, `.attribute()`, `.text()` — matching this shape directly, with no manual state-tracking of a start/end tag stack the way an event-based reader would require for the same queries.

**Why it fits this project's stated priorities.** The instruction was explicit: optimize for correctness, maintainability, and architectural simplicity, not theoretical maximum throughput. `roxmltree` is a small, focused, mature, minimally-dependent crate whose entire job is "parse a document, let me query it" — the same "one focused job, done well" shape `zip` already occupies in this workspace, and the same shape this platform's own Collector Contract rewards (a Collector that does one thing, deterministically, with clear failure modes).

**A safety property worth naming directly, not incidental.** `modDesc.xml` comes from untrusted, community-submitted mod content — the same "genuinely adversarial input surface" `EvidenceCollection.md` already named explicitly when evaluating the `zip` crate for archive collection (Sprint 4 Phase 2's empirical investigation). `roxmltree` does not process DTDs or resolve external entities — by design, not by omission — which removes XML's classic adversarial-input attack surface (entity-expansion/XXE-style attacks) from this Collector entirely, rather than requiring this platform to configure a more capable parser defensively. For a Collector whose only job is to observe untrusted content safely, a parser that structurally cannot be tricked into entity expansion is a stronger default than a more capable one configured to behave safely.

**Why not `quick-xml`, despite being the ecosystem's more widely-used choice.** This is not a maturity or quality concern — `quick-xml` is mature, well-maintained, and well-documented. It is a fit concern: its strengths (streaming, zero-copy, write support, very high throughput) solve problems this Collector doesn't have. `modDesc.xml` files are small manifests, not large documents needing streaming throughput; this Collector never writes XML; and reconstructing "does element X exist, what are its children" from a raw event stream requires more code, doing the same job `roxmltree`'s tree API gives for free. Using the more powerful tool here would be complexity spent on a capability this Collector will never need — the same "capability before abstraction" discipline this project applies everywhere else, applied to a dependency choice.

**Why not a `serde`-based typed deserialization approach.** Tempting for "clean" typed extraction, but it requires committing to a fixed Rust struct shape mirroring `modDesc.xml`'s schema up front, and fails harder on documents that are malformed in one place but still meaningfully inspectable elsewhere — exactly the kind of partial-but-real information a query-based tree walk tolerates gracefully and a strict deserializer does not. It also pulls in more machinery (`serde` plus derive macros) than "observe specific, named facts" actually requires.

## Known limitations, disclosed directly

- **No streaming — the full document loads into memory as a tree.** Irrelevant at `modDesc.xml`'s typical size (a few kilobytes), but this recommendation would not automatically extend to a future Collector needing to process much larger XML content.
- **No write or serialization support.** Not a gap for this Collector — Evidence Collection never produces XML, only observes it — but worth naming so it isn't silently assumed to cover a need it doesn't have.
- **No schema (XSD) validation against Farming Simulator's own `modDesc.xml` structure.** `roxmltree` validates well-formedness only. This is consistent with, not a gap in, the existing architecture — judging whether a well-formed document is *structurally adequate* is interpretation, which belongs to the Rule Engine (`EvidenceCollection.md`'s own Non-Responsibilities), not to Collection.

---

# Sprint Confirmation

**Capability:** Multi-Source Evidence Collection.

**Sprint 7 Scope:**
- XML Evidence collection (`XmlCollector`, discovery, well-formedness, content extraction — `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md` Phases 2–3)
- Evidence normalization (consistent `XmlInspection`-category Evidence shape, including the now-resolved missing-manifest case)
- `AssessmentService` orchestration (Phase 4 — inline composition, no coordinator, per the approved architecture)
- Evidence exposure (existing `AssessmentReport`, `modiq-cli`, Sandbox — no structural change required, confirmed in the capability plan's AssessmentReport impact analysis)

**Out of Scope, confirmed unchanged:**
- Rule evaluation (`modiq-rules` untouched this Sprint)
- Dependency analysis (declared-dependency *identification* is in scope as a factual observation; cross-referencing or resolving those dependencies against other mods is not)
- Knowledge inference (architecturally prohibited for a Collector regardless of Sprint scope — `EvidenceCollection.md`)
- Assessment scoring (not a capability this platform has anywhere — consistent with `Vision.md`'s explain-don't-score principle)

This matches `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`'s own scope exactly, narrowed only by this session's Rule Engine and absence-as-evidence resolutions — no scope was redesigned.

---

# Implementation Readiness

**1. Is Sprint 7 implementation scope fully defined?** Yes. Capability, in-scope work, and out-of-scope boundaries are all confirmed above, consistent with the capability plan.

**2. Does any unresolved architectural ambiguity remain?** No. Composition architecture was resolved in the prior session; Rule Engine timing, absence-as-evidence, and Governance Register timing were resolved by this session's authorization; the dependency question is resolved by this document's recommendation.

**3. Can Sprint 7 implementation begin?** Yes, pending your acceptance of the `roxmltree` recommendation above — this document proposes it; it is not authorized for use until you confirm it, consistent with this project's standing "no new external dependency without explicit authorization" rule.

**Sprint 7 is ready for Chief Architect implementation authorization.**

---

Await implementation authorization. No code changed, no commits, no branch created.
