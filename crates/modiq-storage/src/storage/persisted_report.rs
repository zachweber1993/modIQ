//! Storage's own persisted representation of a report's content.
//!
//! Deliberately not `modiq_report::AssessmentReport` and not any
//! Runtime entity type — see
//! `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`.
//! Runtime identity (`AssessmentId`/`EvidenceId`/`FindingId`) is never
//! preserved; Finding→Evidence and Recommendation→Finding
//! cross-references are preserved as positions within this persisted
//! report only, not as the original, process-local identifiers.
use std::collections::HashMap;

use modiq_report::report::AssessmentReport;
use modiq_runtime::assessment::{
    AssessmentStatus, Evidence, EvidenceCategory, EvidenceId, Finding, FindingId, FindingSeverity,
    Recommendation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistedAssessmentStatus {
    Created,
    CollectingEvidence,
    EvaluatingRules,
    Completed,
}

impl From<AssessmentStatus> for PersistedAssessmentStatus {
    fn from(status: AssessmentStatus) -> Self {
        match status {
            AssessmentStatus::Created => Self::Created,
            AssessmentStatus::CollectingEvidence => Self::CollectingEvidence,
            AssessmentStatus::EvaluatingRules => Self::EvaluatingRules,
            AssessmentStatus::Completed => Self::Completed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistedEvidenceCategory {
    XmlInspection,
    LuaAnalysis,
    RuntimeLogs,
    AssetValidation,
    DependencyResolution,
    PerformanceObservations,
    FileStructureAnalysis,
    StructuralDuplication,
}

impl From<EvidenceCategory> for PersistedEvidenceCategory {
    fn from(category: EvidenceCategory) -> Self {
        match category {
            EvidenceCategory::XmlInspection => Self::XmlInspection,
            EvidenceCategory::LuaAnalysis => Self::LuaAnalysis,
            EvidenceCategory::RuntimeLogs => Self::RuntimeLogs,
            EvidenceCategory::AssetValidation => Self::AssetValidation,
            EvidenceCategory::DependencyResolution => Self::DependencyResolution,
            EvidenceCategory::PerformanceObservations => Self::PerformanceObservations,
            EvidenceCategory::FileStructureAnalysis => Self::FileStructureAnalysis,
            EvidenceCategory::StructuralDuplication => Self::StructuralDuplication,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedEvidence {
    category: PersistedEvidenceCategory,
    description: String,
    location: Option<String>,
}

impl PersistedEvidence {
    fn from_evidence(evidence: &Evidence) -> Self {
        Self {
            category: PersistedEvidenceCategory::from(evidence.category()),
            description: evidence.description().to_string(),
            location: evidence.location().map(str::to_string),
        }
    }

    pub fn category(&self) -> PersistedEvidenceCategory {
        self.category
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistedFindingSeverity {
    Error,
    Warning,
    Informational,
    BestPractice,
}

impl From<FindingSeverity> for PersistedFindingSeverity {
    fn from(severity: FindingSeverity) -> Self {
        match severity {
            FindingSeverity::Error => Self::Error,
            FindingSeverity::Warning => Self::Warning,
            FindingSeverity::Informational => Self::Informational,
            FindingSeverity::BestPractice => Self::BestPractice,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedFinding {
    severity: PersistedFindingSeverity,
    description: String,
    /// Positions into this same persisted report's own `evidence`
    /// list — not the original, process-local `EvidenceId` values.
    /// An id with no resolvable position (Runtime does not guarantee
    /// referential integrity here; GOV-005/GOV-006) is omitted rather
    /// than causing this transformation to fail.
    evidence_indices: Vec<usize>,
    rule_reference: String,
}

impl PersistedFinding {
    fn from_finding(finding: &Finding, evidence_positions: &HashMap<EvidenceId, usize>) -> Self {
        Self {
            severity: PersistedFindingSeverity::from(finding.severity()),
            description: finding.description().to_string(),
            evidence_indices: finding
                .evidence_ids()
                .iter()
                .filter_map(|id| evidence_positions.get(id).copied())
                .collect(),
            rule_reference: finding.rule_reference().identifier().to_string(),
        }
    }

    pub fn severity(&self) -> PersistedFindingSeverity {
        self.severity
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn evidence_indices(&self) -> &[usize] {
        &self.evidence_indices
    }

    pub fn rule_reference(&self) -> &str {
        &self.rule_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedRecommendation {
    action: String,
    /// Positions into this same persisted report's own `findings`
    /// list — not the original, process-local `FindingId` values.
    finding_indices: Vec<usize>,
    repair_recipe_reference: Option<String>,
}

impl PersistedRecommendation {
    fn from_recommendation(
        recommendation: &Recommendation,
        finding_positions: &HashMap<FindingId, usize>,
    ) -> Self {
        Self {
            action: recommendation.action().to_string(),
            finding_indices: recommendation
                .finding_ids()
                .iter()
                .filter_map(|id| finding_positions.get(id).copied())
                .collect(),
            repair_recipe_reference: recommendation
                .repair_recipe_reference()
                .map(|reference| reference.identifier().to_string()),
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn finding_indices(&self) -> &[usize] {
        &self.finding_indices
    }

    pub fn repair_recipe_reference(&self) -> Option<&str> {
        self.repair_recipe_reference.as_deref()
    }
}

/// Storage's own persisted representation of an `AssessmentReport`'s
/// content. Faithfulness is judged by content and order, never by
/// Runtime identity — see
/// `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`,
/// Section 6.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedAssessmentReport {
    status: PersistedAssessmentStatus,
    evidence: Vec<PersistedEvidence>,
    findings: Vec<PersistedFinding>,
    recommendations: Vec<PersistedRecommendation>,
}

impl PersistedAssessmentReport {
    /// Builds Storage's own representation from an `AssessmentReport`,
    /// reading only its already-public getters. Never touches, and
    /// never needs, `modiq-runtime`'s or `modiq-report`'s own
    /// internals.
    pub fn from_report(report: &AssessmentReport) -> Self {
        let evidence_positions: HashMap<EvidenceId, usize> = report
            .evidence()
            .iter()
            .enumerate()
            .map(|(index, evidence)| (evidence.id(), index))
            .collect();

        let finding_positions: HashMap<FindingId, usize> = report
            .findings()
            .iter()
            .enumerate()
            .map(|(index, finding)| (finding.id(), index))
            .collect();

        Self {
            status: PersistedAssessmentStatus::from(report.status()),
            evidence: report
                .evidence()
                .iter()
                .map(PersistedEvidence::from_evidence)
                .collect(),
            findings: report
                .findings()
                .iter()
                .map(|finding| PersistedFinding::from_finding(finding, &evidence_positions))
                .collect(),
            recommendations: report
                .recommendations()
                .iter()
                .map(|recommendation| {
                    PersistedRecommendation::from_recommendation(recommendation, &finding_positions)
                })
                .collect(),
        }
    }

    pub fn status(&self) -> PersistedAssessmentStatus {
        self.status
    }

    pub fn evidence(&self) -> &[PersistedEvidence] {
        &self.evidence
    }

    pub fn findings(&self) -> &[PersistedFinding] {
        &self.findings
    }

    pub fn recommendations(&self) -> &[PersistedRecommendation] {
        &self.recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::{
        Assessment, AssessmentContext, AssessmentSubject, RuleReference, VersionProfileReference,
    };

    fn sample_evidence() -> Evidence {
        Evidence::with_location(
            EvidenceCategory::FileStructureAnalysis,
            "missing modDesc.xml",
            "root",
        )
        .unwrap()
    }

    fn sample_finding(evidence_ids: Vec<EvidenceId>) -> Finding {
        Finding::new(
            FindingSeverity::Warning,
            "declared version mismatch",
            evidence_ids,
            RuleReference::new("version-compatibility-rule"),
        )
        .unwrap()
    }

    fn sample_recommendation(finding_ids: Vec<FindingId>) -> Recommendation {
        Recommendation::new("update the mod", finding_ids, None).unwrap()
    }

    #[test]
    fn from_report_preserves_empty_assessment_content() {
        let assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );

        let persisted =
            PersistedAssessmentReport::from_report(&AssessmentReport::generate(&assessment));

        assert_eq!(persisted.status(), PersistedAssessmentStatus::Created);
        assert!(persisted.evidence().is_empty());
        assert!(persisted.findings().is_empty());
        assert!(persisted.recommendations().is_empty());
    }

    #[test]
    fn from_report_preserves_content_and_order() {
        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = sample_finding(vec![evidence.id()]);
        assessment.add_finding(finding.clone()).unwrap();
        let recommendation = sample_recommendation(vec![finding.id()]);
        assessment
            .add_recommendation(recommendation.clone())
            .unwrap();

        let report = AssessmentReport::generate(&assessment);
        let persisted = PersistedAssessmentReport::from_report(&report);

        assert_eq!(
            persisted.status(),
            PersistedAssessmentStatus::EvaluatingRules
        );

        assert_eq!(persisted.evidence().len(), 1);
        assert_eq!(
            persisted.evidence()[0].category(),
            PersistedEvidenceCategory::FileStructureAnalysis
        );
        assert_eq!(persisted.evidence()[0].description(), "missing modDesc.xml");
        assert_eq!(persisted.evidence()[0].location(), Some("root"));

        assert_eq!(persisted.findings().len(), 1);
        assert_eq!(
            persisted.findings()[0].severity(),
            PersistedFindingSeverity::Warning
        );
        assert_eq!(
            persisted.findings()[0].description(),
            "declared version mismatch"
        );
        // The Finding's one EvidenceId resolves to position 0 in this
        // same persisted report's own evidence list.
        assert_eq!(persisted.findings()[0].evidence_indices(), &[0]);
        assert_eq!(
            persisted.findings()[0].rule_reference(),
            "version-compatibility-rule"
        );

        assert_eq!(persisted.recommendations().len(), 1);
        assert_eq!(persisted.recommendations()[0].action(), "update the mod");
        assert_eq!(persisted.recommendations()[0].finding_indices(), &[0]);
        assert_eq!(
            persisted.recommendations()[0].repair_recipe_reference(),
            None
        );
    }

    #[test]
    fn from_report_omits_unresolvable_evidence_references() {
        // Referential integrity between a Finding's evidence_ids and
        // the Assessment's own evidence is not enforced by Runtime
        // (GOV-005/GOV-006). A Finding referencing an EvidenceId that
        // does not resolve within this report must not cause this
        // transformation to fail.
        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let dangling_evidence_id = EvidenceId::generate();
        let finding = sample_finding(vec![dangling_evidence_id]);
        assessment.add_finding(finding).unwrap();

        let report = AssessmentReport::generate(&assessment);
        let persisted = PersistedAssessmentReport::from_report(&report);

        assert!(persisted.findings()[0].evidence_indices().is_empty());
    }
}
