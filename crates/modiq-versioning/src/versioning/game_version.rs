/// A supported Farming Simulator release, identified by name.
///
/// Sprint 8 establishes Version Profile infrastructure with a single,
/// minimal profile (`VersionProfile::fs25`); exhaustive version
/// knowledge is explicitly out of scope
/// (`docs/engineering/SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameVersion(String);

impl GameVersion {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_the_name() {
        let version = GameVersion::new("FS25");

        assert_eq!(version.name(), "FS25");
    }
}
