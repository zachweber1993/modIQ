/// Errors produced when constructing an invalid AssessmentSubject.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AssessmentSubjectError {
    /// A subject must carry a label that identifies what is being
    /// assessed; an empty label carries no identifying content.
    #[error("assessment subject label must not be empty")]
    EmptyLabel,
}
