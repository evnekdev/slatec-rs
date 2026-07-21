# slatec-sys

Profile-gated raw declarations for the selected SLATEC Fortran corpus. This crate performs no source acquisition, compilation, or linking.

## Public raw API status

ABI-shaped generated declaration modules are private implementation details.
Their existence does not mean a routine has complete semantics or documentation
review. Reviewed routines have one canonical mathematical path.

The deterministic inventory in `generated/raw-api/` separates retained
identities, generated candidates, ABI-validated declarations, reviewed drivers
and subsidiaries, provider/link/runtime/documentation coverage, safe wrappers,
and explicit exclusions. Its correction layer is source-hash guarded. See
[`docs/architecture/slatec-sys-public-raw-api.md`](../../docs/architecture/slatec-sys-public-raw-api.md).

## Canonical generated raw API

The canonical surface contains 812 routines. Straightforward numerical
interfaces are source-hash and compiler-ABI guarded; callback-bearing
interfaces additionally carry source-reconstructed callback fingerprints; and
complex, fixed `CHARACTER*1`, and `LOGICAL` interfaces use controlled GNU
MinGW ABI probes. Long or variable strings and unresolved combined ABIs remain
excluded.

Use mathematical modules such as `special`, `quadrature`,
`linear_algebra::{dense,banded,packed,sparse,eigen}`, `interpolation`, and
`pde::fishpack`, not ABI-shaped implementation modules. Provider features use the same
mathematical family names. The callback paths under `quadrature::callbacks`,
`linear_algebra::sparse::callbacks`, and `ode::callbacks` remain unsafe raw
callbacks rather than safe Rust closure wrappers.

Every public path re-exports one authoritative extern item. The terminal
disposition for all 1,517 retained identities is in the
[final coverage guide](../../docs/api/raw-api-final-coverage.md) and
[`final-disposition.json`](../../generated/raw-api/final-disposition.json).

## Canonical BLAS API

The feasible, historically user-callable BLAS corpus uses the canonical paths
`slatec_sys::blas::level1`,
`slatec_sys::blas::level2`, and `slatec_sys::blas::level3`. Enable `blas` or
one of `blas-level1`, `blas-level2`, and `blas-level3`; the corresponding
`slatec-src` feature selects the native source closure when a source provider
is used.

For the supported GNU profile, CHARACTER selector arguments are passed as
one-byte buffers followed by trailing `FortranCharacterLength` values. Complex
arguments use the documented `Complex32` storage record. Complex-valued
Fortran function returns (`CDCDOT`, `CDOTC`, and `CDOTU`) are available after
independent controlled and selected-source return-ABI probes. See
[`docs/api/raw-blas.md`](../../docs/api/raw-blas.md) for direct-call examples
and the full ABI contract.

## Scalar special foundations

Forty historically user-callable scalar routines use the canonical
`slatec_sys::special::{elementary,gamma,beta,error}` modules. Enable the
matching `special-*` feature or `special`.
The functions use direct `f32`/`f64` GNU MinGW scalar returns and scalar
arguments passed by address. Their generated Rustdoc records the source,
native symbol, domains, XERROR/FNLIB behavior, and raw Safety contract.

FNLIB saved initialization and the legacy error runtime are process-global;
raw callers must synchronize concurrent calls as needed. See
[`docs/api/raw-special-foundations.md`](../../docs/api/raw-special-foundations.md)
for direct-call and provider guidance.

## Real Airy functions

The eight real FNLIB Airy drivers are available at
`slatec_sys::special::airy::{ai,aie,bi,bie,dai,daie,dbi,dbie}`. Enable
`special-airy`. These unsafe scalar functions use the same direct GNU MinGW return
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

## Packaging and support profile

This crate ships declarations and generated Rust metadata only. It contains no
downloaded SLATEC corpus, native object, archive, or provider selection and has
no build script or Cargo `links` identity. `slatec-src` owns provider integration.
GNU MinGW on `x86_64-pc-windows-gnu` is the strongest validated native ABI;
other providers and platforms remain experimental until independently audited.
