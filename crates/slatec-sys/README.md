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

## Batch A generated public raw API

Batch A promotes the largest mechanically eligible group of historically
user-callable, non-callback numerical interfaces. These canonical paths have
an exact source hash, normalized ABI fingerprint, observed symbol, provider
closure, bulk compile/link coverage, and generated conservative Rustdoc. They
are not hand-reviewed semantic contracts, safe wrappers, or a numerical
validation claim. Callback, CHARACTER, complex-return, ambiguous, missing,
and subsidiary interfaces remain explicitly deferred. See
[`docs/api/raw-batch-a.md`](../../docs/api/raw-batch-a.md).

Use mathematical modules such as `special::numerical`,
`quadrature::numerical`, `linear_algebra::dense`, and
`interpolation::numerical`, not `generated::*`. A matching `batch-a-*`
`slatec-src` provider feature selects the verified native closure; this crate
itself stays provider-neutral.

## Batch B generated callback raw API

Batch B promotes 47 callback-bearing raw interfaces whose outer ABI and
callback ABI are source-reconstructed from the selected provider cache. The
new paths live under `quadrature::callbacks`,
`linear_algebra::sparse::callbacks`, and `ode::callbacks`; enable
`batch-b-quadrature`, `batch-b-linear-algebra`, `batch-b-ode`, or the matching
broad mathematical family alias. These are still unsafe raw callbacks, not
safe Rust closure wrappers. See
[`docs/api/raw-batch-b-callbacks.md`](../../docs/api/raw-batch-b-callbacks.md).

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

## Reviewed scalar special foundations

R2B promotes 40 historically user-callable scalar routines in the canonical
`slatec_sys::special::{elementary,gamma,beta,error}` modules. Enable the
matching `special-*` feature or `special`; the previous
`slatec_sys::families::special_{group}` paths remain compatibility re-exports.
The functions use direct `f32`/`f64` GNU MinGW scalar returns and scalar
arguments passed by address. Their generated Rustdoc records the source,
native symbol, domains, XERROR/FNLIB behavior, and raw Safety contract.

FNLIB saved initialization and the legacy error runtime are process-global;
raw callers must synchronize concurrent calls as needed. See
[`docs/api/raw-special-foundations.md`](../../docs/api/raw-special-foundations.md)
for direct-call and provider guidance.

## Reviewed real Airy functions

R2C promotes the eight real FNLIB Airy drivers at
`slatec_sys::special::airy::{ai,aie,bi,bie,dai,daie,dbi,dbie}`. Enable
`special-airy`; `slatec_sys::families::special_airy::*` remains a compatibility
re-export. These unsafe scalar functions use the same direct GNU MinGW return
ABI and by-address input contract as the reviewed scalar foundations. Complex
Amos Airy drivers and Airy subsidiaries are intentionally not promoted; see
[`docs/api/raw-special-airy.md`](../../docs/api/raw-special-airy.md).

## `all` declaration aggregate

The `all` feature directly selects every authored public mathematical family
aggregate. It is declaration-only and deliberately does not select a source,
system, or external provider. Its exact coverage is checked by
[`generated/raw-api/all-feature-coverage.json`](../../generated/raw-api/all-feature-coverage.json).

## Native link granularity

Raw declaration features expose Rust names only. They neither require all
native symbols nor cause `slatec-sys` to emit a native link directive. With the
GNU MinGW `slatec-src` source provider, the final linker extracts only archive
members needed by referenced symbols and their transitive closure. The audited
reports and the limits for external providers are documented in
[`docs/architecture/native-link-granularity.md`](../../docs/architecture/native-link-granularity.md).
The safe facade has an additional operation-granularity policy; it does not
change raw declaration or provider-feature semantics. See
[`docs/architecture/safe-facade-link-granularity.md`](../../docs/architecture/safe-facade-link-granularity.md).
