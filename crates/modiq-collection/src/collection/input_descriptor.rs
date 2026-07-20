use super::input_descriptor_error::InputDescriptorError;

/// Identifies where Evidence Collection should look, supplied by the
/// application layer through the Engine (`EvidenceCollection.md`: The
/// Input Descriptor). It identifies *where* to look, not *what* will
/// be found there.
///
/// This is a minimal, concrete representation, sufficient to prove the
/// Evidence Collection boundary established by ADR-0008. Which
/// specification owns its authoritative definition, and what content
/// it eventually carries beyond an opaque string, remains an open
/// governance question (GOV-009) — not resolved by this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDescriptor {
    value: String,
}

impl InputDescriptor {
    /// Creates a new InputDescriptor. `value` must not be empty.
    pub fn new(value: impl Into<String>) -> Result<Self, InputDescriptorError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(InputDescriptorError::EmptyValue);
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
        let descriptor = InputDescriptor::new("a/mod/path").expect("value is non-empty");

        assert_eq!(descriptor.value(), "a/mod/path");
    }

    #[test]
    fn new_rejects_an_empty_value() {
        let result = InputDescriptor::new("");

        assert_eq!(result, Err(InputDescriptorError::EmptyValue));
    }

    #[test]
    fn new_rejects_a_whitespace_only_value() {
        let result = InputDescriptor::new("   ");

        assert_eq!(result, Err(InputDescriptorError::EmptyValue));
    }

    #[test]
    fn descriptors_with_identical_content_are_equal() {
        let first = InputDescriptor::new("a/mod/path").unwrap();
        let second = InputDescriptor::new("a/mod/path").unwrap();

        assert_eq!(first, second);
    }
}
