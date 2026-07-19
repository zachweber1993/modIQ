/// Errors produced when constructing an invalid Finding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FindingError {
    /// A Finding must state a conclusion; an empty description carries
    /// nothing for a Recommendation or Report to reflect.
    #[error("finding description must not be empty")]
    EmptyDescription,
}
