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
}
