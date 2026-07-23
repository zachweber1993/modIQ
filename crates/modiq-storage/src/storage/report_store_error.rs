#[derive(Debug, thiserror::Error)]
pub enum ReportStoreError {
    #[error("failed to write the persisted report: {0}")]
    Write(#[source] std::io::Error),

    #[error("failed to read the persisted report: {0}")]
    Read(#[source] std::io::Error),

    #[error("failed to serialize the persisted report: {0}")]
    Serialize(#[source] serde_json::Error),

    #[error("failed to deserialize the persisted report: {0}")]
    Deserialize(#[source] serde_json::Error),

    #[error("no report is stored under the given key")]
    NotFound,
}
