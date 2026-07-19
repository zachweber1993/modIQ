/// Errors produced when constructing an invalid Finding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FindingError {
    /// A Finding must state a conclusion; an empty description carries
    /// nothing for a Recommendation or Report to reflect.
    #[error("finding description must not be empty")]
    EmptyDescription,

    /// A Finding must reference at least one Evidence item (INV-013);
    /// a Finding with no Evidence reference carries no traceable basis
    /// for its conclusion.
    #[error("finding must reference at least one evidence item")]
    EmptyEvidenceIds,
}
