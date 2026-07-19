/// Represents an actionable recommendation produced by an Assessment.
///
/// Recommendations are generated from engineering findings and
/// describe actions that improve the assessed subject.
///
/// Recommendations do not evaluate evidence directly and are
/// never produced without supporting findings.
use super::finding_id::FindingId;
use super::recommendation_error::RecommendationError;
use super::recommendation_id::RecommendationId;
use super::repair_recipe_reference::RepairRecipeReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommendation {
    id: RecommendationId,
    action: String,
    finding_ids: Vec<FindingId>,
    repair_recipe_reference: Option<RepairRecipeReference>,
}

impl Recommendation {
    /// Creates a new Recommendation.
    ///
    /// `action` must not be empty; an empty action carries no guidance
    /// for a user to act upon.
    ///
    /// `finding_ids` must reference at least one Finding (INV-014,
    /// GOV-006). Referential integrity — whether each id actually
    /// resolves within the Assessment it is added to — is not checked
    /// here; that remains a separate, still-open governance question.
    pub fn new(
        action: impl Into<String>,
        finding_ids: Vec<FindingId>,
        repair_recipe_reference: Option<RepairRecipeReference>,
    ) -> Result<Self, RecommendationError> {
        let action = action.into();
        if action.trim().is_empty() {
            return Err(RecommendationError::EmptyAction);
        }
        if finding_ids.is_empty() {
            return Err(RecommendationError::EmptyFindingIds);
        }

        Ok(Self {
            id: RecommendationId::generate(),
            action,
            finding_ids,
            repair_recipe_reference,
        })
    }

    pub fn id(&self) -> RecommendationId {
        self.id
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn finding_ids(&self) -> &[FindingId] {
        &self.finding_ids
    }

    pub fn repair_recipe_reference(&self) -> Option<&RepairRecipeReference> {
        self.repair_recipe_reference.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assessment::{EvidenceId, Finding, FindingSeverity, RuleReference};

    fn sample_finding_id() -> FindingId {
        Finding::new(
            FindingSeverity::Informational,
            "sample finding",
            vec![EvidenceId::generate()],
            RuleReference::new("sample-rule"),
        )
        .unwrap()
        .id()
    }

    fn sample_finding_ids() -> Vec<FindingId> {
        vec![sample_finding_id()]
    }

    #[test]
    fn new_succeeds_with_a_valid_action() {
        let finding_ids = sample_finding_ids();
        let recommendation =
            Recommendation::new("update the mod dependency", finding_ids.clone(), None)
                .expect("action is non-empty and finding_ids is non-empty");

        assert_eq!(recommendation.action(), "update the mod dependency");
        assert_eq!(recommendation.finding_ids(), finding_ids.as_slice());
        assert_eq!(recommendation.repair_recipe_reference(), None);
    }

    #[test]
    fn new_rejects_an_empty_action() {
        let result = Recommendation::new("", sample_finding_ids(), None);

        assert_eq!(result, Err(RecommendationError::EmptyAction));
    }

    #[test]
    fn new_rejects_a_whitespace_only_action() {
        let result = Recommendation::new("   ", sample_finding_ids(), None);

        assert_eq!(result, Err(RecommendationError::EmptyAction));
    }

    #[test]
    fn new_rejects_empty_finding_ids() {
        let result = Recommendation::new("finding-less recommendation", vec![], None);

        assert_eq!(result, Err(RecommendationError::EmptyFindingIds));
    }

    #[test]
    fn new_preserves_provided_finding_ids() {
        let finding_ids = vec![sample_finding_id()];

        let recommendation =
            Recommendation::new("resolve the missing dependency", finding_ids.clone(), None)
                .unwrap();

        assert_eq!(recommendation.finding_ids(), finding_ids.as_slice());
    }

    #[test]
    fn new_preserves_provided_repair_recipe_reference() {
        let reference = RepairRecipeReference::new("sample-repair-recipe");

        let recommendation = Recommendation::new(
            "apply the repair recipe",
            sample_finding_ids(),
            Some(reference.clone()),
        )
        .unwrap();

        assert_eq!(recommendation.repair_recipe_reference(), Some(&reference));
    }

    #[test]
    fn each_recommendation_receives_a_unique_id() {
        let finding_ids = sample_finding_ids();
        let first = Recommendation::new("identical content", finding_ids.clone(), None).unwrap();
        let second = Recommendation::new("identical content", finding_ids, None).unwrap();

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn recommendation_with_identical_content_but_different_identity_is_not_equal() {
        let finding_ids = sample_finding_ids();
        let first = Recommendation::new("identical content", finding_ids.clone(), None).unwrap();
        let second = Recommendation::new("identical content", finding_ids, None).unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn cloned_recommendation_is_equal_to_its_source() {
        let recommendation = Recommendation::new(
            "consider using recommended structure",
            sample_finding_ids(),
            None,
        )
        .unwrap();

        assert_eq!(recommendation.clone(), recommendation);
    }
}
