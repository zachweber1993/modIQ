use std::fs;
use std::io::Read;
use std::path::Path;

use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::assessment_input::AssessmentInput;
use super::collection_error::CollectionError;

/// The manifest file name every loadable Farming Simulator mod is
/// expected to carry at its root.
const MANIFEST_FILE_NAME: &str = "modDesc.xml";

/// The exact prefix used when reporting a mod's declared `descVersion`
/// attribute as Evidence (Sprint 8: Version Profile-aware compatibility
/// checking). `modiq-rules`'s `VersionCompatibilityRule` parses this
/// same, independently-duplicated prefix to recognize and evaluate
/// this specific kind of `XmlInspection` fact — a data-format
/// convention between the two crates, not a code dependency
/// (`EvidenceCollection.md`: Collector Contract, "a Collector receives
/// nothing else"; `modiq-collection` and `modiq-rules` remain
/// architecturally independent).
const DECLARED_DESC_VERSION_PREFIX: &str = "modDesc.xml declares descVersion: ";

/// The exact prefix used when reporting a `storeItem` declaration's own
/// `xmlFilename` attribute as Evidence (Store Item Reference Integrity,
/// `ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md` §5).
/// `modiq-rules`'s `StoreItemReferenceRule` recognizes this same,
/// independently-duplicated prefix — a data-format convention between
/// the two crates, not a code dependency, mirroring
/// `DECLARED_DESC_VERSION_PREFIX`'s own established precedent.
///
/// Only a declaration carrying a non-empty path uses this prefix. A
/// declaration whose `xmlFilename` is absent or empty is still recorded
/// as Evidence, under its own distinct description, so that neither
/// degenerate form is recognized by that Rule's own content-shape
/// filter.
const DECLARED_STORE_ITEM_PREFIX: &str = "modDesc.xml declares store item: ";

/// Produces `XmlInspection` Evidence from an Assessment Input's
/// manifest file (`modDesc.xml`) — the platform's first
/// content-inspecting Collector, and the first implementation of
/// Multi-Source Evidence Collection
/// (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`).
///
/// Runs independently of, and alongside, whichever structural
/// Collector (`EvidenceCollector`, `ArchiveCollector`) also inspects
/// the same Assessment Input. Determines entirely on its own whether a
/// file or archive entry named exactly `modDesc.xml` exists at the
/// input's root — never by consuming another Collector's output
/// (`EvidenceCollection.md`: Collector Contract, Inputs: "A Collector
/// receives nothing else"). A missing manifest is itself recorded as
/// Evidence, not silently treated as Empty Collection (Sprint 7
/// authorization).
pub struct XmlCollector;

impl XmlCollector {
    /// Collects Evidence for the given Assessment Input.
    ///
    /// Always resolves to `Ok`, never `Empty Collection`: the manifest
    /// is either found and well-formed (one confirmation item plus one
    /// item per declared dependency), found but not well-formed (one
    /// item), or absent (one item). A malformed or unreadable manifest
    /// is a factual observation about the assessment subject, not a
    /// Collector failure — this Collector never aborts because of the
    /// manifest's own content. A genuine `CollectionError` is reserved
    /// for the Assessment Input's own root being unreachable at all —
    /// the same failure the structural Collector running alongside
    /// this one would independently encounter.
    pub fn collect(&self, input: &AssessmentInput) -> Result<Vec<Evidence>, CollectionError> {
        let value = input.value();

        let content = if Self::is_archive_location(value) {
            Self::read_from_archive(value)?
        } else {
            Self::read_from_filesystem(value)
        };

        Ok(match content {
            Some(bytes) => Self::inspect(&bytes),
            None => vec![Self::absent_manifest_evidence()],
        })
    }

    /// The same `.zip`-suffix, case-insensitive check
    /// `AssessmentService` uses to route the structural Collectors
    /// (`SPRINT4_IMPLEMENTATION_PLAN.md`: Approved Routing & Collector
    /// Shape) — duplicated deliberately, not shared, since each
    /// Collector determines applicability independently
    /// (`EvidenceCollection.md`: Collector Contract).
    fn is_archive_location(value: &str) -> bool {
        value.to_ascii_lowercase().ends_with(".zip")
    }

    /// Reads the manifest's raw bytes from a filesystem location: the
    /// input itself, if it is a file named `modDesc.xml`, or a direct
    /// child of the input, if the input is a directory. Any failure to
    /// locate or read the file is treated as the manifest being
    /// absent, not as a Collector failure — the Assessment Input's own
    /// accessibility is the structural Collector's concern, verified
    /// independently of this Collector.
    fn read_from_filesystem(value: &str) -> Option<Vec<u8>> {
        let root = Path::new(value);

        let manifest_path = if root.is_dir() {
            root.join(MANIFEST_FILE_NAME)
        } else if root
            .file_name()
            .is_some_and(|name| name == MANIFEST_FILE_NAME)
        {
            root.to_path_buf()
        } else {
            return None;
        };

        fs::read(&manifest_path).ok()
    }

    /// Reads the manifest's raw bytes from an archive entry named
    /// exactly `modDesc.xml` at the archive's root. Returns a genuine
    /// `CollectionError` only when the archive location itself cannot
    /// be opened or parsed at all — the same failure the structural
    /// `ArchiveCollector` running alongside this one independently
    /// detects. A missing or unreadable manifest entry within an
    /// otherwise-valid archive is not an error here.
    fn read_from_archive(value: &str) -> Result<Option<Vec<u8>>, CollectionError> {
        let file = fs::File::open(value).map_err(|_| CollectionError::Inaccessible {
            path: value.to_string(),
        })?;
        let mut archive = zip::ZipArchive::new(file).map_err(|_| CollectionError::Unsupported {
            path: value.to_string(),
        })?;

        let mut entry = match archive.by_name(MANIFEST_FILE_NAME) {
            Ok(entry) => entry,
            Err(_) => return Ok(None),
        };

        let mut bytes = Vec::new();
        match entry.read_to_end(&mut bytes) {
            Ok(_) => Ok(Some(bytes)),
            Err(_) => Ok(None),
        }
    }

    /// Inspects the manifest's raw content: well-formedness first,
    /// then, only if well-formed, the declared `descVersion` attribute
    /// (Sprint 8) and every declared `<dependency>` element's text
    /// content, in document order (already deterministic — a
    /// document's own byte order — with no additional sort needed,
    /// unlike filesystem or archive traversal).
    fn inspect(bytes: &[u8]) -> Vec<Evidence> {
        let text = match std::str::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => return vec![Self::malformed_manifest_evidence()],
        };

        let document = match roxmltree::Document::parse(text) {
            Ok(document) => document,
            Err(_) => return vec![Self::malformed_manifest_evidence()],
        };

        let mut evidence = vec![
            Self::found_manifest_evidence(),
            Self::declared_desc_version_evidence(document.root_element().attribute("descVersion")),
        ];

        for node in document.descendants() {
            if !node.has_tag_name("dependency") {
                continue;
            }
            let name = node.text().unwrap_or("").trim();
            if name.is_empty() {
                continue;
            }
            evidence.push(
                Evidence::with_location(
                    EvidenceCategory::XmlInspection,
                    format!("modDesc.xml declares dependency: {name}"),
                    MANIFEST_FILE_NAME,
                    Some("Declared Dependency".to_string()),
                    Some(MANIFEST_FILE_NAME.to_string()),
                    Some(name.to_string()),
                )
                .expect("description and location are non-empty"),
            );
        }

        // Store-item declarations are appended strictly after every
        // dependency item, preserving the position of every Evidence
        // item this Collector already produced.
        //
        // Only `storeItem` elements that are direct children of the
        // canonical `storeItems` element are recognized — never
        // unrestricted descendant matching, which would also accept a
        // `storeItem` element appearing anywhere else in the document
        // (`ARCHITECTURAL_RESOLUTION_STORE_ITEM_REFERENCE_INTEGRITY.md`
        // §5; Implementation Authorization, Q4).
        for store_items in document
            .root_element()
            .children()
            .filter(|node| node.has_tag_name("storeItems"))
        {
            for store_item in store_items
                .children()
                .filter(|node| node.has_tag_name("storeItem"))
            {
                evidence.push(Self::declared_store_item_evidence(
                    store_item.attribute("xmlFilename"),
                ));
            }
        }

        evidence
    }

    /// Reports one `storeItem` declaration as a purely factual
    /// observation — the raw declared value, never an interpretation of
    /// whether the path resolves to anything. Whether a declared path
    /// matches an archive entry is the Rule Engine's later, separate
    /// concern (`StoreItemReferenceRule`, `modiq-rules`), exactly as
    /// `declared_desc_version_evidence` already leaves compatibility
    /// judgment to `VersionCompatibilityRule`.
    ///
    /// Every structurally valid declaration is preserved, including one
    /// whose `xmlFilename` attribute is absent or empty: an absent
    /// attribute is represented without inventing a path (`content` is
    /// `None`), and a present-but-empty value is preserved verbatim in
    /// `content`. Both degenerate forms carry their own description
    /// rather than `DECLARED_STORE_ITEM_PREFIX`, so neither can produce
    /// a missing-file conclusion downstream.
    fn declared_store_item_evidence(xml_filename: Option<&str>) -> Evidence {
        let (description, content) = match xml_filename {
            Some(path) if !path.trim().is_empty() => (
                format!("{DECLARED_STORE_ITEM_PREFIX}{path}"),
                Some(path.to_string()),
            ),
            Some(path) => (
                "modDesc.xml declares a store item with an empty or whitespace-only xmlFilename \
                 attribute."
                    .to_string(),
                Some(path.to_string()),
            ),
            None => (
                "modDesc.xml declares a store item with no xmlFilename attribute.".to_string(),
                None,
            ),
        };

        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            description,
            MANIFEST_FILE_NAME,
            Some("Declared Store Item".to_string()),
            Some(MANIFEST_FILE_NAME.to_string()),
            content,
        )
        .expect("description and location are non-empty")
    }

    fn found_manifest_evidence() -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "modDesc.xml was found and is well-formed XML.",
            MANIFEST_FILE_NAME,
            Some("Manifest Found".to_string()),
            Some(MANIFEST_FILE_NAME.to_string()),
            None,
        )
        .expect("description and location are non-empty")
    }

    /// Reports the manifest's declared `descVersion` attribute as a
    /// purely factual observation — the raw declared value, never an
    /// interpretation of whether it is supported (Sprint 8, Decision
    /// 2: "Extraction is evidence collection only... No compatibility
    /// interpretation shall occur within the Collector"). Whether a
    /// declared value is recognized is the Rule Engine's later,
    /// separate concern (`VersionCompatibilityRule`, `modiq-rules`).
    fn declared_desc_version_evidence(desc_version: Option<&str>) -> Evidence {
        match desc_version {
            Some(value) => Evidence::with_location(
                EvidenceCategory::XmlInspection,
                format!("{DECLARED_DESC_VERSION_PREFIX}{value}"),
                MANIFEST_FILE_NAME,
                Some("Declared Version".to_string()),
                Some(MANIFEST_FILE_NAME.to_string()),
                Some(value.to_string()),
            )
            .expect("description and location are non-empty"),
            None => Evidence::with_location(
                EvidenceCategory::XmlInspection,
                "modDesc.xml does not declare a descVersion attribute.",
                MANIFEST_FILE_NAME,
                Some("Declared Version".to_string()),
                Some(MANIFEST_FILE_NAME.to_string()),
                None,
            )
            .expect("description and location are non-empty"),
        }
    }

    fn malformed_manifest_evidence() -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "modDesc.xml was found but is not well-formed XML.",
            MANIFEST_FILE_NAME,
            Some("Malformed Manifest".to_string()),
            Some(MANIFEST_FILE_NAME.to_string()),
            None,
        )
        .expect("description and location are non-empty")
    }

    fn absent_manifest_evidence() -> Evidence {
        Evidence::new(
            EvidenceCategory::XmlInspection,
            "No modDesc.xml was found at the assessment input's root.",
            None,
            None,
            None,
        )
        .expect("description is non-empty")
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    use super::*;

    /// Mirrors `EvidenceCollector`'s and `ArchiveReader`'s own test
    /// helper of the same shape.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-collection-xml-test-{}-{}-{}",
                std::process::id(),
                label,
                unique
            ));
            fs::create_dir_all(&path).expect("can create a temporary test directory");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_file(path: &Path, contents: &str) {
        let mut file = File::create(path).expect("can create a temporary test file");
        file.write_all(contents.as_bytes())
            .expect("can write a temporary test file");
    }

    fn write_archive(path: &Path, files: &[(&str, &str)]) {
        let file = File::create(path).expect("can create a temporary archive file");
        let mut writer = ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        for (name, content) in files {
            writer
                .start_file(*name, options)
                .expect("can start a file entry");
            writer
                .write_all(content.as_bytes())
                .expect("can write file entry content");
        }
        writer.finish().expect("can finalize the archive");
    }

    const WELL_FORMED_MANIFEST: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc descVersion="93">
    <author>Example</author>
    <version>1.0.0.0</version>
    <dependencies>
        <dependency>FS25_exampleModOne</dependency>
        <dependency>FS25_exampleModTwo</dependency>
    </dependencies>
</modDesc>"#;

    const WELL_FORMED_MANIFEST_NO_DEPENDENCIES: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc descVersion="93">
    <author>Example</author>
</modDesc>"#;

    const WELL_FORMED_MANIFEST_NO_DESC_VERSION: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc>
    <author>Example</author>
</modDesc>"#;

    const MALFORMED_MANIFEST: &str = "<modDesc><author>Example</modDesc>";

    const MANIFEST_WITH_STORE_ITEMS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc descVersion="93">
    <author>Example</author>
    <storeItems>
        <storeItem xmlFilename="objects/first/first.xml"/>
        <storeItem xmlFilename="objects/second/second.xml"/>
    </storeItems>
</modDesc>"#;

    const MANIFEST_WITH_DEGENERATE_STORE_ITEMS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc descVersion="93">
    <author>Example</author>
    <storeItems>
        <storeItem/>
        <storeItem xmlFilename=""/>
    </storeItems>
</modDesc>"#;

    const MANIFEST_WITH_NON_CANONICAL_STORE_ITEM: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc descVersion="93">
    <author>Example</author>
    <storeItems>
        <storeItem xmlFilename="objects/canonical.xml"/>
    </storeItems>
    <vehicleTypes>
        <storeItem xmlFilename="objects/misplaced.xml"/>
    </vehicleTypes>
</modDesc>"#;

    #[test]
    fn collect_reports_absence_for_a_directory_with_no_manifest() {
        let dir = TempDir::new("dir-absent");
        write_file(&dir.path().join("notes.txt"), "not a manifest");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].category(), EvidenceCategory::XmlInspection);
        assert_eq!(evidence[0].location(), None);
    }

    #[test]
    fn collect_finds_a_well_formed_manifest_in_a_directory_and_extracts_dependencies() {
        let dir = TempDir::new("dir-well-formed");
        write_file(&dir.path().join("modDesc.xml"), WELL_FORMED_MANIFEST);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 4);
        assert!(evidence[0].description().contains("well-formed"));
        assert_eq!(evidence[0].location(), Some("modDesc.xml"));
        assert!(
            evidence[1]
                .description()
                .contains("declares descVersion: 93")
        );
        assert!(evidence[2].description().contains("FS25_exampleModOne"));
        assert!(evidence[3].description().contains("FS25_exampleModTwo"));
        for item in &evidence {
            assert_eq!(item.category(), EvidenceCategory::XmlInspection);
        }
    }

    #[test]
    fn collect_finds_a_well_formed_manifest_with_no_declared_dependencies() {
        let dir = TempDir::new("dir-no-deps");
        write_file(
            &dir.path().join("modDesc.xml"),
            WELL_FORMED_MANIFEST_NO_DEPENDENCIES,
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 2);
        assert!(evidence[0].description().contains("well-formed"));
        assert!(
            evidence[1]
                .description()
                .contains("declares descVersion: 93")
        );
    }

    #[test]
    fn collect_reports_absence_of_a_desc_version_attribute() {
        let dir = TempDir::new("dir-no-desc-version");
        write_file(
            &dir.path().join("modDesc.xml"),
            WELL_FORMED_MANIFEST_NO_DESC_VERSION,
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 2);
        assert!(evidence[0].description().contains("well-formed"));
        assert!(
            evidence[1]
                .description()
                .contains("does not declare a descVersion attribute")
        );
    }

    #[test]
    fn collect_reports_malformed_for_an_unparseable_manifest() {
        let dir = TempDir::new("dir-malformed");
        write_file(&dir.path().join("modDesc.xml"), MALFORMED_MANIFEST);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("not well-formed"));
    }

    #[test]
    fn collect_treats_a_single_non_manifest_file_input_as_absent() {
        let dir = TempDir::new("single-file-absent");
        let file_path = dir.path().join("notes.txt");
        write_file(&file_path, "just a file");
        let input = AssessmentInput::new(file_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("file is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("No modDesc.xml"));
    }

    #[test]
    fn collect_finds_a_manifest_when_the_input_is_the_manifest_file_itself() {
        let dir = TempDir::new("single-file-manifest");
        let file_path = dir.path().join("modDesc.xml");
        write_file(&file_path, WELL_FORMED_MANIFEST_NO_DEPENDENCIES);
        let input = AssessmentInput::new(file_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("file is accessible");

        assert_eq!(evidence.len(), 2);
        assert!(evidence[0].description().contains("well-formed"));
        assert!(
            evidence[1]
                .description()
                .contains("declares descVersion: 93")
        );
    }

    #[test]
    fn collect_reports_absence_for_an_archive_with_no_manifest() {
        let dir = TempDir::new("archive-absent");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("notes.txt", "not a manifest")]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("archive is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("No modDesc.xml"));
    }

    #[test]
    fn collect_finds_a_well_formed_manifest_at_an_archive_root_and_extracts_dependencies() {
        let dir = TempDir::new("archive-well-formed");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("modDesc.xml", WELL_FORMED_MANIFEST)]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("archive is accessible");

        assert_eq!(evidence.len(), 4);
        assert!(evidence[0].description().contains("well-formed"));
        assert!(
            evidence[1]
                .description()
                .contains("declares descVersion: 93")
        );
        assert!(evidence[2].description().contains("FS25_exampleModOne"));
        assert!(evidence[3].description().contains("FS25_exampleModTwo"));
    }

    #[test]
    fn collect_ignores_a_manifest_not_at_the_archive_root() {
        let dir = TempDir::new("archive-nested-manifest");
        let archive_path = dir.path().join("mod.zip");
        write_archive(
            &archive_path,
            &[("nested/modDesc.xml", WELL_FORMED_MANIFEST)],
        );
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("archive is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("No modDesc.xml"));
    }

    #[test]
    fn collect_reports_malformed_for_an_unparseable_archived_manifest() {
        let dir = TempDir::new("archive-malformed");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("modDesc.xml", MALFORMED_MANIFEST)]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("archive is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("not well-formed"));
    }

    #[test]
    fn collect_reports_inaccessible_for_a_nonexistent_archive() {
        let dir = TempDir::new("archive-nonexistent");
        let missing = dir.path().join("does-not-exist.zip");
        let input = AssessmentInput::new(missing.display().to_string()).unwrap();

        let result = XmlCollector.collect(&input);

        assert_eq!(
            result,
            Err(CollectionError::Inaccessible {
                path: missing.display().to_string()
            })
        );
    }

    #[test]
    fn collect_reports_unsupported_for_a_malformed_archive_location() {
        let dir = TempDir::new("archive-invalid");
        let archive_path = dir.path().join("mod.zip");
        fs::write(&archive_path, b"this is plain text, not a zip archive").unwrap();
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let result = XmlCollector.collect(&input);

        assert_eq!(
            result,
            Err(CollectionError::Unsupported {
                path: archive_path.display().to_string()
            })
        );
    }

    #[test]
    fn collect_is_deterministic_across_repeated_calls() {
        let dir = TempDir::new("deterministic");
        write_file(&dir.path().join("modDesc.xml"), WELL_FORMED_MANIFEST);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let first = XmlCollector
            .collect(&input)
            .expect("directory is accessible");
        let second = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.category(), b.category());
            assert_eq!(a.description(), b.description());
            assert_eq!(a.location(), b.location());
        }
    }

    #[test]
    fn collect_extracts_one_evidence_item_per_declared_store_item_in_document_order() {
        let dir = TempDir::new("store-items");
        write_file(&dir.path().join("modDesc.xml"), MANIFEST_WITH_STORE_ITEMS);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        // Manifest-found, declared-version, then one item per
        // declaration, in document order.
        assert_eq!(evidence.len(), 4);
        assert_eq!(
            evidence[2].description(),
            "modDesc.xml declares store item: objects/first/first.xml"
        );
        assert_eq!(
            evidence[3].description(),
            "modDesc.xml declares store item: objects/second/second.xml"
        );
    }

    #[test]
    fn collect_records_the_declared_store_item_path_as_evidence_content() {
        let dir = TempDir::new("store-item-content");
        write_file(&dir.path().join("modDesc.xml"), MANIFEST_WITH_STORE_ITEMS);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence[2].content(), Some("objects/first/first.xml"));
        assert_eq!(evidence[3].content(), Some("objects/second/second.xml"));
    }

    #[test]
    fn collect_labels_and_locates_declared_store_item_evidence_consistently() {
        let dir = TempDir::new("store-item-label");
        write_file(&dir.path().join("modDesc.xml"), MANIFEST_WITH_STORE_ITEMS);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        for item in &evidence[2..] {
            assert_eq!(item.category(), EvidenceCategory::XmlInspection);
            assert_eq!(item.label(), Some("Declared Store Item"));
            assert_eq!(item.source(), Some("modDesc.xml"));
            assert_eq!(item.location(), Some("modDesc.xml"));
        }
    }

    #[test]
    fn collect_appends_store_item_evidence_after_every_dependency_item() {
        let dir = TempDir::new("store-item-ordering");
        let manifest = r#"<?xml version="1.0" encoding="utf-8"?>
<modDesc descVersion="93">
    <dependencies>
        <dependency>FS25_exampleModOne</dependency>
        <dependency>FS25_exampleModTwo</dependency>
    </dependencies>
    <storeItems>
        <storeItem xmlFilename="objects/first/first.xml"/>
    </storeItems>
</modDesc>"#;
        write_file(&dir.path().join("modDesc.xml"), manifest);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        // Positions 0-3 are exactly what this Collector already
        // produced before store-item extraction existed; the new item
        // is appended at position 4 and shifts nothing.
        assert_eq!(evidence.len(), 5);
        assert!(evidence[0].description().contains("well-formed"));
        assert!(evidence[1].description().contains("declares descVersion"));
        assert!(evidence[2].description().contains("FS25_exampleModOne"));
        assert!(evidence[3].description().contains("FS25_exampleModTwo"));
        assert!(evidence[4].description().contains("declares store item"));
    }

    #[test]
    fn collect_preserves_a_store_item_declaration_with_no_xml_filename_attribute() {
        let dir = TempDir::new("store-item-absent-attribute");
        write_file(
            &dir.path().join("modDesc.xml"),
            MANIFEST_WITH_DEGENERATE_STORE_ITEMS,
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 4);
        assert_eq!(
            evidence[2].description(),
            "modDesc.xml declares a store item with no xmlFilename attribute."
        );
        // No path is invented for an absent attribute.
        assert_eq!(evidence[2].content(), None);
    }

    #[test]
    fn collect_preserves_a_store_item_declaration_with_an_empty_xml_filename_attribute() {
        let dir = TempDir::new("store-item-empty-attribute");
        write_file(
            &dir.path().join("modDesc.xml"),
            MANIFEST_WITH_DEGENERATE_STORE_ITEMS,
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert!(
            evidence[3]
                .description()
                .contains("empty or whitespace-only xmlFilename")
        );
        // The empty value itself is preserved, not discarded.
        assert_eq!(evidence[3].content(), Some(""));
    }

    #[test]
    fn collect_does_not_use_the_declaration_prefix_for_a_degenerate_store_item() {
        let dir = TempDir::new("store-item-degenerate-prefix");
        write_file(
            &dir.path().join("modDesc.xml"),
            MANIFEST_WITH_DEGENERATE_STORE_ITEMS,
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        for item in &evidence[2..] {
            assert!(
                !item
                    .description()
                    .starts_with("modDesc.xml declares store item: "),
                "a degenerate declaration must not carry the declaration prefix"
            );
        }
    }

    #[test]
    fn collect_recognizes_only_store_items_that_are_direct_children_of_store_items() {
        let dir = TempDir::new("store-item-canonical-position");
        write_file(
            &dir.path().join("modDesc.xml"),
            MANIFEST_WITH_NON_CANONICAL_STORE_ITEM,
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        let declared: Vec<&str> = evidence
            .iter()
            .filter_map(Evidence::content)
            .filter(|content| content.ends_with(".xml"))
            .collect();

        assert_eq!(declared, vec!["objects/canonical.xml"]);
    }

    #[test]
    fn collect_reports_no_store_item_evidence_when_none_is_declared() {
        let dir = TempDir::new("store-item-absent");
        write_file(&dir.path().join("modDesc.xml"), WELL_FORMED_MANIFEST);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = XmlCollector
            .collect(&input)
            .expect("directory is accessible");

        assert!(
            evidence
                .iter()
                .all(|item| item.label() != Some("Declared Store Item"))
        );
    }

    #[test]
    fn collect_extracts_store_items_from_an_archived_manifest() {
        let dir = TempDir::new("store-item-archive");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("modDesc.xml", MANIFEST_WITH_STORE_ITEMS)]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = XmlCollector.collect(&input).expect("archive is accessible");

        assert_eq!(evidence.len(), 4);
        assert_eq!(evidence[2].content(), Some("objects/first/first.xml"));
    }
}
