//! Architectural proof for Sprint 1: a complete deterministic Assessment
//! pipeline executed entirely through the `AssessmentService` orchestration
//! layer, spanning the Runtime Domain (modiq-runtime), Rule Engine
//! (modiq-rules), and Reporting (modiq-report) crates.
//!
//! This test drives the engine's public API only — it does not
//! orchestrate the pipeline itself, since that would duplicate
//! `AssessmentService::execute`'s own implementation.

use modiq_engine::engine::AssessmentService;
use modiq_runtime::assessment::{
    AssessmentContext, AssessmentStatus, AssessmentSubject, Evidence, EvidenceCategory,
};

fn sample_evidence() -> Evidence {
    Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
        .expect("category and description are valid")
}

#[test]
fn complete_deterministic_assessment_pipeline_via_the_engine() {
    let service = AssessmentService;
    let evidence = sample_evidence();

    let report = service.execute(AssessmentSubject, AssessmentContext, vec![evidence.clone()]);

    // Evidence Collection
    assert_eq!(report.evidence(), &[evidence]);
    // Rule Evaluation + Finding Collection
    assert_eq!(report.findings().len(), 1);
    // Recommendation Collection
    assert_eq!(report.recommendations().len(), 1);
    // Assessment Report Generation happens before Assessment Completion
    // (DataModel.md Runtime Lifecycle), so the returned report reflects
    // the Assessment mid-evaluation, not yet Completed.
    assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
}

#[test]
fn pipeline_with_no_evidence_via_the_engine_produces_an_empty_report() {
    let service = AssessmentService;

    let report = service.execute(AssessmentSubject, AssessmentContext, vec![]);

    assert!(report.evidence().is_empty());
    assert!(report.findings().is_empty());
    assert!(report.recommendations().is_empty());
}

#[test]
fn each_execution_produces_an_independent_assessment() {
    let service = AssessmentService;
    let evidence = sample_evidence();

    let first = service.execute(AssessmentSubject, AssessmentContext, vec![evidence.clone()]);
    let second = service.execute(AssessmentSubject, AssessmentContext, vec![evidence]);

    // Distinct Assessments, but deterministic, identical results for
    // identical input. Each execution's Finding is freshly assigned its
    // own identity by RuleEngine (mirroring AssessmentId/EvidenceId);
    // determinism is judged by content, not by incidental identity.
    assert_ne!(first.assessment_id(), second.assessment_id());
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
    // identity, and finding_ids is compared structurally rather than by
    // value since it references each execution's own Finding.
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
