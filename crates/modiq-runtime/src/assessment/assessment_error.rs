use super::assessment_status::AssessmentStatus;

/// Errors produced when an invalid Assessment lifecycle transition is attempted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AssessmentError {
    /// The Assessment was not in the state required for the attempted
    /// transition (RuntimeInvariants.md INV-010, INV-011).
    #[error("invalid lifecycle transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: AssessmentStatus,
        to: AssessmentStatus,
    },

    /// The Assessment has already completed and rejects further
    /// lifecycle transitions (RuntimeInvariants.md INV-012).
    #[error("assessment is already completed and accepts no further transitions")]
    AssessmentCompleted,

    /// Evidence may only be added while the Assessment is actively
    /// collecting evidence (RuntimeInvariants.md INV-002, INV-003).
    #[error("evidence may only be added while collecting evidence (current status: {status:?})")]
    EvidenceCollectionNotActive { status: AssessmentStatus },

    /// Findings may only be added while the Assessment is actively
    /// evaluating rules (RuntimeInvariants.md INV-004). Findings become
    /// immutable once evaluation completes because the aggregate no
    /// longer permits mutation outside EvaluatingRules.
    #[error("findings may only be added while evaluating rules (current status: {status:?})")]
    FindingCollectionNotActive { status: AssessmentStatus },

    /// Recommendations may only be added while the Assessment is
    /// actively evaluating rules; they become immutable once evaluation
    /// completes because the aggregate no longer permits mutation
    /// outside EvaluatingRules.
    #[error(
        "recommendations may only be added while evaluating rules (current status: {status:?})"
    )]
    RecommendationCollectionNotActive { status: AssessmentStatus },

    /// Recommendations require at least one Finding to exist
    /// (RuntimeInvariants.md INV-005).
    #[error("recommendations require at least one finding to exist")]
    RecommendationRequiresFinding,
}
