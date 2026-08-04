/// One category of change a Runtime-owned `RecommendationStep` may
/// describe. A closed-set projection of `modiq-knowledge`'s own
/// `RepairStepKind` (`Glossary.md`: Repair Recipe — XML changes, Lua
/// changes, Dependency installation, Asset replacement, Version
/// updates), named distinctly from it so that Runtime never holds
/// Knowledge's own type (ADR-0007: Opaque Runtime References).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationStepKind {
    XmlChange,
    LuaChange,
    DependencyInstallation,
    AssetReplacement,
    VersionUpdate,
}

/// One structured step of a `Recommendation`'s Runtime-owned repair
/// structure — a projection of a `RepairRecipe`'s own steps, populated
/// only at `Recommendation` construction time (ADR-0007: population at
/// construction only), supplementing `Recommendation::action` rather
/// than replacing it.
///
/// Carries no identity of its own; it is a plain nested value, not one
/// of ADR-0007's four identity-bearing entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationStep {
    kind: RecommendationStepKind,
    instruction: String,
}

impl RecommendationStep {
    pub fn new(kind: RecommendationStepKind, instruction: impl Into<String>) -> Self {
        Self {
            kind,
            instruction: instruction.into(),
        }
    }

    pub fn kind(&self) -> RecommendationStepKind {
        self.kind
    }

    pub fn instruction(&self) -> &str {
        &self.instruction
    }
}
