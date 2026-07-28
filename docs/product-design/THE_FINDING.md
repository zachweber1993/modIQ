# The Finding — Version 1 Product Design

| Property | Value |
|---|---|
| **Initiative** | Workspace Evolution — Finding Experience |
| **Phase** | Product Design — Session 3 |
| **Status** | Approved in principle |
| **Builds on** | Workspace Evolution — Version 1 Product Design Decisions |
| **Scope** | The Finding as a product object: purpose, anatomy, information hierarchy, progressive disclosure, multi-Finding presentation, and its relationship to Recommendations and Evidence |
| **Out of scope** | Product philosophy, Assessment Framework internals, engine implementation, data model, visual UI |

Every decision below is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose of a Finding

A Finding exists to convert raw Evidence into a single, understandable unit of meaning. Evidence by itself — an XML fragment, a Lua reference, a log line — is not something a user can act on; it has to be interpreted first. A Finding is that interpretation: the smallest thing a user needs to read to understand one specific aspect of the Assessment Subject.

**Every Finding answers one question:** *what is true about this one aspect of the mod, and how much does it matter?* This is narrower than the question the Assessment Overview answers ("does the mod work, overall") — a Finding answers it for exactly one aspect, at exactly one severity.

**How a Finding differs from Evidence:** Evidence is raw and objective — what was observed. A Finding is interpreted and meaningful — what it means. A user reads Evidence to verify a Finding; a user does not discover a Finding by reading Evidence directly. This is why Evidence is only ever reached through a Finding, never browsed on its own (Workspace Evolution §5).

**How a Finding differs from a Recommendation:** A Finding states what is true. A Recommendation states what to do about it. A Finding can stand alone with no Recommendation at all — the Glossary itself lists "Informational observations" as a Finding type, and not every true observation implies an action. A Recommendation, by contrast, cannot exist without a Finding to justify it — this is the same constraint already established in Workspace Evolution: Recommendations attach only to Final Findings.

---

## 2. Anatomy of a Finding

**Always present**, regardless of what produced the Finding or what workspace state it was produced in:

- **Title** — a short, plain-language statement of what was found. Never a Rule name or internal identifier — a user should never need to know what produced a Finding to understand it.
- **Summary** — one or two sentences explaining why the Title matters in context.
- **Category** — the Mod Health dimension(s) it belongs to (Compatibility, Stability, Maintainability, Performance, Structure, Engineering Quality), directly matching the dimensions already shown in the Assessment Overview.
- **Severity** — Error / Warning / Informational / Best Practice, per the Glossary's Finding types.
- **Provisional/Final status** — always visible, per Workspace Evolution §2.
- **Evidence availability** — an indication that Evidence exists and can be opened. Never the Evidence content itself at this level.
- **Updated indication** — present only when applicable, per Workspace Evolution §3.

**Present only when applicable, and only once Final:**

- **Recommendation** (plain or Repair Recipe) — only if one exists. Many Findings, especially Informational and Best Practice ones, may have none.
- **Confidence** — attached to the Recommendation, never to the Finding itself. A Finding is a factual observation; Confidence measures certainty in a conclusion, which is the Recommendation's role, not the observation's. This is consistent with the already-approved decision that Confidence belongs to a Recommendation, not to the Assessment as a whole.

**Never present inside a Finding:**

- Confidence as a bare, Finding-level number.
- Evidence content (only its availability).
- Other Findings' content. A Finding does not reference or summarize a different Finding — each one is a self-contained unit of meaning. Cross-Finding patterns are the Overview's job (aggregating across all of them), not any individual Finding's.
- Rule identifiers or other internal naming. The Title is written for a reader, not for traceability to what produced it.

**Decision:** the same set of fields appears whether a Finding came from the original Assessment or a later incremental update, and whether it is Provisional or Final — only the *values* differ, never the *shape*. This is what makes every Finding "feel like the same product object," which is the explicit goal of this section.

---

## 3. Information Hierarchy

The order a user consumes a single Finding's content is fixed:

**Title → Severity/Category → Summary → Recommendation (if Final and present) → Evidence (on expansion)**

- **Title first** because it is what makes scanning many Findings possible at all (§5) — a user should be able to triage a whole Assessment by titles alone before reading a single sentence of prose.
- **Severity/Category alongside the Title** because they tell the user how much attention this Finding deserves before they invest in reading the Summary.
- **Summary next** because it answers "why does this matter" in the fewest possible words, before anything else competes for the user's attention.
- **Recommendation before Evidence** because once a user understands what's wrong, the next question is naturally "what do I do," not "prove it to me." Requiring Evidence to be read before the Recommendation can be reached would add a verification step to every Finding a user might not even feel the need to verify — this is addressed more fully, and reconciled against an apparent tension, in §8.
- **Evidence last** because it is consulted on demand by a user who specifically wants to verify — it is the deepest and most detailed content, and correctly sits at the end of the reading order and behind an expansion (§4).

---

## 4. Progressive Disclosure

**Always visible, no expansion required:**

- Title, Category, Severity, Provisional/Final status, Updated indication, Summary.
- If Final and a Recommendation exists: a one-line Recommendation headline (e.g., what to do, stated in a single sentence) — not the full detail.

**Requires expansion (a single expansion per Finding, not two):**

- The full Recommendation detail, including the complete Repair Recipe when the Recommendation takes that form, shown as one unit — consistent with the earlier decision that a Repair Recipe is not a further drill-down beneath its Recommendation.
- Evidence, positioned below the Recommendation detail within that same expanded view.

**Why this split:** the always-visible layer is exactly what's needed to triage a whole Assessment without opening anything (§5) and to know, at a glance, whether a given Finding needs action. The expansion layer is exactly what's needed to act with full information or to verify — but neither is forced on a user who only wants the headline. This directly reduces cognitive load across many Findings while never hiding anything a transparent Assessment requires — everything is one click away, nothing is hidden permanently.

---

## 5. Multiple Findings

- **Grouping:** by Category first — the same Mod Health dimensions already used in the Assessment Overview. This gives the user one consistent mental model from Layer 1 down into Layer 2: a dimension that looks unhealthy in the Overview leads directly to the group of Findings that explains why.
- **Ordering within a Category:** by Severity — Errors, then Warnings, then Best Practice, then Informational. The most consequential Findings are always encountered first.
- **No further grouping or sorting axes** — no custom filters, tags, or user-defined views. This is deliberately excluded from V1 scope, consistent with the standing Non-Goal against workspace customization.
- **Sequential or independent?** Both, in different senses. Each Finding is **independent in content** — a user can open any single Finding and understand it completely without having read another one first. But the list as a whole has **one fixed presentation order** (Category, then Severity) that the user scans top to bottom by default — the order is designed, not left to the user to construct.

**Decision:** independence at the Finding level supports "understanding" (nothing requires prior context); a fixed, designed order at the list level supports "acting" (the most important things are encountered first without the user having to organize them themselves).

---

## 6. Relationship to Recommendations

- A Recommendation appears **only inside the Finding it belongs to** — never on a separate list or page. This preserves the single path to a Recommendation established in Workspace Evolution: through its Finding, and only through its Finding.
- It becomes visible **exactly when its Finding becomes Final** — not before. A Provisional Finding shows no Recommendation-related content at all, not even a placeholder such as "recommendation pending," because that would imply every Finding eventually gets one, when many legitimately do not.
- A **Repair Recipe** fits into this experience as the form the Recommendation takes when it is reproducible — it is presented as the Recommendation, written as structured steps, not as a separate section the user must additionally open.
- A user understands that a Recommendation is justified by the Finding they're reading **because of where it physically sits** — inside that Finding's own expanded content, directly beneath the Summary. There is no indirection, no reference number, no separate lookup required to know which Finding a given Recommendation belongs to.

---

## 7. Relationship to Evidence

- A user naturally chooses to inspect Evidence when they want to **verify** a claim before trusting or acting on it — most often for higher-severity Findings, or when the attached Recommendation asks for real effort (a Repair Recipe involving actual changes). Evidence is where a careful user goes to confirm a Finding is warranted before investing that effort.
- Its role in understanding is **confirmatory, not introductory.** A user should already understand what happened and what to do from the Title, Summary, and Recommendation before ever opening Evidence — Evidence answers "how do we know," not "what does this mean."
- It supports trust without overwhelming by staying **collapsed by default, scoped to exactly the one Finding it justifies**, and presented in the same plain terms the Finding already used — it never introduces new vocabulary the user has to separately learn in order to read it.

**Decision:** Evidence exists to justify a Finding, not compete with it — it is always available, never mandatory, and never positioned as something the user must pass through before reaching a Recommendation.

---

## 8. Narrative Flow

Opening any Finding should let a user answer, in order: **What happened? Why does it matter? How do we know? What should I do?**

Mapping this design onto that progression surfaces one real tension worth resolving explicitly, not glossing over: the plain narrative order places "how do we know" (Evidence) before "what should I do" (Recommendation), while §3 and §4 place the Recommendation *before* Evidence in both reading order and disclosure.

These are two different orders, and both are correct at once:

- The **justification order** — how the conclusion is actually built, and the order the Stable Product Principles describe — is Evidence → Finding → Recommendation. This is never violated: a Recommendation is never shown without a Final Finding behind it, and Evidence is never fabricated or skipped, only made optional to inspect.
- The **reading order** a user actually experiences is Title → Summary → Recommendation → Evidence-on-demand, because forcing every user to read Evidence before they're allowed to see the Recommendation would turn verification from a choice into a toll — directly at odds with "curiosity should never create friction."

**Resolution:** the narrative's four questions are still all answered, just not strictly in the sequence they're listed: "What happened" and "what should I do" are answered immediately and are always visible; "why does it matter" is answered by the Summary, also always visible; "how do we know" is answered by Evidence, available the moment a user wants it, positioned last in reading order without being last in justification. A user who wants the full justification chain in the "textbook" order can still read Evidence before acting on the Recommendation — nothing prevents that — the design simply doesn't require it of the user who doesn't need it. With this distinction stated, the Finding experience supports the narrative progression fully.

---

## 9. Consistency Review

- **Against Workspace Evolution:** Provisional Findings still carry no Recommendation (restated, not changed). Evidence is still reached only through expanding a Finding — this session extends that same single expansion to also reveal the Recommendation detail, rather than introducing a second expansion path. The Assessment Overview's per-dimension Mod Health grouping and this session's Category-based Finding grouping (§5) now explicitly mirror each other — a link this session makes precise rather than assumed.
- **Against the Stable Product Principles:** Progressive understanding is reflected in the four-layer disclosure of Title/Summary/Recommendation-headline/full-detail. Curiosity never creates friction — nothing gates the Recommendation behind a required Evidence read. The workspace remaining receptive to additional material is untouched by this session — an updated Finding still passes through Provisional to Final exactly as already established. Recommendations supported by Findings, Findings by Evidence — reinforced, not weakened, by the justification/reading-order distinction in §8.
- **Against Product Definition:** the Finding is confirmed as the atomic unit of the Assessment itself, not a separate feature layered on top of it — consistent with "the Assessment remains the product." Pre-interpreting Evidence into a Finding is also a direct instance of "modIQ performs work on behalf of the user" — the user is never asked to read raw Evidence to figure out what it means themselves.
- **Filter check:** headline-first ordering and single-expansion disclosure serve "understanding"; Category/Severity-ordered grouping across many Findings serves "getting" an assessment (fast triage); Recommendation-before-Evidence-by-default serves "acting" without a mandatory verification toll.
- **One observation, not a resolution:** this design leans on the term "Recommendation" constantly, but unlike Finding, Evidence, and Repair Recipe, **the Glossary has no standalone entry for it** — Repair Recipe is defined in terms of it ("a structured recommendation"), but "Recommendation" itself is never independently defined. This is a precise gap, not a vague one, and is noted for the Product Owner / Chief Architect to decide whether it warrants a future Glossary addition — it is not resolved here, consistent with the instruction not to redefine glossary terminology in this session.

No conflicts remain unresolved. This draft does not modify Vision.md, Principles.md, ProductSpecification.md, Glossary.md, or Architecture.md, and has not been evaluated for architectural feasibility.
