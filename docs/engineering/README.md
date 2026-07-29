# Engineering

Engineering documentation records the platform's actual architectural and implementation history: what has been built, what was decided along the way, and why. Unlike `docs/product-design/` and `docs/interaction-design/` — each a closed, frozen corpus produced by a bounded phase — this directory is a **living record** that has grown continuously since Sprint 0 and will keep growing. It has no reading order to complete and no closeout to certify as a whole; individual initiatives within it close, the directory does not.

---

## Authority

Engineering documentation implements the architecture; it does not define it. Authority flows from `docs/architecture/`, `docs/adrs/`, and `docs/implementation/RuntimeInvariants.md` — engineering records here must remain consistent with those documents, not the reverse. Where an engineering record surfaces a genuine conflict with an authoritative architecture document, that conflict is raised for governance review (a Governance Register item, an Architecture Evaluation), never resolved silently within the engineering record itself.

Since Sprint 14's own reconciliation established the Product & Interaction Design track, engineering documentation is also accountable to the frozen `docs/product-design/` and `docs/interaction-design/` corpus wherever an initiative's own scope touches it — most directly, everything organized under `ENGINEERING_ALIGNMENT_PROGRAM.md`.

---

## Document Categories

Engineering documentation is a large and continually growing body of architectural, governance, sprint, and implementation records, and will keep growing for as long as the platform does. Rather than enumerate every document (a list of this kind would drift out of date faster than it could be kept accurate — the exact failure mode `PROJECT_HANDOFF_v1.1.md`'s and `CHIEF_ARCHITECT_HANDOFF_v1.1.md`'s own staleness already demonstrated), this index organizes records by category and names the specific documents that are authoritative *right now* (below), so that it remains accurate regardless of how large this directory grows. For the complete chronological record, see `ENGINEERING_LOG.md`.

| Category | Contains | Naming pattern |
|---|---|---|
| **Handoffs** | Point-in-time continuity briefings for a role (Chief Architect, Lead Engineer, Technical Director, Project) — superseded by later versions, never edited in place | `*_HANDOFF_v*.md` |
| **Governance** | The Governance Register and its supporting records | `GOVERNANCE.md`, `GOV-*`, `GOV###_*` |
| **Alignment Programs** | Master roadmaps governing a bounded body of architectural evolution work spanning multiple initiatives — organizes evaluation sequence and process; does not perform evaluation itself | `*_ALIGNMENT_PROGRAM.md` |
| **Architecture Evaluations & Resolutions** | Per-question evaluation of alternatives and the resulting decision, for a specific Governance Register item or named architectural question | `*_ARCHITECTURE_EVALUATION.md`, `*_ARCHITECTURAL_RESOLUTION.md` |
| **Implementation Authorizations & Sprint Plans** | The engineering envelope translating an Architectural Resolution into scoped, authorized implementation work | `*_IMPLEMENTATION_AUTHORIZATION.md`, `SPRINT*_IMPLEMENTATION_PLAN.md`, `*_SPRINT_PLAN.md` |
| **Sprint Records** | Capability definitions, implementation reports, deviations, and repository reviews produced during or immediately after a Sprint | `SPRINT#_*.md` |
| **Engineering Releases** | The frozen, versioned record of a completed Sprint or milestone | `ENGINEERING_RELEASE_*.md` (see also `docs/releases/` for earlier releases predating this directory's current convention) |
| **Investigations** | Evidence-acquisition work preceding a capability question, deliberately scoped short of architecture or implementation | `INV-*.md` |
| **Proposals** | Draft architectural or governance decisions staged for review before acceptance, per this repository's two-step drafting discipline | `PROPOSAL_*.md`, `*_PROPOSAL.md` |
| **Reviews, Studies & Assessments** | Retrospective or analytical work not itself a decision — repository health checks, workflow consolidation, milestone reviews | `*_REVIEW.md`, `*_STUDY.md`, `*_ASSESSMENT.md`, `*_REPORT.md` |
| **Product-Track Continuity Records** | Phase-transition records for the Product Definition / Product Design / Interaction Design track — carry no Documentation Authority ranking; recorded here only because that track's own closeouts live in this directory by convention | `PRODUCT_DEFINITION_CLOSEOUT.md`, `PRODUCT_DESIGN_CLOSEOUT.md` |

A future category not yet represented (an "Engineering Guide" or a standalone "Implementation Note," for instance) should be added to this table at the point the first such document is created, following whatever naming pattern that document establishes — not pre-declared here speculatively.

---

## Start Here — Authoritative & Current

The category table above describes *where things go*; these are the specific documents that are *current*, right now, and should be read before any older document in the same category:

| Document | Role |
|---|---|
| `GOVERNANCE.md` | The Governance Register — every architectural question ever raised, its status, and its resolution. The single source of truth for "what's open, what's resolved." |
| `ENGINEERING_LOG.md` | The chronological engineering record — every Sprint, evaluation, and governance action, in order, with full rationale. The single source of truth for "what happened, and why." |
| `PROJECT_HANDOFF_v1.1.md` | Current Project-level continuity handoff. |
| `CHIEF_ARCHITECT_HANDOFF_v1.1.md` | Current Chief Architect continuity handoff. |
| `LEAD_ENGINEER_HANDOFF_v3.0.md` | Current Lead Engineer continuity handoff. |
| `ENGINEERING_ALIGNMENT_PROGRAM.md` | The active master roadmap governing reconciliation between the existing deterministic platform and the frozen Product Design / Interaction Design corpus. |

Where any document in this table is itself found to be stale, that staleness should be raised as its own reconciliation item (as `GOVERNANCE.md`'s own Sprint 14 entry already did for the handoff pair above) rather than corrected silently through an unrelated change.

---

## Status

Living index — created alongside `ENGINEERING_ALIGNMENT_PROGRAM.md`. This directory has no completion state to reach; this README's own currency is maintained by updating the "Start Here" table above whenever a handoff, the Alignment Program, or an equivalent authoritative document is superseded.

---

## See Also

- `docs/governance/PROJECT_STATUS.md` — current release, milestone, and phase status across both the Sprint/Engineering track and the Product & Interaction Design track.
- `docs/governance/ROADMAP.md` — the long-term development roadmap.
- `docs/adrs/README.md` — the Architecture Decision Record index; engineering documents implement what ADRs decide.
- `docs/product-design/README.md`, `docs/interaction-design/README.md` — the frozen corpus Engineering Alignment reconciles this platform against.
- `docs/DOCUMENTATION_MAP.md` — how this directory fits into the repository's documentation as a whole.
