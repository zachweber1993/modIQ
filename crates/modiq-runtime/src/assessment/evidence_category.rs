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
}
