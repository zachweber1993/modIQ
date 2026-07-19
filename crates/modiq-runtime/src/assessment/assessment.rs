/// Aggregate root for deterministic engineering assessments.
///
/// Owns all runtime assessment state and enforces assessment invariants.
use super::assessment_error::AssessmentError;
use super::assessment_id::AssessmentId;
use super::assessment_status::AssessmentStatus;
use super::context::AssessmentContext;
use super::evidence::Evidence;
use super::finding::Finding;
use super::recommendation::Recommendation;
use super::subject::AssessmentSubject;

#[derive(Debug)]
pub struct Assessment {
    id: AssessmentId,
    subject: AssessmentSubject,
    context: AssessmentContext,
    status: AssessmentStatus,
    evidence: Vec<Evidence>,
    findings: Vec<Finding>,
    recommendations: Vec<Recommendation>,
}

impl Assessment {
    /// Creates a new Assessment for the given subject and context.
    ///
    /// Per AssessmentCreation.md, this generates a unique AssessmentId,
    /// enters the Created lifecycle state (RuntimeInvariants.md INV-001),
    /// and initializes empty Evidence, Finding, and Recommendation
    /// collections.
    pub fn new(subject: AssessmentSubject, context: AssessmentContext) -> Self {
        Self {
            id: AssessmentId::generate(),
            subject,
            context,
            status: AssessmentStatus::Created,
            evidence: Vec::new(),
            findings: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn id(&self) -> AssessmentId {
        self.id
    }

    pub fn subject(&self) -> &AssessmentSubject {
        &self.subject
    }

    pub fn context(&self) -> &AssessmentContext {
        &self.context
    }

    pub fn status(&self) -> AssessmentStatus {
        self.status
    }

    /// Returns whether the Assessment is currently in the rule evaluation
    /// phase.
    ///
    /// While `true`, collected Evidence is available and immutable
    /// (RuntimeInvariants.md INV-002, INV-003): `evidence()` reflects the
    /// complete, final Evidence set for this Assessment, and
    /// `add_evidence` unconditionally rejects further additions.
    pub fn is_evaluating(&self) -> bool {
        self.status == AssessmentStatus::EvaluatingRules
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

    /// Transitions the Assessment into evidence collection.
    ///
    /// Valid only from `Created` (RuntimeInvariants.md INV-010, INV-011).
    pub fn begin_evidence_collection(&mut self) -> Result<(), AssessmentError> {
        self.transition(
            AssessmentStatus::Created,
            AssessmentStatus::CollectingEvidence,
        )
    }

    /// Transitions the Assessment into rule evaluation.
    ///
    /// Valid only from `CollectingEvidence` (RuntimeInvariants.md INV-003,
    /// INV-010, INV-011).
    pub fn begin_rule_evaluation(&mut self) -> Result<(), AssessmentError> {
        self.transition(
            AssessmentStatus::CollectingEvidence,
            AssessmentStatus::EvaluatingRules,
        )
    }

    /// Completes the Assessment.
    ///
    /// Valid only from `EvaluatingRules` (RuntimeInvariants.md INV-010,
    /// INV-011).
    pub fn complete(&mut self) -> Result<(), AssessmentError> {
        self.transition(
            AssessmentStatus::EvaluatingRules,
            AssessmentStatus::Completed,
        )
    }

    /// Adds Evidence to the Assessment.
    ///
    /// Valid only while the Assessment is actively collecting evidence
    /// (RuntimeInvariants.md INV-002). Once rule evaluation begins,
    /// previously collected Evidence becomes immutable because this is
    /// the sole mutation path and it stops accepting new Evidence
    /// (INV-003). Evidence is mutated only through this aggregate method
    /// (INV-006, INV-007, INV-009), and every call enforces lifecycle
    /// validity before mutating state, never silently ignoring an
    /// invalid call (INV-008).
    pub fn add_evidence(&mut self, evidence: Evidence) -> Result<(), AssessmentError> {
        if self.status == AssessmentStatus::Completed {
            return Err(AssessmentError::AssessmentCompleted);
        }

        if self.status != AssessmentStatus::CollectingEvidence {
            return Err(AssessmentError::EvidenceCollectionNotActive {
                status: self.status,
            });
        }

        self.evidence.push(evidence);
        Ok(())
    }

    /// Adds a Finding to the Assessment.
    ///
    /// Valid only while the Assessment is actively evaluating rules
    /// (RuntimeInvariants.md INV-004). Findings become immutable after
    /// the evaluation phase because the aggregate no longer permits
    /// mutation outside EvaluatingRules: this is the sole mutation path,
    /// and it stops accepting new Findings once evaluation completes.
    /// Findings are mutated only through this aggregate method
    /// (INV-006, INV-007, INV-009), and every call enforces lifecycle
    /// validity before mutating state, never silently ignoring an
    /// invalid call (INV-008).
    pub fn add_finding(&mut self, finding: Finding) -> Result<(), AssessmentError> {
        if self.status == AssessmentStatus::Completed {
            return Err(AssessmentError::AssessmentCompleted);
        }

        if self.status != AssessmentStatus::EvaluatingRules {
            return Err(AssessmentError::FindingCollectionNotActive {
                status: self.status,
            });
        }

        self.findings.push(finding);
        Ok(())
    }

    /// Adds a Recommendation to the Assessment.
    ///
    /// Requires at least one Finding to already exist
    /// (RuntimeInvariants.md INV-005). Valid only while the Assessment
    /// is actively evaluating rules; Recommendations become immutable
    /// once evaluation completes because the aggregate no longer
    /// permits mutation outside EvaluatingRules. Recommendations are
    /// mutated only through this aggregate method (INV-006, INV-007,
    /// INV-009), and every call enforces lifecycle and data validity
    /// before mutating state, never silently ignoring an invalid call
    /// (INV-008).
    pub fn add_recommendation(
        &mut self,
        recommendation: Recommendation,
    ) -> Result<(), AssessmentError> {
        if self.status == AssessmentStatus::Completed {
            return Err(AssessmentError::AssessmentCompleted);
        }

        if self.status != AssessmentStatus::EvaluatingRules {
            return Err(AssessmentError::RecommendationCollectionNotActive {
                status: self.status,
            });
        }

        if self.findings.is_empty() {
            return Err(AssessmentError::RecommendationRequiresFinding);
        }

        self.recommendations.push(recommendation);
        Ok(())
    }

    /// Advances `status` from `required` to `next`, or returns an
    /// `AssessmentError` without mutating state.
    ///
    /// A Completed Assessment always rejects further transitions
    /// (RuntimeInvariants.md INV-012), independent of the requested
    /// target state.
    fn transition(
        &mut self,
        required: AssessmentStatus,
        next: AssessmentStatus,
    ) -> Result<(), AssessmentError> {
        if self.status == AssessmentStatus::Completed {
            return Err(AssessmentError::AssessmentCompleted);
        }

        if self.status != required {
            return Err(AssessmentError::InvalidStateTransition {
                from: self.status,
                to: next,
            });
        }

        self.status = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assessment::EvidenceCategory;

    fn sample_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    #[test]
    fn new_assessment_begins_in_created_state() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        assert_eq!(assessment.status(), AssessmentStatus::Created);
    }

    #[test]
    fn new_assessment_initializes_empty_collections() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        assert!(assessment.evidence().is_empty());
        assert!(assessment.findings().is_empty());
        assert!(assessment.recommendations().is_empty());
    }

    #[test]
    fn new_assessment_preserves_subject_and_context() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        assert_eq!(assessment.subject(), &AssessmentSubject);
        assert_eq!(assessment.context(), &AssessmentContext);
    }

    #[test]
    fn each_assessment_receives_a_unique_id() {
        let first = Assessment::new(AssessmentSubject, AssessmentContext);
        let second = Assessment::new(AssessmentSubject, AssessmentContext);

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn begin_evidence_collection_succeeds_from_created() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        assert!(assessment.begin_evidence_collection().is_ok());
        assert_eq!(assessment.status(), AssessmentStatus::CollectingEvidence);
    }

    #[test]
    fn begin_rule_evaluation_succeeds_from_collecting_evidence() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();

        assert!(assessment.begin_rule_evaluation().is_ok());
        assert_eq!(assessment.status(), AssessmentStatus::EvaluatingRules);
    }

    #[test]
    fn complete_succeeds_from_evaluating_rules() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assert!(assessment.complete().is_ok());
        assert_eq!(assessment.status(), AssessmentStatus::Completed);
    }

    #[test]
    fn full_lifecycle_sequence_succeeds_end_to_end() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        assert_eq!(assessment.status(), AssessmentStatus::Created);
        assessment.begin_evidence_collection().unwrap();
        assert_eq!(assessment.status(), AssessmentStatus::CollectingEvidence);
        assessment.begin_rule_evaluation().unwrap();
        assert_eq!(assessment.status(), AssessmentStatus::EvaluatingRules);
        assessment.complete().unwrap();
        assert_eq!(assessment.status(), AssessmentStatus::Completed);
    }

    #[test]
    fn begin_rule_evaluation_rejects_skipping_evidence_collection() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.begin_rule_evaluation();

        assert_eq!(
            result,
            Err(AssessmentError::InvalidStateTransition {
                from: AssessmentStatus::Created,
                to: AssessmentStatus::EvaluatingRules,
            })
        );
        assert_eq!(assessment.status(), AssessmentStatus::Created);
    }

    #[test]
    fn complete_rejects_skipping_evidence_collection() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.complete();

        assert_eq!(
            result,
            Err(AssessmentError::InvalidStateTransition {
                from: AssessmentStatus::Created,
                to: AssessmentStatus::Completed,
            })
        );
        assert_eq!(assessment.status(), AssessmentStatus::Created);
    }

    #[test]
    fn complete_rejects_skipping_rule_evaluation() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();

        let result = assessment.complete();

        assert_eq!(
            result,
            Err(AssessmentError::InvalidStateTransition {
                from: AssessmentStatus::CollectingEvidence,
                to: AssessmentStatus::Completed,
            })
        );
        assert_eq!(assessment.status(), AssessmentStatus::CollectingEvidence);
    }

    #[test]
    fn begin_evidence_collection_rejects_repeated_call() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();

        let result = assessment.begin_evidence_collection();

        assert_eq!(
            result,
            Err(AssessmentError::InvalidStateTransition {
                from: AssessmentStatus::CollectingEvidence,
                to: AssessmentStatus::CollectingEvidence,
            })
        );
        assert_eq!(assessment.status(), AssessmentStatus::CollectingEvidence);
    }

    #[test]
    fn begin_evidence_collection_rejects_backwards_transition() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        let result = assessment.begin_evidence_collection();

        assert_eq!(
            result,
            Err(AssessmentError::InvalidStateTransition {
                from: AssessmentStatus::EvaluatingRules,
                to: AssessmentStatus::CollectingEvidence,
            })
        );
        assert_eq!(assessment.status(), AssessmentStatus::EvaluatingRules);
    }

    #[test]
    fn completed_assessment_rejects_further_transitions() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.complete().unwrap();

        assert_eq!(
            assessment.begin_evidence_collection(),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(
            assessment.begin_rule_evaluation(),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(
            assessment.complete(),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(assessment.status(), AssessmentStatus::Completed);
    }

    #[test]
    fn add_evidence_succeeds_during_evidence_collection() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();

        let result = assessment.add_evidence(evidence.clone());

        assert!(result.is_ok());
        assert_eq!(assessment.evidence().len(), 1);
        assert_eq!(assessment.evidence()[0], evidence);
    }

    #[test]
    fn add_evidence_accumulates_multiple_items() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();

        assessment.add_evidence(sample_evidence()).unwrap();
        assessment.add_evidence(sample_evidence()).unwrap();
        assessment.add_evidence(sample_evidence()).unwrap();

        assert_eq!(assessment.evidence().len(), 3);
    }

    #[test]
    fn add_evidence_rejects_before_evidence_collection_begins() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.add_evidence(sample_evidence());

        assert_eq!(
            result,
            Err(AssessmentError::EvidenceCollectionNotActive {
                status: AssessmentStatus::Created,
            })
        );
        assert!(assessment.evidence().is_empty());
    }

    #[test]
    fn add_evidence_rejects_once_rule_evaluation_has_started() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.add_evidence(sample_evidence()).unwrap();
        assessment.begin_rule_evaluation().unwrap();

        let result = assessment.add_evidence(sample_evidence());

        assert_eq!(
            result,
            Err(AssessmentError::EvidenceCollectionNotActive {
                status: AssessmentStatus::EvaluatingRules,
            })
        );
        // Evidence collected before rule evaluation began remains intact
        // and untouched by the rejected call (INV-003).
        assert_eq!(assessment.evidence().len(), 1);
    }

    #[test]
    fn add_evidence_rejects_after_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.complete().unwrap();

        let result = assessment.add_evidence(sample_evidence());

        assert_eq!(result, Err(AssessmentError::AssessmentCompleted));
        assert!(assessment.evidence().is_empty());
    }

    #[test]
    fn is_evaluating_is_false_before_rule_evaluation_begins() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assert!(!assessment.is_evaluating());

        assessment.begin_evidence_collection().unwrap();
        assert!(!assessment.is_evaluating());
    }

    #[test]
    fn is_evaluating_is_true_once_rule_evaluation_begins() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assert!(assessment.is_evaluating());
    }

    #[test]
    fn is_evaluating_is_false_after_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.complete().unwrap();

        assert!(!assessment.is_evaluating());
    }

    #[test]
    fn evidence_is_fully_available_and_unchanged_throughout_evaluation() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let first = sample_evidence();
        let second = sample_evidence();
        assessment.add_evidence(first.clone()).unwrap();
        assessment.add_evidence(second.clone()).unwrap();

        assessment.begin_rule_evaluation().unwrap();
        assert!(assessment.is_evaluating());
        assert_eq!(assessment.evidence(), &[first.clone(), second.clone()]);

        assessment.complete().unwrap();
        assert_eq!(assessment.evidence(), &[first, second]);
    }

    #[test]
    fn repeated_add_evidence_attempts_during_evaluation_never_mutate_state() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.add_evidence(sample_evidence()).unwrap();
        assessment.begin_rule_evaluation().unwrap();

        for _ in 0..3 {
            let result = assessment.add_evidence(sample_evidence());
            assert_eq!(
                result,
                Err(AssessmentError::EvidenceCollectionNotActive {
                    status: AssessmentStatus::EvaluatingRules,
                })
            );
        }

        assert_eq!(assessment.evidence().len(), 1);
    }

    #[test]
    fn findings_and_recommendations_remain_empty_throughout_the_lifecycle() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.add_evidence(sample_evidence()).unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assert!(assessment.findings().is_empty());
        assert!(assessment.recommendations().is_empty());

        assessment.complete().unwrap();

        assert!(assessment.findings().is_empty());
        assert!(assessment.recommendations().is_empty());
    }

    #[test]
    fn add_finding_succeeds_during_rule_evaluation() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        let result = assessment.add_finding(Finding);

        assert!(result.is_ok());
        assert_eq!(assessment.findings().len(), 1);
        assert_eq!(assessment.findings()[0], Finding);
    }

    #[test]
    fn add_finding_accumulates_multiple_findings() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assessment.add_finding(Finding).unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_finding(Finding).unwrap();

        assert_eq!(assessment.findings().len(), 3);
    }

    #[test]
    fn add_finding_rejects_while_created() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.add_finding(Finding);

        assert_eq!(
            result,
            Err(AssessmentError::FindingCollectionNotActive {
                status: AssessmentStatus::Created,
            })
        );
        assert!(assessment.findings().is_empty());
    }

    #[test]
    fn add_finding_rejects_while_collecting_evidence() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();

        let result = assessment.add_finding(Finding);

        assert_eq!(
            result,
            Err(AssessmentError::FindingCollectionNotActive {
                status: AssessmentStatus::CollectingEvidence,
            })
        );
        assert!(assessment.findings().is_empty());
    }

    #[test]
    fn add_finding_rejects_after_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.complete().unwrap();

        let result = assessment.add_finding(Finding);

        assert_eq!(result, Err(AssessmentError::AssessmentCompleted));
        // The Finding added before completion remains; the rejected call added nothing.
        assert_eq!(assessment.findings().len(), 1);
    }

    #[test]
    fn repeated_add_finding_attempts_before_evaluation_never_mutate_state() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        for _ in 0..3 {
            let result = assessment.add_finding(Finding);
            assert_eq!(
                result,
                Err(AssessmentError::FindingCollectionNotActive {
                    status: AssessmentStatus::Created,
                })
            );
        }

        assert!(assessment.findings().is_empty());
    }

    #[test]
    fn findings_remain_readable_after_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.complete().unwrap();

        assert_eq!(assessment.findings(), &[Finding, Finding]);
    }

    #[test]
    fn evidence_is_unaffected_while_findings_are_added() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assessment.add_finding(Finding).unwrap();
        assessment.add_finding(Finding).unwrap();

        assert_eq!(assessment.evidence(), &[evidence]);
        assert_eq!(assessment.findings().len(), 2);
    }

    #[test]
    fn add_recommendation_succeeds_after_a_finding_exists() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();

        let result = assessment.add_recommendation(Recommendation);

        assert!(result.is_ok());
        assert_eq!(assessment.recommendations().len(), 1);
        assert_eq!(assessment.recommendations()[0], Recommendation);
    }

    #[test]
    fn add_recommendation_accumulates_multiple_recommendations() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();

        assessment.add_recommendation(Recommendation).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();

        assert_eq!(assessment.recommendations().len(), 2);
    }

    #[test]
    fn add_recommendation_rejects_without_a_finding() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        let result = assessment.add_recommendation(Recommendation);

        assert_eq!(result, Err(AssessmentError::RecommendationRequiresFinding));
        assert!(assessment.recommendations().is_empty());
    }

    #[test]
    fn add_recommendation_rejects_while_created() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.add_recommendation(Recommendation);

        assert_eq!(
            result,
            Err(AssessmentError::RecommendationCollectionNotActive {
                status: AssessmentStatus::Created,
            })
        );
        assert!(assessment.recommendations().is_empty());
    }

    #[test]
    fn add_recommendation_rejects_while_collecting_evidence() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();

        let result = assessment.add_recommendation(Recommendation);

        assert_eq!(
            result,
            Err(AssessmentError::RecommendationCollectionNotActive {
                status: AssessmentStatus::CollectingEvidence,
            })
        );
        assert!(assessment.recommendations().is_empty());
    }

    #[test]
    fn add_recommendation_rejects_after_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();
        assessment.complete().unwrap();

        let result = assessment.add_recommendation(Recommendation);

        assert_eq!(result, Err(AssessmentError::AssessmentCompleted));
        // The Recommendation added before completion remains; the
        // rejected call added nothing.
        assert_eq!(assessment.recommendations().len(), 1);
    }

    #[test]
    fn recommendations_remain_readable_after_completion() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();
        assessment.complete().unwrap();

        assert_eq!(assessment.recommendations(), &[Recommendation]);
    }

    #[test]
    fn evidence_and_findings_are_unaffected_while_recommendations_are_added() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();

        assessment.add_recommendation(Recommendation).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();

        assert_eq!(assessment.evidence(), &[evidence]);
        assert_eq!(assessment.findings(), &[Finding]);
        assert_eq!(assessment.recommendations().len(), 2);
    }

    #[test]
    fn full_pipeline_then_completion_rejects_all_further_mutation() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(Finding).unwrap();
        assessment.add_recommendation(Recommendation).unwrap();
        assessment.complete().unwrap();

        assert_eq!(
            assessment.add_evidence(sample_evidence()),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(
            assessment.add_finding(Finding),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(
            assessment.add_recommendation(Recommendation),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(assessment.evidence(), &[evidence]);
        assert_eq!(assessment.findings(), &[Finding]);
        assert_eq!(assessment.recommendations(), &[Recommendation]);
    }
}
