/// Aggregate root for deterministic engineering assessments.
///
/// Owns all runtime assessment state and enforces assessment invariants.
use super::assessment_error::AssessmentError;
use super::assessment_id::AssessmentId;
use super::assessment_status::AssessmentStatus;
use super::context::AssessmentContext;
use super::evidence::Evidence;
use super::evidence_id::EvidenceId;
use super::finding::Finding;
use super::finding_id::FindingId;
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

    /// Looks up an Evidence item owned by this Assessment by its id.
    ///
    /// Returns `None` if no Evidence with this id has been collected.
    pub fn evidence_by_id(&self, id: EvidenceId) -> Option<&Evidence> {
        self.evidence.iter().find(|evidence| evidence.id() == id)
    }

    /// Looks up a Finding owned by this Assessment by its id.
    ///
    /// Returns `None` if no Finding with this id has been produced.
    pub fn finding_by_id(&self, id: FindingId) -> Option<&Finding> {
        self.findings.iter().find(|finding| finding.id() == id)
    }

    /// Resolves the Evidence a Finding references, against this
    /// Assessment's own Evidence collection.
    ///
    /// As the aggregate root, Assessment is the only component with
    /// the context to resolve this relationship — a Finding holds only
    /// the identifiers of the Evidence it was derived from, not the
    /// Evidence itself.
    ///
    /// Only Evidence actually present in this Assessment is returned;
    /// an `EvidenceId` that does not resolve is silently omitted
    /// rather than treated as an error. `Finding::new` already
    /// requires `evidence_ids` to be non-empty (INV-013, GOV-005);
    /// requiring every reference to actually resolve is a separate,
    /// still-open governance question, and this method intentionally
    /// reflects the relationship as currently stored rather than
    /// validating it.
    pub fn evidence_for_finding(&self, finding: &Finding) -> Vec<&Evidence> {
        finding
            .evidence_ids()
            .iter()
            .filter_map(|id| self.evidence_by_id(*id))
            .collect()
    }

    /// Resolves the Findings a Recommendation references, against this
    /// Assessment's own Finding collection.
    ///
    /// Only Findings actually present in this Assessment are
    /// returned; a `FindingId` that does not resolve is silently
    /// omitted rather than treated as an error, for the same reason as
    /// `evidence_for_finding`: `Recommendation::new` already requires
    /// `finding_ids` to be non-empty (INV-014, GOV-006), but requiring
    /// every reference to actually resolve remains a separate,
    /// still-open governance question.
    pub fn findings_for_recommendation(&self, recommendation: &Recommendation) -> Vec<&Finding> {
        recommendation
            .finding_ids()
            .iter()
            .filter_map(|id| self.finding_by_id(*id))
            .collect()
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
    use crate::assessment::{EvidenceCategory, FindingSeverity, RuleReference};

    fn sample_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    fn sample_finding() -> Finding {
        Finding::new(
            FindingSeverity::Informational,
            "sample finding",
            vec![EvidenceId::generate()],
            RuleReference::new("sample-rule"),
        )
        .expect("severity, description, evidence_ids, and rule reference are valid")
    }

    fn sample_recommendation() -> Recommendation {
        Recommendation::new("sample recommendation", vec![FindingId::generate()], None)
            .expect("action and finding_ids are valid")
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
        let finding = sample_finding();

        let result = assessment.add_finding(finding.clone());

        assert!(result.is_ok());
        assert_eq!(assessment.findings().len(), 1);
        assert_eq!(assessment.findings()[0], finding);
    }

    #[test]
    fn add_finding_accumulates_multiple_findings() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assessment.add_finding(sample_finding()).unwrap();
        assessment.add_finding(sample_finding()).unwrap();
        assessment.add_finding(sample_finding()).unwrap();

        assert_eq!(assessment.findings().len(), 3);
    }

    #[test]
    fn add_finding_rejects_while_created() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.add_finding(sample_finding());

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

        let result = assessment.add_finding(sample_finding());

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
        assessment.add_finding(sample_finding()).unwrap();
        assessment.complete().unwrap();

        let result = assessment.add_finding(sample_finding());

        assert_eq!(result, Err(AssessmentError::AssessmentCompleted));
        // The Finding added before completion remains; the rejected call added nothing.
        assert_eq!(assessment.findings().len(), 1);
    }

    #[test]
    fn repeated_add_finding_attempts_before_evaluation_never_mutate_state() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        for _ in 0..3 {
            let result = assessment.add_finding(sample_finding());
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
        let first = sample_finding();
        let second = sample_finding();
        assessment.add_finding(first.clone()).unwrap();
        assessment.add_finding(second.clone()).unwrap();
        assessment.complete().unwrap();

        assert_eq!(assessment.findings(), &[first, second]);
    }

    #[test]
    fn evidence_is_unaffected_while_findings_are_added() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();

        assessment.add_finding(sample_finding()).unwrap();
        assessment.add_finding(sample_finding()).unwrap();

        assert_eq!(assessment.evidence(), &[evidence]);
        assert_eq!(assessment.findings().len(), 2);
    }

    #[test]
    fn add_recommendation_succeeds_after_a_finding_exists() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(sample_finding()).unwrap();
        let recommendation = sample_recommendation();

        let result = assessment.add_recommendation(recommendation.clone());

        assert!(result.is_ok());
        assert_eq!(assessment.recommendations().len(), 1);
        assert_eq!(assessment.recommendations()[0], recommendation);
    }

    #[test]
    fn add_recommendation_accumulates_multiple_recommendations() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(sample_finding()).unwrap();

        assessment
            .add_recommendation(sample_recommendation())
            .unwrap();
        assessment
            .add_recommendation(sample_recommendation())
            .unwrap();

        assert_eq!(assessment.recommendations().len(), 2);
    }

    #[test]
    fn add_recommendation_rejects_without_a_finding() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();

        let result = assessment.add_recommendation(sample_recommendation());

        assert_eq!(result, Err(AssessmentError::RecommendationRequiresFinding));
        assert!(assessment.recommendations().is_empty());
    }

    #[test]
    fn add_recommendation_rejects_while_created() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let result = assessment.add_recommendation(sample_recommendation());

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

        let result = assessment.add_recommendation(sample_recommendation());

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
        assessment.add_finding(sample_finding()).unwrap();
        assessment
            .add_recommendation(sample_recommendation())
            .unwrap();
        assessment.complete().unwrap();

        let result = assessment.add_recommendation(sample_recommendation());

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
        assessment.add_finding(sample_finding()).unwrap();
        let recommendation = sample_recommendation();
        assessment
            .add_recommendation(recommendation.clone())
            .unwrap();
        assessment.complete().unwrap();

        assert_eq!(assessment.recommendations(), &[recommendation]);
    }

    #[test]
    fn evidence_and_findings_are_unaffected_while_recommendations_are_added() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = sample_finding();
        assessment.add_finding(finding.clone()).unwrap();

        assessment
            .add_recommendation(sample_recommendation())
            .unwrap();
        assessment
            .add_recommendation(sample_recommendation())
            .unwrap();

        assert_eq!(assessment.evidence(), &[evidence]);
        assert_eq!(assessment.findings(), &[finding]);
        assert_eq!(assessment.recommendations().len(), 2);
    }

    #[test]
    fn full_pipeline_then_completion_rejects_all_further_mutation() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = sample_finding();
        assessment.add_finding(finding.clone()).unwrap();
        let recommendation = sample_recommendation();
        assessment
            .add_recommendation(recommendation.clone())
            .unwrap();
        assessment.complete().unwrap();

        assert_eq!(
            assessment.add_evidence(sample_evidence()),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(
            assessment.add_finding(sample_finding()),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(
            assessment.add_recommendation(sample_recommendation()),
            Err(AssessmentError::AssessmentCompleted)
        );
        assert_eq!(assessment.evidence(), &[evidence]);
        assert_eq!(assessment.findings(), &[finding]);
        assert_eq!(assessment.recommendations(), &[recommendation]);
    }

    #[test]
    fn evidence_by_id_finds_collected_evidence() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();

        assert_eq!(assessment.evidence_by_id(evidence.id()), Some(&evidence));
    }

    #[test]
    fn evidence_by_id_returns_none_for_an_unknown_id() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        let unknown = sample_evidence().id();

        assert_eq!(assessment.evidence_by_id(unknown), None);
    }

    #[test]
    fn finding_by_id_finds_a_produced_finding() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = sample_finding();
        assessment.add_finding(finding.clone()).unwrap();

        assert_eq!(assessment.finding_by_id(finding.id()), Some(&finding));
    }

    #[test]
    fn finding_by_id_returns_none_for_an_unknown_id() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        let unknown = sample_finding().id();

        assert_eq!(assessment.finding_by_id(unknown), None);
    }

    #[test]
    fn evidence_for_finding_resolves_referenced_evidence() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let first_evidence = sample_evidence();
        let second_evidence = sample_evidence();
        assessment.add_evidence(first_evidence.clone()).unwrap();
        assessment.add_evidence(second_evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = Finding::new(
            FindingSeverity::Warning,
            "missing dependency detected",
            vec![first_evidence.id(), second_evidence.id()],
            RuleReference::new("sample-rule"),
        )
        .unwrap();
        assessment.add_finding(finding.clone()).unwrap();

        assert_eq!(
            assessment.evidence_for_finding(&finding),
            vec![&first_evidence, &second_evidence]
        );
    }

    #[test]
    fn evidence_for_finding_omits_evidence_ids_that_do_not_resolve() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        // A dangling reference: this EvidenceId was never added to this
        // Assessment. Requiring every reference to resolve is
        // governance-pending (see evidence_for_finding's own
        // documentation); today this is resolved permissively rather
        // than rejected.
        let dangling_id = sample_evidence().id();
        let finding = Finding::new(
            FindingSeverity::Warning,
            "missing dependency detected",
            vec![dangling_id],
            RuleReference::new("sample-rule"),
        )
        .unwrap();
        assessment.add_finding(finding.clone()).unwrap();

        assert!(assessment.evidence_for_finding(&finding).is_empty());
    }

    #[test]
    fn findings_for_recommendation_resolves_referenced_findings() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let first_finding = sample_finding();
        let second_finding = sample_finding();
        assessment.add_finding(first_finding.clone()).unwrap();
        assessment.add_finding(second_finding.clone()).unwrap();
        let recommendation = Recommendation::new(
            "resolve the missing dependency",
            vec![first_finding.id(), second_finding.id()],
            None,
        )
        .unwrap();
        assessment
            .add_recommendation(recommendation.clone())
            .unwrap();

        assert_eq!(
            assessment.findings_for_recommendation(&recommendation),
            vec![&first_finding, &second_finding]
        );
    }

    #[test]
    fn findings_for_recommendation_omits_finding_ids_that_do_not_resolve() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        assessment.begin_rule_evaluation().unwrap();
        assessment.add_finding(sample_finding()).unwrap();
        // A dangling reference: this FindingId was never produced within
        // this Assessment. Requiring every reference to resolve is the
        // pending INV-005 refinement; today this is resolved
        // permissively rather than rejected.
        let dangling_id = sample_finding().id();
        let recommendation =
            Recommendation::new("resolve the missing dependency", vec![dangling_id], None).unwrap();
        assessment
            .add_recommendation(recommendation.clone())
            .unwrap();

        assert!(
            assessment
                .findings_for_recommendation(&recommendation)
                .is_empty()
        );
    }

    #[test]
    fn full_pipeline_relationships_resolve_correctly() {
        let mut assessment = Assessment::new(AssessmentSubject, AssessmentContext);
        assessment.begin_evidence_collection().unwrap();
        let evidence = sample_evidence();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = Finding::new(
            FindingSeverity::Warning,
            "missing dependency detected",
            vec![evidence.id()],
            RuleReference::new("sample-rule"),
        )
        .unwrap();
        assessment.add_finding(finding.clone()).unwrap();
        let recommendation =
            Recommendation::new("resolve the missing dependency", vec![finding.id()], None)
                .unwrap();
        assessment
            .add_recommendation(recommendation.clone())
            .unwrap();

        assert_eq!(assessment.evidence_for_finding(&finding), vec![&evidence]);
        assert_eq!(
            assessment.findings_for_recommendation(&recommendation),
            vec![&finding]
        );
    }
}
