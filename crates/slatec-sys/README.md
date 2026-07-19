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
