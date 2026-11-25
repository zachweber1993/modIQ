# Engineering Release 2.7

| Property | Value |
|---|---|
| **Release** | 2.7 |
| **Documentation Release** | 2.1 (Frozen — unchanged this workstream; no specification amendment) |
| **Milestone** | Runtime Load Failure Recognition Correction — Candidate A complete. The existing Runtime Load Failure recognition contract now recognizes the governed engine message `Error: Unsupported mod description version in mod {Name}` in both the untimestamped and timestamped log-line forms the real fixture corpus carries, with occurrence-level Evidence preserved in the Collector and per-mod deduplication applied in the Rule |
| **Scope** | The five items the Product Owner's decision record (D5) authorized: timestamp-tolerant recognition of the exact governed message; occurrence-level Evidence preservation in the Collector; per-mod Finding deduplication in the Rule; tests proving the approved behavior; real-fixture integration coverage — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_2.6.md` (Store Item Reference Integrity) |
| **Governing ADRs** | None new — no Architectural Resolution was required (D4) |
| **Governing Documents** | `docs/engineering/RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` (Section 2.2, Section 5, Matrix row 4 — the recognition and Collector/Rule interpretation boundaries this correction operates inside); `docs/implementation/SPRINT11.md` (Risks — Single-sample design risk, the generalization discipline this correction follows). The Product Owner's own decision record (D1–D7, delivered directly to this session) is the sole lifecycle artifact specific to this workstream — see §9 |
| **Capability Number** | None. This is a correction to an existing recognition contract, not a new capability, and carries no `C`-number |

---

## 1. Executive Summary

Runtime Load Failure recognition, introduced by Sprint 11 and `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, matched the governed engine message `Error: Unsupported mod description version in mod {Name}` only when that message began at offset zero of its log line. The real fixture corpus (`fixtures/runtime-logs/single-incompatible-mod/log.txt`) carries the identical engine message twice: untimestamped at line 124, during the log's startup phase, and timestamped at line 1445, during its session phase — `2026-07-22 17:12:01.150 Error: Unsupported mod description version in mod FS25_DodgeChallengerHellcat`. Only the first occurrence was ever recognized; the second, carrying the same governed message after a 23-character timestamp prefix, was not.

Candidate A corrects this. The approved interpretation (D1) reads "engine line" as the engine message content itself, not its physical position at the start of the file line — the timestamp is a transport artifact the logging mechanism prepends, not part of the engine's own message. Recognition now checks for exactly one timestamp shape (`YYYY-MM-DD HH:MM:SS.mmm`, 23 characters, positionally and exactly matched — never parsed as a calendar date, never generalized to any other shape) and, when present, sets it aside before matching the unchanged governed template against what remains. An untimestamped line is matched exactly as before.

This surfaced a second question the Collector/Rule boundary already answers: a log recording the same engine message twice is either one rejection reported twice or two independent rejections of the same mod, and the fixture cannot distinguish the two readings. Per D2, the Collector takes no position — it preserves one Evidence item per observed occurrence, deduplicating nothing, consistent with `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` Section 5's restated invariant that Collectors acquire facts and do not infer meaning. `RuntimeLoadFailureRule` takes the interpretive position: it names each affected mod exactly once in its Finding summary, citing every contributing Evidence id, because both readings converge on the identical, defensible conclusion — this mod was rejected at modDesc validation — and reporting a count would assert what the Evidence cannot establish.

This is a correction to existing recognition behavior, not a new runtime signal, a new Evidence category, or a general-purpose timestamp or error-line parser (D1, D4).

---

## 2. Lifecycle Status and Authorization

Every stage below was completed and approved by the Product Owner, through decisions delivered directly to this session rather than as separate persisted lifecycle documents — see §9.

| Stage | Status |
|---|---|
| Process Determination | Completed — classified as a correction to the existing recognition contract, not requiring a new Architectural Resolution (D4) |
| PO Decisions | Completed — D1 (timestamp recognition), D2 (occurrence Evidence and Finding deduplication), D3 (`Load mod:` remains corroborating-only, unmodified), D4 (architectural classification), D5 (scope and exclusions), D6 (test requirements) |
| Implementation Authorization | Issued, carrying D1–D6 |
| Implementation | Completed, across the Collector, the Rule, and engine-level integration tests |
| PO Review | Completed — implementation and the two authorized test-only corrections accepted as-is (D7) |
| Engineering Release 2.7 | **Authorized by the Product Owner for the documentation stage; this document** |

No Architectural Resolution was adopted and none was required (D4). The approved D1 interpretation is recorded in §4 below and is reflected in the implementation's own doc comments, per D4's instruction.

---

## 3. Scope Delivered

Exactly the five items D5 authorized, and no more:

**Collection.**
- `RuntimeLogCollector` recognizes the governed message against the **engine line** — the line's own content with the logging mechanism's timestamp prefix, where present, set aside. The template itself (`FAILURE_LINE_PREFIX`) is unchanged.
- Exactly one timestamp shape is recognized: `YYYY-MM-DD HH:MM:SS.mmm`, 23 characters, checked positionally and exactly (digit, dash, colon, space, and dot positions individually verified), followed by at least one space. No calendar validity, timezone, alternative separator, or optional-millisecond form is accepted. This is not general timestamp parsing.
- An untimestamped line is matched exactly as before this correction — the timestamp check is additive, not a replacement path.
- Evidence `content` always carries the complete original line verbatim, including the timestamp where one was present — the only datum distinguishing two otherwise byte-identical occurrences.
- One Evidence item per recognized occurrence. The Collector performs no deduplication; each occurrence receives its own Evidence id (D2).

**Evaluation.**
- `RuntimeLoadFailureRule` deduplicates by mod name when building its Finding summary: a mod name is named once regardless of how many Evidence items recognize it, in first-observed order, using a single linear pass (no hash-based collection), so repeated evaluation of identical input is byte-identical.
- Every contributing Evidence id is retained and cited by the Finding, regardless of deduplication in the summary text.
- Distinct mod names remain distinct — deduplication is per mod name, not a collapse of the Finding as a whole.

**Unmodified in this release (D3).**
- `Load mod:` remains corroborating evidence only. It is not required for recognition and was not read, matched, or otherwise touched by this correction.
- The Matrix wording discrepancy `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` carries around this signal remains deferred housekeeping, outside this release's scope.

---

## 4. Approved Decisions

As recorded in the Product Owner's decision record (D1–D7), delivered directly to this session. Nothing below was decided by this release; it is recorded here for the historical record, consistent with this document series' own convention of recording adopted decisions rather than re-deciding them.

| Decision | Content |
|---|---|
| **D1 — Timestamped recognition** | "Engine line" means the engine message content, not the line's physical offset. The governed message is recognized both at the start of an untimestamped line and after the recognized timestamp prefix of a timestamped line. No broader error parsing or general timestamp parsing is authorized |
| **D2 — Repeated occurrences and deduplication** | Collector: one Evidence item per occurrence, no deduplication, own id per occurrence, raw line preserved verbatim. Rule: one Finding per distinct mod name, the name appears once in the summary, every contributing Evidence id is retained, distinct mod names remain distinct Findings, output remains deterministic. The accepted ambiguity — repeated occurrences may be one logical rejection reported twice or genuinely separate rejections — is not resolved; the system preserves occurrences as Evidence while grouping Findings by mod name |
| **D3 — `Load mod:` corroborating signal** | Corroborating only, not required for recognition, not modified in this release. The Matrix wording discrepancy is deferred housekeeping, outside scope |
| **D4 — Architectural classification** | A correction to the existing recognition contract. No new Architectural Resolution is required. The approved interpretation is recorded in the Implementation Authorization and reflected in code comments |
| **D5 — Scope** | Authorized: timestamp-tolerant recognition of the exact governed message; occurrence-level Evidence preservation; per-mod Finding deduplication; tests for the approved behavior; real-fixture integration coverage. Explicitly excluded: new runtime signals; general error or warning recognition; modDesc version extraction; hash-based mod identity; runtime intake architecture changes; Application Support discovery or intake changes; persistence changes; Evidence-content persistence expansion; UI/frontend work; broad parsing heuristics; additional runtime capabilities; architectural expansion; unrelated refactoring; documentation changes outside a separately authorized stage |
| **D6 — Test requirements** | Timestamped recognition; untimestamped recognition; both forms recognized; full raw line retained in Evidence content; malformed timestamp shape rejected; other timestamped messages rejected; Collector non-deduplication; Rule one-name-per-summary; every contributing Evidence id retained; distinct mod names remain distinct; deterministic behavior; real-fixture Collector and end-to-end integration tests |
| **D7 — Implementation acceptance** | Implementation accepted as-is. Two authorized test-only corrections accepted: a strengthened duplicate-occurrence test, and an added malformed-timestamp-shape regression test. Verification confirmed: formatting clean, workspace check clean, 370/370 workspace tests, clean diff, clippy with one unrelated pre-existing warning |

**No ADR was created. No Governance Register entry was created.**

---

## 5. Implementation Summary

| File | Nature of change |
|---|---|
| `crates/modiq-collection/src/collection/runtime_log_collector.rs` | **Modified.** Adds `LOG_TIMESTAMP_WIDTH`, `engine_line`, and `has_log_timestamp`; `recognized_failure_evidence` now matches against the engine line rather than the raw line. The governed template (`FAILURE_LINE_PREFIX`) and Evidence `content` behavior (complete original line, verbatim) are unchanged. Plus 9 tests |
| `crates/modiq-rules/src/rules/runtime_load_failure_rule.rs` | **Modified.** `evaluate` now collects distinct mod names via a single linear pass (`Vec`, first-observed order) rather than mapping every occurrence into the summary; `evidence_ids` is unchanged — every contributing id is still collected and cited. Plus 5 tests |
| `crates/modiq-engine/src/engine/assessment_service.rs` | **Modified — `#[cfg(test)]` only.** 2 integration tests against the real `single-incompatible-mod` and `clean-base-game` fixtures. No production code or signature change |

Diff shape: **586 insertions, 2 deletions** across the three files the Implementation Authorization named, no file outside that set. Both deletions are the two single-line replacements that switch matching from the raw line to `engine_line` and from the collecting `Vec` map to the deduplicating loop — no test assertion was deleted, and no existing test was weakened, altered, or removed.

**Test coverage added: 16 tests** (9 Collector + 5 Rule + 2 engine-integration), verified directly against the diff rather than assumed:

| Crate | Before | After |
|---|---|---|
| `modiq_collection` | 80 | 89 (+9) |
| `modiq_rules` | 71 | 76 (+5) |
| `modiq_engine` | 31 | 33 (+2) |
| **Root workspace total** | **354** | **370 (+16)** |

The "before" figures are Engineering Release 2.6's own closing counts, which this workstream's baseline HEAD (`e822cb5`) carries unchanged; the "after" figures are this session's own directly-run `cargo test --workspace` result (§7).

Coverage spans: timestamped recognition; untimestamped recognition (regression); both forms recognized in one file; verbatim content preservation including the timestamp; a malformed timestamp shape (single-digit month) correctly falling back to non-recognition; a timestamped line carrying unrelated message text correctly remaining unrecognized; non-deduplication of repeated occurrences at the Collector; one-name-per-summary deduplication at the Rule with every contributing Evidence id retained; distinct mod names remaining distinct; deterministic output under repeated evaluation of identical input; and both occurrences of the real, checked-in `single-incompatible-mod` fixture, end-to-end, against a real `AssessmentService` execution.

---

## 6. Repository Impact

| Area | Impact |
|---|---|
| `crates/modiq-collection` | `runtime_log_collector.rs` only. No other Collector touched |
| `crates/modiq-rules` | `runtime_load_failure_rule.rs` only. No other Rule touched; dispatch order in `engine.rs` unchanged |
| `crates/modiq-runtime` | **Unchanged.** No new `EvidenceCategory`, no entity field, no invariant change |
| `crates/modiq-engine` | Tests only. `AssessmentService`'s public entry points unchanged |
| `crates/modiq-storage`, `modiq-knowledge`, `modiq-versioning`, `modiq-cli` | **Unchanged** |
| `apps/console`, `apps/sandbox` | **Unchanged** |
| DTO / transport / UI / Dashboard | **Unchanged** |
| Cargo manifests, dependencies, schemas, configuration | **Unchanged** |
| `GOVERNANCE.md`, ADRs, architecture documents | **Unchanged** |
| `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md` | **Intentionally not synchronized by this release** — deferred to a separate Repository Closeout, following the identical choice Engineering Releases 2.4, 2.5, and 2.6 each made |

**One behavioral consequence, recorded:** any already-persisted assessment of a mod whose log carries the timestamped session-phase occurrence, reassessed after this correction, will now also surface the timestamped occurrence as Evidence (previously silently unrecognized), though the Rule's deduplicated Finding summary is unchanged in wording for a mod already recognized via its untimestamped occurrence. This is a consequence of the correction working as approved, not a contract or schema change.

---

## 7. Validation Status

Re-run fresh at this release-review time, as D7 recorded and this session directly confirmed:

```
Root workspace:
cargo fmt --all -- --check            → clean
cargo check --workspace --all-targets → clean
cargo test --workspace                → clean, 0 failures, 370 total passing
  modiq_collection 89 · modiq_rules 76 · modiq_engine 33 · (remaining crates unchanged)
cargo clippy --workspace --all-targets → one pre-existing, unrelated module_inception
                                          warning in modiq-runtime; no new warning in
                                          any of the three modified crates
git diff --check                      → clean
```

Baseline preserved: the pre-correction root suite stood at 354 passing (Engineering Release 2.6's own closing count); it now stands at 370, the entire delta accounted for by the 16 tests this correction added.

---

## 8. Known Limitations and Accepted Ambiguity

1. **Repeated-occurrence interpretation is accepted, not resolved (D2).** Whether two occurrences of the identical engine message represent one rejection logged twice or two separate rejections of the same mod cannot be determined from the fixture corpus available to this platform. The Rule's one-Finding-per-mod-name behavior produces the same, defensible conclusion under either reading; it does not claim to have resolved which reading is correct.
2. **Exactly one timestamp shape is recognized.** `YYYY-MM-DD HH:MM:SS.mmm` only, matched positionally and exactly. No other log-transport timestamp shape (a different separator, a missing millisecond component, a timezone offset) is recognized by this correction, and none is claimed to be.
3. **`Load mod:` remains unexamined (D3).** This correction neither strengthens nor weakens the existing Matrix wording discrepancy around that signal; it remains deferred housekeeping.
4. **No claim of completeness over failure classes.** As `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`'s own Matrix (row 4) and `SPRINT11.md`'s Single-sample design risk already state, recognition remains bounded to the one governed template this corpus evidences — this correction widens where that template is recognized within a line, not what is recognized.

---

## 9. Documentation Note

By the nature of this workstream, the Product Owner's decisions (D1–D7) were delivered directly to this session as a decision record rather than as separate persisted lifecycle documents (a Capability Definition, Architecture Evaluation, or Implementation Plan/Report of the kind C1–C15 and Store Item Reference Integrity each produced). This is consistent with D4's classification of this workstream as a correction rather than a new capability, and no retrospective reconstruction of lifecycle documents in that fuller shape is performed by this release.

This Engineering Release document is written to carry the load-bearing content of that decision record in full — §2 records every lifecycle stage and its outcome, §3 records the delivered scope, §4 records every decision verbatim in substance, §5 records the implementation and its verified coverage, and §7 records verification independently confirmed by this session, not merely asserted.

---

## 10. Final Release Recommendation

**Release.** The correction is implemented within its authorized scope (D5), consistent with every approved decision (D1–D7), verified against all required gates with the pre-correction baseline fully preserved (354 → 370, entirely accounted for by added tests), and grounded directly against the real fixture corpus this session independently confirmed. No release blocker exists.

The implementation remains, as of this document, an **uncommitted working-tree state** — not staged, not committed, not pushed, not tagged, not published. This release document records the implementation's acceptance; it does not change the repository's committed history.

**Recommend:** Product Owner review of this release document, followed by separate, explicit authorization for Repository Closeout and for each Git operation it would require.

---

## Repository Status

The Runtime Load Failure Recognition Correction — Candidate A is complete and accepted. The root workspace is clean at 370/370. No Governance Register item and no ADR resulted from this workstream. HEAD remains `e822cb5a7fc4bb4968e671c4f8fabc7d55f9cd21` on `feature/runtime-implementation`, confirmed directly against `origin/feature/runtime-implementation` (0 ahead / 0 behind) at the time this document was drafted. The correction across the three implementation/test files, and this release document itself, exist only as uncommitted working-tree state. `PROJECT_STATUS.md`, `CHANGELOG.md`, and `ENGINEERING_LOG.md` are **intentionally not synchronized by this release**, consistent with the identical choice Engineering Releases 2.4, 2.5, and 2.6 each made in deferring those three files to a separate Repository Closeout. That separate step remains this release's own explicit exclusion, not performed here.

---

## Repository Timeline

```
Engineering Release 2.6 — Store Item Reference Integrity complete
        ↓
Runtime Load Failure Recognition Correction — Candidate A identified: the governed
message was recognized only at offset zero of a log line, missing the timestamped
session-phase occurrence the real fixture corpus also carries
        ↓
Product Owner Decisions D1–D7 — timestamp-tolerant recognition over the engine
message content; occurrence-level Evidence with per-mod Finding deduplication;
Load mod: left unmodified; classified as a correction, no Architectural Resolution
required; scope and exclusions fixed; test requirements fixed
        ↓
Implementation — Collector, Rule, and engine-integration tests; 354 → 370 tests
        ↓
PO Review — implementation and two test-only corrections accepted as-is (D7)
        ↓
Engineering Release 2.7 — this document
        ↓
Repository Closeout — pending, not authorized by this release
```
