# Changelog

All notable changes to this workspace will be documented here. The project uses
a coordinated pre-1.0 version while the raw ABI and selective safe facade are
stabilized.

## [0.1.0] - pending publication

This section is the release candidate prepared for the first coordinated
publication. No crate has been published and no tag has been created by this
repository yet.

### Added

- A layered architecture with provider-neutral raw declarations in
  `slatec-sys`, provider selection and native linking in `slatec-src`, shared
  contracts in `slatec-core`, and a selective safe facade in `slatec`.
- Canonical mathematical paths for 821 public raw routines, backed by complete
  terminal dispositions for all 1,517 retained corpus identities.
- One authoritative extern declaration per native symbol, with canonical paths
  and compatibility re-exports implemented as the same item.
- Generated routine, argument, family, documentation-quality,
  catalogue/export, declaration-ownership, feature/provider, and package
  readiness audits.
- Safe APIs for reviewed BLAS, special-function, quadrature, roots, nonlinear,
  least-squares, differential-equation, optimization, transform,
  interpolation, and structured linear/PDE families.

### Changed

- Canonical namespaces now remove empty navigation layers, treat dense,
  banded, packed, and sparse storage as siblings, and place eigenvalue routines
  under `linear_algebra::eigen`.
- `slatec-src` now owns the Cargo native `links = "slatec"` namespace;
  declaration-only `slatec-sys` has no build script and selects no provider.
- Routine navigation is split into a branching family overview, dedicated
  family pages with public routines first, and an alphabetical index.
- Runtime error-path tests validate child-process containment and documented
  GNU runtime outcomes instead of assuming one universal exit code.

### Known limitations

- GNU MinGW on `x86_64-pc-windows-gnu` is the strongest validated native ABI
  and runtime profile; other toolchains and providers require independent
  validation.
- The safe facade intentionally covers only reviewed families and does not wrap
  all 812 raw routines.
- Some raw argument semantics remain explicitly unavailable where selected
  source prologues do not provide separable evidence.
- Downloaded SLATEC source and prebuilt archives are not redistributed while
  source-rights review remains unresolved; source builds consume a separately
  acquired checksum-verified cache.

[0.1.0]: https://github.com/evnekdev/slatec-rs/releases/tag/v0.1.0
