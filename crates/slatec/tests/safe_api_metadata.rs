use std::path::PathBuf;

fn metadata_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("generated/safe-api")
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
