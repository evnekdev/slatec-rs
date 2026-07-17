use std::path::PathBuf;

fn metadata_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("generated/safe-api")
        .join(name)
}

fn linkage_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("generated/linkage")
        .join(name)
}

#[test]
fn safe_api_metadata_is_compact_and_contains_no_source_text() {
    for name in [
        "manifest.json",
        "wrapper-index.json",
        "deferred-routines.json",
        "validation-summary.md",
        "matrix-wrapper-index.json",
        "deferred-matrix-routines.json",
        "matrix-validation-summary.md",
        "special-function-candidates.json",
        "special-function-wrapper-index.json",
        "special-function-family-summary.json",
        "special-function-runtime-state.json",
        "special-function-deferred.json",
        "special-function-validation-summary.md",
        "special-function-manifest.json",
        "root-candidate-index.json",
        "root-wrapper-index.json",
        "root-status-map.json",
        "root-callback-policy.json",
        "root-deferred.json",
        "root-validation-summary.md",
        "linear-least-squares-candidate-index.json",
        "nonnegative-least-squares-wrapper-index.json",
        "nonnegative-least-squares-status-map.json",
        "nonnegative-least-squares-matrix-layout.json",
        "nonnegative-least-squares-workspace.json",
        "nonnegative-least-squares-deferred.json",
        "nonnegative-least-squares-validation-summary.md",
        "bounded-least-squares-candidate-index.json",
        "bounded-least-squares-wrapper-index.json",
        "bounded-least-squares-status-map.json",
        "bounded-least-squares-bound-encoding.json",
        "bounded-least-squares-options.json",
        "bounded-least-squares-workspace.json",
        "bounded-least-squares-deferred.json",
        "bounded-least-squares-validation-summary.md",
    ] {
        let contents = std::fs::read(metadata_path(name)).expect("committed safe API metadata");
        assert!(contents.len() < 64 * 1024, "{name} must remain compact");
    }

    for name in [
        "function-index.json",
        "fortran-argument-map.json",
        "example-coverage.json",
        "capability-summary.json",
        "function-index.md",
    ] {
        let contents = std::fs::read(metadata_path(name)).expect("safe API documentation metadata");
        assert!(
            contents.len() < 256 * 1024,
            "{name} must remain a compact structural index"
        );
        assert!(
            !contents
                .windows(17)
                .any(|window| window == b"      SUBROUTINE "),
            "{name} must not contain copied fixed-form source"
        );
    }
}

#[test]
fn family_linkage_metadata_is_compact_and_contains_no_native_bytes() {
    for name in [
        "family-link-report.json",
        "family-to-raw-symbols.json",
        "family-to-source-closure.json",
        "symbol-retention-report.json",
        "validation-summary.md",
    ] {
        let contents = std::fs::read(linkage_path(name)).expect("committed linkage metadata");
        assert!(contents.len() < 128 * 1024, "{name} must remain compact");
        assert!(
            !contents.starts_with(b"!<arch>\n"),
            "{name} is not an archive"
        );
        assert!(!contents.starts_with(b"MZ"), "{name} is not an executable");
        assert!(
            !contents
                .windows(17)
                .any(|window| window == b"      SUBROUTINE "),
            "{name} must not contain copied fixed-form source"
        );
    }
}
