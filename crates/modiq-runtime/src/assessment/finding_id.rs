/// Unique identifier for a Finding, scoped to a single process.
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FindingId(u64);

impl FindingId {
    /// Generates a new, unique FindingId.
    pub fn generate() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_unique_ids() {
        let first = FindingId::generate();
        let second = FindingId::generate();

        assert_ne!(first, second);
    }
}
