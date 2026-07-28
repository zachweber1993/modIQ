# Repository Synchronization Policy

| Property | Value |
|----------|-------|
| **Document** | RepositorySynchronizationPolicy.md |
| **Project** | modIQ |
| **Purpose** | Formalizes how approved artifacts enter the repository, and distinguishes Repository Synchronization from Repository Evolution. |
| **Status** | Active |
| **Applies to** | Both repository lineages — the Engineering Specification track and the Product Track (Product Definition / Product Design / Interaction Design) — equally. |
| **Last Updated** | 2026-07-28 |

---

# Purpose

Recent Product Design and Interaction Design sessions established a workflow by practice before it existed as policy: artifacts were drafted in conversation, approved by the Product Owner, and then synchronized into the repository — sometimes promptly, sometimes only once a dedicated synchronization session caught the repository up. This document formalizes that workflow as a standing policy, and introduces a classification distinguishing routine synchronization from changes to the repository's own structure.

This document does not modify `docs/engineering/GOVERNANCE.md`'s Governance Register, Crate Boundary Rules, or Change Categories, and does not modify `DocumentationRelease.md`'s Documentation Release lifecycle. It governs a distinct concern — how content already approved elsewhere enters the repository — that neither of those documents currently addresses, and that applies across both lineages rather than to one.

---

# Repository Synchronization Policy

1. **Conversation is a drafting environment.** Product Design, Interaction Design, proposals, and other draft artifacts may be developed, reviewed, and revised in conversation before any repository change is made. A draft carries no repository status while it remains only in conversation.

2. **Product Owner approval establishes acceptance.** An artifact becomes approved when the Product Owner accepts it — "approved in principle," a Chief Architect approval statement recorded and accepted, or an equivalent explicit acceptance. Approval is a decision about the artifact's content; it is separate from, and prior to, any decision about repository placement.

3. **Approved artifacts are synchronized into the repository working tree.** Once approved, an artifact should be written into the appropriate repository location promptly — in the same session where practical — rather than left to accumulate only in conversation history pending a later catch-up synchronization.

4. **The repository is the project's canonical source of truth.** An approved artifact that exists only in conversation history is not yet part of the project's record. Repository state, not conversation history, is what a new contributor, a future session, or a future Product Owner review can rely on.

5. **Synchronization does not imply commit or push.** Writing an approved artifact into the working tree is a distinct act from committing or pushing it. Commit and push remain separately authorized actions, requiring explicit instruction each time, regardless of how this policy affects working-tree preparation.

6. **Synchronization is a transcription activity, not a design activity.** Placing an already-approved artifact into the repository — including the minimum documentation-index and cross-reference updates needed to make it discoverable — does not itself require new Product Owner approval, because no new decision is being made: the content was already approved, and only its location is being resolved. This is what distinguishes Synchronization from Evolution, below.

---

# Repository Change Classification

Two categories of repository work are distinguished by the same test: **does the change place already-approved content where it belongs, or does it change the repository itself?**

## Repository Synchronization

Preserves alignment between already-approved work and the repository. Requires no additional Product Owner approval beyond whatever already approved the underlying artifact — it may be prepared directly in the working tree.

Examples:

- Adding approved artifacts
- Updating indexes
- Updating README files
- Updating `PROJECT_STATUS.md`
- Updating `CHANGELOG.md`
- Updating documentation navigation
- Repairing documentation links

## Repository Evolution

Changes the repository itself — its structure, conventions, or governance mechanisms — rather than reflecting a decision already made elsewhere. Requires explicit Product Owner approval **before** implementation, not merely before commit.

Examples:

- New documentation structures
- New directory hierarchies
- Documentation architecture changes
- Naming convention changes
- Major repository organization
- New governance mechanisms

**The distinguishing consequence:** Synchronization may proceed and be reported afterward, since it carries out a decision already made. Evolution requires the decision itself to be sought first — proposing the change and naming it as Evolution, rather than presenting it as an accomplished fact alongside routine synchronization work.

---

# Relationship to Existing Governance

This classification is a different axis from `GOVERNANCE.md`'s existing Level 1–4 Change Categories, not a replacement for it. Level 1–4 classifies changes to specification and architectural *content* (wording, invariants, APIs, aggregate design); Repository Synchronization and Repository Evolution classify changes to the *repository* itself (placement, structure, navigation). The two can apply to the same piece of work independently — for example, a Level 4 Architectural change to `Architecture.md`, once approved through its own governance process, still needs to be synchronized into the repository, which is a Repository Synchronization activity in the sense of this document.

This policy also does not extend `DocumentationRelease.md`'s Documentation Release lifecycle to the Product Track. Product Design and Interaction Design artifacts continue to carry no Documentation Authority ranking and are not subject to Documentation Release governance; this policy governs only how they — like any other approved artifact — enter the repository once accepted.

---

# Retroactive Note

Applying this classification honestly to the two sessions that immediately preceded it: creating `docs/product-design/` and `docs/interaction-design/` (new directories) and `docs/DOCUMENTATION_MAP.md` (a new documentation-architecture artifact) would, under today's classification, register as Repository Evolution rather than Repository Synchronization. Both were carried out under an explicit, contemporaneous grant — "Repository Modification Responsibilities" authorized creating documentation files and folders directly — so they satisfy this policy's own requirement (approval before implementation) rather than violating it retroactively. Going forward, that kind of authorization should be sought as its own explicit step when a session's work turns out to include Repository Evolution, rather than folded into a broader synchronization grant as it was in that earlier session.

---

# Future Evolution

This policy is expected to be stable but not frozen — it describes a workflow convention, not architectural intent. Changes to it should themselves be treated as Repository Evolution: proposed and approved before being written here.
