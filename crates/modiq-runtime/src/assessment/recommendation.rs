/// Represents an actionable recommendation produced by an Assessment.
///
/// Recommendations are generated from engineering findings and
/// describe actions that improve the assessed subject.
///
/// Recommendations do not evaluate evidence directly and are
/// never produced without supporting findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommendation;
