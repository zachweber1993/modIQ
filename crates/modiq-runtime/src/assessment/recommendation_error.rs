/// Errors produced when constructing an invalid Recommendation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecommendationError {
    /// A Recommendation must state an action; an empty action carries
    /// no guidance for a user to act upon.
    #[error("recommendation action must not be empty")]
    EmptyAction,

    /// A Recommendation must reference at least one Finding (INV-014);
    /// a Recommendation with no Finding reference carries no traceable
    /// basis for its action.
    #[error("recommendation must reference at least one finding")]
    EmptyFindingIds,
}
