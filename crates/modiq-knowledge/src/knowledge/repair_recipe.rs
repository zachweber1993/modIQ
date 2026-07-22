/// A structured engineering repair procedure (`KnowledgeModel.md`: Repair
/// Recipe).
///
/// Owned by the Knowledge Domain, not by any Rule (`modiq-knowledge`'s own
/// README: knowledge is authored here and consumed elsewhere, never the
/// reverse). `identifier` is the stable value a Runtime `RepairRecipeReference`
/// carries; `guidance` is the structured corrective-action text a
/// Recommendation's action is built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairRecipe {
    identifier: String,
    guidance: String,
}

impl RepairRecipe {
    /// General-purpose, infallible constructor. Used internally by this
    /// module's own named recipes (below) and available for tests and
    /// future recipes — not intended to be called from outside
    /// `modiq-knowledge` with hand-authored content.
    pub fn new(identifier: impl Into<String>, guidance: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            guidance: guidance.into(),
        }
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn guidance(&self) -> &str {
        &self.guidance
    }

    /// The platform's minimum viable Repair Recipe for a declared
    /// `descVersion` the active Version Profile does not recognize
    /// (Sprint 9: Repair Guidance). Authored here, in `modiq-knowledge` —
    /// not in `modiq-rules` — so that `VersionCompatibilityRule` consumes
    /// this content rather than authoring it (mirroring
    /// `VersionProfile::fs25()`'s identical shape in `modiq-versioning`).
    pub fn version_compatibility_declared_version_mismatch() -> Self {
        Self::new(
            "version-compatibility-declared-version-mismatch",
            "Verify the mod's declared descVersion against a supported Farming Simulator \
             release before relying on it, or confirm compatibility manually.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_the_identifier_and_guidance() {
        let recipe = RepairRecipe::new("sample-identifier", "sample guidance");

        assert_eq!(recipe.identifier(), "sample-identifier");
        assert_eq!(recipe.guidance(), "sample guidance");
    }

    #[test]
    fn recipes_with_identical_content_are_equal() {
        let first = RepairRecipe::new("sample-identifier", "sample guidance");
        let second = RepairRecipe::new("sample-identifier", "sample guidance");

        assert_eq!(first, second);
    }

    #[test]
    fn recipes_with_different_identifiers_are_not_equal() {
        let first = RepairRecipe::new("first-identifier", "sample guidance");
        let second = RepairRecipe::new("second-identifier", "sample guidance");

        assert_ne!(first, second);
    }

    #[test]
    fn version_compatibility_declared_version_mismatch_has_a_stable_identifier() {
        let recipe = RepairRecipe::version_compatibility_declared_version_mismatch();

        assert_eq!(
            recipe.identifier(),
            "version-compatibility-declared-version-mismatch"
        );
        assert!(!recipe.guidance().is_empty());
    }

    #[test]
    fn version_compatibility_declared_version_mismatch_is_deterministic() {
        let first = RepairRecipe::version_compatibility_declared_version_mismatch();
        let second = RepairRecipe::version_compatibility_declared_version_mismatch();

        assert_eq!(first, second);
    }
}
