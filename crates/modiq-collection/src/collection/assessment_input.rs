use super::assessment_input_error::AssessmentInputError;

/// A stable reference to a filesystem object, at the moment collection
/// begins, supplied by the application layer through the Engine
/// (`EvidenceCollection.md`: Assessment Input). It identifies *where*
/// to look, not *what* will be found there.
///
/// Resolved by GOV-009. For the filesystem case: a file or a directory
/// are both valid values; a non-existent path is not rejected here —
/// existence and accessibility are checked when collection is actually
/// attempted (`EvidenceCollector::collect`), not at construction. This
/// type validates only that the value itself is well-formed (the
/// "Invalid Input" outcome, GOV-010); it never touches the filesystem.
///
/// Collection consumes an AssessmentInput; it never creates or
/// reinterprets one (`EvidenceCollection.md`: Non-Responsibilities).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentInput {
    value: String,
}

impl AssessmentInput {
    /// Creates a new AssessmentInput. `value` must not be empty.
    pub fn new(value: impl Into<String>) -> Result<Self, AssessmentInputError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AssessmentInputError::EmptyValue);
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeeds_with_a_valid_value() {
        let input = AssessmentInput::new("a/mod/path").expect("value is non-empty");

        assert_eq!(input.value(), "a/mod/path");
    }

    #[test]
    fn new_rejects_an_empty_value() {
        let result = AssessmentInput::new("");

        assert_eq!(result, Err(AssessmentInputError::EmptyValue));
    }

    #[test]
    fn new_rejects_a_whitespace_only_value() {
        let result = AssessmentInput::new("   ");

        assert_eq!(result, Err(AssessmentInputError::EmptyValue));
    }

    #[test]
    fn inputs_with_identical_content_are_equal() {
        let first = AssessmentInput::new("a/mod/path").unwrap();
        let second = AssessmentInput::new("a/mod/path").unwrap();

        assert_eq!(first, second);
    }
}
