/// Errors produced when constructing an invalid AssessmentInput.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AssessmentInputError {
    /// An Assessment Input must identify something to inspect; an empty
    /// value carries no location for Evidence Collection to act on.
    /// This is the "Invalid Input" outcome (GOV-010): collection never
    /// begins.
    #[error("assessment input must not be empty")]
    EmptyValue,
}
