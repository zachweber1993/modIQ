/// Represents objective evidence collected during an Assessment.
///
/// Evidence consists of a single observed fact about the assessment
/// subject. Evidence itself is neutral and contains no
/// interpretation, severity, or recommendations.
///
/// Rules consume evidence to produce findings.
use super::evidence_category::EvidenceCategory;
use super::evidence_error::EvidenceError;
use super::evidence_id::EvidenceId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    id: EvidenceId,
    category: EvidenceCategory,
    description: String,
    location: Option<String>,
}

impl Evidence {
    /// Creates new Evidence with no known location.
    ///
    /// `description` must not be empty; an empty description carries
    /// no factual content for a Rule to evaluate.
    pub fn new(
        category: EvidenceCategory,
        description: impl Into<String>,
    ) -> Result<Self, EvidenceError> {
        let description = description.into();
        if description.trim().is_empty() {
            return Err(EvidenceError::EmptyDescription);
        }

        Ok(Self {
            id: EvidenceId::generate(),
            category,
            description,
            location: None,
        })
    }

    /// Creates new Evidence identifying where it was observed.
    ///
    /// Both `description` and `location` must not be empty.
    pub fn with_location(
        category: EvidenceCategory,
        description: impl Into<String>,
        location: impl Into<String>,
    ) -> Result<Self, EvidenceError> {
        let location = location.into();
        if location.trim().is_empty() {
            return Err(EvidenceError::EmptyLocation);
        }

        let mut evidence = Self::new(category, description)?;
        evidence.location = Some(location);
        Ok(evidence)
    }

    pub fn id(&self) -> EvidenceId {
        self.id
    }

    pub fn category(&self) -> EvidenceCategory {
        self.category
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeeds_with_a_valid_description() {
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "missing modDesc.xml",
        )
        .expect("description is non-empty");

        assert_eq!(evidence.category(), EvidenceCategory::FileStructureAnalysis);
        assert_eq!(evidence.description(), "missing modDesc.xml");
        assert_eq!(evidence.location(), None);
    }

    #[test]
    fn new_rejects_an_empty_description() {
        let result = Evidence::new(EvidenceCategory::FileStructureAnalysis, "");

        assert_eq!(result, Err(EvidenceError::EmptyDescription));
    }

    #[test]
    fn new_rejects_a_whitespace_only_description() {
        let result = Evidence::new(EvidenceCategory::FileStructureAnalysis, "   ");

        assert_eq!(result, Err(EvidenceError::EmptyDescription));
    }

    #[test]
    fn with_location_succeeds_with_valid_description_and_location() {
        let evidence = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "invalid moddesc version attribute",
            "modDesc.xml:3",
        )
        .expect("description and location are non-empty");

        assert_eq!(evidence.location(), Some("modDesc.xml:3"));
    }

    #[test]
    fn with_location_rejects_an_empty_location() {
        let result = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "invalid moddesc version attribute",
            "",
        );

        assert_eq!(result, Err(EvidenceError::EmptyLocation));
    }

    #[test]
    fn with_location_rejects_an_empty_description() {
        let result = Evidence::with_location(EvidenceCategory::XmlInspection, "", "modDesc.xml:3");

        assert_eq!(result, Err(EvidenceError::EmptyDescription));
    }

    #[test]
    fn each_evidence_receives_a_unique_id() {
        let first = Evidence::new(EvidenceCategory::LuaAnalysis, "identical content").unwrap();
        let second = Evidence::new(EvidenceCategory::LuaAnalysis, "identical content").unwrap();

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn evidence_with_identical_content_but_different_identity_is_not_equal() {
        let first = Evidence::new(EvidenceCategory::LuaAnalysis, "identical content").unwrap();
        let second = Evidence::new(EvidenceCategory::LuaAnalysis, "identical content").unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn cloned_evidence_is_equal_to_its_source() {
        let evidence =
            Evidence::new(EvidenceCategory::DependencyResolution, "missing dependency").unwrap();

        assert_eq!(evidence.clone(), evidence);
    }
}
