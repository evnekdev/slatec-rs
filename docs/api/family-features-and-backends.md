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
| `nonlinear` | `nonlinear-easy` |

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

## Backends and offline policy

Exactly one backend must be selected whenever a numerical family is enabled.

- `prebuilt` means a target-specific packaged archive requiring no network or
  compiler. It currently fails with a rights-blocked diagnostic: no historical
  SLATEC archive is published until source and binary redistribution are
  cleared.
- `source-build` compiles from `SLATEC_SOURCE_CACHE` and never accesses the
  network. Populate that cache explicitly with
  `slatec-corpus acquire-provider-sources`; the build fails with the exact
  command when the cache is absent or invalid.
- `system` links an explicitly located validated static archive using
  `SLATEC_SYSTEM_LIB_DIR` and optional `SLATEC_SYSTEM_LIB_NAME`. The supported
  GNU profile also requires `SLATEC_SYSTEM_RUNTIME_LIB_DIR` containing static
  `libgfortran.a` and `libquadmath.a`.
- `external-backend` emits no native build or link instruction. It is intended
  for applications whose final link is managed outside Cargo.
- `bundled` is a deprecated compatibility alias with exactly the offline
  `source-build` meaning. It never downloads source.

Incompatible combinations fail in `slatec-src/build.rs` with a direct error.
`slatec-src` itself is a `#![no_std]` provider-selection crate and exposes no
numerical API. Its build script contains no network client.

The source builder currently supports the validated GNU Fortran 14.2.0
`x86_64-w64-mingw32` profile. Other compiler versions are not claimed as
validated merely because their target string matches. Other targets must select
`system` or `external-backend`. No native backend is selected by default while
prebuilt publication is blocked.

Explicit acquisition from a repository checkout:

```text
cargo run -p slatec-tools --bin slatec-corpus -- acquire-provider-sources --output-dir evidence/provider-sources
set SLATEC_SOURCE_CACHE=evidence/provider-sources
cargo build --offline --no-default-features --features std,source-build,special-gamma --target x86_64-pc-windows-gnu
```

The source build statically links `libgfortran` and `libquadmath`. The tested
consumer has no GNU runtime DLL imports, but static `libquadmath` introduces
LGPL redistribution obligations and `libgfortran` remains governed by its exact
GPL/GCC Runtime Library Exception notices. See `generated/licensing/`.

## Static retention

Every physical selected source is compiled to a separate object and archived
without whole-archive linking. The platform linker therefore extracts only
objects referenced by the program and their transitive support closure. The
compiler-observed closure and eleven native binary checks are recorded under
`generated/linkage/`. Narrow examples check their own family roots and reject
unrelated public family roots while permitting required support symbols.

## Examples

```toml
[dependencies]
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "special-gamma"] }
```

For a `no_std` consumer that supplies the final native link:

```toml
[dependencies]
slatec = { version = "0.1", default-features = false, features = ["external-backend", "blas-level1"] }
```
