use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::archive_reader::ArchiveEntry;

/// Produces Evidence from archive entries already discovered by
/// `ArchiveReader`.
///
/// This is a transformation step only, not yet a Collector
/// (`EvidenceCollection.md`: Collector Contract): it does not detect
/// duplicate entries (GOV-011, Duplicate Archive Entry Policy), does
/// not filter traversal-unsafe entries (GOV-011, Archive Traversal
/// Boundary Policy), and does not enforce resource limits (GOV-011,
/// Question 3). Those responsibilities are layered on top of this
/// transformation in a later phase, per `SPRINT4_IMPLEMENTATION_PLAN.md`.
pub struct ArchiveEvidenceBuilder;

impl ArchiveEvidenceBuilder {
    /// Builds one Evidence item per archive entry, in the same order the
    /// entries are given. `ArchiveReader::entries()` already imposes
    /// deterministic (sorted) ordering; this method preserves it rather
    /// than re-deriving or altering it.
    ///
    /// Each Evidence item reuses `EvidenceCategory::FileStructureAnalysis`
    /// (`SPRINT4_IMPLEMENTATION_PLAN.md`: Approved Routing & Collector
    /// Shape) — an archive entry's existence, name, and kind is the same
    /// conceptual category of structural observation already established
    /// for the filesystem case, whether the structure being observed
    /// sits on disk directly or inside an archive.
    ///
    /// Only an entry's name (as the Evidence location) and whether it is
    /// a file or directory (as the Evidence description) are used.
    /// `ArchiveEntry::size` is not read here — `Evidence` has no field
    /// for it — and no other archive metadata (timestamps, permissions,
    /// comments) is available on `ArchiveEntry` to read in the first
    /// place, satisfying the Archive Metadata Policy (GOV-011,
    /// `EvidenceCollection.md`) structurally rather than by convention.
    pub fn build(&self, entries: &[ArchiveEntry]) -> Vec<Evidence> {
        entries.iter().map(Self::evidence_for_entry).collect()
    }

    fn evidence_for_entry(entry: &ArchiveEntry) -> Evidence {
        let description = if entry.is_dir() {
            "Directory discovered during archive collection."
        } else {
            "File discovered during archive collection."
        };

        Evidence::with_location(
            EvidenceCategory::FileStructureAnalysis,
            description,
            entry.name(),
        )
        .expect("description and location are non-empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_produces_no_evidence_for_no_entries() {
        let evidence = ArchiveEvidenceBuilder.build(&[]);

        assert!(evidence.is_empty());
    }

    #[test]
    fn build_produces_one_evidence_item_per_entry_in_order() {
        let entries = vec![
            ArchiveEntry::new("alpha.txt", 5, false),
            ArchiveEntry::new("nested/", 0, true),
            ArchiveEntry::new("nested/detail.txt", 6, false),
        ];

        let evidence = ArchiveEvidenceBuilder.build(&entries);

        assert_eq!(evidence.len(), 3);
        assert_eq!(evidence[0].location(), Some("alpha.txt"));
        assert_eq!(evidence[1].location(), Some("nested/"));
        assert_eq!(evidence[2].location(), Some("nested/detail.txt"));
    }

    #[test]
    fn build_produces_file_structure_analysis_category_for_every_entry() {
        let entries = vec![
            ArchiveEntry::new("file.txt", 1, false),
            ArchiveEntry::new("dir/", 0, true),
        ];

        let evidence = ArchiveEvidenceBuilder.build(&entries);

        for item in &evidence {
            assert_eq!(item.category(), EvidenceCategory::FileStructureAnalysis);
        }
    }

    #[test]
    fn build_distinguishes_files_from_directories_in_description() {
        let entries = vec![
            ArchiveEntry::new("file.txt", 1, false),
            ArchiveEntry::new("dir/", 0, true),
        ];

        let evidence = ArchiveEvidenceBuilder.build(&entries);

        assert_eq!(
            evidence[0].description(),
            "File discovered during archive collection."
        );
        assert_eq!(
            evidence[1].description(),
            "Directory discovered during archive collection."
        );
    }

    #[test]
    fn build_does_not_use_entry_size_or_any_excluded_metadata() {
        // ArchiveEntry exposes only name, size, and is_dir; size has no
        // corresponding Evidence field, and no timestamp, permission, or
        // comment field exists on ArchiveEntry at all (Archive Metadata
        // Policy, GOV-011). Two entries differing only in size must
        // produce identical Evidence content.
        let small = ArchiveEntry::new("same_name.txt", 1, false);
        let large = ArchiveEntry::new("same_name.txt", 1_000_000, false);

        let evidence_small = ArchiveEvidenceBuilder.build(&[small]);
        let evidence_large = ArchiveEvidenceBuilder.build(&[large]);

        assert_eq!(
            evidence_small[0].description(),
            evidence_large[0].description()
        );
        assert_eq!(evidence_small[0].location(), evidence_large[0].location());
        assert_eq!(evidence_small[0].category(), evidence_large[0].category());
    }

    #[test]
    fn build_is_deterministic_for_identical_input() {
        let entries = vec![
            ArchiveEntry::new("alpha.txt", 5, false),
            ArchiveEntry::new("nested/", 0, true),
            ArchiveEntry::new("nested/detail.txt", 6, false),
        ];

        let first = ArchiveEvidenceBuilder.build(&entries);
        let second = ArchiveEvidenceBuilder.build(&entries);

        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.category(), b.category());
            assert_eq!(a.description(), b.description());
            assert_eq!(a.location(), b.location());
        }
        // Each build freshly assigns Evidence identity, mirroring every
        // other Runtime identity in the platform; determinism is judged
        // by content and order, not by incidental identity.
        assert_ne!(first[0].id(), second[0].id());
    }
}
