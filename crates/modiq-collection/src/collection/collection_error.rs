/// Errors produced when a Collector cannot complete collection for a
/// well-formed AssessmentInput.
///
/// Distinct from `AssessmentInputError`: that type represents the
/// "Invalid Input" outcome (a malformed AssessmentInput; collection
/// never begins). This type represents the two outcomes where
/// collection is attempted but aborts (GOV-010, `EvidenceCollection.md`
/// Collection Outcomes). Neither variant is produced for the "Empty
/// Collection" outcome, which is not an error — it is `Ok(vec![])`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CollectionError {
    /// The AssessmentInput is well-formed, but the location it names
    /// cannot be reached: it does not exist, access is denied, or it
    /// sits on storage that is not currently available. Collection
    /// aborts.
    #[error("assessment input is not accessible: {path}")]
    Inaccessible { path: String },

    /// The location is reachable but is not a supported kind for this
    /// Collector (for example, a symbolic link, which Phase 5
    /// intentionally does not traverse; or a device file, pipe, or
    /// socket). Collection aborts.
    #[error("assessment input is not a supported kind of location: {path}")]
    Unsupported { path: String },
}
