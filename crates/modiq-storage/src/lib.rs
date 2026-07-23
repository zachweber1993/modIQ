//! Storage — persistence for AssessmentReport.
//!
//! Storage defines and owns its own representation of a report's
//! content; it does not persist or reconstruct `modiq_report`'s own
//! `AssessmentReport` type or any Runtime entity's identity. See
//! `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`.

pub mod storage;
