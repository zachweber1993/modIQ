# Implementation Report — Historical Assessment Analysis

## Summary

Implemented Historical Assessment Analysis exactly as scoped in `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` §5: a read-only, Storage-domain capability that enumerates every `AssessmentReport` `modiq-storage` currently holds, deterministically aggregates recurring `(rule_reference, severity)` patterns across them, and surfaces the result through a new `modiq-cli history` command. No Knowledge Domain involvement, no participation in `AssessmentService::execute`, no new domain types.

---

## Capability Summary

After this work, modIQ can now show an engineer which Rule/Severity patterns recur across every Assessment Report ever persisted, and how often — visibility that did not exist before, since each stored report was previously reachable only individually, by key.

---

## Architectural Invariants vs. Implementation Assumptions

Stated explicitly, per Chief Architect direction, so neither is later mistaken for the other:

- **Architectural invariants** are binding rules this implementation was required to satisfy, established *before* this work began by prior architecture-level decisions — `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §6.2's Adopted Architectural Constraint and `GOVERNANCE.md`'s Storage Crate Boundary Rule. They are listed, and their satisfaction verified, in **Implementation Constraints** below. Changing any of them requires a future Architecture Evaluation, Architectural Resolution, or ADR — never a routine implementation decision, and never a side effect of a later Sprint.
- **Implementation assumptions** are provisional shape decisions this implementation made where the Capability Definition and Sprint Authorization left a genuine open question (`CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` §13's four Chief Architect Questions) without explicit direction. They are listed in **Assumptions Made** below. None of them is architecturally binding: any may be revisited or changed by ordinary future implementation work — a later Sprint choosing a different `list_keys` shape, or adding a minimum-occurrence threshold, requires no Architecture Evaluation, Governance Register item, or ADR, precisely because none was elevated to that status here.

---

## Files and Crates Modified

**`modiq-storage`**
- `crates/modiq-storage/src/storage/report_store.rs` — added `ReportStore::list_keys`.
- `crates/modiq-storage/src/storage/history_analysis.rs` — new file: `PatternFrequency` and `recurring_patterns`.
- `crates/modiq-storage/src/storage/mod.rs` — registered the new module and re-exports.

**`modiq-cli`**
- `crates/modiq-cli/src/commands/history.rs` — new file: `HistoryCommand`.
- `crates/modiq-cli/src/commands/mod.rs` — registered the new module and re-export.
- `crates/modiq-cli/src/app.rs` — added `history` to `Application::run`'s dispatch match; updated doc comment (four → five commands); added a dispatch-level test.
- `crates/modiq-cli/src/commands/help.rs` — added the `history` usage line; updated the "mentions all commands" test (four → five).

No other crate was touched.

---

## Public API Changes

- `modiq_storage::storage::ReportStore` gains one new method: `list_keys(&self) -> Result<Vec<ReportKey>, ReportStoreError>`.
- `modiq_storage::storage` gains two new public re-exports: `PatternFrequency`, `recurring_patterns`.
- `modiq-cli` gains one new command, `history`, taking no arguments.

`AssessmentService`'s two public entry points (`execute`, `execute_from_assessment_input`), `RuleEngine::evaluate`'s signature, and every existing `modiq-storage`/`modiq-cli` public item are unchanged.

---

## Repository Impact

Crates touched: `modiq-storage`, `modiq-cli`. No new external dependency. Tests: 253 → 264 root workspace (`modiq-storage` 10 → 18; `modiq-cli` 15 → 18); Sandbox unaffected, reverified at 9/9.

---

## Specification References

- `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` (this work's own authorizing Capability Definition).
- `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §6.2 (Adopted Architectural Constraint this implementation satisfies by construction).
- `GOVERNANCE.md`'s Storage Crate Boundary Rule (confirmed, not amended — see Architectural Validation, below).

---

## Invariants Implemented

None. This capability does not touch `Assessment`'s lifecycle or any Runtime Invariant; it operates entirely on already-persisted `PersistedAssessmentReport` snapshots.

---

## Tests Added

**`modiq-storage::storage::report_store`** (+3): `list_keys` against a never-written root is empty; `list_keys` returns every stored key; `list_keys`'s own result is sorted regardless of store order.

**`modiq-storage::storage::history_analysis`** (+5): empty store produces an empty result; repeated identical `(rule_reference, severity)` pairs are counted correctly; distinct pairs (by rule, by severity, or both) are not conflated; the result is sorted correctly even when reports are stored in a non-matching order (the determinism claim's own direct test, per this project's standing discipline — not inferred from a single run); analysis performs no mutation (the original report remains retrievable, unchanged, afterward).

**`modiq-cli::commands::history`** (+2): an empty store reports "No recurring patterns found" (success path); a store with a recurring pattern across multiple reports reports the correct rule reference and occurrence count.

**`modiq-cli::app`** (+1): `Application::run(&["history"])` dispatches successfully — a thin dispatch-level test, mirroring `version_command_succeeds`'s own shape; deep functional coverage lives in `HistoryCommand`'s own module.

**`modiq-cli::commands::help`** (updated, not added): the "mentions all commands" test now asserts five commands, including `history`.

Both successful-path (empty store, recurring pattern, sorted output) and failure-path (a corrupt/unreadable store surfaces `ReportStoreError` through `ExitCode::ExecutionFailure`, exercised indirectly through `ReportStoreError`'s existing, already-tested `Read`/`Deserialize` variants — no new failure mode was introduced) are covered.

---

## Design Decisions

- **Aggregation lives in a new `modiq-storage` module (`history_analysis`), not as a method on `ReportStore` itself.** `ReportStore` remains the low-level read/write primitive (`store`, `retrieve`, `list_keys`); the deterministic aggregation composes over it as a separate, free function, consistent with `GOVERNANCE.md`'s own framing of Storage's responsibility as durability and retrieval, not analysis.
- **A linear scan (`Vec` + `iter_mut().find(...)`), not a `HashMap`, aggregates counts.** `PersistedFindingSeverity` derives neither `Hash` nor `Ord` today; adding either purely to enable a map key was judged an unnecessary, non-evidence-driven change to an existing type, given this capability's own explicitly deferred scale concerns (Capability Definition, Risks). The linear approach needs no change to any existing type.
- **Sorting uses `format!("{:?}", severity)` as a tiebreak**, not a new `Ord` impl on `PersistedFindingSeverity` — for the same reason: avoids modifying an existing type for a capability whose own Capability Definition explicitly disfavors new domain-type changes absent evidence they're required.
- **`list_keys` treats a non-existent root as an empty, valid store**, not an error — consistent with `ReportStore::new` itself never requiring the root to pre-exist (it's created lazily on first `store`).
- **The `history` command takes no arguments** — it always analyzes everything the store holds, matching the Capability Definition's explicit exclusion of any filtering/query parameter.

---

## Assumptions Made

**None of the following is an architectural invariant** (see "Architectural Invariants vs. Implementation Assumptions," above) — each is a provisional implementation-level choice, made because the Capability Definition and Sprint Authorization left it genuinely open, not because any of them was adopted as binding. Any may be changed by a future Sprint's ordinary implementation decision, without requiring an Architecture Evaluation, Governance Register item, or ADR.

- The Capability Definition's four open questions (list_keys shape, CLI vs. library-only surface, minimum-occurrence threshold, and the Design Guidance interpretation) were resolved as follows, absent an explicit override: `list_keys` returns bare `Vec<ReportKey>` (Question 1); a `modiq-cli` subcommand was included, since the Sprint Authorization's own success criteria names a complete, reachable workflow ending in "Candidate Output" (Question 2); no minimum-occurrence threshold is applied — every observed pair is reported, regardless of count (Question 3, not previously decided); the Design Guidance interpretation recorded in the Capability Definition (reuse existing closed-set vocabulary; do not touch `modiq-knowledge`) was applied as stated (Question 4).
- Empirically unvalidated: this capability has not been exercised against a real, multi-report `.modiq-storage` directory produced by actual `modiq-cli assess` usage over time — only against test-constructed reports. The underlying mechanism (`retrieve`, already used this way by `RetrieveCommand`) is unchanged, so this risk is judged low, but it is stated plainly rather than implied validated.

---

## Known Limitations

- No minimum-occurrence filtering — every observed `(rule_reference, severity)` pair is reported, including ones seen only once. Named as Question 3 in the Capability Definition; not decided by this implementation.
- No pagination or size limit on `list_keys`/`recurring_patterns` — reads every stored report into memory on each invocation. Explicitly deferred, per the Capability Definition's own Risks section (scale, low, explicitly deferred).
- The `.modiq-storage` root location remains the same fixed, non-configurable default (`STORAGE_ROOT`) every other command already uses — unchanged by this work, not newly introduced.

---

## Architectural Validation

`CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` §4 predicted no separate Architecture Evaluation would be required, on three grounds. All three are confirmed, not merely assumed, by the implementation actually produced:

- **Prediction: this capability would satisfy `GOVERNANCE.md`'s Storage "Must never" list by construction.** Confirmed: the implementation never runs during, or is consulted by, Evidence Collection/Rule Evaluation/Report generation (it has no call site anywhere in `modiq-engine`); it never mutates a stored report (`list_keys` and `recurring_patterns` are both read-only, confirmed directly by `recurring_patterns_never_mutates_the_store`); it persists nothing new (no write path was added at all); and it required zero change to `AssessmentService`'s public entry points (confirmed directly — neither `modiq-engine` nor `modiq-runtime` was touched).
- **Prediction: the Adopted Architectural Constraint (never participate in live execution) would be satisfied by construction.** Confirmed: `HistoryCommand` and `recurring_patterns` are reachable only from `modiq-cli`'s own dispatch and are never called from `AssessmentService`, `RuleEngine`, or any Collector.
- **Prediction: no new domain type would be required.** Confirmed: `PatternFrequency` is a plain struct over `String` and the already-existing `PersistedFindingSeverity`; no enum was added, and no existing type's shape changed (only its usage, via a new caller).

No prediction was disproved.

---

## Architectural Concerns

None. No conflict was found between this implementation and any Engineering Specification, Crate Boundary Rule, or the Adopted Architectural Constraint.

---

## Governance Observations

No Governance Register item, ADR, or crate boundary rule change is indicated by this work's evidence. `GOVERNANCE.md`'s existing Storage Crate Boundary Rule already correctly describes this capability's behavior (Architectural Validation, above) — nothing was found that rule does not already state.

---

## Implementation Constraints

**These are architectural invariants, not implementation assumptions** (see "Architectural Invariants vs. Implementation Assumptions," above) — binding since `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §6.2 and `GOVERNANCE.md`'s Storage Crate Boundary Rule, not choices this implementation was free to make differently. All were honored:

- Read-only over persisted reports — confirmed (`recurring_patterns_never_mutates_the_store`).
- Outside `AssessmentService::execute` — confirmed (no call site added anywhere in `modiq-engine`).
- No Knowledge Domain mutation — confirmed (`modiq-knowledge` was not touched at all; not even a dependency edge was added).
- No MKB writes — confirmed, same basis.
- No new domain types unless evidence required them — no new enum was introduced; `PatternFrequency` is a plain aggregation struct over existing types, judged the minimum necessary to return `recurring_patterns`'s own result at all (a bare tuple `(String, PersistedFindingSeverity, usize)` was considered and rejected only because it has no named accessors — a smaller, not larger, footprint than a new enum would have been).

No constraint required deviation.

---

## Recommendations

- The four Chief Architect Questions this implementation resolved by assumption (Assumptions Made, above) are worth explicit confirmation at the next review, particularly the minimum-occurrence threshold question, which is the one most likely to affect real usability once exercised against a real, accumulated `.modiq-storage` directory.
- If a real, multi-week `.modiq-storage` directory becomes available, re-running `history` against it (rather than only test-constructed reports) would validate the "empirically unvalidated" item named above.
- `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md` remains suspended, not resolved; no action is recommended on it here.

---

## Validation Summary

- `cargo fmt` ✅
- `cargo check --workspace` ✅ (zero warnings)
- `cargo test --workspace` ✅ (253 → 264 passed)
- `apps/sandbox/src-tauri` ✅ (9 → 9 passed, unaffected)
