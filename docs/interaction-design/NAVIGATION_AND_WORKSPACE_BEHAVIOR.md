# Navigation & Workspace Behavior — Version 1 Interaction Design

| Property | Value |
|---|---|
| **Initiative** | Interaction Design — Navigation & Workspace Behavior |
| **Phase** | Interaction Design — Session 6 |
| **Status** | Approved |
| **Governed by** | Workspace Evolution, The Assessment Experience, The Assessment Report, The Finding, Evidence, Assessment Intake & Upload, Assessing & Progressive Discovery, Finding Presentation, Evidence Exploration, Assessment Report Experience (frozen constraints) |
| **Scope** | The behavioral semantics of movement throughout the workspace: continuity, orientation, scope, focus, returning, reversibility, stability, trust, and completion |
| **Out of scope** | Product concepts, implementation, runtime behavior, engine internals, data model, visual layout, interface components (menus, sidebars, buttons, tabs) |

This document specifies behavior, not concepts, and not components. Every product term used below is used exactly as already defined and is not reinterpreted here. Every decision is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose

Movement exists so a user can examine different parts and depths of one understanding — not so they can be relocated between separate places. Every prior document in this series already built the workspace this way, one decision at a time: Intake never navigates to a different screen when Assessing begins; a Finding's Evidence appears inside the Finding, not on its own page; the Assessment Report is the Assessment, not a separate destination reached by finishing it. This document's task is to name the single law those separate decisions were already following, not to invent a new one.

**Navigation is not transportation** because nothing in this workspace is ever "elsewhere." Everything reachable from any point is either already visible or one step of focus away, always co-located within the same single environment. There is no boundary between "here" and "there" for movement to cross.

**Movement preserves understanding rather than interrupting it** because nothing about it ever requires a user to re-establish where they are or what they were doing. Interruption would mean starting over; since context is never lost (§3), there is nothing for movement to interrupt.

---

## 2. Workspace Continuity

The workspace behaves as one continuous environment across every state it can be in — Intake, Assessing, Reviewing — and across every scope a user can examine within Reviewing — Overview, Finding, Recommendation, Evidence. This has never been three or four separate rules; it has been one rule, applied consistently at every level already designed: Intake's own action disappears in place rather than navigating away; Evidence unfolds inside a Finding's own expansion rather than opening elsewhere; the Report is recognized as the same Assessment a user may have watched form, not a new arrival.

**Decision:** focus changes; understanding continues. No movement in this workspace, at any level, suggests a user has left the Assessment — because at no level does one exist to leave.

---

## 3. Orientation

Orientation is never a checkpoint a user passes through and leaves behind — it is maintained continuously by a single, repeated pattern: whatever context sits above the thing currently being examined stays visible while it's being examined. A Finding's Title, Category, and Severity remain visible while its Evidence is open. An Assessment's Identity remains visible while a specific Finding is open. Nothing that establishes "where am I" is ever hidden by moving to look at something more specific.

**Decision:** orientation is cumulative, not re-derived. A user never has to reconstruct context they already had — each level of focus adds to what's already visible rather than replacing it.

---

## 4. Scope

Movement changes scope, not identity. Across the entire Assessment → Finding → Recommendation → Evidence progression, there is only ever one object: the Assessment itself, currently Reviewing. A Finding is not a separate thing a user visits — it is the same Assessment, examined at a narrower resolution. This is the same claim The Assessment Report already made about the Report itself ("not a separate artifact... the Assessment, viewed in its Reviewing state"), extended here to every level beneath it: Recommendation and Evidence are further narrowings of the identical object, not new objects reached by leaving the last one.

**Decision:** every movement in this chain narrows or widens how much of one coherent object is currently in view. Nothing a user moves toward has an identity independent of the Assessment it belongs to.

---

## 5. Focus

Attention shifts between overview and detail through expansion and collapse — the same mechanism already established at every individual level (a Finding expanding to reveal Recommendation and Evidence; the Overview pointing toward Findings) — and nothing new is introduced here beyond naming its general shape.

**Decision — one step at a time.** Every shift in focus moves exactly one level of scope, never several at once: Overview to Finding is one step; Finding to Evidence is one further step, nested inside the first. A user is never asked to reorient across a large, unpredictable distance in a single movement — depth is always gained or given up incrementally, which is what makes each step feel intentional rather than disruptive.

---

## 6. Returning

Returning from deeper exploration is not retracing a path, because nothing was ever departed from (§2). Widening focus back out uses the identical mechanism as narrowing it in the first place, simply inverted — there is no separate "return" behavior to design, learn, or trust independently of the forward movement a user already understands.

**Decision:** what a user carries back isn't their own path — it's the understanding they gained while focus was narrower. Because the wider context never disappeared while they were focused in, "returning" is simply that wider context becoming the center of attention again, now read with more behind it.

---

## 7. Reversibility

Movement never feels irreversible, and nothing about it ever requires rebuilding orientation — not because reversibility is a separate guarantee this document adds, but because it follows directly from §3. Orientation is never lost in the first place, so there is nothing to reconstruct when a user moves back out. Reversibility here is a consequence of continuous orientation, not an independent mechanism requiring its own design.

**Decision:** movement preserves confidence because it never puts anything at risk of being lost — the same context that made a movement inward legible is still there, unchanged, the moment a user moves back out.

---

## 8. Stability

Terminology, ordering, relationships, and expectations all remain fixed while a user moves — Category/Severity ordering never reshuffles as new content arrives or as a user moves through it; the Finding → Recommendation → Evidence relationship is never restructured depending on where a user currently is; every term is used exactly as the Glossary defines it, everywhere, regardless of scope.

**Decision:** this stability is what makes "one continuous environment" (§2) true in practice, not just in description. If any of these four things changed depending on where a user currently stood, the workspace would in effect be several different places wearing the same appearance — stability is the actual mechanism underneath continuity, not a separate nicety alongside it.

---

## 9. Trust

Predictable movement builds trust the same way consistent Finding presentation already builds trust at the content level (Finding Presentation, §8) and the same way accumulated consistency builds trust at the whole-Assessment level (Assessment Report Experience, §9) — applied here to *behavior* rather than to content. Because every movement of the same kind behaves identically everywhere it occurs, a user's very first movement in the workspace already teaches them how every later one will behave.

**"I always know where I am"** follows from §3. **"I always know why I'm here"** follows from §4 — because nothing a user is looking at has an identity separate from the Assessment, there is never a question of how something unrelated ended up in front of them. **"I always know how this relates to everything else"** follows from §8 — stable relationships mean a user's understanding of how the parts connect never has to be revised mid-session.

**Decision:** behavioral consistency is itself the trust mechanism here — not a claim the product makes about itself, but the direct, repeated experience of movement never once behaving differently than expected.

---

## 10. Completion

Successful movement is not "I reached another page" — there is no page to have reached. Success is "I understand more than I did a moment ago," and, like every other kind of completion already established in this product, it is never marked by an event. This is now the third time this exact pattern has appeared: Assessing has no "complete" announcement (Assessing & Progressive Discovery, §8), reading a Report has no "finished" announcement (Assessment Report Experience, §10), and now, movement itself has no "arrived" announcement. modIQ does not tell a user they've finished anything — understanding simply accumulates, and the user is the one who notices.

**Decision:** movement increases understanding rather than merely relocating attention, and its completion is recognized the same quiet way every other completion in this product already is — by the user, not by the interface.

---

## 11. Accessibility & Interaction Principles

Everything already established in Sessions 1–5 — keyboard interaction, discoverability, reversibility, interruption tolerance, user control, no forced pacing, stable attention position, independent expansion, severity-bounded presentation, non-disruptive expansion, no compounding exploration cost, time-invariant re-entry, order-trust — continues to apply and is not repeated here. Two principles are genuinely new to movement itself:

- **Single-step locality.** Every movement changes scope by exactly one level at a time (§5) — a user is never required to reorient across more than one step of focus in a single movement.
- **Symmetric reversal.** Moving outward uses the exact same mechanism as moving inward, only inverted (§6) — there is no separate, differently-behaved "return" action anywhere in this workspace to learn on top of the forward one.

---

## 12. Interaction Consistency Review

- **Against Workspace Evolution:** the single-workspace, three-state model and "no route-based navigation" principle are used exactly as defined — this document names the general law they already instantiate, rather than adding a new rule.
- **Against The Assessment Report:** §4's "scope, not identity" claim was checked directly against that document's own position that the Report "is not a separate artifact... it is the Assessment, viewed in its Reviewing state" — confirmed consistent, and extended to the Finding/Recommendation/Evidence chain beneath it rather than contradicting or narrowing that claim.
- **Against Finding Presentation and Evidence Exploration:** the single-expansion mechanism, non-destructive Finding↔Evidence movement, and "no separate exit" treatments are reused exactly, not modified — §6 and §11's symmetric-reversal principle is a direct restatement of what both documents already implied without naming it as a general rule.
- **A dependency was checked and made explicit rather than left implicit:** §7 (Reversibility) does not introduce an independent guarantee — it is derived directly from §3 (Orientation), and this document states that dependency outright instead of presenting the two as separately-justified claims.
- **A recurring pattern was identified, not invented:** §10 recognized that this is the third document (after Assessing & Progressive Discovery and Assessment Report Experience) to independently arrive at "completion is never announced, only recognized by the user" — named explicitly here as a cross-document pattern rather than treated as a new decision specific to movement.
- **No implementation decisions:** no interface component — menu, sidebar, button, tab, breadcrumb, history stack — is named or implied anywhere in this document.
- **No new abstractions:** no navigation history mechanism, no distinct "back" behavior, no lateral cross-Finding movement system was introduced; each was either already covered elsewhere (lateral movement — Finding Presentation, §10) or explicitly declined.

No terminology drifted. No behavioral contradiction remains. No Product Design artifact was changed.

---

## 13. Chief Architect Reflection

**Strengths:** this document is honest about its own nature — a synthesis session, not a fresh design session — and it earns that honesty by finding real, previously-unnamed structure (scope-not-identity in §4; single-step locality in §5; reversibility as a direct consequence of orientation rather than an independent guarantee in §7) rather than padding restatement to look like new design. The recurring "quiet completion" pattern named in §10 is a genuine cross-document finding, not an invented one.

**Remaining ambiguity:** this document treated movement primarily as a change in *depth* — narrowing or widening focus along the Assessment → Finding → Recommendation → Evidence chain. Lateral movement — between sibling Findings at the same depth — was touched only by reference to Finding Presentation's own already-established mental-navigation treatment (§10 there), not re-examined here as its own category. That integration point is named, not resolved, and is not believed to require new design — but it has not been explicitly verified against everything established in this document either.

**Readiness for Interaction Design Closeout:** that so little genuinely new behavior was required to answer "how does movement work" — after five prior sessions already built it correctly, piece by piece, without anyone stating the general law until now — is itself evidence the corpus is internally consistent and ready to be closed out. Session 7 should be able to certify the whole Version 1 Interaction Design program precisely because this session found laws to *name*, not gaps to *fill*.
