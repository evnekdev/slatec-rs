# `slatec-sys` instructions

Scope: `crates/slatec-sys/**`.

## Responsibility

This crate defines unsafe raw Rust declarations, ABI support types, canonical raw family modules, and declaration-only feature gates. It must not build, acquire, select, or link a numerical provider by itself.

## Rules

- Keep declarations faithful to the validated Fortran ABI: argument order, mutability, scalar-vs-array shape, callback signatures, hidden character lengths, logical/integer kinds, complex ABI, and return convention.
- Every promoted raw routine needs a canonical mathematical-family path and sufficient safety/documentation coverage.
- Avoid duplicate independent declarations for the same native symbol. Prefer one authoritative declaration with deliberate compatibility re-exports.
- Do not organize the stable public API primarily by ABI shape.
- `all` must transitively include every public mathematical family aggregate and no provider, test-only, or implementation-selection features.
- New public family features must be added to `all` and to its mechanical coverage validation in the same change.
- Merely enabling raw features must not create native symbol references.
- Keep unsupported, unvalidated, missing-provider, and catalogue-only routines explicitly classified rather than silently omitted.
- Preserve source-hash-guarded correction metadata and deterministic inventories.
- Compatibility paths must not become a second preferred API.

## Required checks for raw API changes

Run the relevant subset of:

```bash
cargo check -p slatec-sys --no-default-features
cargo check -p slatec-sys --all-features
cargo test -p slatec-sys
RUSTDOCFLAGS="-D warnings" cargo doc -p slatec-sys --no-deps
cargo test --doc -p slatec-sys
```

Also run raw-FFI generation/validation, canonical-path checks, documentation audits, `all` coverage validation, link probes, source-hash checks, deterministic regeneration, and package-content audit affected by the change.
