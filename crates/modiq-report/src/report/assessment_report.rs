use modiq_runtime::assessment::{
    Assessment, AssessmentId, AssessmentStatus, Evidence, Finding, Recommendation,
};

/// Structured assessment report.
///
/// An owned, point-in-time snapshot of an Assessment's summary, Evidence,
/// Findings, and Recommendations. Report generation performs no analysis
/// (DataModel.md); it only reflects state already produced through the
/// Assessment aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentReport {
    assessment_id: AssessmentId,
    status: AssessmentStatus,
    evidence: Vec<Evidence>,
    findings: Vec<Finding>,
    recommendations: Vec<Recommendation>,
}

impl AssessmentReport {
    /// Generates a report snapshot from the given Assessment's current state.
    pub fn generate(assessment: &Assessment) -> Self {
        Self {
            assessment_id: assessment.id(),
            status: assessment.status(),
            evidence: assessment.evidence().to_vec(),
            findings: assessment.findings().to_vec(),
            recommendations: assessment.recommendations().to_vec(),
        }
    }

    pub fn assessment_id(&self) -> AssessmentId {
        self.assessment_id
    }

    pub fn status(&self) -> AssessmentStatus {
        self.status
    }

    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }

    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    pub fn recommendations(&self) -> &[Recommendation] {
        &self.recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::{AssessmentContext, AssessmentSubject};

    #[test]
    fn generate_reflects_empty_assessment_state() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let report = AssessmentReport::generate(&assessment);

        assert_eq!(report.assessment_id(), assessment.id());
        assert_eq!(report.status(), AssessmentStatus::Created);
        assert!(report.evidence().is_empty());
        assert!(report.findings().is_empty());
        assert!(report.recommendations().is_empty());
    }

    #[test]
    fn generate_reflects_collected_evidence_findings_and_recommendations() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.add_evidence(Evidence).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();

        let report = AssessmentReport::generate(&assessment);

        assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
        assert_eq!(report.evidence(), &[Evidence]);
        assert_eq!(report.findings(), &[Finding]);
        assert_eq!(report.recommendations(), &[Recommendation]);
    }

    #[test]
    fn generate_after_completion_matches_generate_before_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.add_evidence(Evidence).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();

        let before = AssessmentReport::generate(&assessment);
        assessment.complete().unwrap();
        let after = AssessmentReport::generate(&assessment);

        assert_eq!(before.evidence(), after.evidence());
        assert_eq!(before.findings(), after.findings());
        assert_eq!(before.recommendations(), after.recommendations());
        assert_eq!(after.status(), AssessmentStatus::Completed);
    }
}
