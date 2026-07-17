# Family features and native providers

The safe API and its implementation provider are selected independently. A
narrow family feature compiles only its safe Rust module and activates the
reviewed raw declarations and native dependency closure for that family.

## Public families

Broad aliases expand to the narrow features below; there is no feature per
routine.

| Domain alias | Narrow features |
| --- | --- |
| `blas` | `blas-level1`, `blas-level2`, `blas-level3` |
| `special` | `special-elementary`, `special-gamma`, `special-beta`, `special-error`, `special-airy`, `special-bessel`, `special-integrals`, `special-polynomials` |
| `quadrature` | `quadrature-basic`, `quadrature-breakpoints`, `quadrature-weighted`, `quadrature-oscillatory`, `quadrature-fourier`, `quadrature-nonadaptive` |
| `roots` | `roots-scalar`, `roots-polynomial` |

`roots-polynomial` remains an explicit deferred empty family: no polynomial
root wrapper is exposed until its interface validation gate is cleared.

## Capability layers

- Core-only APIs use `core` and caller-owned slices. They compile with
  `--no-default-features` plus the relevant family and an explicit backend.
- `alloc` enables Rust's standalone `alloc` crate. It does not enable or
  require `std`; a `no_std` consumer supplies an allocator.
- `std` enables `alloc` and is required by current callback containment,
  process-global native serialization, and FNLIB error-state handling.

The Rust API can therefore be `no_std` compatible while the selected native
backend remains hosted. No bare-metal execution has been validated.

## Backends

Exactly one backend must be selected whenever a numerical family is enabled.

- `bundled` is the hosted default. It downloads individual source files from
  their recorded Netlib URLs, verifies every SHA-256, compiles the enabled
  family closure, applies the GNU MinGW machine-constant overrides, and emits
  static link directives. It does not distribute the acquired bytes.
- `source-build` performs the same compilation from a verified
  `SLATEC_SOURCE_CACHE` and never accesses the network.
- `system` links a system archive. `SLATEC_SYSTEM_LIB_DIR` and
  `SLATEC_SYSTEM_LIB_NAME` are optional explicit overrides.
- `external-backend` emits no native build or link instruction. It is intended
  for applications whose final link is managed outside Cargo.

Incompatible combinations fail in `slatec-src/build.rs` with a direct error.
`slatec-src` itself is a `#![no_std]` provider-selection crate and exposes no
numerical API; only its build script uses hosted acquisition and compiler
facilities.

The automatic source builder currently supports the validated GNU Fortran
`x86_64-w64-mingw32` profile. Other targets must select `system` or
`external-backend`. The default provider requires a compatible GNU Fortran
compiler to build from source; it does not require a preinstalled SLATEC
archive, manual linker flags, or a SLATEC DLL.

## Static retention

Every physical selected source is compiled to a separate object and archived
without whole-archive linking. The platform linker therefore extracts only
objects referenced by the program and their transitive support closure. The
compiler-observed closure and five native binary checks are recorded under
`generated/linkage/`. The single-gamma check confirms that Airy, Bessel,
quadrature, root, and BLAS entry points are not retained.

## Examples

```toml
[dependencies]
slatec = { version = "0.1", features = ["special-gamma"] }
```

For a `no_std` consumer that supplies the final native link:

```toml
[dependencies]
slatec = { version = "0.1", default-features = false, features = ["external-backend", "blas-level1"] }
```
