use modiq_collection::collection::{EvidenceCollector, InputDescriptor, InputDescriptorError};
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

    /// Executes one complete deterministic Assessment, using Evidence
    /// Collection (`modiq-collection`) to produce its Evidence from an
    /// Input Descriptor, rather than accepting already-constructed
    /// Evidence directly (ADR-0008).
    ///
    /// Added alongside `execute` rather than changing its signature:
    /// whether `execute` itself should evolve to accept an Input
    /// Descriptor remains open (ADR-0009, GOV-008). This method
    /// resolves the Input Descriptor, invokes Evidence Collection, and
    /// then delegates to the existing, unchanged `execute` for the
    /// rest of the pipeline.
    pub fn execute_from_descriptor(
        &self,
        subject: AssessmentSubject,
        context: AssessmentContext,
        input: impl Into<String>,
    ) -> Result<AssessmentReport, InputDescriptorError> {
        let descriptor = InputDescriptor::new(input)?;
        let evidence = EvidenceCollector.collect(&descriptor);

        Ok(self.execute(subject, context, evidence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::{AssessmentStatus, EvidenceCategory};

    fn sample_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    #[test]
    fn execute_with_evidence_produces_a_finding_and_recommendation() {
        let service = AssessmentService;
        let evidence = sample_evidence();

        let report = service.execute(AssessmentSubject, AssessmentContext, vec![evidence.clone()]);

        assert_eq!(report.evidence(), &[evidence]);
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

        let report = service.execute(
            AssessmentSubject,
            AssessmentContext,
            vec![sample_evidence()],
        );

        assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
    }

    #[test]
    fn separate_executions_are_independent_and_deterministic() {
        let service = AssessmentService;
        let evidence = sample_evidence();

        let first = service.execute(AssessmentSubject, AssessmentContext, vec![evidence.clone()]);
        let second = service.execute(AssessmentSubject, AssessmentContext, vec![evidence]);

        assert_ne!(first.assessment_id(), second.assessment_id());
        assert_eq!(first.evidence(), second.evidence());
        // Each execution's Finding is freshly assigned its own identity by
        // RuleEngine (mirroring AssessmentId/EvidenceId); determinism is
        // judged by content, not by incidental identity.
        assert_eq!(first.findings().len(), second.findings().len());
        for (first_finding, second_finding) in first.findings().iter().zip(second.findings()) {
            assert_eq!(first_finding.severity(), second_finding.severity());
            assert_eq!(first_finding.description(), second_finding.description());
            assert_eq!(first_finding.evidence_ids(), second_finding.evidence_ids());
            assert_eq!(
                first_finding.rule_reference(),
                second_finding.rule_reference()
            );
        }
        // Same rationale for Recommendation: content is compared, not
        // identity, and finding_ids is compared structurally rather than
        // by value since it references each execution's own Finding.
        assert_eq!(
            first.recommendations().len(),
            second.recommendations().len()
        );
        for (first_recommendation, second_recommendation) in
            first.recommendations().iter().zip(second.recommendations())
        {
            assert_eq!(
                first_recommendation.action(),
                second_recommendation.action()
            );
            assert_eq!(
                first_recommendation.finding_ids().len(),
                second_recommendation.finding_ids().len()
            );
            assert_eq!(
                first_recommendation.repair_recipe_reference(),
                second_recommendation.repair_recipe_reference()
            );
        }
    }

    #[test]
    fn execute_from_descriptor_produces_a_finding_and_recommendation_via_the_real_pipeline() {
        let service = AssessmentService;

        let report = service
            .execute_from_descriptor(AssessmentSubject, AssessmentContext, "a/mod/path")
            .expect("input is non-empty");

        assert_eq!(report.evidence().len(), 1);
        assert_eq!(report.findings().len(), 1);
        assert_eq!(report.recommendations().len(), 1);
    }

    #[test]
    fn execute_from_descriptor_rejects_an_empty_input() {
        let service = AssessmentService;

        let result = service.execute_from_descriptor(AssessmentSubject, AssessmentContext, "");

        assert_eq!(result, Err(InputDescriptorError::EmptyValue));
    }

    #[test]
    fn execute_from_descriptor_reflects_state_at_report_generation_prior_to_completion() {
        let service = AssessmentService;

        let report = service
            .execute_from_descriptor(AssessmentSubject, AssessmentContext, "a/mod/path")
            .expect("input is non-empty");

        assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
    }
}
