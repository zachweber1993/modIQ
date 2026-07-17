/// Lifecycle states of an Assessment aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssessmentStatus {
    /// The initial state of every Assessment (RuntimeInvariants.md INV-001).
    Created,
    /// Evidence collection is in progress (RuntimeInvariants.md INV-002).
    CollectingEvidence,
    /// Rule evaluation is in progress; Evidence is now immutable (RuntimeInvariants.md INV-003).
    EvaluatingRules,
    /// The Assessment has concluded (RuntimeInvariants.md INV-012).
    Completed,
}
