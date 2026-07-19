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
use super::rule_reference::RuleReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    id: FindingId,
    severity: FindingSeverity,
    description: String,
    evidence_ids: Vec<EvidenceId>,
    rule_reference: RuleReference,
}

impl Finding {
    /// Creates a new Finding.
    ///
    /// `description` must not be empty; an empty description carries
    /// no conclusion for a Recommendation or Report to reflect.
    ///
    /// `evidence_ids` is accepted as provided, including empty. A
    /// requirement that every Finding reference at least one Evidence
    /// item is a new invariant pending governance approval
    /// (SPRINT2_IMPLEMENTATION_PLAN.md: Governance Prerequisites) and
    /// is not enforced here.
    pub fn new(
        severity: FindingSeverity,
        description: impl Into<String>,
        evidence_ids: Vec<EvidenceId>,
        rule_reference: RuleReference,
    ) -> Result<Self, FindingError> {
        let description = description.into();
        if description.trim().is_empty() {
            return Err(FindingError::EmptyDescription);
        }

        Ok(Self {
            id: FindingId::generate(),
            severity,
            description,
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

    pub fn description(&self) -> &str {
        &self.description
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

    #[test]
    fn new_succeeds_with_a_valid_description() {
        let finding = Finding::new(
            FindingSeverity::Warning,
            "missing dependency detected",
            vec![],
            sample_rule_reference(),
        )
        .expect("description is non-empty");

        assert_eq!(finding.severity(), FindingSeverity::Warning);
        assert_eq!(finding.description(), "missing dependency detected");
        assert!(finding.evidence_ids().is_empty());
        assert_eq!(finding.rule_reference(), &sample_rule_reference());
    }

    #[test]
    fn new_rejects_an_empty_description() {
        let result = Finding::new(
            FindingSeverity::Warning,
            "",
            vec![],
            sample_rule_reference(),
        );

        assert_eq!(result, Err(FindingError::EmptyDescription));
    }

    #[test]
    fn new_rejects_a_whitespace_only_description() {
        let result = Finding::new(
            FindingSeverity::Warning,
            "   ",
            vec![],
            sample_rule_reference(),
        );

        assert_eq!(result, Err(FindingError::EmptyDescription));
    }

    #[test]
    fn new_currently_permits_empty_evidence_ids_pending_governance_approval() {
        let finding = Finding::new(
            FindingSeverity::Informational,
            "evidence-less finding, pending governance",
            vec![],
            sample_rule_reference(),
        );

        assert!(finding.is_ok());
    }

    #[test]
    fn new_preserves_provided_evidence_ids() {
        let evidence = Evidence::new(EvidenceCategory::FileStructureAnalysis, "evidence").unwrap();
        let evidence_ids = vec![evidence.id()];

        let finding = Finding::new(
            FindingSeverity::Error,
            "invalid moddesc version",
            evidence_ids.clone(),
            sample_rule_reference(),
        )
        .unwrap();

        assert_eq!(finding.evidence_ids(), evidence_ids.as_slice());
    }

    #[test]
    fn each_finding_receives_a_unique_id() {
        let first = Finding::new(
            FindingSeverity::Informational,
            "identical content",
            vec![],
            sample_rule_reference(),
        )
        .unwrap();
        let second = Finding::new(
            FindingSeverity::Informational,
            "identical content",
            vec![],
            sample_rule_reference(),
        )
        .unwrap();

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn finding_with_identical_content_but_different_identity_is_not_equal() {
        let first = Finding::new(
            FindingSeverity::Informational,
            "identical content",
            vec![],
            sample_rule_reference(),
        )
        .unwrap();
        let second = Finding::new(
            FindingSeverity::Informational,
            "identical content",
            vec![],
            sample_rule_reference(),
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn cloned_finding_is_equal_to_its_source() {
        let finding = Finding::new(
            FindingSeverity::BestPractice,
            "consider using recommended structure",
            vec![],
            sample_rule_reference(),
        )
        .unwrap();

        assert_eq!(finding.clone(), finding);
    }
}
