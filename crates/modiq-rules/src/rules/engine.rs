use modiq_runtime::assessment::{Evidence, Finding, Recommendation};

use super::evidence_presence_rule::EvidencePresenceRule;
use super::structural_duplication_rule::StructuralDuplicationRule;

/// The result of evaluating Evidence against a single deterministic Rule:
/// one Finding and one Recommendation derived from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOutcome {
    pub finding: Finding,
    pub recommendation: Recommendation,
}

/// Coordinates deterministic rule execution.
pub struct RuleEngine;

impl RuleEngine {
    /// Evaluates the given Evidence against every concrete Rule this
    /// platform has (RuleEngine.md: Rule Selection, Evidence
    /// Evaluation, Finding Generation, Recommendation Generation),
    /// returning zero, one, or several outcomes — one per Rule that
    /// matched (GOV-012, Question 1).
    ///
    /// Rules are evaluated in a fixed, explicit declaration order —
    /// `EvidencePresenceRule`, then `StructuralDuplicationRule` — never
    /// an order derived from Evidence's own arrival sequence (GOV-012,
    /// Question 2). Rules compose independently: each is evaluated
    /// against the full Evidence set regardless of whether another
    /// Rule also matches it, and no Rule suppresses another (GOV-012,
    /// Question 3). This is deliberately a fixed sequence of `if let`
    /// checks, not a trait, registry, or dispatch table — an
    /// implementation detail GOV-012 leaves open, provided no such
    /// abstraction is introduced (`GOVERNANCE.md`: Crate Boundary
    /// Rules, ADR-0010, GOV-004).
    pub fn evaluate(&self, evidence: &[Evidence]) -> Vec<RuleOutcome> {
        let mut outcomes = Vec::new();

        if let Some(outcome) = EvidencePresenceRule.evaluate(evidence) {
            outcomes.push(outcome);
        }
        if let Some(outcome) = StructuralDuplicationRule.evaluate(evidence) {
            outcomes.push(outcome);
        }

        outcomes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::{EvidenceCategory, FindingSeverity};

    fn structural_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    fn duplication_evidence() -> Evidence {
        Evidence::new(
            EvidenceCategory::StructuralDuplication,
            "duplicate entry names detected",
        )
        .expect("category and description are valid")
    }

    #[test]
    fn evaluate_returns_no_outcomes_for_no_evidence() {
        let engine = RuleEngine;

        assert_eq!(engine.evaluate(&[]), vec![]);
    }

    #[test]
    fn evaluate_returns_one_outcome_when_only_the_generic_rule_matches() {
        let engine = RuleEngine;

        let outcomes = engine.evaluate(&[structural_evidence()]);

        assert_eq!(outcomes.len(), 1);
        assert_eq!(
            outcomes[0].finding.rule_reference().identifier(),
            "evidence-presence-rule"
        );
        assert_eq!(
            outcomes[0].finding.severity(),
            FindingSeverity::Informational
        );
    }

    #[test]
    fn evaluate_returns_both_outcomes_when_both_rules_match_in_declaration_order() {
        let engine = RuleEngine;

        let outcomes = engine.evaluate(&[duplication_evidence()]);

        // duplication_evidence() is non-empty, so EvidencePresenceRule
        // matches unconditionally; it is also StructuralDuplication
        // category, so StructuralDuplicationRule matches too. Neither
        // Rule suppresses the other (GOV-012, Question 3).
        assert_eq!(outcomes.len(), 2);
        assert_eq!(
            outcomes[0].finding.rule_reference().identifier(),
            "evidence-presence-rule"
        );
        assert_eq!(
            outcomes[1].finding.rule_reference().identifier(),
            "structural-duplication-rule"
        );
        assert_eq!(
            outcomes[0].finding.severity(),
            FindingSeverity::Informational
        );
        assert_eq!(outcomes[1].finding.severity(), FindingSeverity::Warning);
    }

    #[test]
    fn evaluate_still_produces_exactly_two_outcomes_for_multiple_matching_items() {
        // Each Rule still produces at most one outcome regardless of
        // how many Evidence items it matches — EvidencePresenceRule
        // does not multiply per item, and neither does
        // StructuralDuplicationRule; it references every matching item
        // within its own single Finding instead (confirmed directly in
        // structural_duplication_rule.rs's own tests).
        let engine = RuleEngine;

        let outcomes = engine.evaluate(&[duplication_evidence(), duplication_evidence()]);

        assert_eq!(outcomes.len(), 2);
        assert_eq!(outcomes[1].finding.evidence_ids().len(), 2);
    }

    #[test]
    fn evaluate_outcome_order_is_independent_of_evidence_arrival_order() {
        // GOV-012 (Question 2) requires Rule declaration order, never
        // an order derived from Evidence's own arrival sequence. Both
        // orderings of the same two Evidence items must still produce
        // outcomes in the same Rule order: EvidencePresenceRule, then
        // StructuralDuplicationRule.
        let engine = RuleEngine;

        let structural_first = engine.evaluate(&[structural_evidence(), duplication_evidence()]);
        let duplication_first = engine.evaluate(&[duplication_evidence(), structural_evidence()]);

        for outcomes in [&structural_first, &duplication_first] {
            assert_eq!(outcomes.len(), 2);
            assert_eq!(
                outcomes[0].finding.rule_reference().identifier(),
                "evidence-presence-rule"
            );
            assert_eq!(
                outcomes[1].finding.rule_reference().identifier(),
                "structural-duplication-rule"
            );
        }
    }

    #[test]
    fn evaluate_ordering_is_deterministic_across_repeated_calls() {
        let engine = RuleEngine;
        let evidence = [duplication_evidence()];

        let first = engine.evaluate(&evidence);
        let second = engine.evaluate(&evidence);

        let first_references: Vec<&str> = first
            .iter()
            .map(|outcome| outcome.finding.rule_reference().identifier())
            .collect();
        let second_references: Vec<&str> = second
            .iter()
            .map(|outcome| outcome.finding.rule_reference().identifier())
            .collect();
        assert_eq!(first_references, second_references);
        assert_eq!(first.len(), second.len());
    }
}
