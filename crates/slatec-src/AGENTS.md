# `slatec-src` instructions

Scope: `crates/slatec-src/**`.

## Responsibility

This crate selects and configures implementation providers and exact native source closures. It owns source/system/external backend behaviour, but not raw Rust declaration design or safe numerical APIs.

## Rules

- Source acquisition is separate from ordinary builds. Builds must not silently access the network.
- Source inputs must be checksum-verified and traceable to the selected manifest/overlay.
- Preserve one independently extractable object per selected original source for the validated GNU MinGW source backend.
- Do not use source amalgamation, partial linking (`ld -r`), or whole-archive linking without explicit evidence, documentation, and regression tests.
- A broad family may build a large intermediate archive; that must not imply a large final executable.
- Feature closures must contain the exact required sources and genuine transitive dependencies—no unexplained unrelated families.
- Keep source, system, and external backend guarantees distinct. Do not claim source-backend guarantees for arbitrary external libraries.
- Do not modify pristine numerical sources in place. Any permitted overlay/correction must be deterministic, hash-guarded, and auditable.
- Do not commit source caches, objects, archives, binaries, maps, or compiler logs.
- Preserve global native-runtime locking/concurrency policy where required by the underlying routines.

## Required checks for provider changes

Run provider feature checks, source-cache verification, source-closure validation, archive-member/native-link audit, affected native regressions, external/system backend checks where available, deterministic metadata regeneration, and package-content audit.
