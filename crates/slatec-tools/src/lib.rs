#![forbid(unsafe_code)]
#![recursion_limit = "256"]

pub mod acquire;
pub mod archive;
pub mod blas1_concurrency;
pub mod complete_corpus;
pub mod diagnostics;
pub mod error;
pub mod extract;
pub mod ffi_inventory;
pub mod ffi_validation;
pub mod fixed_form;
pub mod fortran_state;
pub mod full_corpus;
pub mod hash;
pub mod linkage;
pub mod manifest;
pub mod native_origin_audit;
pub mod native_probe;
pub mod ode_audit;
pub mod optimization_audit;
pub mod policy;
pub mod program_units;
pub mod prologues;
pub mod provider;
pub mod public_module_roadmap;
pub mod raw_ffi;
pub mod routine_catalogue;
pub mod runtime_profile;
pub mod runtime_storage_policy;
pub mod safe_api_docs;
pub mod safe_banded;
pub mod safe_bounded_constrained_linear_least_squares;
pub mod safe_bounded_linear_least_squares;
pub mod safe_bspline;
pub mod safe_constrained_linear_least_squares;
pub mod safe_dassl;
pub mod safe_fftpack;
pub mod safe_fftpack_complex;
pub mod safe_least_squares;
pub mod safe_linear_least_squares;
pub mod safe_linear_programming_deferred;
pub mod safe_lp_in_memory;
pub mod safe_nonlinear;
pub mod safe_nonlinear_expert;
pub mod safe_ode_sdrive;
pub mod safe_pchip;
pub mod safe_piecewise_polynomial;
pub mod safe_quadrature;
pub mod safe_roots;
pub mod safe_special;

pub const TOOL_NAME: &str = "slatec-corpus";
pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TOOL_SEMANTIC_VERSION: &str = "1";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acquire::acquire;
    use crate::archive::{inspect_archive, verify_artifact};
    use crate::extract::extract_raw;
    use crate::manifest::{generate, snapshot_id};
    use crate::policy::Policy;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn fixture_policy(archive: &Path) -> Policy {
        Policy {
            schema_version: 1,
            version: 1,
            source_id: "slatec-source-archive".to_owned(),
            artifact_name: "slatec_src.tgz".to_owned(),
            url: "https://example.invalid/slatec_src.tgz".to_owned(),
            sha256: hash::file(archive).unwrap(),
            compressed_bytes: fs::metadata(archive).unwrap().len(),
            regular_members: 741,
            selected_files: 735,
            profile: "default".to_owned(),
            semantic_hash: "test-policy-hash".to_owned(),
            path: PathBuf::from("test-policy.toml"),
        }
    }

    fn write_fixture(root: &Path) -> PathBuf {
        let archive = root.join("slatec_src.tgz");
        let mut entries = vec![("src/".to_owned(), b'5', Vec::new())];
        entries.push((
            "src/sgeir.f".to_owned(),
            b'0',
            b"      SUBROUTINE SGEIR\n      END\n".to_vec(),
        ));
        entries.push((
            "src/qk15w.f".to_owned(),
            b'0',
            b"      SUBROUTINE QK15W\n      END\n".to_vec(),
        ));
        entries.push((
            "src/dqk15w.f".to_owned(),
            b'0',
            b"      SUBROUTINE DQK15W\n      END\n".to_vec(),
        ));
        for number in 0..732 {
            entries.push((format!("src/a{number:03}.f"), b'0', b"      END\n".to_vec()));
        }
        entries.push(("src/sgeir.f.0".to_owned(), b'0', b"old\n".to_vec()));
        for name in ["changes", "MD5", ".depend", "index", "index.html"] {
            entries.push((format!("src/{name}"), b'0', b"metadata\n".to_vec()));
        }
        fs::write(&archive, gzip_tar(&entries)).unwrap();
        archive
    }

    fn gzip_tar(entries: &[(String, u8, Vec<u8>)]) -> Vec<u8> {
        let mut tar = Vec::new();
        for (path, kind, contents) in entries {
            let mut header = [0_u8; 512];
            header[..path.len()].copy_from_slice(path.as_bytes());
            write_octal(&mut header[100..108], 0o644);
            write_octal(&mut header[108..116], 0);
            write_octal(&mut header[116..124], 0);
            write_octal(&mut header[124..136], contents.len() as u64);
            write_octal(&mut header[136..148], 0);
            header[148..156].fill(b' ');
            header[156] = *kind;
            header[257..263].copy_from_slice(b"ustar\0");
            let checksum: u32 = header.iter().map(|byte| u32::from(*byte)).sum();
            let rendered = format!("{checksum:06o}\0 ");
            header[148..156].copy_from_slice(rendered.as_bytes());
            tar.extend_from_slice(&header);
            tar.extend_from_slice(contents);
            let padding = (512 - contents.len() % 512) % 512;
            tar.resize(tar.len() + padding, 0);
        }
        tar.resize(tar.len() + 1024, 0);
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&tar).unwrap();
        encoder.finish().unwrap()
    }

    fn write_octal(field: &mut [u8], number: u64) {
        let rendered = format!("{:0width$o}\0", number, width = field.len() - 1);
        field.copy_from_slice(rendered.as_bytes());
    }

    #[test]
    fn verifies_and_selects_the_policy_fixture() {
        let temp = tempfile::tempdir().unwrap();
        let archive = write_fixture(temp.path());
        let policy = fixture_policy(&archive);
        verify_artifact(&archive, &policy).unwrap();
        let inventory = inspect_archive(&archive, &policy).unwrap();
        assert_eq!(inventory.regular_member_count, 741);
        assert_eq!(inventory.selected_fortran_member_count, 735);
        assert!(
            inventory
                .members
                .iter()
                .any(|member| member.normalized_path == "src/sgeir.f" && member.selected)
        );
        assert!(
            inventory
                .members
                .iter()
                .any(|member| member.normalized_path == "src/sgeir.f.0" && !member.selected)
        );
        assert!(
            inventory
                .members
                .iter()
                .any(|member| member.normalized_path == "src/changes" && !member.selected)
        );
    }

    #[test]
    fn rejects_bad_size_hash_and_partial_artifacts() {
        let temp = tempfile::tempdir().unwrap();
        let archive = write_fixture(temp.path());
        let mut policy = fixture_policy(&archive);
        policy.sha256 = "0".repeat(64);
        assert!(verify_artifact(&archive, &policy).is_err());
        let mut policy = fixture_policy(&archive);
        policy.compressed_bytes += 1;
        assert!(verify_artifact(&archive, &policy).is_err());
        let partial = temp.path().join("partial.tgz");
        let bytes = fs::read(&archive).unwrap();
        fs::write(&partial, &bytes[..bytes.len() / 2]).unwrap();
        assert!(verify_artifact(&partial, &fixture_policy(&archive)).is_err());
    }

    #[test]
    fn rejects_unsafe_archive_members() {
        for (path, kind) in [
            ("/absolute.f", b'0'),
            ("src/../traversal.f", b'0'),
            ("src/link", b'2'),
            ("src/hard", b'1'),
            ("src/fifo", b'6'),
            ("src/Case.f", b'0'),
            ("src/case.f", b'0'),
        ] {
            let temp = tempfile::tempdir().unwrap();
            let archive = temp.path().join("unsafe.tgz");
            let entries = if path.contains("Case") || path.contains("case") {
                vec![
                    ("src/".to_owned(), b'5', Vec::new()),
                    ("src/Case.f".to_owned(), b'0', Vec::new()),
                    ("src/case.f".to_owned(), b'0', Vec::new()),
                ]
            } else {
                vec![(path.to_owned(), kind, Vec::new())]
            };
            fs::write(&archive, gzip_tar(&entries)).unwrap();
            let policy = fixture_policy(&archive);
            assert!(inspect_archive(&archive, &policy).is_err(), "{path}");
        }
    }

    #[test]
    fn rejects_duplicate_paths_before_extraction() {
        let temp = tempfile::tempdir().unwrap();
        let archive = temp.path().join("duplicate.tgz");
        fs::write(
            &archive,
            gzip_tar(&[
                ("src/".to_owned(), b'5', Vec::new()),
                ("src/same.f".to_owned(), b'0', Vec::new()),
                ("src/same.f".to_owned(), b'0', Vec::new()),
            ]),
        )
        .unwrap();
        assert!(inspect_archive(&archive, &fixture_policy(&archive)).is_err());
    }

    #[test]
    fn offline_acquisition_uses_a_valid_local_artifact_without_network() {
        let temp = tempfile::tempdir().unwrap();
        let archive = write_fixture(temp.path());
        let policy = fixture_policy(&archive);
        acquire(&archive, &temp.path().join("evidence"), &policy, true).unwrap();
        assert!(
            temp.path()
                .join("evidence/acquisition/slatec-source-archive.json")
                .is_file()
        );
    }

    #[test]
    fn raw_extraction_and_manifests_are_deterministic() {
        let temp = tempfile::tempdir().unwrap();
        let archive = write_fixture(temp.path());
        let policy = fixture_policy(&archive);
        let inventory = inspect_archive(&archive, &policy).unwrap();
        let evidence_a = temp.path().join("evidence-a");
        let evidence_b = temp.path().join("evidence-b");
        let snapshot = snapshot_id(&policy);
        let raw = extract_raw(&archive, &evidence_a, &snapshot, &policy, &inventory).unwrap();
        assert_eq!(
            hash::file(&raw.join("src/sgeir.f")).unwrap(),
            inventory
                .members
                .iter()
                .find(|member| member.normalized_path == "src/sgeir.f")
                .unwrap()
                .raw_content_sha256
                .clone()
                .unwrap()
        );
        let output_a = temp.path().join("out-a").join("corpus");
        let output_b = temp.path().join("out-b").join("corpus");
        let first = generate(&output_a, &policy, &inventory).unwrap();
        let second = generate(&output_b, &policy, &inventory).unwrap();
        assert_eq!(first.semantic_hash, second.semantic_hash);
        for file in [
            "artifact-manifest.json",
            "archive-members.json",
            "source-files.json",
            "selected-providers.json",
            "diagnostics.json",
            "manifest.json",
            "manifest-summary.md",
        ] {
            assert_eq!(
                fs::read(output_a.join(file)).unwrap(),
                fs::read(output_b.join(file)).unwrap()
            );
        }
        extract_raw(&archive, &evidence_b, &snapshot, &policy, &inventory).unwrap();
    }

    #[test]
    fn failed_extraction_does_not_promote_a_complete_tree() {
        let temp = tempfile::tempdir().unwrap();
        let archive = write_fixture(temp.path());
        let policy = fixture_policy(&archive);
        let mut inventory = inspect_archive(&archive, &policy).unwrap();
        let member = inventory
            .members
            .iter_mut()
            .find(|member| member.entry_type == "regular")
            .unwrap();
        member.raw_content_sha256 = Some("0".repeat(64));
        let snapshot = snapshot_id(&policy);
        assert!(
            extract_raw(
                &archive,
                temp.path().join("evidence").as_path(),
                &snapshot,
                &policy,
                &inventory
            )
            .is_err()
        );
        assert!(
            !temp
                .path()
                .join("evidence/extracted")
                .join(&snapshot)
                .exists()
        );
    }
}
