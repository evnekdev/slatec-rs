# Batch C complex and flag-bearing public raw interfaces

Batch C promotes 97 historically public SLATEC routines after classifying all
retained interfaces that contain complex, `CHARACTER`, or `LOGICAL` values.
It is a generated unsafe tier: every promoted routine has a selected source
hash, an observed symbol, a normalized ABI fingerprint, a canonical
mathematical path, coherent declaration and provider features, a compile probe,
and a native link probe.

The authoritative outputs are under
[`generated/raw-api/`](../../generated/raw-api/):

- `batch-c-classification.json` has exactly one Batch C classification for each
  of the 1,517 retained identities;
- `batch-c-candidates.json` lists the 97 new public declarations;
- `batch-c-exclusions.json` preserves explicit deferment reasons;
- `batch-c-abi-evidence.json` records the supported compiler-profile results;
- `batch-c-provider-reconciliation.json` checks exact provider closures;
- `batch-c-summary.md` gives reproducible counts; and
- `batch-c-manifest.json` guards deterministic regeneration.

## Supported ABI profile

The guarantees apply to GNU Fortran targeting `x86_64-pc-windows-gnu`. They do
not establish compatibility with an archive built by a different Fortran
compiler, compiler version, target, or ABI mode. System and externally managed
providers must independently match this profile.

`COMPLEX` and `DOUBLE COMPLEX` use the crate's `#[repr(C)]` `Complex32` and
`Complex64` records, with real followed by imaginary storage. Controlled probes
verify size, alignment, scalar and array mutation, and both precisions. Separate
selected-source Rust probes call `CDOTU` and `CDCDOT`. On the supported profile,
complex function values use the compiler-observed direct aggregate return; no
hidden result pointer or guessed C shim is introduced.

Raw `LOGICAL` uses `FortranLogical = i32`, never Rust `bool`. The probe verifies
by-reference scalar input, `.FALSE.` as zero, `.TRUE.` as one, array element
layout, output mutation, and logical function return.

Raw character flags remain byte pointers plus trailing
`FortranCharacterLength = usize` values. Probes cover one and multiple character
dummies and an ordinary trailing integer, establishing that all hidden lengths
follow ordinary arguments in dummy order. Batch C accepts only fixed
`CHARACTER*1` option-style values. Longer or variable strings, filenames,
formatted output, returned strings, and unresolved mutability remain deferred.

## Public paths and features

Mathematics determines the public path; ABI shape only determines generation
and validation. Representative paths are:

```rust
slatec_sys::blas::level1::cdotu
slatec_sys::linear_algebra::dense::complex::cgefa
slatec_sys::special::complex::cairy
slatec_sys::nonlinear::complex::cminpack
slatec_sys::pde::fishpack::complex::cmgnbn
```

The generated declaration owners live privately under `batch_c`; public users
should use canonical mathematical modules. The ABI-shaped `generated` module
remains transitional. Existing paths are retained as re-exports where a routine
was already public, and no independent duplicate `extern` declaration is added.

Declaration features are the coherent `batch-c-blas`,
`batch-c-linear-algebra`, `batch-c-special`, `batch-c-nonlinear`, and
`batch-c-fishpack` groups. `slatec-sys/all` reaches them through mathematical
family aliases and remains declaration-only. Matching `slatec-src` features
select only generated source dependency closures; a direct raw consumer may
instead supply ABI-compatible external symbols.

## Callback and safety boundary

A combined callback interface is eligible only when Batch B already proves the
callback call shape. Batch C does not redesign callback aliases. Rust callbacks
must use the exact generated ABI, must not unwind through Fortran, must not
retain borrowed pointers, and must obey process-global native-state constraints.

All Batch C calls are unsafe. Callers must provide non-null, aligned pointers
with the complete source-defined readable or writable extent; honor array rank,
stride, leading dimensions, workspace, option values, and aliasing rules; link a
matching provider; and serialize access where SLATEC global state requires it.
Generated conservative argument documentation states ABI obligations but does
not invent missing semantic intent or constitute a safe wrapper.

## Validation and exclusions

Regenerate and validate with:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-batch-c --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-batch-c --offline
```

When only Batch C evidence is stale after an inventory change, use
`generate-raw-batch-c-reports --offline`. It refreshes report files without
rewriting canonical declarations, probes, or reviewed Rustdoc.

Two generated compile batches import all canonical paths, and two native link
batches reference every promoted symbol. Representative native tests exercise
complex returns, extended-precision complex accumulation, complex matrix
mutation, and status-bearing complex output. The raw-FFI profile suite supplies
the controlled complex64, character-hidden-length, and logical runtime probes.
These levels prove ABI crossing and plausible mutation, not numerical validation
of every promoted routine.

Batch D retains long or variable character values, unresolved text/I/O
contracts, unsupported callback combinations, `ENTRY` or alternate returns,
provider-missing identities, compiler descriptors, and any future complex or
logical kind that lacks direct compiler evidence. Earlier exclusion history is
retained in the classification rather than overwritten.
