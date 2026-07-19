/// Unique identifier for a Recommendation, scoped to a single process.
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecommendationId(u64);

impl RecommendationId {
    /// Generates a new, unique RecommendationId.
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
        let first = RecommendationId::generate();
        let second = RecommendationId::generate();

        assert_ne!(first, second);
    }
}
