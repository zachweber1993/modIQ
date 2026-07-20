use modiq_collection::collection::{AssessmentInputError, CollectionError};

/// Errors produced by `AssessmentService::execute_from_assessment_input`.
///
/// Unifies the two upstream failure sources
/// `execute_from_assessment_input` calls in sequence — constructing an
/// AssessmentInput, then invoking Evidence Collection — into one
/// result type, since orchestrating both is the Engine's own
/// responsibility. Represents the same four Collection Outcomes
/// (GOV-010) as before: `InvalidInput` is "collection never begins";
/// `Collection` covers both "collection aborts" outcomes (Inaccessible
/// and Unsupported); Empty Collection is not an error at all and is
/// not represented here.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AssessmentExecutionError {
    #[error(transparent)]
    InvalidInput(#[from] AssessmentInputError),

    #[error(transparent)]
    Collection(#[from] CollectionError),
}
