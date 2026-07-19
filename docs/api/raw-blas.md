# Reviewed raw BLAS API

`slatec-sys` R2A provides the feasible, historically user-callable BLAS
subroutines at stable canonical paths:

```rust
use slatec_sys::blas::level1::daxpy;
use slatec_sys::blas::level2::dgemv;
use slatec_sys::blas::level3::dgemm;
```

Enable `blas` for all levels, or `blas-level1`, `blas-level2`, or
`blas-level3` for a narrow declaration surface. With the source provider, use
the identically named `slatec-src` feature (normally through `slatec` with
`source-build`). `slatec-sys` itself never selects or bundles a provider.

## Calling contract

The reviewed ABI is GNU Fortran `x86_64-w64-mingw32`. All scalar and array
arguments are passed by mutable pointer because that is the native Fortran
calling convention, even when the routine treats an argument as input. Arrays
are column-major. Leading dimensions and increments must satisfy the exact
routine documentation generated on its canonical declaration; the caller owns
all buffers and native code does not retain them.

For every CHARACTER selector (`TRANS`, `UPLO`, `DIAG`, `SIDE`, and similar),
pass a one-byte `c_char` buffer, followed after all visible Fortran arguments
by one `FortranCharacterLength` value per selector. The generated declaration
orders those hidden lengths. Complex routines use `slatec_sys::Complex32`, the
reviewed GNU Fortran `COMPLEX` storage record.

Every raw call is `unsafe`: null pointers are not accepted for reviewed BLAS
arguments, all readable and writable extents must be valid, pointer aliasing
must obey the routine's mutation contract, and no panic may cross the FFI
boundary. See the generated declaration's `# Safety` section for each routine.

## Examples

The package includes direct raw examples rather than safe-wrapper examples:

```text
cargo run -p slatec --example raw_blas_level1_axpy --features source-build,blas
cargo run -p slatec --example raw_blas_level2_gemv --features source-build,blas
cargo run -p slatec --example raw_blas_level2_triangular_solve --features source-build,blas
cargo run -p slatec --example raw_blas_level3_gemm --features source-build,blas
cargo run -p slatec --example raw_blas_complex --features source-build,blas
```

Run these only on the supported GNU MinGW target with a verified source cache.
They cover vector operations, general matrix-vector multiplication,
triangular solving, general matrix multiplication, character hidden lengths,
and complex storage.

## Boundary and exclusions

The canonical declarations are re-exported at the legacy compatibility paths
`slatec_sys::families::blas_level1`, `blas_level2`, and `blas_level3`; no
second `extern` declaration is created. `slatec_sys::generated` remains
transitional and is not a stable raw namespace.

`CDCDOT`, `CDOTC`, and `CDOTU` are complex-valued Fortran functions and remain
explicitly excluded: the selected compiler's complex return ABI has not yet
been reviewed. The `MP*` entries classified with linear kernels are
multiprecision implementation subsidiaries rather than public BLAS routines.
Catalogue-only entries with no selected executable provider are reported as
such. The exact complete disposition is generated in
[`blas-family-report.json`](../../generated/raw-api/blas-family-report.json).
