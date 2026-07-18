use modiq_report::report::AssessmentReport;
use modiq_rules::rules::RuleEngine;
use modiq_runtime::assessment::{Assessment, AssessmentContext, AssessmentSubject, Evidence};

/// Coordinates the lifecycle of an Assessment.
///
/// Owns orchestration only: it sequences calls into the Runtime Domain
/// (modiq-runtime), Rule Engine (modiq-rules), and Reporting
/// (modiq-report) crates without owning runtime state, rule logic, or
/// reporting logic itself. Each of those responsibilities remains
/// implemented, and enforced, entirely within its own crate.
pub struct AssessmentService;

impl AssessmentService {
    /// Executes one complete deterministic Assessment: creation, Evidence
    /// collection, Rule evaluation, Finding and Recommendation
    /// collection, Assessment Report generation, and completion.
    ///
    /// Returns the Assessment Report generated prior to completion.
    pub fn execute(
        &self,
        subject: AssessmentSubject,
        context: AssessmentContext,
        evidence: Vec<Evidence>,
    ) -> AssessmentReport {
        let mut assessment = Assessment::new(subject, context);

        assessment
            .begin_evidence_collection()
            .expect("a newly created Assessment is always in the Created state");

        for item in evidence {
            assessment.add_evidence(item).expect(
                "evidence collection is active immediately after begin_evidence_collection",
            );
        }

        assessment
            .begin_rule_evaluation()
            .expect("evidence collection was just entered and has not yet transitioned away");

        let rule_engine = RuleEngine;
        if let Some(outcome) = rule_engine.evaluate(assessment.evidence()) {
            assessment
                .add_finding(outcome.finding)
                .expect("rule evaluation is active immediately after begin_rule_evaluation");
            assessment
                .add_recommendation(outcome.recommendation)
                .expect("a Finding was just added, and rule evaluation is still active");
        }

        let report = AssessmentReport::generate(&assessment);

        assessment
            .complete()
            .expect("rule evaluation is still active and has not yet completed");

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::AssessmentStatus;

    #[test]
    fn execute_with_evidence_produces_a_finding_and_recommendation() {
        let service = AssessmentService;

        let report = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);

        assert_eq!(report.evidence(), &[Evidence]);
        assert_eq!(report.findings().len(), 1);
        assert_eq!(report.recommendations().len(), 1);
    }

    #[test]
    fn execute_without_evidence_produces_no_findings_or_recommendations() {
        let service = AssessmentService;

        let report = service.execute(AssessmentSubject, AssessmentContext, vec![]);

        assert!(report.evidence().is_empty());
        assert!(report.findings().is_empty());
        assert!(report.recommendations().is_empty());
    }

    #[test]
    fn execute_reflects_state_at_report_generation_prior_to_completion() {
        let service = AssessmentService;

        let report = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);

        assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
    }

    #[test]
    fn separate_executions_are_independent_and_deterministic() {
        let service = AssessmentService;

        let first = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);
        let second = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);

        assert_ne!(first.assessment_id(), second.assessment_id());
        assert_eq!(first.evidence(), second.evidence());
        assert_eq!(first.findings(), second.findings());
        assert_eq!(first.recommendations(), second.recommendations());
    }
}
