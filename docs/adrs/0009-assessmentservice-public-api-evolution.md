# ADR-0009: AssessmentService Public API Evolution

| Property | Value |
|----------|-------|
| **ADR** | 0009 |
| **Title** | AssessmentService Public API Evolution |
| **Status** | Accepted (anticipatory — records that a breaking change is expected; the change itself is not authorized) |
| **Project** | modIQ |
| **Date** | 2026-07-19 |

---

# Context

ADR-0008 establishes Evidence Collection as a subsystem orchestrated by `modiq-engine`, invoked before Rule Engine evaluation, with its output added to the Assessment. Today, `AssessmentService::execute` (or whichever entry point ultimately orchestrates the pipeline) accepts no notion of "where the Assessment Subject's content is" — the sandbox currently constructs Evidence itself, ad hoc, before calling into the Engine at all. Once Evidence Collection exists, the Engine's public entry point will need to accept some form of input identifying the Assessment Subject's content, so it can pass that along to Evidence Collection.

`GOVERNANCE.md`'s Public API Policy treats public APIs as contracts and requires governance approval for breaking changes. `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` recommended that this specific consequence be recorded in its own ADR rather than combined with the subsystem-boundary decision, since the two are separable: the boundary decision (ADR-0008) does not itself require the API to change today, and the API's eventual new shape is a distinct design question with its own alternatives. The Technical Director directed that this ADR be created separately. This ADR does that: it records that the change is anticipated and why, without deciding its shape.

---

# Decision

`modiq-engine`'s public Assessment-execution entry point will require a breaking signature change at the point Evidence Collection is actually implemented, to accept an input identifying the Assessment Subject's content (an "Input Descriptor" — see `EvidenceCollection.md`).

This ADR accepts that consequence as an anticipated, necessary future change. It does **not** authorize implementing it now, and it does not select among the ways such a change could be shaped. Recorded for future reference, at the conceptual level only, are the kinds of approaches available when this is implemented — not a recommendation among them:

- Extending the existing entry point with an additional required input, changing its signature directly.
- Introducing a new, parallel entry point that accepts an Input Descriptor, with the existing entry point deprecated per the Public API Policy's `Deprecated` state rather than removed outright.
- Some other shape not yet considered, since concrete API design is explicitly out of scope for this ADR.

Whichever approach is eventually chosen must go through `GOVERNANCE.md`'s Public API Policy (breaking changes require governance approval) at the time it is proposed for implementation — this ADR does not substitute for that approval; it only establishes that the need is real and expected, so implementation planning isn't surprised by it.

---

# Consequences

- Existing consumers of `AssessmentService::execute` (today: `apps/sandbox`, and `modiq-engine`'s own integration tests) will need to change when this is implemented. This is flagged now so it is planned for, not discovered mid-implementation.
- No code changes result from this ADR. It exists to make an already-foreseeable breaking change a recorded architectural fact rather than an implementation surprise.
- The exact new signature, and whether it is a breaking modification or an additive parallel path, remains an open question for the implementation phase, tracked as GOV-008.

---

# Relationship to Other Specifications

- ADR-0008 (Evidence Collection Subsystem Boundary) — this ADR records a direct consequence of that decision.
- `GOVERNANCE.md` — Public API Policy governs how this change must eventually be approved; GOV-008 tracks the open design question.
- `EvidenceCollection.md` — defines the Input Descriptor concept this future API change would carry.

---

# Status

Accepted as a record of an anticipated, necessary future breaking change.

No implementation is authorized by this ADR. The specific API shape and its implementation require separate governance approval per GOV-008.
