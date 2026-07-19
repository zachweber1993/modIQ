/// Opaque reference identifying the Rule that produced a Finding.
///
/// Runtime stores this value only; it does not own or evaluate the
/// Rule it identifies (GOVERNANCE.md: Knowledge Domain boundary). Once
/// `modiq-knowledge` is integrated, this reference is the seam through
/// which a real Knowledge-sourced identifier will flow, in place of
/// today's `modiq-rules`-internal constant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleReference(String);

impl RuleReference {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self(identifier.into())
    }

    pub fn identifier(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_the_identifier() {
        let reference = RuleReference::new("evidence-presence-rule");

        assert_eq!(reference.identifier(), "evidence-presence-rule");
    }
}
