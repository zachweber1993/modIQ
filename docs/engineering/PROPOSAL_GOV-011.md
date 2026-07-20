# Proposal: GOV-011 Resolution — Archive Collection Model

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_GOV-011.md |
| **Stage** | Proposal (Architecture Review) — revised for final resolution |
| **Prepared by** | Engineering, for Technical Director review |
| **Governance item addressed** | GOV-011 (`docs/engineering/GOVERNANCE.md`) |
| **Sprint context** | Sprint 4 Phase 1 (original draft) and Phase 2 (Boundary-Proving) findings, per `SPRINT4_IMPLEMENTATION_PLAN.md` |
| **Revision note** | This revision supersedes the original draft's Question 2 candidate, which Phase 2 found technically unachievable against the investigated dependency, and incorporates the Technical Director's decisions on Questions 2 and 4. It does not alter Questions 1 or 3. |
| **Status** | Presented for final policy-level resolution of GOV-011. Implementation details for Questions 2 and 4 remain undetermined and are explicitly not decided by this document — see Future Implementation Scope. No Rust code, no `Cargo.toml` change, no archive collection has been implemented as of this document. |

---

## Governance Decisions vs. Implementation Work

This proposal is deliberately organized to keep these separate, per instruction:

- **A governance decision** states *what the platform's policy is* — what counts as a valid or invalid input, what the platform does or does not treat as acceptable, at the level `EvidenceCollection.md`'s Collector Contract already operates. Questions 1 and 4 are governance decisions. Question 2's *direction* (do not silently accept, do not fabricate) is a governance decision. Question 3's *mechanism* (bound by metadata-derived count and ratio) is a governance decision.
- **Implementation work** states *how* a policy is realized in Rust — which type represents a fact, which method detects a condition, what a threshold's exact numeric value is. None of that is decided here. Every section below that touches implementation is explicitly labeled as such, and none of it is authorized by this document.

---

## Question 1 — Malformed or corrupt archive: Unsupported Input, or a new outcome?

**Policy (Recommended by Engineering, confirmed achievable by Phase 2, not yet re-affirmed by the Technical Director this round):** an instance of the existing Unsupported Input outcome. No new outcome.

**Evidence incorporated:** Phase 2 confirmed both a truncated archive and a non-archive file bearing a `.zip`-shaped name fail cleanly via a `Result::Err` at archive-open time, with no panic in either case — directly compatible with the existing Unsupported Input category's architectural meaning ("reachable but not a supported kind"), unchanged from the original draft's reasoning.

## Question 2 — Duplicate entry names within a single archive

**Policy — Approved by Technical Director: Option (b).** The platform will not silently accept last-write-wins behavior, and will not fabricate Evidence items for entries the investigated dependency cannot expose. If duplicate central-directory records are detected, their existence shall become an observable fact recorded by the archive collection process.

**This supersedes the original draft's candidate**, which proposed that each duplicately-named entry produce its own discrete Evidence item. Phase 2 found this technically unachievable against the investigated dependency (see Evidence Incorporated, below) — the earlier of two duplicately-named entries is not observable at all through that dependency's ordinary enumeration API, so "each entry produces its own Evidence item" cannot be honestly implemented against it.

**Implementation is not decided here.** "Their existence shall become an observable fact" states the policy; it does not select a Rust type, a detection mechanism, or a representation (a new Evidence category, a new Collection outcome, a count field, or something else). See Future Implementation Scope.

## Question 3 — Resource limits

**Policy (Recommended by Engineering, confirmed achievable by Phase 2, not yet re-affirmed by the Technical Director this round):** bound both entry count and the ratio between an entry's claimed uncompressed size and its compressed size, checked from archive metadata alone, before any content is decompressed. An archive exceeding either bound is categorized as Unsupported Input (Question 1's category), not a new outcome.

**Evidence incorporated:** Phase 2 confirmed both quantities are recoverable from metadata alone — verified empirically, not assumed, by timing metadata-only enumeration against full decompression of the same archive (metadata enumeration was measured at approximately 1,270 times faster). This directly validates the mechanism this policy depends on. **Exact numeric thresholds remain undecided** — Phase 2 investigated the mechanism, not calibrated production values; this document does not propose specific numbers.

## Question 4 — Archive traversal boundary (the Zip Slip case), including absolute paths

**Policy — Approved by Technical Director, in two parts:**

1. **General traversal policy (approved in the prior review round):** the collector SHALL normalize archive entry paths. An entry containing a path-traversal sequence SHALL be treated as an invalid archive entry and skipped — not followed, not recorded as Evidence. Collection continues for all remaining valid entries. An archive is not rejected in its entirety solely because one or more entries are invalid in this way; only an archive that cannot be read or parsed at all terminates collection.
2. **Absolute paths (approved this round, extending the above):** an archive entry that was originally an absolute path is treated as a path-traversal violation under the same policy — skipped, not recorded, collection continues. **This determination is independent of any dependency's internal sanitization.** The collector shall not rely on a dependency's sanitized path representation as evidence that such an entry is acceptable.

**Evidence incorporated, and its direct implementation consequence:** Phase 2 found that the investigated dependency's own safety accessor (`enclosed_name()`) correctly rejects (returns no usable value for) genuine `..`-based traversal entries, but *sanitizes* an absolute-path entry into an accepted, safe-looking relative path rather than rejecting it. Under the policy now approved, that sanitized result cannot be trusted as sufficient — an eventual implementation will need its own, independent check for "was this entry's raw, as-stored name an absolute path" (evaluated before or instead of relying on the dependency's own safety judgment), since the dependency's sanitized output does not preserve the fact that the original entry was absolute. This is stated here as a fact this policy implies, not as an authorized implementation detail — see Future Implementation Scope.

---

## Evidence Incorporated (Phase 2 Boundary-Proving, full summary)

Investigated against the `zip` crate (v8.6.0), entirely in a standalone project outside this repository — no production Rust, no crate modification, no `Cargo.toml` change. Nine constructed fixtures; ten empirical checks.

- **Deterministic entry enumeration** — confirmed identical order across repeated reads; order matches central-directory/physical order, not auto-sorted (an explicit sort remains necessary, as already planned for the archive collector generally).
- **Malformed archive reporting** — confirmed clean `Result::Err` at open time for both a truncated archive and non-archive content; no panic (Question 1).
- **Metadata availability without extraction** — confirmed empirically via timing comparison, not assumed (Question 3).
- **Resource-limit relevant quantities** — confirmed available and cheaply computable: an entry count of 5,000 and a compression ratio of approximately 1029:1 were both read from metadata alone (Question 3).
- **Duplicate entry names** — confirmed, and independently cross-verified via `unzip -l`, Python's `zipfile`, and a raw central-directory-record byte count, that a fixture containing two identically-named entries is only partially observable through the investigated dependency: `archive.len()` reported 1, not 2, and the earlier entry is not accessible through the dependency's ordinary enumeration API at all (Question 2 — the finding that changed this proposal's candidate).
- **Path traversal (`enclosed_name()`)** — confirmed correct rejection of `..`-based traversal entries; confirmed sanitizing (not rejecting) behavior for absolute-path entries (Question 4).
- **Archive Metadata Policy fields** — confirmed `last_modified()`, `unix_mode()`, and `comment()` exist as accessor methods distinct from `name()`/`size()`/`is_dir()`, confirming the previously approved Archive Metadata Policy is straightforward to honor by simply never calling them.

Full detail: `docs/engineering/ENGINEERING_LOG.md`, Sprint 4 Phase 2 entry.

---

## Required Documentation Updates (following approval — not performed by this proposal)

- `docs/engineering/GOVERNANCE.md` — GOV-011's Resolution field updated to record final policy text for all four questions, superseding the "Partially resolved" state currently recorded; Status changed to Resolved if the Technical Director confirms Questions 1 and 3 alongside this round's Questions 2 and 4.
- `docs/architecture/EvidenceCollection.md` — amendment recording the archive-specific Collection Outcomes (Question 1, Question 3's bound-exceeded case), the duplicate-entry observable-fact policy (Question 2), and the extended Traversal Boundary Policy covering both relative traversal and absolute paths (Question 4) — mirroring how the Symbolic Link Policy and Collection Atomicity were recorded following the filesystem case's own resolution.
- `docs/implementation/CrateRoadmap.md` — a revision history entry noting GOV-011's resolution, once recorded.
- `docs/engineering/ENGINEERING_LOG.md` — an entry recording the resolution, following the established Status/Affected Crates/Affected Documents/Notes structure.

---

## Future Implementation Scope (Phase 3 and later — not authorized by this proposal)

Named for planning visibility only; no Rust type, method, or dependency change is authorized here:

- **Question 2's representation mechanism.** How "duplicate central-directory records were detected" becomes an observable fact is undetermined: candidates include a new Evidence item distinct from per-entry Evidence, a new Collection outcome, or a count/flag surfaced some other way. This is real design work for Phase 3, informed by whatever `EvidenceCollection.md`'s amendment (above) settles on conceptually.
- **Question 2's detection mechanism.** Phase 2 observed the discrepancy by independently counting raw central-directory records outside the investigated dependency's own API (`unzip -l`, byte-signature counting) — whether the eventual collector performs an analogous independent count, uses a lower-level parsing facility, or takes another approach is undetermined.
- **Question 4's independent absolute-path check.** As stated above, the eventual collector cannot rely solely on the investigated dependency's `enclosed_name()` result — it needs its own check against each entry's raw, as-stored name. The exact form of that check (a leading-slash test, a platform-aware drive-letter test, a shared normalization routine also used for relative traversal detection, or something else) is undetermined.
- **Question 3's numeric thresholds.** Entry-count and compression-ratio limits remain provisional; production values require calibration against real fixtures during implementation, not assertion in a governance document.
- **Dependency selection itself.** This proposal's evidence was gathered against one candidate dependency (`zip` v8.6.0). Whether it remains the selected dependency, notwithstanding the Question 2 limitation now designed around rather than avoided by switching dependencies, is a decision `SPRINT4_IMPLEMENTATION_PLAN.md`'s own Phase 2 dependency-selection criteria still govern — this proposal does not re-open or re-affirm that choice.

---

## Remaining Open Questions Requiring Technical Director Review

- Should Questions 1 and 3 be formally confirmed alongside this round's decisions, so that GOV-011 can be marked fully Resolved at the policy level in one action — or held open pending a separate, explicit confirmation round?
- Is GOV-011's Resolved status intended to require Question 2's and Question 4's implementation-mechanism questions (Future Implementation Scope, above) to also be settled first, or — consistent with how GOV-010's four-outcome model was resolved at a conceptual level while its own representation mechanism was left to implementation — is policy-level resolution sufficient to unblock Phase 3, with the remaining mechanism questions worked out during implementation itself?
- No other open question was identified by Phase 2 beyond those already named under Questions 2 and 4.

---

## What This Proposal Does Not Do

No Rust file was modified. No `Cargo.toml` or `Cargo.lock` was modified. No archive collection was implemented. `AssessmentService` was not touched. No routing or dispatch abstraction was introduced or implied. `docs/engineering/GOVERNANCE.md` is not modified by this document — the documentation updates named above remain to be performed following Technical Director review of this proposal, per this project's established proposal-then-record discipline.
