# ADR-0007: Runtime Entity Design Pattern

| Property | Value |
|----------|-------|
| **ADR** | 0007 |
| **Title** | Runtime Entity Design Pattern |
| **Status** | Accepted |
| **Project** | modIQ |
| **Date** | 2026-07-19 |

---

# Context

Sprint 2 gave Evidence, Finding, and Recommendation real field content, extended Assessment with relationship-resolution behavior, and introduced two new opaque reference types (RuleReference, RepairRecipeReference). Doing so required resolving several implementation-level design questions that DataModel.md, RuntimeInvariants.md, and the existing ADRs left open: how Runtime entities should be identified, how they should relate to one another, how they should be validated at construction, and how — or whether — they should reference the Knowledge Domain.

These questions were resolved consistently across all three entities during Sprint 2, each following the identity and construction pattern already established by `AssessmentId` and the `Assessment` aggregate itself in Sprint 1. This ADR records the resulting pattern as a durable architectural decision, so that future Runtime entities extend it deliberately rather than each reinventing an answer.

This ADR describes the pattern established, not the specific fields or methods of any one type. Field lists and method signatures remain implementation detail, free to change without a new ADR, so long as they continue to follow the pattern recorded here.

---

# Decision

Runtime entities in modIQ shall follow a single, consistent design pattern, with the following elements.

## Aggregate Root Pattern

`Assessment` remains the sole aggregate root of the Runtime Domain (ADR-0003). Evidence, Finding, and Recommendation are owned exclusively by Assessment and are mutated only through its own methods (`add_evidence`, `add_finding`, `add_recommendation`). No child entity mutates itself, another entity, or the aggregate's collections directly.

## Entity Identity

Every Runtime entity that another entity or component needs to reference receives a stable identity — `AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId` — generated once, at construction, using a single, consistent mechanism (a process-local, monotonic counter) rather than a different scheme per entity. Identity never changes for the lifetime of the value and is never derived from field content.

## Value Objects

Runtime entities are composed of plain value-object fields — strings, enums, and value-object identifiers — rather than behavior-bearing objects. An entity holds data; it does not hold logic that acts on other entities or on the aggregate that owns it.

## Opaque Runtime References

Where a Runtime entity must refer to something owned by another domain — the Rule that produced a Finding, the Repair Recipe that informs a Recommendation — it holds an opaque reference value (`RuleReference`, `RepairRecipeReference`) rather than the referenced domain's own type. Runtime stores these references; it does not resolve, own, or evaluate what they identify. This preserves the Knowledge Domain boundary (ADR-0001; GOVERNANCE.md: Knowledge Domain) while still letting the entity that needs traceability data carry it.

## Constructor Validation

Every Runtime entity with descriptive text content validates that content at construction, through a fallible constructor (`Evidence::new`, `Finding::new`, `Recommendation::new`) returning a dedicated error type (`EvidenceError`, `FindingError`, `RecommendationError`). Validation is deliberately minimal: it rejects content that cannot mean anything — empty or whitespace-only descriptive text — and nothing more. An entity is never constructed in a state that violates its own minimal validity.

## Identity-Based Equality

Runtime entities compare equal only when they share the same identity, not merely the same content. Two independently constructed entities with identical field values are distinct entities and are not equal. This follows directly from Entity Identity: callers that need to compare "the same entity across two reads" must compare the same value, or a clone of it, rather than two separately constructed instances with matching content.

## Aggregate-Owned Relationship Resolution

Cross-entity references — a Finding's Evidence references, a Recommendation's Finding references — are resolved only by the aggregate root, since only Assessment holds every side of the relationship. Child entities never resolve their own references; they hold only the identifiers needed for the aggregate to do so. Relationship resolution is read-only and does not, by itself, validate or reject anything about the relationship it resolves.

## Governance-Controlled Invariants

Whether a relationship must resolve — whether a Finding must reference at least one Evidence item, whether a Recommendation's Finding references must exist within the Assessment — is a content-level invariant, not a structural one. Introducing such an invariant is a Level 3 (Behavioral) change under GOVERNANCE.md and requires governance approval before enforcement code is written. Until approved, relationship resolution remains permissive: unresolvable references are omitted from resolution results, never rejected at construction or at aggregate-mutation time.

## Deterministic Runtime Behavior

Every Runtime entity constructor and every Assessment mutation method is deterministic: identical input produces identical output, with the sole, deliberate exception of each entity's own freshly assigned identity (ADR-0005). Determinism is judged by content, not by incidental identity — a rule already applied consistently across every test written against this pattern during Sprint 2.

---

# Consequences

Benefits include:

- A single, predictable shape for every Runtime entity, so a new entity can be reviewed against an established pattern rather than requiring its own design conversation.
- A structural boundary against Knowledge Domain ownership, satisfied by opaque references rather than by discipline alone.
- Aggregate integrity concerns live in exactly one place — Assessment — never duplicated or reinvented per entity.
- A named, repeatable path (the Governance Register) for the invariant questions this pattern surfaces, rather than each one being resolved ad hoc during implementation.

Trade-offs include:

- Relationship resolution is currently permissive by necessity. Two Governance Register items (GOV-005, GOV-006) remain open as a direct consequence of this pattern's deliberate restraint, and resolving them will require revisiting `Assessment`'s mutation methods once governance approval lands.
- Opaque references (`RuleReference`, `RepairRecipeReference`) carry no structure of their own today; they are placeholders for a future Knowledge Domain integration that has not yet begun.

---

# Relationship to Other Specifications

This decision is reflected in:

- DataModel.md
- RuntimeInvariants.md
- GOVERNANCE.md
- SPRINT2_IMPLEMENTATION_PLAN.md
- ADR-0001 (Runtime Domain vs Knowledge Domain)
- ADR-0003 (Assessment as the Aggregate Root)
- ADR-0005 (Deterministic Assessment Engine)

Future Runtime entities should follow this pattern. Deviating from it — introducing an entity without a stable identity, resolving a relationship outside the aggregate root, or referencing a Knowledge Domain type directly — should be treated as an architectural question requiring its own ADR, not an implementation detail.

---

# Status

Accepted.

This decision is authoritative as of Engineering Release 0.2.
