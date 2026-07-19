/// The classification of a Finding's conclusion (Glossary.md: Finding).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingSeverity {
    Error,
    Warning,
    Informational,
    BestPractice,
}
