/// The closed set of objective evidence categories recognized by the
/// platform (Glossary.md: Evidence).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceCategory {
    XmlInspection,
    LuaAnalysis,
    RuntimeLogs,
    AssetValidation,
    DependencyResolution,
    PerformanceObservations,
    FileStructureAnalysis,

    /// A collection mechanism observed that its own inspection could
    /// not fully resolve every item in the assessment subject's
    /// structure — for example, an archive containing more than one
    /// entry under the same name (GOV-011, Duplicate Archive Entry
    /// Policy). Deliberately named for the kind of fact observed
    /// (a structural naming collision), not for the collection
    /// mechanism that happened to observe it, so the same category
    /// applies to any future collector that can encounter an
    /// analogous ambiguity.
    StructuralDuplication,
}
