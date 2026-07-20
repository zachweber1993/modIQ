use std::fs;
use std::path::Path;

use modiq_runtime::assessment::{Evidence, EvidenceCategory};

use super::assessment_input::AssessmentInput;
use super::collection_error::CollectionError;

/// Produces Evidence from an AssessmentInput (`EvidenceCollection.md`:
/// Collector Contract).
///
/// This is the first real collector: it inspects the actual local
/// filesystem location an AssessmentInput names — a file or a
/// directory — and records which files and directories are
/// structurally present. It does not read file contents, does not
/// parse anything, and does not traverse into archives (ZIP, XML, and
/// Lua inspection remain future capabilities, per
/// `PROPOSAL_FILESYSTEM_COLLECTION.md`).
pub struct EvidenceCollector;

impl EvidenceCollector {
    /// Collects Evidence for the given AssessmentInput.
    ///
    /// Resolves to one of the four Collection Outcomes (GOV-010):
    /// `Ok` with one or more Evidence items, `Ok(vec![])` for Empty
    /// Collection (a valid, accessible, supported location containing
    /// nothing), or `Err` for Inaccessible or Unsupported Input.
    /// Collection is atomic: if any part of a directory traversal
    /// fails, the whole call returns `Err` and no partial Evidence is
    /// returned — there is no path through this method that discards
    /// an error and returns a partial result.
    ///
    /// The AssessmentInput's own location is inspected without
    /// following it if it is itself a symbolic link (Phase 5 Symbolic
    /// Link Policy): a symbolic link at the root is treated as
    /// Unsupported, since accepting it would require deciding whether
    /// to follow it, which Phase 5 does not do.
    pub fn collect(&self, input: &AssessmentInput) -> Result<Vec<Evidence>, CollectionError> {
        let root = Path::new(input.value());

        let metadata = fs::symlink_metadata(root).map_err(|_| CollectionError::Inaccessible {
            path: input.value().to_string(),
        })?;
        let file_type = metadata.file_type();

        if file_type.is_symlink() {
            return Err(CollectionError::Unsupported {
                path: input.value().to_string(),
            });
        }

        if file_type.is_file() {
            let evidence = Evidence::with_location(
                EvidenceCategory::FileStructureAnalysis,
                "File discovered during filesystem collection.",
                input.value(),
            )
            .expect("description and location are non-empty because value() is non-empty");
            return Ok(vec![evidence]);
        }

        if file_type.is_dir() {
            let mut evidence = Vec::new();
            Self::collect_directory(root, root, &mut evidence)?;
            return Ok(evidence);
        }

        Err(CollectionError::Unsupported {
            path: input.value().to_string(),
        })
    }

    /// Traverses one directory level, recursing into subdirectories.
    /// Entries are sorted by filename before processing, so that
    /// repeated collection over an unchanged directory always
    /// produces Evidence in the same order (`EvidenceCollection.md`:
    /// Determinism Expectations) — the filesystem itself makes no such
    /// guarantee on its own. Symbolic links encountered here are
    /// skipped, not followed and not recorded (Phase 5 Symbolic Link
    /// Policy).
    ///
    /// Every fallible step propagates its error immediately rather
    /// than being skipped, so that any failure anywhere in the tree
    /// aborts the entire traversal (Collection Atomicity) instead of
    /// silently omitting the part that failed.
    fn collect_directory(
        base: &Path,
        dir: &Path,
        evidence: &mut Vec<Evidence>,
    ) -> Result<(), CollectionError> {
        let inaccessible = || CollectionError::Inaccessible {
            path: dir.display().to_string(),
        };

        let read_dir = fs::read_dir(dir).map_err(|_| inaccessible())?;
        let mut entries = Vec::new();
        for entry in read_dir {
            entries.push(entry.map_err(|_| inaccessible())?);
        }
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let file_type = entry.file_type().map_err(|_| inaccessible())?;

            if file_type.is_symlink() {
                continue;
            }

            let path = entry.path();
            let relative = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .display()
                .to_string();

            if file_type.is_file() {
                evidence.push(
                    Evidence::with_location(
                        EvidenceCategory::FileStructureAnalysis,
                        "File discovered during filesystem collection.",
                        relative,
                    )
                    .expect("description and location are non-empty"),
                );
            } else if file_type.is_dir() {
                evidence.push(
                    Evidence::with_location(
                        EvidenceCategory::FileStructureAnalysis,
                        "Directory discovered during filesystem collection.",
                        relative,
                    )
                    .expect("description and location are non-empty"),
                );
                Self::collect_directory(base, &path, evidence)?;
            }
            // Any other entry kind (device, pipe, socket) is skipped,
            // the same as a symbolic link: Unsupported Input describes
            // the AssessmentInput's own root, not an individual entry
            // discovered partway through an otherwise-valid directory.
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    /// A directory under the OS temp folder, unique per test, removed
    /// when it goes out of scope. Not a production dependency — a
    /// small, self-contained test helper, avoiding a new external
    /// crate for something this narrow.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-collection-test-{}-{}-{}",
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

    #[test]
    fn collect_produces_one_evidence_item_for_a_single_file() {
        let dir = TempDir::new("single-file");
        let file_path = dir.path().join("sample.txt");
        write_file(&file_path, "sample content");
        let input = AssessmentInput::new(file_path.display().to_string()).unwrap();

        let evidence = EvidenceCollector
            .collect(&input)
            .expect("file is accessible");

        assert_eq!(evidence.len(), 1);
        assert_eq!(
            evidence[0].category(),
            EvidenceCategory::FileStructureAnalysis
        );
        assert_eq!(
            evidence[0].location(),
            Some(file_path.display().to_string().as_str())
        );
    }

    #[test]
    fn collect_discovers_files_and_directories_in_deterministic_order() {
        let dir = TempDir::new("tree");
        write_file(&dir.path().join("notes.txt"), "notes");
        fs::create_dir(dir.path().join("nested")).unwrap();
        write_file(&dir.path().join("nested").join("detail.txt"), "detail");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = EvidenceCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 3);
        assert_eq!(evidence[0].location(), Some("nested"));
        assert_eq!(
            evidence[0].category(),
            EvidenceCategory::FileStructureAnalysis
        );
        assert_eq!(
            evidence[1].location(),
            Some(
                Path::new("nested")
                    .join("detail.txt")
                    .display()
                    .to_string()
                    .as_str()
            )
        );
        assert_eq!(evidence[2].location(), Some("notes.txt"));
    }

    #[test]
    fn collect_succeeds_with_zero_evidence_for_an_empty_directory() {
        let dir = TempDir::new("empty");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = EvidenceCollector
            .collect(&input)
            .expect("directory is accessible");

        assert!(evidence.is_empty());
    }

    #[test]
    fn collect_reports_inaccessible_for_a_nonexistent_path() {
        let dir = TempDir::new("nonexistent-parent");
        let missing = dir.path().join("does-not-exist");
        let input = AssessmentInput::new(missing.display().to_string()).unwrap();

        let result = EvidenceCollector.collect(&input);

        assert_eq!(
            result,
            Err(CollectionError::Inaccessible {
                path: missing.display().to_string()
            })
        );
    }

    #[test]
    fn collect_is_deterministic_across_repeated_calls() {
        let dir = TempDir::new("deterministic");
        write_file(&dir.path().join("notes.txt"), "notes");
        fs::create_dir(dir.path().join("nested")).unwrap();
        write_file(&dir.path().join("nested").join("detail.txt"), "detail");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let first = EvidenceCollector
            .collect(&input)
            .expect("directory is accessible");
        let second = EvidenceCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.category(), b.category());
            assert_eq!(a.description(), b.description());
            assert_eq!(a.location(), b.location());
        }
        // Each collection freshly assigns Evidence identity, mirroring
        // every other Runtime identity in the platform; determinism is
        // judged by content and order, not by incidental identity.
        assert_ne!(first[0].id(), second[0].id());
    }

    #[cfg(unix)]
    #[test]
    fn collect_reports_unsupported_for_a_symbolic_link_root() {
        use std::os::unix::fs::symlink;

        let dir = TempDir::new("symlink-root");
        let target = dir.path().join("target.txt");
        write_file(&target, "target content");
        let link = dir.path().join("link.txt");
        symlink(&target, &link).expect("can create a symbolic link");
        let input = AssessmentInput::new(link.display().to_string()).unwrap();

        let result = EvidenceCollector.collect(&input);

        assert_eq!(
            result,
            Err(CollectionError::Unsupported {
                path: link.display().to_string()
            })
        );
    }

    #[cfg(unix)]
    #[test]
    fn collect_skips_symbolic_links_found_during_directory_traversal() {
        use std::os::unix::fs::symlink;

        let dir = TempDir::new("symlink-entry");
        let target = dir.path().join("real.txt");
        write_file(&target, "real content");
        let link = dir.path().join("link.txt");
        symlink(&target, &link).expect("can create a symbolic link");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let evidence = EvidenceCollector
            .collect(&input)
            .expect("directory is accessible");

        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].location(), Some("real.txt"));
    }

    #[cfg(unix)]
    #[test]
    fn collect_is_atomic_and_returns_no_evidence_when_a_nested_directory_is_inaccessible() {
        use std::os::unix::fs::PermissionsExt;

        let dir = TempDir::new("atomic");
        write_file(&dir.path().join("aaa-visible.txt"), "visible");
        let blocked = dir.path().join("zzz-blocked");
        fs::create_dir(&blocked).unwrap();
        write_file(&blocked.join("hidden.txt"), "hidden");
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o000))
            .expect("can restrict directory permissions");
        let input = AssessmentInput::new(dir.path().display().to_string()).unwrap();

        let result = EvidenceCollector.collect(&input);

        // Restore permissions so TempDir's Drop can clean up, before
        // asserting (a failed assertion should not leak an
        // unremovable directory).
        fs::set_permissions(&blocked, fs::Permissions::from_mode(0o755))
            .expect("can restore directory permissions");

        assert!(
            matches!(result, Err(CollectionError::Inaccessible { .. })),
            "expected Inaccessible, got {result:?}"
        );
    }
}
