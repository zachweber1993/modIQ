/// One category of change a Repair Recipe step may describe
/// (`Glossary.md`: Repair Recipe — XML changes, Lua changes,
/// Dependency installation, Asset replacement, Version updates).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairStepKind {
    XmlChange,
    LuaChange,
    DependencyInstallation,
    AssetReplacement,
    VersionUpdate,
}

/// One structured step within a Repair Recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairStep {
    kind: RepairStepKind,
    instruction: String,
}

impl RepairStep {
    pub fn new(kind: RepairStepKind, instruction: impl Into<String>) -> Self {
        Self {
            kind,
            instruction: instruction.into(),
        }
    }

    pub fn kind(&self) -> RepairStepKind {
        self.kind
    }

    pub fn instruction(&self) -> &str {
        &self.instruction
    }
}

/// A structured engineering repair procedure (`KnowledgeModel.md`: Repair
/// Recipe).
///
/// Owned by the Knowledge Domain, not by any Rule (`modiq-knowledge`'s own
/// README: knowledge is authored here and consumed elsewhere, never the
/// reverse). `identifier` is the stable value a Runtime `RepairRecipeReference`
/// carries; `steps` (Initiative 3, Item 5) is the structured corrective-action
/// content a Recommendation's action is built from, replacing the flat
/// `guidance: String` this type previously carried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairRecipe {
    identifier: String,
    steps: Vec<RepairStep>,
}

impl RepairRecipe {
    /// General-purpose, infallible constructor. Used internally by this
    /// module's own named recipes (below) and available for tests and
    /// future recipes — not intended to be called from outside
    /// `modiq-knowledge` with hand-authored content.
    pub fn new(identifier: impl Into<String>, steps: Vec<RepairStep>) -> Self {
        Self {
            identifier: identifier.into(),
            steps,
        }
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn steps(&self) -> &[RepairStep] {
        &self.steps
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
            vec![RepairStep::new(
                RepairStepKind::VersionUpdate,
                "Verify the mod's declared descVersion against a supported Farming Simulator \
                 release before relying on it, or confirm compatibility manually.",
            )],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_steps() -> Vec<RepairStep> {
        vec![RepairStep::new(RepairStepKind::XmlChange, "sample step")]
    }

    #[test]
    fn new_preserves_the_identifier_and_steps() {
        let recipe = RepairRecipe::new("sample-identifier", sample_steps());

        assert_eq!(recipe.identifier(), "sample-identifier");
        assert_eq!(recipe.steps().len(), 1);
        assert_eq!(recipe.steps()[0].kind(), RepairStepKind::XmlChange);
        assert_eq!(recipe.steps()[0].instruction(), "sample step");
    }

    #[test]
    fn recipes_with_identical_content_are_equal() {
        let first = RepairRecipe::new("sample-identifier", sample_steps());
        let second = RepairRecipe::new("sample-identifier", sample_steps());

        assert_eq!(first, second);
    }

    #[test]
    fn recipes_with_different_identifiers_are_not_equal() {
        let first = RepairRecipe::new("first-identifier", sample_steps());
        let second = RepairRecipe::new("second-identifier", sample_steps());

        assert_ne!(first, second);
    }

    #[test]
    fn version_compatibility_declared_version_mismatch_has_a_stable_identifier() {
        let recipe = RepairRecipe::version_compatibility_declared_version_mismatch();

        assert_eq!(
            recipe.identifier(),
            "version-compatibility-declared-version-mismatch"
        );
        assert_eq!(recipe.steps().len(), 1);
        assert_eq!(recipe.steps()[0].kind(), RepairStepKind::VersionUpdate);
        assert!(!recipe.steps()[0].instruction().is_empty());
    }

    #[test]
    fn version_compatibility_declared_version_mismatch_is_deterministic() {
        let first = RepairRecipe::version_compatibility_declared_version_mismatch();
        let second = RepairRecipe::version_compatibility_declared_version_mismatch();

        assert_eq!(first, second);
    }
}
