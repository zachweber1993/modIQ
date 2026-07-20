use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::input_descriptor::InputDescriptor;

/// Produces Evidence from an Input Descriptor (`EvidenceCollection.md`:
/// Collector Contract).
///
/// This minimal implementation proves the Evidence Collection boundary
/// established by ADR-0008 without implementing any real inspection:
/// it does not read a filesystem, does not traverse an archive, and
/// does not parse XML or Lua. It produces one deterministic Evidence
/// item reflecting the descriptor it was given, exactly as a real
/// Collector's output would be consumed by the rest of the pipeline.
pub struct EvidenceCollector;

impl EvidenceCollector {
    /// Collects Evidence for the given Input Descriptor.
    ///
    /// Always succeeds: the only currently representable failure mode
    /// (an empty descriptor) is already rejected earlier, at
    /// `InputDescriptor::new`. A real Collector performing actual
    /// I/O-bound inspection will need its own failure channel
    /// (`EvidenceCollection.md`: Determinism Expectations) —
    /// deliberately not introduced here, since GOV-010 (Collection
    /// Error Model) remains open and no real failure mode exists yet
    /// to design it against.
    pub fn collect(&self, descriptor: &InputDescriptor) -> Vec<Evidence> {
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            format!(
                "Evidence collected for input descriptor: {}",
                descriptor.value()
            ),
        )
        .expect("description is non-empty because descriptor.value() is non-empty");

        vec![evidence]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_descriptor() -> InputDescriptor {
        InputDescriptor::new("a/mod/path").expect("value is non-empty")
    }

    #[test]
    fn collect_produces_exactly_one_evidence_item() {
        let collector = EvidenceCollector;

        let evidence = collector.collect(&sample_descriptor());

        assert_eq!(evidence.len(), 1);
    }

    #[test]
    fn collect_produces_a_file_structure_analysis_category() {
        let collector = EvidenceCollector;

        let evidence = collector.collect(&sample_descriptor());

        assert_eq!(
            evidence[0].category(),
            EvidenceCategory::FileStructureAnalysis
        );
    }

    #[test]
    fn collect_description_reflects_the_descriptor_value() {
        let collector = EvidenceCollector;
        let descriptor = InputDescriptor::new("mods/example.zip").unwrap();

        let evidence = collector.collect(&descriptor);

        assert!(evidence[0].description().contains("mods/example.zip"));
    }

    #[test]
    fn collect_is_deterministic_for_identical_input() {
        let collector = EvidenceCollector;
        let descriptor = sample_descriptor();

        let first = collector.collect(&descriptor);
        let second = collector.collect(&descriptor);

        assert_eq!(first[0].category(), second[0].category());
        assert_eq!(first[0].description(), second[0].description());
        // Each collection freshly assigns Evidence identity, mirroring
        // every other Runtime identity in the platform; determinism is
        // judged by content, not by incidental identity.
        assert_ne!(first[0].id(), second[0].id());
    }
}
