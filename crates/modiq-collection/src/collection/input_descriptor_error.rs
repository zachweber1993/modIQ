/// Errors produced when constructing an invalid InputDescriptor.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InputDescriptorError {
    /// An Input Descriptor must identify something to inspect; an empty
    /// value carries no location for Evidence Collection to act on.
    #[error("input descriptor must not be empty")]
    EmptyValue,
}
