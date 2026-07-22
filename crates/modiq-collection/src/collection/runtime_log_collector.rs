use std::fs;
use std::io::Read;
use std::path::Path;

use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::assessment_input::AssessmentInput;
use super::collection_error::CollectionError;

/// The runtime log file name a bundled submission is expected to carry
/// at its root, confirmed directly against all three real fixtures in
/// `fixtures/runtime-logs/` (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`,
/// Section 1.2).
const LOG_FILE_NAME: &str = "log.txt";

/// The recognized failure template (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`,
/// Section 2.2): generalized over the mod name, not the message text.
/// The one real, captured failure signature this platform has evidence
/// for (`single-incompatible-mod`).
const FAILURE_LINE_PREFIX: &str = "Error: Unsupported mod description version in mod ";

/// Produces `RuntimeLogs` Evidence from a bundled runtime log
/// (Sprint 11: Runtime Evidence Processing Architecture).
///
/// Runs independently of, and alongside, every other Collector
/// inspecting the same Assessment Input — never by consuming another
/// Collector's output (`EvidenceCollection.md`: Collector Contract).
/// Determines entirely on its own whether a file named exactly
/// `log.txt` exists at the input's root, mirroring `XmlCollector`'s own
/// precedent for `modDesc.xml`.
///
/// Unlike `XmlCollector`'s treatment of a missing manifest, a missing
/// or unrecognized log is Legitimate Absence, not a recorded fact
/// (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, Section 1.2): most
/// Assessments will have no log bundled at all, and that is the
/// ordinary case, not an anomaly.
pub struct RuntimeLogCollector;

impl RuntimeLogCollector {
    /// Collects Evidence for the given Assessment Input.
    ///
    /// Always resolves to `Ok`. Returns `Ok(vec![])` (Empty Collection)
    /// when no log is found, the log cannot be read, or the log's
    /// content contains no line matching the recognized failure
    /// template — none of these is a Collector failure. A genuine
    /// `CollectionError` is reserved for the Assessment Input's own
    /// root being unreachable at all — the same failure the structural
    /// Collector running alongside this one would independently
    /// encounter.
    pub fn collect(&self, input: &AssessmentInput) -> Result<Vec<Evidence>, CollectionError> {
        let value = input.value();

        let content = if Self::is_archive_location(value) {
            Self::read_from_archive(value)?
        } else {
            Self::read_from_filesystem(value)
        };

        Ok(match content {
            Some(bytes) => Self::inspect(&bytes),
            None => Vec::new(),
        })
    }

    /// The same `.zip`-suffix, case-insensitive check every other
    /// Collector in this crate already uses — duplicated deliberately,
    /// not shared (`EvidenceCollection.md`: Collector Contract).
    fn is_archive_location(value: &str) -> bool {
        value.to_ascii_lowercase().ends_with(".zip")
    }

    /// Reads the log's raw bytes from a filesystem location: the input
    /// itself, if it is a file named `log.txt`, or a direct child of
    /// the input, if the input is a directory. Any failure to locate
    /// or read the file is treated as the log being absent, not as a
    /// Collector failure, mirroring `XmlCollector`'s identical
    /// treatment of manifest-read failures.
    fn read_from_filesystem(value: &str) -> Option<Vec<u8>> {
        let root = Path::new(value);

        let log_path = if root.is_dir() {
            root.join(LOG_FILE_NAME)
        } else if root.file_name().is_some_and(|name| name == LOG_FILE_NAME) {
            root.to_path_buf()
        } else {
            return None;
        };

        fs::read(&log_path).ok()
    }

    /// Reads the log's raw bytes from an archive entry named exactly
    /// `log.txt` at the archive's root. Returns a genuine
    /// `CollectionError` only when the archive location itself cannot
    /// be opened or parsed at all — the same failure the structural
    /// `ArchiveCollector` running alongside this one independently
    /// detects.
    fn read_from_archive(value: &str) -> Result<Option<Vec<u8>>, CollectionError> {
        let file = fs::File::open(value).map_err(|_| CollectionError::Inaccessible {
            path: value.to_string(),
        })?;
        let mut archive = zip::ZipArchive::new(file).map_err(|_| CollectionError::Unsupported {
            path: value.to_string(),
        })?;

        let mut entry = match archive.by_name(LOG_FILE_NAME) {
            Ok(entry) => entry,
            Err(_) => return Ok(None),
        };

        let mut bytes = Vec::new();
        match entry.read_to_end(&mut bytes) {
            Ok(_) => Ok(Some(bytes)),
            Err(_) => Ok(None),
        }
    }

    /// Inspects the log's raw content for the recognized failure
    /// template, in line order (already deterministic — a text file's
    /// own byte order — with no additional sort needed). Produces one
    /// `RuntimeLogs` Evidence item per matching line; a log containing
    /// no matching line produces no Evidence at all
    /// (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, Section 2.1: an
    /// Evidence item exists only for a recognized signal).
    ///
    /// Non-UTF-8 content is treated as containing no recognized
    /// signal, not as a Collector failure — the same discipline
    /// `XmlCollector` applies to unreadable manifest bytes, scoped
    /// here to "no signal found" rather than a distinct malformed-log
    /// fact, since no fixture evidences a need to represent malformed
    /// log content as its own Evidence item.
    fn inspect(bytes: &[u8]) -> Vec<Evidence> {
        let Ok(text) = std::str::from_utf8(bytes) else {
            return Vec::new();
        };

        text.lines()
            .filter_map(Self::recognized_failure_evidence)
            .collect()
    }

    /// Recognizes exactly one line shape: the documented failure
    /// template, generalized over the named mod but not the message
    /// text itself (Section 2.2). Any other line, including one merely
    /// resembling this shape, is not recognized.
    fn recognized_failure_evidence(line: &str) -> Option<Evidence> {
        let mod_name = line.strip_prefix(FAILURE_LINE_PREFIX)?.trim();
        if mod_name.is_empty() {
            return None;
        }

        Some(
            Evidence::with_location(
                EvidenceCategory::RuntimeLogs,
                format!(
                    "Runtime log records: Unsupported mod description version in mod {mod_name}"
                ),
                LOG_FILE_NAME,
            )
            .expect("description and location are non-empty"),
        )
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

    /// Mirrors `XmlCollector`'s and `EvidenceCollector`'s own test
    /// helper of the same shape.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-collection-runtime-log-test-{}-{}-{}",
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

    /// Mirrors the real, captured shape of `single-incompatible-mod`'s
    /// own log content (`fixtures/runtime-logs/single-incompatible-mod/log.txt`):
    /// an `Available mod:` enumeration entry immediately followed by
    /// the rejection line, with no `Load mod:` line for that mod
    /// anywhere in the file.
    const CLEAN_LOG: &str = "\
GIANTS Engine Runtime 10.0.0
Available mod: (Hash: aaaa) (Version: 1.0.0.0) FS25_SomeCompatibleMod
Load mod: FS25_SomeCompatibleMod
Application quit";

    const INCOMPATIBLE_MOD_LOG: &str = "\
GIANTS Engine Runtime 10.0.0
Available mod: (Hash: 98e7e08a7f9175c321ab783abf123603) (Version: 1.0.0.4) FS25_DodgeChallengerHellcat
Error: Unsupported mod description version in mod FS25_DodgeChallengerHellcat
Application quit";

    #[test]
    fn collect_reports_empty_for_a_directory_with_no_log() {
        let dir = TempDir::new("dir-absent");
        write_file(&dir.path().join("notes.txt"), "not a log");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("directory is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_reports_empty_for_a_clean_log_with_no_recognized_failure() {
        let dir = TempDir::new("dir-clean");
        write_file(&dir.path().join("log.txt"), CLEAN_LOG);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("directory is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_recognizes_the_documented_failure_template_in_a_directory() {
        let dir = TempDir::new("dir-failure");
        write_file(&dir.path().join("log.txt"), INCOMPATIBLE_MOD_LOG);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].category(), EvidenceCategory::RuntimeLogs);
        assert_eq!(evidence[0].location(), Some("log.txt"));
        assert!(
            evidence[0]
                .description()
                .contains("FS25_DodgeChallengerHellcat")
        );
    }

    #[test]
    fn collect_generalizes_over_a_different_mod_name() {
        let dir = TempDir::new("dir-different-mod");
        write_file(
            &dir.path().join("log.txt"),
            "Error: Unsupported mod description version in mod FS25_SomeOtherMod",
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(evidence[0].description().contains("FS25_SomeOtherMod"));
    }

    #[test]
    fn collect_does_not_recognize_a_merely_similar_line() {
        let dir = TempDir::new("dir-similar");
        write_file(
            &dir.path().join("log.txt"),
            "Error: Something else entirely went wrong in mod FS25_SomeMod",
        );
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("directory is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_treats_a_single_non_log_file_input_as_empty() {
        let dir = TempDir::new("single-file-absent");
        let file_path = dir.path().join("notes.txt");
        write_file(&file_path, "just a file");
        let input = AssessmentInput::new(file_path.display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("file is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_finds_a_log_when_the_input_is_the_log_file_itself() {
        let dir = TempDir::new("single-file-log");
        let file_path = dir.path().join("log.txt");
        write_file(&file_path, INCOMPATIBLE_MOD_LOG);
        let input = AssessmentInput::new(file_path.display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("file is accessible");

        assert_eq!(evidence.len(), 1);
    }

    #[test]
    fn collect_reports_empty_for_an_archive_with_no_log() {
        let dir = TempDir::new("archive-absent");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("notes.txt", "not a log")]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("archive is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_recognizes_the_documented_failure_template_at_an_archive_root() {
        let dir = TempDir::new("archive-failure");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("log.txt", INCOMPATIBLE_MOD_LOG)]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("archive is accessible");

        assert_eq!(evidence.len(), 1);
        assert!(
            evidence[0]
                .description()
                .contains("FS25_DodgeChallengerHellcat")
        );
    }

    #[test]
    fn collect_ignores_a_log_not_at_the_archive_root() {
        let dir = TempDir::new("archive-nested-log");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("nested/log.txt", INCOMPATIBLE_MOD_LOG)]);
        let input = AssessmentInput::new(archive_path.display().to_string()).unwrap();

        let evidence = RuntimeLogCollector
            .collect(&input)
            .expect("archive is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_reports_inaccessible_for_a_nonexistent_archive() {
        let dir = TempDir::new("archive-nonexistent");
        let missing = dir.path().join("does-not-exist.zip");
        let input = AssessmentInput::new(missing.display().to_string()).unwrap();

        let result = RuntimeLogCollector.collect(&input);

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

        let result = RuntimeLogCollector.collect(&input);

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
        write_file(&dir.path().join("log.txt"), INCOMPATIBLE_MOD_LOG);
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let first = RuntimeLogCollector
            .collect(&input)
            .expect("directory is accessible");
        let second = RuntimeLogCollector
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
