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
    label: Option<String>,
    source: Option<String>,
    content: Option<String>,
}

impl Evidence {
    /// Creates new Evidence with no known location.
    ///
    /// `description` must not be empty; an empty description carries
    /// no factual content for a Rule to evaluate.
    ///
    /// `label`, `source`, and `content` (Initiative 3, Item 4) are
    /// each optional — a Collector supplies whichever it can actually
    /// observe; none is required to populate all three.
    pub fn new(
        category: EvidenceCategory,
        description: impl Into<String>,
        label: Option<String>,
        source: Option<String>,
        content: Option<String>,
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
            label,
            source,
            content,
        })
    }

    /// Creates new Evidence identifying where it was observed.
    ///
    /// `description` and `location` must not be empty. `label`,
    /// `source`, and `content` are each optional, identically to
    /// [`Evidence::new`].
    pub fn with_location(
        category: EvidenceCategory,
        description: impl Into<String>,
        location: impl Into<String>,
        label: Option<String>,
        source: Option<String>,
        content: Option<String>,
    ) -> Result<Self, EvidenceError> {
        let location = location.into();
        if location.trim().is_empty() {
            return Err(EvidenceError::EmptyLocation);
        }

        let mut evidence = Self::new(category, description, label, source, content)?;
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

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
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
            None,
            None,
            None,
        )
        .expect("description is non-empty");

        assert_eq!(evidence.category(), EvidenceCategory::FileStructureAnalysis);
        assert_eq!(evidence.description(), "missing modDesc.xml");
        assert_eq!(evidence.location(), None);
        assert_eq!(evidence.label(), None);
        assert_eq!(evidence.source(), None);
        assert_eq!(evidence.content(), None);
    }

    #[test]
    fn new_rejects_an_empty_description() {
        let result = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "",
            None,
            None,
            None,
        );

        assert_eq!(result, Err(EvidenceError::EmptyDescription));
    }

    #[test]
    fn new_rejects_a_whitespace_only_description() {
        let result = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "   ",
            None,
            None,
            None,
        );

        assert_eq!(result, Err(EvidenceError::EmptyDescription));
    }

    #[test]
    fn new_preserves_label_source_and_content_when_provided() {
        let evidence = Evidence::new(
            EvidenceCategory::XmlInspection,
            "declared descVersion",
            Some("Declared Version".to_string()),
            Some("modDesc.xml".to_string()),
            Some("<modDesc descVersion=\"42\">".to_string()),
        )
        .expect("description is non-empty");

        assert_eq!(evidence.label(), Some("Declared Version"));
        assert_eq!(evidence.source(), Some("modDesc.xml"));
        assert_eq!(evidence.content(), Some("<modDesc descVersion=\"42\">"));
    }

    #[test]
    fn with_location_succeeds_with_valid_description_and_location() {
        let evidence = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "invalid moddesc version attribute",
            "modDesc.xml:3",
            None,
            None,
            None,
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
            None,
            None,
            None,
        );

        assert_eq!(result, Err(EvidenceError::EmptyLocation));
    }

    #[test]
    fn with_location_rejects_an_empty_description() {
        let result = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "",
            "modDesc.xml:3",
            None,
            None,
            None,
        );

        assert_eq!(result, Err(EvidenceError::EmptyDescription));
    }

    #[test]
    fn with_location_preserves_label_source_and_content_when_provided() {
        let evidence = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "invalid moddesc version attribute",
            "modDesc.xml:3",
            Some("Version Attribute".to_string()),
            Some("modDesc.xml".to_string()),
            None,
        )
        .expect("description and location are non-empty");

        assert_eq!(evidence.label(), Some("Version Attribute"));
        assert_eq!(evidence.source(), Some("modDesc.xml"));
        assert_eq!(evidence.content(), None);
    }

    #[test]
    fn each_evidence_receives_a_unique_id() {
        let first = Evidence::new(
            EvidenceCategory::LuaAnalysis,
            "identical content",
            None,
            None,
            None,
        )
        .unwrap();
        let second = Evidence::new(
            EvidenceCategory::LuaAnalysis,
            "identical content",
            None,
            None,
            None,
        )
        .unwrap();

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn evidence_with_identical_content_but_different_identity_is_not_equal() {
        let first = Evidence::new(
            EvidenceCategory::LuaAnalysis,
            "identical content",
            None,
            None,
            None,
        )
        .unwrap();
        let second = Evidence::new(
            EvidenceCategory::LuaAnalysis,
            "identical content",
            None,
            None,
            None,
        )
        .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn cloned_evidence_is_equal_to_its_source() {
        let evidence = Evidence::new(
            EvidenceCategory::DependencyResolution,
            "missing dependency",
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(evidence.clone(), evidence);
    }
}
