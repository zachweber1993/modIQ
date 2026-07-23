/// Opaque identifier for a persisted `AssessmentReport`, minted by
/// Storage itself.
///
/// `AssessmentReport`'s own Runtime identity (`AssessmentId`) has no
/// externally accessible representation and is not meaningful across
/// process boundaries — its generator restarts at 1 on every process
/// invocation (see
/// `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`).
/// Storage mints its own identifier instead, following this
/// platform's own convention that identity is freshly assigned per
/// construction, exactly as `AssessmentId`/`EvidenceId`/`FindingId`
/// already do.
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportKey(String);

impl ReportKey {
    /// Generates a new key, unique across process invocations.
    ///
    /// Combines a nanosecond timestamp (varies across process
    /// invocations) with an in-process sequence counter (guards
    /// against same-instant collisions within one process).
    pub fn generate() -> Self {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is after the Unix epoch")
            .as_nanos();
        Self(format!("{nanos}-{sequence}"))
    }

    /// Reconstructs a key from its own previously-obtained raw value.
    ///
    /// This value is opaque and carries no meaning beyond identifying
    /// a specific stored report; it is not, and must never be
    /// confused with, an `AssessmentId`.
    pub fn from_raw(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_distinct_keys() {
        let first = ReportKey::generate();
        let second = ReportKey::generate();

        assert_ne!(first, second);
    }

    #[test]
    fn from_raw_preserves_the_value() {
        let key = ReportKey::from_raw("some-previously-generated-value");

        assert_eq!(key.value(), "some-previously-generated-value");
    }

    #[test]
    fn from_raw_round_trips_a_generated_key() {
        let original = ReportKey::generate();

        let reconstructed = ReportKey::from_raw(original.value());

        assert_eq!(reconstructed, original);
    }
}
