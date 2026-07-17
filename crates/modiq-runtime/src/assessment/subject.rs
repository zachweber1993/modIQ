/// Represents the artifact being evaluated by an Assessment.
///
/// The subject identifies *what* is being assessed.
/// It contains identifying information about the target,
/// but no assessment state or results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentSubject;
