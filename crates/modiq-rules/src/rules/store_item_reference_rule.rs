use modiq_runtime::assessment::{
    Evidence, EvidenceCategory, EvidenceId, Finding, FindingSeverity, FindingStatus,
    ModHealthDimension, RuleReference,
};

use super::engine::RuleOutcome;

/// The exact prefix `XmlCollector` (`modiq-collection`,
/// `xml_collector.rs`) uses when reporting a `storeItem` declaration
/// carrying a non-empty `xmlFilename`. This Rule and that Collector
/// remain architecturally independent (`EvidenceCollection.md`:
/// Collector Contract) — this is a data-format convention between them,
/// not a code dependency; duplicated here deliberately rather than
/// shared through a new crate dependency, mirroring
/// `VersionCompatibilityRule`'s own `DECLARED_DESC_VERSION_PREFIX` and
/// `DeclaredDependencyDuplicationRule`'s own `DECLARED_DEPENDENCY_PREFIX`
/// precedents.
///
/// A declaration whose `xmlFilename` is absent, empty, or
/// whitespace-only carries a different description by construction, so
/// neither degenerate form reaches this Rule's own content-shape filter
/// (`ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md` §5;
/// Implementation Authorization, Q1).
const DECLARED_STORE_ITEM_PREFIX: &str = "modDesc.xml declares store item: ";

/// The exact description `ArchiveEvidenceBuilder` (`modiq-collection`,
/// `archive_evidence.rs`) uses for one discovered archive **file**
/// entry. The same established description convention distinguishes
/// both the entry's kind (a file, never a directory) and its provenance
/// (archive collection, never filesystem collection) in one already-
/// existing field — no new Evidence field, and no new source/label
/// combination, is made load-bearing by this Rule (Implementation
/// Authorization, Q2).
const ARCHIVE_FILE_ENTRY_DESCRIPTION: &str = "File discovered during archive collection.";

/// The Store Item Reference Integrity Rule
/// (`ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md`).
///
/// Evaluates whether each ordinary, mod-root-relative path declared by
/// a `storeItem` entry in a mod's root `modDesc.xml` matches a file
/// entry collected from the same assessed ZIP archive. Reports only
/// what the collected inventory establishes: that a declared path had
/// no matching collected file entry. It establishes nothing about
/// runtime loadability or functional correctness (Resolution §5 A5,
/// §6 B2, §9).
///
/// **ZIP archive assessments only** (Resolution §5 A1/A2). Archive
/// provenance is established from the presence of at least one archive
/// file entry in the Evidence set — never from whether a declared
/// target itself resolves — so a ZIP carrying a valid manifest and a
/// genuinely missing target is still evaluated, while a
/// filesystem-directory assessment produces nothing at all.
///
/// **This is the first Rule in this crate to read Evidence from two
/// categories** (`XmlInspection` for declarations,
/// `FileStructureAnalysis` for archive entries) and the first to read
/// `Evidence::content()`. Both are permitted by the already-adopted
/// dispatch model: `RuleEngine::evaluate` passes the full, unpartitioned
/// Evidence set to every Rule, and the dispatch mechanism carries no
/// concept of category ownership (GOV-012, Question 3, as interpreted
/// and adopted by `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`
/// Decision 1).
///
/// **C2 Rule Conclusion Non-Contradiction Constraint — discharged by
/// complete disjointness.** This Rule is the third interpreter of
/// `XmlInspection` Evidence. Its content-shape filter
/// (`DECLARED_STORE_ITEM_PREFIX`) is lexically disjoint from
/// `VersionCompatibilityRule`'s (`"modDesc.xml declares descVersion: "`)
/// and `DeclaredDependencyDuplicationRule`'s (`"modDesc.xml declares
/// dependency: "`) — the three diverge at the first character after
/// `declares `, and no prefix is a prefix of another. No single Evidence
/// item can therefore satisfy more than one of the three filters, so no
/// item can support two Rules' conclusions at all, contradictory or
/// otherwise — the strongest available form of discharge. Verified
/// directly against both sibling Rules' own constants at implementation
/// time.
///
/// **`StructuralDuplication` accounting** (Resolution §6 B3). This Rule
/// performs a membership test, never a count. Duplicate archive entry
/// names are collapsed by the central directory to a single observable
/// entry *before* any Evidence is produced (`ArchiveCollector`), which
/// degrades count fidelity while preserving membership exactly: the
/// name still appears in the inventory once. A collapsed duplicate can
/// therefore neither hide a declared path nor fabricate one, and this
/// Rule consumes no `StructuralDuplication` Evidence and is unaffected
/// by its presence. That item is accounted for here, explicitly, rather
/// than silently absorbed as an ordinary structural item — the
/// distinction the Duplicate Archive Entry Policy (GOV-011) requires a
/// structurally-reasoning Rule to observe. No runtime mechanism is
/// introduced to do so.
///
/// Assigned `ModHealthDimension::Structure` (Resolution §9): the fact
/// being judged is a defect in the mod's own declaration-to-content
/// structure, not a compatibility relationship with a game version.
pub struct StoreItemReferenceRule;

impl StoreItemReferenceRule {
    /// Evaluates each supported declared store-item path against the
    /// collected archive file entries.
    ///
    /// Returns one `RuleOutcome` per unmatched supported declaration,
    /// each citing exactly that one declaration's own Evidence
    /// (Implementation Authorization, Q5) — and an empty result when the
    /// assessment is not a ZIP archive assessment, when no supported
    /// declaration exists, or when every supported declaration matched.
    ///
    /// Assigns `FindingSeverity::Warning` (`DataModel.md`: Finding
    /// Severity), not `Error`: a declared path with no matching
    /// collected entry is a genuine, Evidence-supported structural
    /// concern, but the collected archive inventory alone cannot
    /// conclusively establish that the mod will not function.
    pub fn evaluate(&self, evidence: &[Evidence]) -> Vec<RuleOutcome> {
        let archive_file_entries: Vec<&str> = evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::FileStructureAnalysis)
            .filter(|item| item.description() == ARCHIVE_FILE_ENTRY_DESCRIPTION)
            .filter_map(Evidence::location)
            .collect();

        // The ZIP-only gate, established from archive provenance alone
        // and entirely independently of whether any declared target
        // resolves. A filesystem-directory assessment produces no
        // archive file entry and is therefore never evaluated; a ZIP
        // assessment always produces at least one (its own manifest
        // entry, without which no declaration could have been collected
        // in the first place), so a missing declared target is still
        // evaluated.
        if archive_file_entries.is_empty() {
            return Vec::new();
        }

        evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::XmlInspection)
            .filter(|item| item.description().starts_with(DECLARED_STORE_ITEM_PREFIX))
            .filter_map(|item| item.content().map(|path| (item.id(), path)))
            .filter(|(_, path)| Self::is_supported_declared_path(path))
            .filter(|(_, path)| !archive_file_entries.contains(path))
            .map(|(declaration_id, path)| Self::outcome_for(declaration_id, path))
            .collect()
    }

    /// Whether a declared path is one of the ordinary,
    /// mod-root-relative forms this capability supports.
    ///
    /// Every excluded form is skipped entirely — never normalized,
    /// never silently reinterpreted, and never reported as missing
    /// (Resolution §7 C4/C5). An excluded declaration remains fully
    /// traceable as declaration Evidence; it simply yields no conclusion
    /// from this Rule.
    fn is_supported_declared_path(path: &str) -> bool {
        !Self::is_absolute(path)
            && !path.starts_with('$')
            && !path.contains('\\')
            && !Self::contains_dot_segment(path)
    }

    /// A leading separator, or a Windows-style drive-qualified prefix,
    /// makes a declared path absolute rather than mod-root-relative.
    /// Mirrors the shape of `ArchiveCollector`'s own absolute-path
    /// determination, applied here to a declared path rather than to an
    /// archive entry name.
    fn is_absolute(path: &str) -> bool {
        if path.starts_with('/') || path.starts_with('\\') {
            return true;
        }

        let bytes = path.as_bytes();
        bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
    }

    /// Whether any `/`-delimited component is a `.` or `..` segment.
    /// `/` is the only recognized separator (Resolution §7 C3).
    fn contains_dot_segment(path: &str) -> bool {
        path.split('/')
            .any(|component| component == "." || component == "..")
    }

    /// Builds the single Finding for one unmatched declaration. The
    /// summary names the exact declared path and is bounded to the
    /// collected inventory: it states that no matching file entry was
    /// collected, never that the mod is broken, will fail to load, or
    /// carries any runtime consequence (Resolution §6 B2, §9).
    fn outcome_for(declaration_id: EvidenceId, path: &str) -> RuleOutcome {
        let finding = Finding::new(
            FindingSeverity::Warning,
            "Declared store item not found in archive",
            format!(
                "modDesc.xml declares a store item at `{path}`, but no file entry matching that \
                 path was collected from the assessed archive."
            ),
            ModHealthDimension::Structure,
            FindingStatus::Final,
            vec![declaration_id],
            RuleReference::new("store-item-reference-rule"),
        )
        .expect("severity, title, summary, evidence_ids, and rule reference are valid");

        RuleOutcome {
            finding,
            recommendation: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declared_store_item(path: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            format!("{DECLARED_STORE_ITEM_PREFIX}{path}"),
            "modDesc.xml",
            Some("Declared Store Item".to_string()),
            Some("modDesc.xml".to_string()),
            Some(path.to_string()),
        )
        .expect("description and location are valid")
    }

    fn archive_file_entry(name: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::FileStructureAnalysis,
            ARCHIVE_FILE_ENTRY_DESCRIPTION,
            name,
            Some("File Discovered".to_string()),
            Some("Archive Collection".to_string()),
            None,
        )
        .expect("description and location are valid")
    }

    fn archive_directory_entry(name: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::FileStructureAnalysis,
            "Directory discovered during archive collection.",
            name,
            Some("Directory Discovered".to_string()),
            Some("Archive Collection".to_string()),
            None,
        )
        .expect("description and location are valid")
    }

    fn filesystem_file_entry(name: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::FileStructureAnalysis,
            "File discovered during filesystem collection.",
            name,
            Some("File Discovered".to_string()),
            Some("Filesystem Collection".to_string()),
            None,
        )
        .expect("description and location are valid")
    }

    fn structural_duplication() -> Evidence {
        Evidence::new(
            EvidenceCategory::StructuralDuplication,
            "Archive collection detected 1 duplicate entry name(s), which its inspection \
             mechanism could not fully resolve: objects/first.xml.",
            Some("Duplicate Archive Entries".to_string()),
            Some("Archive Collection".to_string()),
            None,
        )
        .expect("description is valid")
    }

    #[test]
    fn returns_nothing_for_empty_evidence() {
        assert!(StoreItemReferenceRule.evaluate(&[]).is_empty());
    }

    #[test]
    fn returns_nothing_when_a_declared_store_item_is_present_in_the_archive() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            archive_file_entry("objects/first.xml"),
            declared_store_item("objects/first.xml"),
        ];

        assert!(StoreItemReferenceRule.evaluate(&evidence).is_empty());
    }

    #[test]
    fn reports_one_finding_when_a_declared_store_item_is_absent_from_the_archive() {
        let declaration = declared_store_item("objects/missing.xml");
        let declaration_id = declaration.id();
        let evidence = vec![archive_file_entry("modDesc.xml"), declaration];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);

        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].finding.severity(), FindingSeverity::Warning);
        assert_eq!(
            outcomes[0].finding.mod_health_dimension(),
            ModHealthDimension::Structure
        );
        assert_eq!(outcomes[0].finding.status(), FindingStatus::Final);
        assert!(
            outcomes[0]
                .finding
                .summary()
                .contains("objects/missing.xml")
        );
        // Exactly one declaration Evidence item is cited.
        assert_eq!(outcomes[0].finding.evidence_ids(), &[declaration_id]);
    }

    #[test]
    fn requires_no_recommendation() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/missing.xml"),
        ];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);

        assert!(outcomes[0].recommendation.is_none());
    }

    #[test]
    fn makes_no_runtime_or_functional_claim() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/missing.xml"),
        ];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);
        let summary = outcomes[0].finding.summary().to_ascii_lowercase();

        for forbidden in ["will not load", "broken", "crash", "fails to", "unusable"] {
            assert!(
                !summary.contains(forbidden),
                "summary must not claim a runtime consequence: found `{forbidden}`"
            );
        }
    }

    #[test]
    fn reports_one_finding_per_unmatched_declaration_independently() {
        let present = declared_store_item("objects/present.xml");
        let first_missing = declared_store_item("objects/first-missing.xml");
        let second_missing = declared_store_item("objects/second-missing.xml");
        let first_missing_id = first_missing.id();
        let second_missing_id = second_missing.id();

        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            archive_file_entry("objects/present.xml"),
            present,
            first_missing,
            second_missing,
        ];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);

        assert_eq!(outcomes.len(), 2);
        assert_eq!(outcomes[0].finding.evidence_ids(), &[first_missing_id]);
        assert_eq!(outcomes[1].finding.evidence_ids(), &[second_missing_id]);
    }

    #[test]
    fn a_directory_entry_does_not_satisfy_a_file_declaration() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            archive_directory_entry("objects/first.xml"),
            declared_store_item("objects/first.xml"),
        ];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);

        assert_eq!(outcomes.len(), 1);
    }

    #[test]
    fn comparison_is_case_sensitive() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            archive_file_entry("Objects/First.xml"),
            declared_store_item("objects/first.xml"),
        ];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);

        assert_eq!(
            outcomes.len(),
            1,
            "a case-only difference must not satisfy a declaration"
        );
    }

    #[test]
    fn excludes_absolute_and_leading_separator_declarations() {
        for path in [
            "/objects/first.xml",
            "\\objects\\first.xml",
            "C:/objects/first.xml",
        ] {
            let evidence = vec![archive_file_entry("modDesc.xml"), declared_store_item(path)];

            assert!(
                StoreItemReferenceRule.evaluate(&evidence).is_empty(),
                "`{path}` must produce no missing-file conclusion"
            );
        }
    }

    #[test]
    fn excludes_dollar_prefixed_declarations() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("$data/objects/first.xml"),
        ];

        assert!(StoreItemReferenceRule.evaluate(&evidence).is_empty());
    }

    #[test]
    fn excludes_backslash_containing_declarations() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects\\first.xml"),
        ];

        assert!(StoreItemReferenceRule.evaluate(&evidence).is_empty());
    }

    #[test]
    fn excludes_declarations_containing_dot_segments() {
        for path in [
            "./objects/first.xml",
            "objects/../first.xml",
            "objects/./first.xml",
            "../first.xml",
        ] {
            let evidence = vec![archive_file_entry("modDesc.xml"), declared_store_item(path)];

            assert!(
                StoreItemReferenceRule.evaluate(&evidence).is_empty(),
                "`{path}` must produce no missing-file conclusion"
            );
        }
    }

    #[test]
    fn a_dot_bearing_filename_that_is_not_a_dot_segment_remains_supported() {
        // `.` only excludes a whole path *segment*; an ordinary
        // filename containing dots is unaffected.
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/my.store.item.xml"),
        ];

        assert_eq!(StoreItemReferenceRule.evaluate(&evidence).len(), 1);
    }

    #[test]
    fn structural_duplication_evidence_does_not_change_membership_semantics() {
        let present = vec![
            archive_file_entry("modDesc.xml"),
            archive_file_entry("objects/first.xml"),
            declared_store_item("objects/first.xml"),
        ];
        let present_with_duplication = vec![
            archive_file_entry("modDesc.xml"),
            archive_file_entry("objects/first.xml"),
            structural_duplication(),
            declared_store_item("objects/first.xml"),
        ];
        let missing = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/first.xml"),
        ];
        let missing_with_duplication = vec![
            archive_file_entry("modDesc.xml"),
            structural_duplication(),
            declared_store_item("objects/first.xml"),
        ];

        assert_eq!(
            StoreItemReferenceRule.evaluate(&present).len(),
            StoreItemReferenceRule
                .evaluate(&present_with_duplication)
                .len(),
            "duplication must not change a matched outcome"
        );
        assert_eq!(
            StoreItemReferenceRule.evaluate(&missing).len(),
            StoreItemReferenceRule
                .evaluate(&missing_with_duplication)
                .len(),
            "duplication must not change an unmatched outcome"
        );
    }

    #[test]
    fn ignores_a_filesystem_assessment_entirely() {
        let evidence = vec![
            filesystem_file_entry("modDesc.xml"),
            declared_store_item("objects/missing.xml"),
        ];

        assert!(
            StoreItemReferenceRule.evaluate(&evidence).is_empty(),
            "a filesystem-directory assessment must produce no missing-reference Finding"
        );
    }

    #[test]
    fn archive_provenance_is_established_independently_of_the_declared_target() {
        // The only archive file entry is the manifest itself; the
        // declared target is genuinely absent. The assessment must
        // still be evaluated.
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/missing.xml"),
        ];

        assert_eq!(StoreItemReferenceRule.evaluate(&evidence).len(), 1);
    }

    #[test]
    fn ignores_a_declaration_carrying_no_content() {
        // A degenerate declaration (absent `xmlFilename`) never carries
        // the declaration prefix, but this Rule is defensive about
        // content regardless.
        let degenerate = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "modDesc.xml declares a store item with no xmlFilename attribute.",
            "modDesc.xml",
            Some("Declared Store Item".to_string()),
            Some("modDesc.xml".to_string()),
            None,
        )
        .expect("description and location are valid");
        let evidence = vec![archive_file_entry("modDesc.xml"), degenerate];

        assert!(StoreItemReferenceRule.evaluate(&evidence).is_empty());
    }

    #[test]
    fn ignores_other_xml_inspection_evidence() {
        let other = Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "modDesc.xml declares dependency: FS25_exampleMod",
            "modDesc.xml",
            Some("Declared Dependency".to_string()),
            Some("modDesc.xml".to_string()),
            Some("FS25_exampleMod".to_string()),
        )
        .expect("description and location are valid");
        let evidence = vec![archive_file_entry("modDesc.xml"), other];

        assert!(StoreItemReferenceRule.evaluate(&evidence).is_empty());
    }

    #[test]
    fn is_deterministic_across_repeated_evaluations() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/alpha.xml"),
            declared_store_item("objects/beta.xml"),
        ];

        let first = StoreItemReferenceRule.evaluate(&evidence);
        let second = StoreItemReferenceRule.evaluate(&evidence);

        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.finding.summary(), b.finding.summary());
            assert_eq!(a.finding.evidence_ids(), b.finding.evidence_ids());
            assert_eq!(a.finding.severity(), b.finding.severity());
        }
    }

    #[test]
    fn declaration_order_is_preserved_in_the_reported_findings() {
        let evidence = vec![
            archive_file_entry("modDesc.xml"),
            declared_store_item("objects/zebra.xml"),
            declared_store_item("objects/alpha.xml"),
        ];

        let outcomes = StoreItemReferenceRule.evaluate(&evidence);

        assert!(outcomes[0].finding.summary().contains("objects/zebra.xml"));
        assert!(outcomes[1].finding.summary().contains("objects/alpha.xml"));
    }
}
