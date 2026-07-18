//! Architectural proof for Sprint 1: a complete deterministic Assessment
//! pipeline executed entirely through the `AssessmentService` orchestration
//! layer, spanning the Runtime Domain (modiq-runtime), Rule Engine
//! (modiq-rules), and Reporting (modiq-report) crates.
//!
//! This test drives the engine's public API only — it does not
//! orchestrate the pipeline itself, since that would duplicate
//! `AssessmentService::execute`'s own implementation.

use modiq_engine::engine::AssessmentService;
use modiq_runtime::assessment::{AssessmentContext, AssessmentStatus, AssessmentSubject, Evidence};

#[test]
fn complete_deterministic_assessment_pipeline_via_the_engine() {
    let service = AssessmentService;

    let report = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);

    // Evidence Collection
    assert_eq!(report.evidence(), &[Evidence]);
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

    let first = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);
    let second = service.execute(AssessmentSubject, AssessmentContext, vec![Evidence]);

    // Distinct Assessments, but deterministic, identical results for
    // identical input.
    assert_ne!(first.assessment_id(), second.assessment_id());
    assert_eq!(first.findings(), second.findings());
    assert_eq!(first.recommendations(), second.recommendations());
}
