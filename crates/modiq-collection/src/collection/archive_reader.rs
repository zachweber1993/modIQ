use std::fs::File;
use std::path::Path;

/// A single structural fact about one entry discovered inside a ZIP
/// archive: its name as stored in the archive, its uncompressed size,
/// and whether it represents a directory.
///
/// Carries nothing beyond what deterministic structural enumeration
/// requires. In particular, no timestamp, permission, ownership,
/// comment, or compression-method field is exposed — the Archive
/// Metadata Policy (GOV-011, `EvidenceCollection.md`) excludes all of
/// these from Assessment Evidence, and this foundation type does not
/// carry data its own architecture has already decided must never
/// participate in Evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    name: String,
    size: u64,
    is_dir: bool,
}

impl ArchiveEntry {
    /// Constructs an `ArchiveEntry` directly, for test code elsewhere in
    /// this crate that exercises logic built on top of `ArchiveReader`
    /// (Evidence generation, and later phases) without needing to open a
    /// real archive for every case. Not part of `ArchiveReader`'s own
    /// behavior — `entries()` remains the only production path that
    /// constructs one from real archive data.
    #[cfg(test)]
    pub(crate) fn new(name: impl Into<String>, size: u64, is_dir: bool) -> Self {
        Self {
            name: name.into(),
            size,
            is_dir,
        }
    }

    /// The entry's name exactly as stored in the archive. Not yet
    /// normalized or validated against the Archive Traversal Boundary
    /// Policy (GOV-011) — that filtering is layered on top of this
    /// foundation in a later phase, not performed here.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The entry's uncompressed size, in bytes, as recorded in the
    /// archive's own metadata. No content is decompressed to produce
    /// this value.
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

/// Errors produced when an archive location cannot be opened or read.
///
/// Distinct from `CollectionError`: this type is scoped to the Archive
/// Reader's own foundation-level responsibility, not to the full
/// Collection Error Model (GOV-010, GOV-011). Mapping these onto
/// `CollectionError`'s Inaccessible/Unsupported outcomes is later
/// integration work, not decided or performed here. Deliberately holds
/// only a `path` for context, not the underlying `std::io::Error` or
/// `zip::result::ZipError` itself, matching `CollectionError`'s own
/// existing shape and keeping this type comparable in tests.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ArchiveReadError {
    /// The location could not be opened or read as a file at all.
    #[error("archive location could not be read: {path}")]
    Io { path: String },

    /// The location was read, but its content is not a well-formed
    /// archive of the supported kind (GOV-011, Question 1).
    #[error("archive is not well-formed: {path}")]
    InvalidArchive { path: String },
}

/// A minimal, deterministic ZIP archive reader.
///
/// This is a foundation type, not yet a Collector
/// (`EvidenceCollection.md`: Collector Contract): it opens a location
/// and enumerates its structural contents deterministically, but does
/// not produce Evidence, does not detect duplicate entries (GOV-011,
/// Duplicate Archive Entry Policy), does not filter traversal-unsafe
/// entries (GOV-011, Archive Traversal Boundary Policy), and does not
/// enforce resource limits (GOV-011, Question 3). Those responsibilities
/// are layered on top of this foundation in a later phase, per
/// `SPRINT4_IMPLEMENTATION_PLAN.md`'s phased approach.
pub struct ArchiveReader {
    archive: zip::ZipArchive<File>,
    path: String,
}

impl ArchiveReader {
    /// Opens the archive at `path`.
    ///
    /// Returns a clean, typed error rather than panicking for any
    /// location that cannot be read as a file (`ArchiveReadError::Io`)
    /// or whose content is not a well-formed archive
    /// (`ArchiveReadError::InvalidArchive`) — consistent with GOV-011's
    /// Question 1 resolution that a malformed archive is a clean
    /// failure, never a panic.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ArchiveReadError> {
        let path = path.as_ref();
        let path_string = path.display().to_string();

        let file = File::open(path).map_err(|_| ArchiveReadError::Io {
            path: path_string.clone(),
        })?;

        let archive = zip::ZipArchive::new(file).map_err(|_| ArchiveReadError::InvalidArchive {
            path: path_string.clone(),
        })?;

        Ok(Self {
            archive,
            path: path_string,
        })
    }

    /// Deterministically enumerates the archive's entries.
    ///
    /// Entries are sorted by name before being returned, so that
    /// repeated enumeration of an unchanged archive always produces
    /// identical output in identical order (`EvidenceCollection.md`:
    /// Determinism Expectations). The archive's own central-directory
    /// order is not assumed to be stable or meaningful on its own —
    /// the same discipline `EvidenceCollector` already applies to
    /// filesystem directory traversal, applied here to a second real
    /// case.
    pub fn entries(&mut self) -> Result<Vec<ArchiveEntry>, ArchiveReadError> {
        let mut entries = Vec::with_capacity(self.archive.len());

        for index in 0..self.archive.len() {
            let entry =
                self.archive
                    .by_index(index)
                    .map_err(|_| ArchiveReadError::InvalidArchive {
                        path: self.path.clone(),
                    })?;

            entries.push(ArchiveEntry {
                name: entry.name().to_string(),
                size: entry.size(),
                is_dir: entry.is_dir(),
            });
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    use super::*;

    /// A directory under the OS temp folder, unique per test, removed
    /// when it goes out of scope. Mirrors `EvidenceCollector`'s own test
    /// helper of the same shape.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-collection-archive-test-{}-{}-{}",
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

    /// Writes a real, well-formed ZIP archive containing the given
    /// (name, content) file entries plus the given explicit directory
    /// entries, in exactly the order provided — so tests can construct
    /// archives whose write order deliberately differs from the
    /// alphabetical order `entries()` is expected to impose.
    fn write_archive(path: &Path, files: &[(&str, &str)], dirs: &[&str]) {
        let file = File::create(path).expect("can create a temporary archive file");
        let mut writer = ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        for dir_name in dirs {
            writer
                .add_directory(*dir_name, options)
                .expect("can add a directory entry");
        }
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

    #[test]
    fn open_succeeds_for_a_well_formed_archive() {
        let dir = TempDir::new("open-success");
        let archive_path = dir.path().join("sample.zip");
        write_archive(&archive_path, &[("notes.txt", "sample content")], &[]);

        let result = ArchiveReader::open(&archive_path);

        assert!(result.is_ok());
    }

    #[test]
    fn open_reports_io_error_for_a_nonexistent_path() {
        let dir = TempDir::new("open-nonexistent");
        let missing = dir.path().join("does-not-exist.zip");

        let result = ArchiveReader::open(&missing);

        assert_eq!(
            result.err(),
            Some(ArchiveReadError::Io {
                path: missing.display().to_string()
            })
        );
    }

    #[test]
    fn open_reports_invalid_archive_for_non_archive_content() {
        let dir = TempDir::new("open-not-a-zip");
        let path = dir.path().join("not_a_zip.zip");
        fs::write(&path, b"this is plain text, not a zip archive").unwrap();

        let result = ArchiveReader::open(&path);

        assert_eq!(
            result.err(),
            Some(ArchiveReadError::InvalidArchive {
                path: path.display().to_string()
            })
        );
    }

    #[test]
    fn open_reports_invalid_archive_for_a_truncated_archive() {
        let dir = TempDir::new("open-truncated");
        let well_formed_path = dir.path().join("well_formed.zip");
        write_archive(
            &well_formed_path,
            &[(
                "notes.txt",
                "some real content so the archive is not trivially small",
            )],
            &[],
        );
        let well_formed_bytes = fs::read(&well_formed_path).unwrap();

        let truncated_path = dir.path().join("truncated.zip");
        fs::write(
            &truncated_path,
            &well_formed_bytes[..well_formed_bytes.len() / 2],
        )
        .unwrap();

        let result = ArchiveReader::open(&truncated_path);

        assert_eq!(
            result.err(),
            Some(ArchiveReadError::InvalidArchive {
                path: truncated_path.display().to_string()
            })
        );
    }

    #[test]
    fn entries_discovers_all_entries_in_deterministic_order() {
        let dir = TempDir::new("entries-order");
        let archive_path = dir.path().join("tree.zip");
        // Written deliberately out of alphabetical order, mirroring
        // EvidenceCollector's own determinism test: physical/write
        // order must not leak into the returned order.
        write_archive(
            &archive_path,
            &[
                ("zeta.txt", "z"),
                ("nested/detail.txt", "detail"),
                ("alpha.txt", "a"),
            ],
            &["nested/"],
        );

        let mut reader = ArchiveReader::open(&archive_path).unwrap();
        let entries = reader.entries().unwrap();

        let names: Vec<&str> = entries.iter().map(ArchiveEntry::name).collect();
        assert_eq!(
            names,
            vec!["alpha.txt", "nested/", "nested/detail.txt", "zeta.txt"]
        );
    }

    #[test]
    fn entries_reports_size_and_kind_correctly() {
        let dir = TempDir::new("entries-metadata");
        let archive_path = dir.path().join("metadata.zip");
        write_archive(
            &archive_path,
            &[("notes.txt", "twelve bytes")],
            &["nested/"],
        );

        let mut reader = ArchiveReader::open(&archive_path).unwrap();
        let entries = reader.entries().unwrap();

        let file_entry = entries.iter().find(|e| e.name() == "notes.txt").unwrap();
        assert!(!file_entry.is_dir());
        assert_eq!(file_entry.size(), "twelve bytes".len() as u64);

        let dir_entry = entries.iter().find(|e| e.name() == "nested/").unwrap();
        assert!(dir_entry.is_dir());
    }

    #[test]
    fn entries_is_empty_for_an_archive_with_no_entries() {
        let dir = TempDir::new("entries-empty");
        let archive_path = dir.path().join("empty.zip");
        write_archive(&archive_path, &[], &[]);

        let mut reader = ArchiveReader::open(&archive_path).unwrap();
        let entries = reader.entries().unwrap();

        assert!(entries.is_empty());
    }

    #[test]
    fn entries_is_deterministic_across_repeated_opens() {
        let dir = TempDir::new("entries-deterministic");
        let archive_path = dir.path().join("repeat.zip");
        write_archive(
            &archive_path,
            &[
                ("zeta.txt", "z"),
                ("nested/detail.txt", "detail"),
                ("alpha.txt", "a"),
            ],
            &["nested/"],
        );

        let mut first_reader = ArchiveReader::open(&archive_path).unwrap();
        let first = first_reader.entries().unwrap();

        let mut second_reader = ArchiveReader::open(&archive_path).unwrap();
        let second = second_reader.entries().unwrap();

        assert_eq!(first, second);
    }
}
