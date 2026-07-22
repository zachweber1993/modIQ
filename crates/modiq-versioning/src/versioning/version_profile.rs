use super::game_version::GameVersion;

/// The compatibility context for a supported Farming Simulator
/// release: a `GameVersion` plus the set of `modDesc.xml` `descVersion`
/// values it recognizes.
///
/// Selects and contextualizes compatibility information; it does not
/// execute Rules, perform Assessments, or store runtime state
/// (`VersionProfile.md`: Non-Responsibilities).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionProfile {
    game_version: GameVersion,
    supported_desc_versions: Vec<u32>,
}

impl VersionProfile {
    pub fn new(game_version: GameVersion, supported_desc_versions: Vec<u32>) -> Self {
        Self {
            game_version,
            supported_desc_versions,
        }
    }

    pub fn game_version(&self) -> &GameVersion {
        &self.game_version
    }

    /// Whether this profile recognizes the given `modDesc.xml`
    /// `descVersion`.
    pub fn supports(&self, desc_version: u32) -> bool {
        self.supported_desc_versions.contains(&desc_version)
    }

    /// The platform's minimum viable Version Profile (Sprint 8): a
    /// single, hardcoded FS25 profile recognizing `descVersion` 93 —
    /// the value already used throughout this repository's own XML
    /// inspection fixtures (`modiq-collection`,
    /// `xml_collector.rs`). Exhaustive Farming Simulator version
    /// knowledge is explicitly out of scope for this Sprint.
    pub fn fs25() -> Self {
        Self::new(GameVersion::new("FS25"), vec![93])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_recognizes_a_declared_desc_version() {
        let profile = VersionProfile::new(GameVersion::new("FS25"), vec![93]);

        assert!(profile.supports(93));
    }

    #[test]
    fn supports_rejects_an_unrecognized_desc_version() {
        let profile = VersionProfile::new(GameVersion::new("FS25"), vec![93]);

        assert!(!profile.supports(42));
    }

    #[test]
    fn fs25_recognizes_desc_version_93() {
        let profile = VersionProfile::fs25();

        assert_eq!(profile.game_version().name(), "FS25");
        assert!(profile.supports(93));
        assert!(!profile.supports(74));
    }
}
