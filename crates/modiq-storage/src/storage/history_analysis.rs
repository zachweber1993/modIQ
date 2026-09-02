//! Deterministic aggregation of recurring Rule/Severity patterns
//! across every report a `ReportStore` currently holds.
//!
//! Read-only and entirely out-of-band from Assessment execution — see
//! `docs/engineering/CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`.
//! Reuses `PersistedFindingSeverity` and the already-existing
//! `rule_reference` string directly; introduces no new enumeration
//! type. Never invoked from, or consulted during, `AssessmentService`'s
//! own execution path.
use super::persisted_report::{PersistedFindingSeverity, PersistedModHealthDimension};
use super::report_store::ReportStore;
use super::report_store_error::ReportStoreError;

/// How many times a specific `(rule_reference, severity)` pair has
/// occurred across every report currently held by a `ReportStore`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternFrequency {
    rule_reference: String,
    severity: PersistedFindingSeverity,
    occurrences: usize,
}

impl PatternFrequency {
    pub fn rule_reference(&self) -> &str {
        &self.rule_reference
    }

    pub fn severity(&self) -> PersistedFindingSeverity {
        self.severity
    }

    pub fn occurrences(&self) -> usize {
        self.occurrences
    }
}

/// Deterministically aggregates every Finding's `(rule_reference,
/// severity)` pair across every report `store` currently holds.
///
/// Read-only: retrieves each report through `ReportStore::retrieve`
/// unchanged, and never mutates, deletes, or reorders anything in
/// `store`. The result is sorted by `(rule_reference, severity)`
/// before being returned, so it is independent of both directory
/// enumeration order and Finding order within any single report.
pub fn recurring_patterns(store: &ReportStore) -> Result<Vec<PatternFrequency>, ReportStoreError> {
    let mut patterns: Vec<PatternFrequency> = Vec::new();

    for key in store.list_keys()? {
        let report = store.retrieve(&key)?;

        for finding in report.findings() {
            let rule_reference = finding.rule_reference();
            let severity = finding.severity();

            match patterns.iter_mut().find(|pattern| {
                pattern.rule_reference == rule_reference && pattern.severity == severity
            }) {
                Some(pattern) => pattern.occurrences += 1,
                None => patterns.push(PatternFrequency {
                    rule_reference: rule_reference.to_string(),
                    severity,
                    occurrences: 1,
                }),
            }
        }
    }

    patterns.sort_by(|a, b| {
        a.rule_reference
            .cmp(&b.rule_reference)
            .then_with(|| format!("{:?}", a.severity).cmp(&format!("{:?}", b.severity)))
    });

    Ok(patterns)
}

/// How many times a specific `ModHealthDimension` value has occurred
/// across every report currently held by a `ReportStore`, counting
/// only records that carry the value.
///
/// A Finding whose `mod_health_dimension()` is `None` — a record
/// persisted before this field existed in the persisted schema —
/// contributes no entry to this result. This is a permanent limitation
/// on historical completeness, not evidence the Finding had no
/// dimension: see
/// `docs/engineering/ARCHITECTURAL_RESOLUTION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md`
/// §5/§7.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModHealthDimensionFrequency {
    mod_health_dimension: PersistedModHealthDimension,
    occurrences: usize,
}

impl ModHealthDimensionFrequency {
    pub fn mod_health_dimension(&self) -> PersistedModHealthDimension {
        self.mod_health_dimension
    }

    pub fn occurrences(&self) -> usize {
        self.occurrences
    }
}

/// Deterministically aggregates every Finding's `ModHealthDimension`
/// across every report `store` currently holds, alongside — never in
/// place of — `recurring_patterns`'s own `(rule_reference, severity)`
/// aggregation, which this function does not read, call, or affect.
///
/// Read-only: retrieves each report through `ReportStore::retrieve`
/// unchanged, and never mutates, deletes, or reorders anything in
/// `store`. The result is sorted by the dimension's own `Debug`
/// representation before being returned, so it is independent of both
/// directory enumeration order and Finding order within any single
/// report.
pub fn mod_health_dimension_recurrence(
    store: &ReportStore,
) -> Result<Vec<ModHealthDimensionFrequency>, ReportStoreError> {
    let mut frequencies: Vec<ModHealthDimensionFrequency> = Vec::new();

    for key in store.list_keys()? {
        let report = store.retrieve(&key)?;

        for finding in report.findings() {
            if let Some(dimension) = finding.mod_health_dimension() {
                match frequencies
                    .iter_mut()
                    .find(|frequency| frequency.mod_health_dimension == dimension)
                {
                    Some(frequency) => frequency.occurrences += 1,
                    None => frequencies.push(ModHealthDimensionFrequency {
                        mod_health_dimension: dimension,
                        occurrences: 1,
                    }),
                }
            }
        }
    }

    frequencies.sort_by(|a, b| {
        format!("{:?}", a.mod_health_dimension).cmp(&format!("{:?}", b.mod_health_dimension))
    });

    Ok(frequencies)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use modiq_runtime::assessment::{
        Assessment, AssessmentContext, AssessmentSubject, Evidence, EvidenceCategory, Finding,
        FindingSeverity, FindingStatus, ModHealthDimension, RuleReference, VersionProfileReference,
    };

    /// A real, unique, temporary directory, mirroring this crate's own
    /// `report_store` test helper of the same shape. Removed when
    /// dropped.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-storage-history-test-{}-{}-{}",
                std::process::id(),
                label,
                unique
            ));
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn report_with_finding(
        severity: FindingSeverity,
        rule: &str,
    ) -> modiq_report::report::AssessmentReport {
        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "sample evidence",
            None,
            None,
            None,
        )
        .unwrap();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment
            .add_finding(
                Finding::new(
                    severity,
                    "Sample finding",
                    "sample finding",
                    ModHealthDimension::EngineeringQuality,
                    FindingStatus::Final,
                    vec![evidence.id()],
                    RuleReference::new(rule),
                )
                .unwrap(),
            )
            .unwrap();

        modiq_report::report::AssessmentReport::generate(&assessment)
    }

    fn report_with_dimension(
        dimension: ModHealthDimension,
    ) -> modiq_report::report::AssessmentReport {
        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "sample evidence",
            None,
            None,
            None,
        )
        .unwrap();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment
            .add_finding(
                Finding::new(
                    FindingSeverity::Warning,
                    "Sample finding",
                    "sample finding",
                    dimension,
                    FindingStatus::Final,
                    vec![evidence.id()],
                    RuleReference::new("sample-rule"),
                )
                .unwrap(),
            )
            .unwrap();

        modiq_report::report::AssessmentReport::generate(&assessment)
    }

    #[test]
    fn recurring_patterns_against_an_empty_store_is_empty() {
        let dir = TempDir::new("empty");
        let store = ReportStore::new(dir.path());

        let patterns = recurring_patterns(&store).expect("an empty store analyzes cleanly");

        assert!(patterns.is_empty());
    }

    #[test]
    fn recurring_patterns_counts_occurrences_of_the_same_rule_and_severity() {
        let dir = TempDir::new("counts");
        let store = ReportStore::new(dir.path());

        for _ in 0..3 {
            let report =
                report_with_finding(FindingSeverity::Warning, "version-compatibility-rule");
            store.store(&report).unwrap();
        }

        let patterns = recurring_patterns(&store).unwrap();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].rule_reference(), "version-compatibility-rule");
        assert_eq!(patterns[0].severity(), PersistedFindingSeverity::Warning);
        assert_eq!(patterns[0].occurrences(), 3);
    }

    #[test]
    fn recurring_patterns_distinguishes_by_rule_reference_and_by_severity() {
        let dir = TempDir::new("distinct");
        let store = ReportStore::new(dir.path());

        store
            .store(&report_with_finding(FindingSeverity::Warning, "rule-a"))
            .unwrap();
        store
            .store(&report_with_finding(FindingSeverity::Error, "rule-a"))
            .unwrap();
        store
            .store(&report_with_finding(FindingSeverity::Warning, "rule-b"))
            .unwrap();

        let patterns = recurring_patterns(&store).unwrap();

        assert_eq!(patterns.len(), 3);
        assert!(patterns.iter().all(|pattern| pattern.occurrences() == 1));
    }

    #[test]
    fn recurring_patterns_is_sorted_regardless_of_store_order() {
        let dir = TempDir::new("sorted");
        let store = ReportStore::new(dir.path());

        // Stored in an order that does not match the expected sorted
        // output, so a passing test cannot be an accident of
        // coincidentally-already-sorted input.
        store
            .store(&report_with_finding(FindingSeverity::Warning, "zebra-rule"))
            .unwrap();
        store
            .store(&report_with_finding(FindingSeverity::Error, "alpha-rule"))
            .unwrap();
        store
            .store(&report_with_finding(
                FindingSeverity::Warning,
                "middle-rule",
            ))
            .unwrap();

        let patterns = recurring_patterns(&store).unwrap();
        let rule_references: Vec<&str> = patterns
            .iter()
            .map(PatternFrequency::rule_reference)
            .collect();

        assert_eq!(
            rule_references,
            vec!["alpha-rule", "middle-rule", "zebra-rule"]
        );
    }

    #[test]
    fn recurring_patterns_never_mutates_the_store() {
        let dir = TempDir::new("read-only");
        let store = ReportStore::new(dir.path());
        let report = report_with_finding(FindingSeverity::Informational, "rule-c");
        let key = store.store(&report).unwrap();

        recurring_patterns(&store).unwrap();

        // The original report is still retrievable, unchanged, after
        // analysis — analysis performed no write, delete, or mutation.
        let retrieved = store.retrieve(&key).unwrap();
        assert_eq!(retrieved.findings().len(), 1);
    }

    #[test]
    fn mod_health_dimension_recurrence_against_an_empty_store_is_empty() {
        let dir = TempDir::new("dimension-empty");
        let store = ReportStore::new(dir.path());

        let frequencies =
            mod_health_dimension_recurrence(&store).expect("an empty store analyzes cleanly");

        assert!(frequencies.is_empty());
    }

    #[test]
    fn mod_health_dimension_recurrence_counts_occurrences_of_the_same_dimension() {
        let dir = TempDir::new("dimension-counts");
        let store = ReportStore::new(dir.path());

        for _ in 0..3 {
            store
                .store(&report_with_dimension(ModHealthDimension::Compatibility))
                .unwrap();
        }

        let frequencies = mod_health_dimension_recurrence(&store).unwrap();

        assert_eq!(frequencies.len(), 1);
        assert_eq!(
            frequencies[0].mod_health_dimension(),
            PersistedModHealthDimension::Compatibility
        );
        assert_eq!(frequencies[0].occurrences(), 3);
    }

    #[test]
    fn mod_health_dimension_recurrence_distinguishes_by_dimension() {
        let dir = TempDir::new("dimension-distinct");
        let store = ReportStore::new(dir.path());

        store
            .store(&report_with_dimension(ModHealthDimension::Compatibility))
            .unwrap();
        store
            .store(&report_with_dimension(ModHealthDimension::Structure))
            .unwrap();

        let frequencies = mod_health_dimension_recurrence(&store).unwrap();

        assert_eq!(frequencies.len(), 2);
        assert!(
            frequencies
                .iter()
                .all(|frequency| frequency.occurrences() == 1)
        );
    }

    #[test]
    fn mod_health_dimension_recurrence_excludes_records_predating_capture() {
        let dir = TempDir::new("dimension-mixed-vintage");
        let store = ReportStore::new(dir.path());

        // A fresh, `Some(...)`-carrying report, stored normally — also
        // ensures the store's own root directory exists before the
        // legacy file below is written directly into it.
        store
            .store(&report_with_dimension(ModHealthDimension::Compatibility))
            .unwrap();

        // A legacy-shape `PersistedAssessmentReport`, predating capture
        // of `mod_health_dimension` — no such key anywhere in this
        // JSON. Written directly to the store's own root, bypassing
        // `ReportStore::store` entirely, since every report that
        // function persists after Phase 1 always carries `Some(...)`:
        // this is the only way to construct a genuine legacy record in
        // a test.
        let legacy_json = r#"{
            "status": "EvaluatingRules",
            "evidence": [],
            "findings": [
                {
                    "severity": "Warning",
                    "title": "Legacy finding",
                    "summary": "legacy finding",
                    "evidence_indices": [],
                    "rule_reference": "legacy-rule"
                }
            ],
            "recommendations": []
        }"#;
        fs::write(dir.path().join("legacy-report.json"), legacy_json)
            .expect("can write a legacy-shape report directly into the store's root");

        let frequencies = mod_health_dimension_recurrence(&store).unwrap();

        assert_eq!(frequencies.len(), 1);
        assert_eq!(
            frequencies[0].mod_health_dimension(),
            PersistedModHealthDimension::Compatibility
        );
        assert_eq!(
            frequencies[0].occurrences(),
            1,
            "the legacy record must contribute no entry"
        );
    }

    #[test]
    fn mod_health_dimension_recurrence_is_sorted_regardless_of_store_order() {
        let dir = TempDir::new("dimension-sorted");
        let store = ReportStore::new(dir.path());

        // Stored in an order that does not match the expected sorted
        // output, so a passing test cannot be an accident of
        // coincidentally-already-sorted input.
        store
            .store(&report_with_dimension(ModHealthDimension::Structure))
            .unwrap();
        store
            .store(&report_with_dimension(ModHealthDimension::Compatibility))
            .unwrap();
        store
            .store(&report_with_dimension(ModHealthDimension::Maintainability))
            .unwrap();

        let frequencies = mod_health_dimension_recurrence(&store).unwrap();
        let dimensions: Vec<PersistedModHealthDimension> = frequencies
            .iter()
            .map(ModHealthDimensionFrequency::mod_health_dimension)
            .collect();

        assert_eq!(
            dimensions,
            vec![
                PersistedModHealthDimension::Compatibility,
                PersistedModHealthDimension::Maintainability,
                PersistedModHealthDimension::Structure,
            ]
        );
    }

    #[test]
    fn mod_health_dimension_recurrence_never_mutates_the_store() {
        let dir = TempDir::new("dimension-read-only");
        let store = ReportStore::new(dir.path());
        let report = report_with_dimension(ModHealthDimension::Performance);
        let key = store.store(&report).unwrap();

        mod_health_dimension_recurrence(&store).unwrap();

        // The original report is still retrievable, unchanged, after
        // analysis — analysis performed no write, delete, or mutation.
        let retrieved = store.retrieve(&key).unwrap();
        assert_eq!(retrieved.findings().len(), 1);
    }

    #[test]
    fn recurring_patterns_is_unaffected_by_a_record_carrying_mod_health_dimension() {
        let dir = TempDir::new("dimension-regression");
        let store = ReportStore::new(dir.path());

        // `report_with_finding` already produces a Finding carrying a
        // real `ModHealthDimension` (Some(EngineeringQuality), after
        // Phase 1) — proving `recurring_patterns`'s own
        // `(rule_reference, severity)` output is unaffected by the new
        // field's presence, not merely unmodified in source.
        store
            .store(&report_with_finding(
                FindingSeverity::Warning,
                "version-compatibility-rule",
            ))
            .unwrap();

        let patterns = recurring_patterns(&store).unwrap();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].rule_reference(), "version-compatibility-rule");
        assert_eq!(patterns[0].severity(), PersistedFindingSeverity::Warning);
        assert_eq!(patterns[0].occurrences(), 1);
    }
}
