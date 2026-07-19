/// Opaque reference identifying the Repair Recipe that informed a
/// Recommendation's content, if any.
///
/// Runtime stores this value only; it does not own or evaluate the
/// Repair Recipe it identifies (GOVERNANCE.md: Knowledge Domain
/// boundary). A Recommendation is never triggered by a Repair Recipe
/// alone (RuleEngine.md: Recommendation Generation) — this reference
/// only informs content when one exists. Once `modiq-knowledge` is
/// integrated, this reference is the seam through which a real
/// Knowledge-sourced identifier will flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairRecipeReference(String);

impl RepairRecipeReference {
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
        let reference = RepairRecipeReference::new("sample-repair-recipe");

        assert_eq!(reference.identifier(), "sample-repair-recipe");
    }
}
