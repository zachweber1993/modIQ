pub mod engine;
pub mod evidence_presence_rule;
pub mod structural_duplication_rule;

pub use engine::{RuleEngine, RuleOutcome};
pub use evidence_presence_rule::EvidencePresenceRule;
pub use structural_duplication_rule::StructuralDuplicationRule;
