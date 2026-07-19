/// Errors produced when constructing an invalid Recommendation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecommendationError {
    /// A Recommendation must state an action; an empty action carries
    /// no guidance for a user to act upon.
    #[error("recommendation action must not be empty")]
    EmptyAction,
}
