use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::archive_evidence::ArchiveEvidenceBuilder;
use super::archive_reader::{ArchiveEntry, ArchiveReadError, ArchiveReader};
use super::collection_error::CollectionError;

/// Maximum number of entries a single archive may contain before
/// collection refuses it as Unsupported Input (GOV-011, Question 3).
/// Read from the archive's own metadata, before any content is
/// decompressed. Provisional: Phase 2 (`PROPOSAL_GOV-011.md`)
/// confirmed entry count is cheaply readable this way but did not
/// calibrate a production value. Chosen with generous headroom above
/// any plausible real Farming Simulator mod archive.
const MAX_ARCHIVE_ENTRIES: usize = 50_000;

/// Maximum ratio between an entry's claimed uncompressed size and its
/// compressed size before that entry is treated as a resource-limit
/// violation (GOV-011, Question 3). Provisional: chosen with an order
/// of magnitude of headroom above the ~1029:1 ratio Phase 2 measured
/// for an ordinary, non-malicious, highly-compressible fixture
/// (`PROPOSAL_GOV-011.md`, Evidence Incorporated), while still bounding
/// pathological archives.
const MAX_COMPRESSION_RATIO: u64 = 10_000;

/// The Sprint 4 Phase 3C Archive Collector (`EvidenceCollection.md`:
/// Collector Contract). Assembles `ArchiveReader` (Phase 3A) and
/// `ArchiveEvidenceBuilder` (Phase 3B) with GOV-011's three remaining
/// policies — resource limits, the Archive Traversal Boundary Policy,
/// and the Duplicate Archive Entry Policy — into one Collector with
/// the same `Result<Vec<Evidence>, CollectionError>` shape
/// `EvidenceCollector` already uses.
///
/// Not yet reachable from `AssessmentService` — routing between this
/// Collector and the filesystem `EvidenceCollector` is Phase 3D,
/// deliberately out of scope here.
pub struct ArchiveCollector;

impl ArchiveCollector {
    /// Collects Evidence from the archive at `path`.
    ///
    /// Resolves to one of the four Collection Outcomes (GOV-010): `Ok`
    /// with one or more Evidence items, `Ok(vec![])` for Empty
    /// Collection (a well-formed, entirely empty archive), or `Err`
    /// for Inaccessible or Unsupported Input. An individual invalid
    /// archive entry (Archive Traversal Boundary Policy) never aborts
    /// collection of the rest of the archive — only a location that
    /// cannot be reached, cannot be parsed at all, or exceeds a
    /// resource limit does.
    pub fn collect(&self, path: impl AsRef<Path>) -> Result<Vec<Evidence>, CollectionError> {
        let path = path.as_ref();
        let path_string = path.display().to_string();

        let mut reader = ArchiveReader::open(path).map_err(Self::map_read_error)?;

        let entries = reader.entries().map_err(Self::map_read_error)?;
        if entries.len() > MAX_ARCHIVE_ENTRIES {
            return Err(CollectionError::Unsupported { path: path_string });
        }

        let sizes = reader.entry_sizes().map_err(Self::map_read_error)?;
        if Self::exceeds_compression_ratio(&sizes) {
            return Err(CollectionError::Unsupported { path: path_string });
        }

        let archive_has_entries = !entries.is_empty();
        let valid_entries: Vec<ArchiveEntry> = entries
            .into_iter()
            .filter(|entry| Self::is_valid_entry_name(entry.name()))
            .collect();

        let mut evidence = ArchiveEvidenceBuilder.build(&valid_entries);

        // An archive with no entries at all cannot contain duplicate
        // names, and its byte layout has no central directory header
        // for the streaming pass to stop at — it runs straight from
        // the End Of Central Directory record's own signature, which
        // `detect_duplicate_entry_names` does not otherwise expect.
        if archive_has_entries {
            let duplicate_names = Self::detect_duplicate_entry_names(path)
                .map_err(|_| CollectionError::Unsupported { path: path_string })?;
            if !duplicate_names.is_empty() {
                evidence.push(Self::duplicate_entry_evidence(&duplicate_names));
            }
        }

        Ok(evidence)
    }

    fn map_read_error(error: ArchiveReadError) -> CollectionError {
        match error {
            ArchiveReadError::Io { path } => CollectionError::Inaccessible { path },
            ArchiveReadError::InvalidArchive { path } => CollectionError::Unsupported { path },
        }
    }

    /// An entry whose claimed uncompressed size vastly exceeds its
    /// compressed size is treated as a resource-limit violation, using
    /// only sizes recorded in the archive's own metadata — no content
    /// is decompressed to make this determination (GOV-011, Question
    /// 3). An entry with zero compressed size is a violation only if
    /// it also claims nonzero uncompressed content (an empty file or
    /// directory legitimately has both sizes at zero).
    fn exceeds_compression_ratio(sizes: &[(u64, u64)]) -> bool {
        sizes.iter().any(|&(uncompressed, compressed)| {
            if compressed == 0 {
                return uncompressed > 0;
            }
            uncompressed / compressed > MAX_COMPRESSION_RATIO
        })
    }

    /// The Archive Traversal Boundary Policy (GOV-011, Question 4):
    /// an entry is invalid, and is skipped rather than followed or
    /// recorded as Evidence, if its raw, as-stored name is itself an
    /// absolute path, or if it normalizes to a location outside the
    /// archive's own conceptual root. This check runs against the raw
    /// name from `ArchiveEntry::name()` — never against a dependency's
    /// own sanitized representation — since Phase 2 found the
    /// underlying `zip` crate sanitizes rather than rejects an
    /// absolute-path entry (`PROPOSAL_GOV-011.md`, Evidence
    /// Incorporated).
    fn is_valid_entry_name(name: &str) -> bool {
        !Self::is_absolute_archive_path(name) && !Self::escapes_root(name)
    }

    /// A Unix-style path beginning with `/` or `\`, or a Windows-style
    /// drive-qualified path (for example `C:\`), is an absolute path
    /// regardless of how any dependency's own safety accessor might
    /// represent it.
    fn is_absolute_archive_path(name: &str) -> bool {
        if name.starts_with('/') || name.starts_with('\\') {
            return true;
        }

        let bytes = name.as_bytes();
        bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
    }

    /// Normalizes `name` by its path components and reports whether
    /// any prefix of the normalized path would resolve outside the
    /// archive's own conceptual root — for example, via `..` segments.
    fn escapes_root(name: &str) -> bool {
        let mut depth: i64 = 0;

        for component in name.split(['/', '\\']) {
            match component {
                "" | "." => continue,
                ".." => {
                    depth -= 1;
                    if depth < 0 {
                        return true;
                    }
                }
                _ => depth += 1,
            }
        }

        false
    }

    /// The Duplicate Archive Entry Policy's detection mechanism
    /// (GOV-011, Question 2).
    ///
    /// **Why this can't be done via central-directory enumeration:**
    /// `ArchiveReader::entries()` reads the central directory through
    /// the `zip` crate's own `ZipArchive`, which indexes every record
    /// into an `IndexMap<Box<[u8]>, ZipFileData>` keyed by filename
    /// (confirmed directly against the pinned dependency, `zip`
    /// v8.6.0's `SharedBuilder::build`, not assumed) as it parses the
    /// central directory. A second record sharing an earlier record's
    /// name overwrites that key's value; the map's size — everything
    /// `entries()`/`len()` ever exposes — is therefore the number of
    /// *distinct names*, not the number of physical records. The
    /// earlier entry is discarded during parsing itself, before any
    /// consumer of the central directory gets a chance to observe it,
    /// so no amount of additional central-directory reading can
    /// recover it (`archive_reader.rs`'s
    /// `entries_collapses_duplicate_entry_names_to_one` test
    /// demonstrates this directly; `PROPOSAL_GOV-011.md`'s Evidence
    /// Incorporated reached the same conclusion independently, via
    /// Phase 2's investigation of an earlier version of this
    /// dependency).
    ///
    /// Detection therefore re-reads the archive as a sequential stream
    /// of local file headers instead, entirely independent of the
    /// central directory. A local file header exists once per physical
    /// entry regardless of name collisions — nothing about the format
    /// deduplicates them — so a sequential walk observes every entry
    /// the central directory's own name-keyed structure cannot. No
    /// entry's compressed data is decompressed to do this — each
    /// entry's data is skipped by compressed length while advancing to
    /// the next header.
    fn detect_duplicate_entry_names(path: &Path) -> Result<Vec<String>, ArchiveReadError> {
        let path_string = path.display().to_string();
        let file = File::open(path).map_err(|_| ArchiveReadError::Io {
            path: path_string.clone(),
        })?;
        let mut stream = BufReader::new(file);

        let mut occurrences: HashMap<String, usize> = HashMap::new();
        loop {
            match zip::read::read_zipfile_from_stream(&mut stream) {
                Ok(Some(entry)) => {
                    *occurrences.entry(entry.name().to_string()).or_insert(0) += 1;
                }
                Ok(None) => break,
                Err(_) => {
                    return Err(ArchiveReadError::InvalidArchive { path: path_string });
                }
            }
        }

        let mut duplicate_names: Vec<String> = occurrences
            .into_iter()
            .filter(|&(_, count)| count > 1)
            .map(|(name, _)| name)
            .collect();
        duplicate_names.sort();

        Ok(duplicate_names)
    }

    /// Builds the single, distinct Evidence item representing the fact
    /// that duplicate entry names were detected (GOV-011, Duplicate
    /// Archive Entry Policy) — never one fabricated item per
    /// duplicately-named entry, since the earlier entry's own content
    /// cannot actually be observed through the collection mechanism in
    /// use.
    fn duplicate_entry_evidence(duplicate_names: &[String]) -> Evidence {
        let description = format!(
            "Archive collection detected {} duplicate entry name(s), which its inspection mechanism could not fully resolve: {}.",
            duplicate_names.len(),
            duplicate_names.join(", ")
        );

        Evidence::new(EvidenceCategory::StructuralDuplication, description)
            .expect("description is non-empty")
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use modiq_runtime::assessment::EvidenceCategory;

    use super::*;

    /// Mirrors `ArchiveReader`'s and `EvidenceCollector`'s own test
    /// helper of the same shape.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-collection-archive-collector-test-{}-{}-{}",
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

    /// One entry's specification for `write_raw_archive`. Local header
    /// fields describe the entry's real physical bytes (kept truthful
    /// so the streaming duplicate-detection pass parses correctly);
    /// central-directory fields are set independently so resource-limit
    /// tests can fabricate an extreme claimed ratio without needing
    /// real data of that size — a well-formed archive's central
    /// directory is read for `entries()`/`entry_sizes()` without ever
    /// being cross-checked against local header data at that point,
    /// matching this crate's own production code path.
    struct RawEntrySpec {
        name: String,
        local_uncompressed_size: u32,
        local_compressed_size: u32,
        data: &'static [u8],
        central_uncompressed_size: u32,
        central_compressed_size: u32,
    }

    impl RawEntrySpec {
        fn plain(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                local_uncompressed_size: 0,
                local_compressed_size: 0,
                data: &[],
                central_uncompressed_size: 0,
                central_compressed_size: 0,
            }
        }

        fn with_claimed_central_sizes(
            name: impl Into<String>,
            central_uncompressed_size: u32,
            central_compressed_size: u32,
        ) -> Self {
            Self {
                central_uncompressed_size,
                central_compressed_size,
                ..Self::plain(name)
            }
        }
    }

    /// A plausible non-zero MS-DOS date (1980-01-01); its value is
    /// never inspected by anything under test.
    const DOS_DATE: u16 = 0x0021;

    /// Hand-builds a well-formed ZIP archive byte-for-byte: a local
    /// file header and data per entry, followed by one central
    /// directory record per entry, followed by an End Of Central
    /// Directory record. Used instead of `zip::ZipWriter` because the
    /// writer itself now rejects duplicate filenames
    /// (`insert_file_data`'s own "Duplicate filename" check) and offers
    /// no way to independently fabricate a central directory record's
    /// claimed sizes — both of which Phase 3C's tests need to exercise
    /// deterministically, without depending on a compression
    /// algorithm's actual achieved ratio for any input.
    fn write_raw_archive(path: &Path, entries: &[RawEntrySpec]) {
        let mut buffer = Vec::new();
        let mut local_offsets = Vec::with_capacity(entries.len());

        for entry in entries {
            local_offsets.push(buffer.len() as u32);
            buffer.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
            buffer.extend_from_slice(&20u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&DOS_DATE.to_le_bytes());
            buffer.extend_from_slice(&0u32.to_le_bytes());
            buffer.extend_from_slice(&entry.local_compressed_size.to_le_bytes());
            buffer.extend_from_slice(&entry.local_uncompressed_size.to_le_bytes());
            buffer.extend_from_slice(&(entry.name.len() as u16).to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(entry.name.as_bytes());
            buffer.extend_from_slice(entry.data);
        }

        let central_directory_start = buffer.len() as u32;

        for (entry, &local_offset) in entries.iter().zip(local_offsets.iter()) {
            buffer.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
            buffer.extend_from_slice(&20u16.to_le_bytes());
            buffer.extend_from_slice(&20u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&DOS_DATE.to_le_bytes());
            buffer.extend_from_slice(&0u32.to_le_bytes());
            buffer.extend_from_slice(&entry.central_compressed_size.to_le_bytes());
            buffer.extend_from_slice(&entry.central_uncompressed_size.to_le_bytes());
            buffer.extend_from_slice(&(entry.name.len() as u16).to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
            buffer.extend_from_slice(&0u32.to_le_bytes());
            buffer.extend_from_slice(&local_offset.to_le_bytes());
            buffer.extend_from_slice(entry.name.as_bytes());
        }

        let central_directory_size = buffer.len() as u32 - central_directory_start;

        buffer.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        buffer.extend_from_slice(&0u16.to_le_bytes());
        buffer.extend_from_slice(&0u16.to_le_bytes());
        buffer.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        buffer.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        buffer.extend_from_slice(&central_directory_size.to_le_bytes());
        buffer.extend_from_slice(&central_directory_start.to_le_bytes());
        buffer.extend_from_slice(&0u16.to_le_bytes());

        fs::write(path, &buffer).expect("can write a hand-built archive fixture");
    }

    #[test]
    fn collect_produces_evidence_for_each_valid_entry_in_deterministic_order() {
        let dir = TempDir::new("valid-entries");
        let archive_path = dir.path().join("valid.zip");
        write_raw_archive(
            &archive_path,
            &[RawEntrySpec::plain("b.txt"), RawEntrySpec::plain("a.txt")],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        let locations: Vec<&str> = evidence.iter().filter_map(Evidence::location).collect();
        assert_eq!(locations, vec!["a.txt", "b.txt"]);
    }

    #[test]
    fn collect_succeeds_with_zero_evidence_for_an_archive_with_no_entries() {
        let dir = TempDir::new("empty-archive");
        let archive_path = dir.path().join("empty.zip");
        write_raw_archive(&archive_path, &[]);

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_reports_inaccessible_for_a_nonexistent_path() {
        let dir = TempDir::new("nonexistent");
        let missing = dir.path().join("does-not-exist.zip");

        let result = ArchiveCollector.collect(&missing);

        assert_eq!(
            result.err(),
            Some(CollectionError::Inaccessible {
                path: missing.display().to_string()
            })
        );
    }

    #[test]
    fn collect_reports_unsupported_for_a_malformed_archive() {
        let dir = TempDir::new("malformed");
        let path = dir.path().join("not_a_zip.zip");
        fs::write(&path, b"this is plain text, not a zip archive").unwrap();

        let result = ArchiveCollector.collect(&path);

        assert_eq!(
            result.err(),
            Some(CollectionError::Unsupported {
                path: path.display().to_string()
            })
        );
    }

    #[test]
    fn collect_skips_an_entry_with_an_absolute_unix_path_but_collects_the_rest() {
        let dir = TempDir::new("absolute-unix-path");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[
                RawEntrySpec::plain("/etc/passwd"),
                RawEntrySpec::plain("notes.txt"),
            ],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        let locations: Vec<&str> = evidence.iter().filter_map(Evidence::location).collect();
        assert_eq!(locations, vec!["notes.txt"]);
    }

    #[test]
    fn collect_skips_an_entry_with_a_windows_drive_qualified_path() {
        let dir = TempDir::new("windows-drive-path");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[
                RawEntrySpec::plain("C:\\evil.txt"),
                RawEntrySpec::plain("notes.txt"),
            ],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        let locations: Vec<&str> = evidence.iter().filter_map(Evidence::location).collect();
        assert_eq!(locations, vec!["notes.txt"]);
    }

    #[test]
    fn collect_skips_an_entry_with_relative_traversal_escaping_root() {
        let dir = TempDir::new("relative-traversal");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[
                RawEntrySpec::plain("../outside.txt"),
                RawEntrySpec::plain("notes.txt"),
            ],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        let locations: Vec<&str> = evidence.iter().filter_map(Evidence::location).collect();
        assert_eq!(locations, vec!["notes.txt"]);
    }

    #[test]
    fn collect_allows_an_entry_whose_traversal_segments_stay_within_root() {
        let dir = TempDir::new("traversal-within-root");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(&archive_path, &[RawEntrySpec::plain("a/../b.txt")]);

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        let locations: Vec<&str> = evidence.iter().filter_map(Evidence::location).collect();
        assert_eq!(locations, vec!["a/../b.txt"]);
    }

    #[test]
    fn collect_does_not_abort_the_whole_archive_for_one_invalid_entry() {
        let dir = TempDir::new("partial-invalid");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[
                RawEntrySpec::plain("../outside.txt"),
                RawEntrySpec::plain("a.txt"),
                RawEntrySpec::plain("b.txt"),
            ],
        );

        let result = ArchiveCollector.collect(&archive_path);

        assert!(result.is_ok());
    }

    #[test]
    fn collect_detects_duplicate_entry_names_and_adds_one_structural_duplication_item() {
        let dir = TempDir::new("duplicate-names");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[
                RawEntrySpec::plain("same.txt"),
                RawEntrySpec::plain("same.txt"),
                RawEntrySpec::plain("other.txt"),
            ],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        // The central directory collapses the two "same.txt" records to
        // one observable entry (documented directly in
        // `archive_reader.rs`'s `entries_collapses_duplicate_entry_names_to_one`),
        // so per-entry Evidence covers "other.txt" and the single
        // surviving "same.txt" — plus one distinct item for the
        // detected duplication itself.
        assert_eq!(evidence.len(), 3);

        let duplication_items: Vec<&Evidence> = evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::StructuralDuplication)
            .collect();
        assert_eq!(duplication_items.len(), 1);
        assert!(duplication_items[0].description().contains("same.txt"));
    }

    #[test]
    fn collect_reports_no_structural_duplication_when_no_names_repeat() {
        let dir = TempDir::new("no-duplicates");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[RawEntrySpec::plain("a.txt"), RawEntrySpec::plain("b.txt")],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        assert!(
            evidence
                .iter()
                .all(|item| item.category() != EvidenceCategory::StructuralDuplication)
        );
    }

    #[test]
    fn collect_reports_unsupported_when_entry_count_exceeds_the_limit() {
        let dir = TempDir::new("entry-count-limit");
        let archive_path = dir.path().join("archive.zip");

        let entries: Vec<RawEntrySpec> = (0..=super::MAX_ARCHIVE_ENTRIES)
            .map(|index| RawEntrySpec::plain(format!("f{index}")))
            .collect();
        write_raw_archive(&archive_path, &entries);

        let result = ArchiveCollector.collect(&archive_path);

        assert_eq!(
            result.err(),
            Some(CollectionError::Unsupported {
                path: archive_path.display().to_string()
            })
        );
    }

    #[test]
    fn collect_reports_unsupported_when_compression_ratio_exceeds_the_limit() {
        let dir = TempDir::new("compression-ratio-limit");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[RawEntrySpec::with_claimed_central_sizes(
                "bomb.txt",
                100_000_000,
                1,
            )],
        );

        let result = ArchiveCollector.collect(&archive_path);

        assert_eq!(
            result.err(),
            Some(CollectionError::Unsupported {
                path: archive_path.display().to_string()
            })
        );
    }

    #[test]
    fn collect_allows_an_entry_whose_compression_ratio_is_within_the_limit() {
        let dir = TempDir::new("compression-ratio-within-limit");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[RawEntrySpec::with_claimed_central_sizes(
                "fine.txt", 5_000, 1,
            )],
        );

        let evidence = ArchiveCollector.collect(&archive_path).unwrap();

        assert_eq!(evidence.len(), 1);
    }

    #[test]
    fn collect_reports_unsupported_for_zero_compressed_size_with_nonzero_uncompressed_size() {
        let dir = TempDir::new("zero-compressed-size");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[RawEntrySpec::with_claimed_central_sizes(
                "suspicious.txt",
                100,
                0,
            )],
        );

        let result = ArchiveCollector.collect(&archive_path);

        assert_eq!(
            result.err(),
            Some(CollectionError::Unsupported {
                path: archive_path.display().to_string()
            })
        );
    }

    #[test]
    fn collect_is_deterministic_across_repeated_calls() {
        let dir = TempDir::new("deterministic");
        let archive_path = dir.path().join("archive.zip");
        write_raw_archive(
            &archive_path,
            &[RawEntrySpec::plain("a.txt"), RawEntrySpec::plain("b.txt")],
        );

        let first = ArchiveCollector.collect(&archive_path).unwrap();
        let second = ArchiveCollector.collect(&archive_path).unwrap();

        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.category(), b.category());
            assert_eq!(a.description(), b.description());
            assert_eq!(a.location(), b.location());
        }
    }
}
