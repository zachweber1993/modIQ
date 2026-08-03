/// One dimension of a mod's overall Mod Health (`Glossary.md`: Mod
/// Health).
///
/// A Finding's own quality axis — which aspect of the mod it concerns —
/// orthogonal to `FindingSeverity`, which expresses how urgent a
/// Finding is. Named `ModHealthDimension`, not `Category`, to avoid
/// colliding with `EvidenceCategory`'s existing meaning (Initiative 3,
/// Item 9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModHealthDimension {
    Compatibility,
    Stability,
    Maintainability,
    Performance,
    Structure,
    EngineeringQuality,
}
