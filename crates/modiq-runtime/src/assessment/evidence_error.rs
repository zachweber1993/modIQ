/// Errors produced when constructing invalid Evidence.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EvidenceError {
    /// Evidence must state an objective observation; an empty
    /// description carries no factual content for a Rule to evaluate.
    #[error("evidence description must not be empty")]
    EmptyDescription,

    /// A provided location must identify something; an empty location
    /// is indistinguishable from no location at all.
    #[error("evidence location must not be empty when provided")]
    EmptyLocation,
}
