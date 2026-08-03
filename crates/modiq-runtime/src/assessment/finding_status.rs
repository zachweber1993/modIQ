/// Whether a Finding's content is still forming or permanently fixed
/// (Initiative 3, Item 10).
///
/// Populated once, at construction, like every other `Finding` field
/// (ADR-0007) — no mutation method exists to transition a Finding from
/// one value to the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingStatus {
    Provisional,
    Final,
}
