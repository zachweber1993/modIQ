/// Represents an engineering conclusion produced by rule evaluation.
///
/// Findings are created by deterministic rules after evaluating
/// collected evidence. A finding represents an interpreted
/// engineering result rather than a raw observation.
///
/// Findings may later be used to generate recommendations
/// and assessment reports.
use super::evidence_id::EvidenceId;
use super::finding_error::FindingError;
use super::finding_id::FindingId;
use super::finding_severity::FindingSeverity;
use super::finding_status::FindingStatus;
use super::mod_health_dimension::ModHealthDimension;
use super::rule_reference::RuleReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    id: FindingId,
    severity: FindingSeverity,
    title: String,
    summary: String,
    mod_health_dimension: ModHealthDimension,
    status: FindingStatus,
    evidence_ids: Vec<EvidenceId>,
    rule_reference: RuleReference,
}

impl Finding {
    /// Creates a new Finding.
    ///
    /// `title` and `summary` must each not be empty; an empty title or
    /// summary carries no conclusion for a Recommendation or Report to
    /// reflect (Initiative 3, Item 1).
    ///
    /// `mod_health_dimension` and `status` are populated once, at
    /// construction, like every other field on this type (ADR-0007) —
    /// neither has a mutation method (Initiative 3, Items 2 and 10).
    ///
    /// `evidence_ids` must reference at least one Evidence item
    /// (INV-013, GOV-005). Referential integrity — whether each id
    /// actually resolves within the Assessment it is added to — is not
    /// checked here; that remains a separate, still-open governance
    /// question.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        severity: FindingSeverity,
        title: impl Into<String>,
        summary: impl Into<String>,
        mod_health_dimension: ModHealthDimension,
        status: FindingStatus,
        evidence_ids: Vec<EvidenceId>,
        rule_reference: RuleReference,
    ) -> Result<Self, FindingError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(FindingError::EmptyTitle);
        }
        let summary = summary.into();
        if summary.trim().is_empty() {
            return Err(FindingError::EmptySummary);
        }
        if evidence_ids.is_empty() {
            return Err(FindingError::EmptyEvidenceIds);
        }

        Ok(Self {
            id: FindingId::generate(),
            severity,
            title,
            summary,
            mod_health_dimension,
            status,
            evidence_ids,
            rule_reference,
        })
    }

    pub fn id(&self) -> FindingId {
        self.id
    }

    pub fn severity(&self) -> FindingSeverity {
        self.severity
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn mod_health_dimension(&self) -> ModHealthDimension {
        self.mod_health_dimension
    }

    pub fn status(&self) -> FindingStatus {
        self.status
    }

    pub fn evidence_ids(&self) -> &[EvidenceId] {
        &self.evidence_ids
    }

    pub fn rule_reference(&self) -> &RuleReference {
        &self.rule_reference
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assessment::{Evidence, EvidenceCategory};

    fn sample_rule_reference() -> RuleReference {
        RuleReference::new("sample-rule")
    }

    fn sample_evidence_ids() -> Vec<EvidenceId> {
        vec![EvidenceId::generate()]
    }

    #[allow(clippy::too_many_arguments)]
    fn build(
        title: &str,
        summary: &str,
        evidence_ids: Vec<EvidenceId>,
    ) -> Result<Finding, FindingError> {
        Finding::new(
            FindingSeverity::Warning,
            title,
            summary,
            ModHealthDimension::Compatibility,
            FindingStatus::Final,
            evidence_ids,
            sample_rule_reference(),
        )
    }

    #[test]
    fn new_succeeds_with_valid_title_and_summary() {
        let evidence_ids = sample_evidence_ids();
        let finding = build(
            "Missing dependency",
            "missing dependency detected",
            evidence_ids.clone(),
        )
        .expect("title, summary, and evidence_ids are all non-empty");

        assert_eq!(finding.severity(), FindingSeverity::Warning);
        assert_eq!(finding.title(), "Missing dependency");
        assert_eq!(finding.summary(), "missing dependency detected");
        assert_eq!(
            finding.mod_health_dimension(),
            ModHealthDimension::Compatibility
        );
        assert_eq!(finding.status(), FindingStatus::Final);
        assert_eq!(finding.evidence_ids(), evidence_ids.as_slice());
        assert_eq!(finding.rule_reference(), &sample_rule_reference());
    }

    #[test]
    fn new_rejects_an_empty_title() {
        let result = build("", "missing dependency detected", sample_evidence_ids());

        assert_eq!(result, Err(FindingError::EmptyTitle));
    }

    #[test]
    fn new_rejects_a_whitespace_only_title() {
        let result = build("   ", "missing dependency detected", sample_evidence_ids());

        assert_eq!(result, Err(FindingError::EmptyTitle));
    }

    #[test]
    fn new_rejects_an_empty_summary() {
        let result = build("Missing dependency", "", sample_evidence_ids());

        assert_eq!(result, Err(FindingError::EmptySummary));
    }

    #[test]
    fn new_rejects_a_whitespace_only_summary() {
        let result = build("Missing dependency", "   ", sample_evidence_ids());

        assert_eq!(result, Err(FindingError::EmptySummary));
    }

    #[test]
    fn new_rejects_empty_evidence_ids() {
        let result = build("Evidence-less finding", "evidence-less finding", vec![]);

        assert_eq!(result, Err(FindingError::EmptyEvidenceIds));
    }

    #[test]
    fn new_preserves_provided_evidence_ids() {
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "evidence",
            None,
            None,
            None,
        )
        .unwrap();
        let evidence_ids = vec![evidence.id()];

        let finding = build(
            "Invalid moddesc version",
            "invalid moddesc version",
            evidence_ids.clone(),
        )
        .unwrap();

        assert_eq!(finding.evidence_ids(), evidence_ids.as_slice());
    }

    #[test]
    fn mod_health_dimension_and_status_are_populated_only_at_construction() {
        let finding = Finding::new(
            FindingSeverity::Error,
            "Runtime load failure",
            "the mod failed to load",
            ModHealthDimension::Stability,
            FindingStatus::Final,
            sample_evidence_ids(),
            sample_rule_reference(),
        )
        .unwrap();

        // No mutation method exists on Finding — this test documents
        // that fact rather than exercising one. Both fields are fixed
        // at whatever `new` received.
        assert_eq!(
            finding.mod_health_dimension(),
            ModHealthDimension::Stability
        );
        assert_eq!(finding.status(), FindingStatus::Final);
    }

    #[test]
    fn each_finding_receives_a_unique_id() {
        let evidence_ids = sample_evidence_ids();
        let first = build(
            "Identical content",
            "identical content",
            evidence_ids.clone(),
        )
        .unwrap();
        let second = build("Identical content", "identical content", evidence_ids).unwrap();

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn finding_with_identical_content_but_different_identity_is_not_equal() {
        let evidence_ids = sample_evidence_ids();
        let first = build(
            "Identical content",
            "identical content",
            evidence_ids.clone(),
        )
        .unwrap();
        let second = build("Identical content", "identical content", evidence_ids).unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn cloned_finding_is_equal_to_its_source() {
        let finding = build(
            "Consider recommended structure",
            "consider using recommended structure",
            sample_evidence_ids(),
        )
        .unwrap();

        assert_eq!(finding.clone(), finding);
    }
}
