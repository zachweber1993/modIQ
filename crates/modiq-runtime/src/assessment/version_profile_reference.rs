/// Opaque reference identifying the Version Profile under which an
/// Assessment was executed.
///
/// Runtime stores this value only; it does not own or evaluate the
/// Version Profile it identifies. The real `VersionProfile` lives in
/// `modiq-versioning`, consulted directly by the Engine and Rule
/// Engine; Runtime never depends on it (Sprint 8 Architectural
/// Resolution, Decision 1 — mirrors `RuleReference` and
/// `RepairRecipeReference`'s existing Opaque Runtime References
/// pattern, ADR-0007).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionProfileReference(String);

impl VersionProfileReference {
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
        let reference = VersionProfileReference::new("FS25");

        assert_eq!(reference.identifier(), "FS25");
    }
}
