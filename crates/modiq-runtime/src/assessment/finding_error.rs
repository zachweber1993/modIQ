/// Errors produced when constructing an invalid Finding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FindingError {
    /// A Finding must state a title; an empty title carries nothing
    /// for a Recommendation or Report to reflect at a glance.
    #[error("finding title must not be empty")]
    EmptyTitle,

    /// A Finding must state a summary; an empty summary carries no
    /// conclusion for a Recommendation or Report to reflect.
    #[error("finding summary must not be empty")]
    EmptySummary,

    /// A Finding must reference at least one Evidence item (INV-013);
    /// a Finding with no Evidence reference carries no traceable basis
    /// for its conclusion.
    #[error("finding must reference at least one evidence item")]
    EmptyEvidenceIds,
}
