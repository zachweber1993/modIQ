use std::fs;
use std::io::Read;
use std::path::Path;

use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::assessment_input::AssessmentInput;
use super::collection_error::CollectionError;

/// The manifest file name every loadable Farming Simulator mod is
/// expected to carry at its root.
const MANIFEST_FILE_NAME: &str = "modDesc.xml";

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
    /// then, only if well-formed, every declared `<dependency>`
    /// element's text content, in document order (already
    /// deterministic — a document's own byte order — with no
    /// additional sort needed, unlike filesystem or archive
    /// traversal).
    fn inspect(bytes: &[u8]) -> Vec<Evidence> {
        let text = match std::str::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => return vec![Self::malformed_manifest_evidence()],
        };

        let document = match roxmltree::Document::parse(text) {
            Ok(document) => document,
            Err(_) => return vec![Self::malformed_manifest_evidence()],
        };

        let mut evidence = vec![Self::found_manifest_evidence()];

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
                )
                .expect("description and location are non-empty"),
            );
        }

        evidence
    }

    fn found_manifest_evidence() -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "modDesc.xml was found and is well-formed XML.",
            MANIFEST_FILE_NAME,
        )
        .expect("description and location are non-empty")
    }

    fn malformed_manifest_evidence() -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            "modDesc.xml was found but is not well-formed XML.",
            MANIFEST_FILE_NAME,
        )
        .expect("description and location are non-empty")
    }

    fn absent_manifest_evidence() -> Evidence {
        Evidence::new(
            EvidenceCategory::XmlInspection,
            "No modDesc.xml was found at the assessment input's root.",
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

    const MALFORMED_MANIFEST: &str = "<modDesc><author>Example</modDesc>";

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

        assert_eq!(evidence.len(), 3);
        assert!(evidence[0].description().contains("well-formed"));
        assert_eq!(evidence[0].location(), Some("modDesc.xml"));
        assert!(evidence[1].description().contains("FS25_exampleModOne"));
        assert!(evidence[2].description().contains("FS25_exampleModTwo"));
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

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("well-formed"));
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

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("well-formed"));
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

        assert_eq!(evidence.len(), 3);
        assert!(evidence[0].description().contains("well-formed"));
        assert!(evidence[1].description().contains("FS25_exampleModOne"));
        assert!(evidence[2].description().contains("FS25_exampleModTwo"));
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
}
