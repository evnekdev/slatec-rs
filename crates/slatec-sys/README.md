# slatec-sys

Profile-gated raw declarations for the selected SLATEC Fortran corpus. This crate performs no source acquisition, compilation, or linking.

## Public raw API status

`slatec_sys::generated` is transitional, ABI-shaped generated access. It is not
a stable namespace and the existence of a generated declaration does not mean
the routine has complete semantics or documentation review. Reviewed routines
have one canonical mathematical path and retain old paths as re-exports.

The deterministic inventory in `generated/raw-api/` separates retained
identities, generated candidates, ABI-validated declarations, reviewed drivers
and subsidiaries, provider/link/runtime/documentation coverage, safe wrappers,
and explicit exclusions. Its correction layer is source-hash guarded. See
[`docs/architecture/slatec-sys-public-raw-api.md`](../../docs/architecture/slatec-sys-public-raw-api.md).

## Reviewed BLAS API

R2A promotes the feasible, historically user-callable BLAS corpus to the
stable canonical paths `slatec_sys::blas::level1`,
`slatec_sys::blas::level2`, and `slatec_sys::blas::level3`. Enable `blas` or
one of `blas-level1`, `blas-level2`, and `blas-level3`; the corresponding
`slatec-src` feature selects the native source closure when a source provider
is used. The compatibility modules
`slatec_sys::families::blas_level{1,2,3}` re-export these same declarations.

For the supported GNU profile, CHARACTER selector arguments are passed as
one-byte buffers followed by trailing `FortranCharacterLength` values. Complex
arguments use the documented `Complex32` storage record. Complex-valued
Fortran function returns (`CDCDOT`, `CDOTC`, and `CDOTU`) are explicitly
excluded until a return ABI is independently reviewed. See
[`docs/api/raw-blas.md`](../../docs/api/raw-blas.md) for direct-call examples
and the full ABI contract.
