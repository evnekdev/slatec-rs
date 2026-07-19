# `slatec-sys` public raw API

## Goal and evidence boundary

`slatec-sys` is the unsafe, profile-specific Rust boundary for feasible SLATEC
program units. It does not download, compile, or bundle Fortran. A callable
routine is only stable after the raw API inventory records a selected provider,
source hash, native symbol, reviewed declaration, canonical Rust path, feature
and provider feature, routine and argument documentation, and the required
validation evidence.

The generator creates exactly one record for every retained catalogue identity:
[`generated/raw-api/routine-status.json`](../../generated/raw-api/routine-status.json).
It replaces the ambiguous historical aggregate with separate,
reproducible coverage metrics in
[`coverage-summary.json`](../../generated/raw-api/coverage-summary.json).

The currently supported ABI is the observed GNU Fortran
`x86_64-w64-mingw32` profile, exposed by
`ffi-profile-gnu-mingw-x86_64`. It is not a portability promise. `INTEGER` and
`LOGICAL` are `i32`; hidden CHARACTER lengths use `usize`; and the observed
lowercase-underscore symbol spelling is part of the profile evidence.

## Status model

Every record has one `raw_api_state`. The finite states distinguish reviewed
drivers and subsidiaries from generated candidates, ABI-validated generated
declarations, source-present-but-unbound routines, callback and special-return
ABI blockers, conflicting/ambiguous/missing symbols, non-callable subsidiaries,
runtime support, block data, documentation/tooling entries, external
dependencies, and catalogue-only identities. The machine-readable enumeration
is `reviewed_public_driver`, `reviewed_public_subsidiary`,
`generated_candidate`, `generated_abi_validated`, `source_present_unbound`,
`unsupported_callback_abi`, `unsupported_complex_return_abi`,
`unsupported_character_return_abi`, `unsupported_entry_or_alternate_return`,
`conflicting_interface`, `ambiguous_symbol`, `missing_symbol`,
`not_independently_callable`, `runtime_or_machine_support`, `block_data`,
`documentation_or_tooling`, `catalogue_only`, and `external_dependency`.

`generated_abi_validated` says that a declaration shape belongs to a successful
compiler/profile batch. It does **not** say that its callback lifetime, pointer
semantics, workspace, error handling, or public documentation have been
reviewed. Only the hash-guarded authored correction layer can emit
`reviewed_public_driver` or `reviewed_public_subsidiary`.

## Canonical namespace

The long-term taxonomy is:

```text
blas                         linear_algebra::{dense,banded,packed,sparse}
eigen                        roots::{scalar,polynomial}
nonlinear                    least_squares
optimization                 quadrature
ode                           dae
pde::fishpack                fftpack
interpolation                approximation
special                      statistics
integral_equations           runtime
```

`module-taxonomy.json` maps every retained identity to one intended module.
R1 creates only the namespaces needed for reviewed paths:

```rust
slatec_sys::roots::scalar::{fzero, dfzero}
slatec_sys::pde::fishpack::{hwscrt, pois3d}
```

R2A additionally establishes stable public BLAS paths:

```rust
slatec_sys::blas::level1::daxpy
slatec_sys::blas::level2::dgemv
slatec_sys::blas::level3::dgemm
```

The complete feasible BLAS set is generated from the source-hash-guarded
family-review policy rather than copied into hand-written `extern` blocks.
Each canonical item re-exports the single ABI-shaped generated declaration;
`slatec_sys::families::blas_level{1,2,3}` are compatibility re-exports of
that same item. The public surface therefore has no duplicate declarations.
The companion [`blas-family-report.json`](../../generated/raw-api/blas-family-report.json)
records all retained BLAS classifications, including excluded complex-return
functions, non-BLAS multiprecision subsidiaries, and catalogue-only entries.

The legacy `slatec_sys::roots::{fzero,dfzero}`,
`slatec_sys::fishpack_cartesian_2d::hwscrt`, and
`slatec_sys::fishpack_pois3d::pois3d` paths are compatibility re-exports. A
routine has one declaration: compatibility paths re-export it and never add a
second `extern` item.

Drivers are organized before subsidiaries. The recorded roles are historically
user-callable driver, expert public primitive, internal subsidiary, shared
utility, runtime support, and not independently callable. Future promoted
subsidiaries belong below an explicit `subsidiary` namespace rather than beside
principal drivers.

## Documentation and correction contract

Every reviewed raw declaration must have a one-sentence purpose, original
routine name, precision, mathematical operation, selected source/provider,
supported ABI profile, native symbol, complete argument list, I/O and mutation
semantics, array extents and leading dimensions, workspace rules, callback
contract where relevant, status meanings, global-state notes, and a `# Safety`
section. Pointer arguments explicitly state nullability, readable/writable
extent, direction, aliasing, and retention. Missing intent is recorded as
unavailable rather than inferred from an argument name.

Documentation fields record evidence as `source_prologue`,
`executable_declaration`, `selected_provider_metadata`,
`compiler_observation`, `authored_override`, `safe_wrapper_audit`, or
`unavailable`. The deterministic correction registry is
[`metadata/raw-api-corrections.json`](../../metadata/raw-api-corrections.json).
It is intentionally small, keyed by stable identity and selected source SHA-256,
and generation rejects it when the source hash or executable argument order
changes.

The deterministic documentation audit reports public extern declarations,
routine and argument documentation, Safety sections, source links, ABI-profile
statements, and exact missing fields. The raw API review queue is therefore a
work plan, not generated prose with guessed semantics.

## Features, providers, and validation

The raw declaration feature in `slatec-sys`, the native closure feature in
`slatec-src`, and an optional safe facade feature in `slatec` are joined in
[`feature-provider-map.json`](../../generated/raw-api/feature-provider-map.json).
Reviewed records fail validation when any required feature is absent. A raw
consumer may use `slatec-sys` with an explicit external provider; `slatec-sys`
does not select a backend implicitly.

Validation levels are separate: source/hash and catalogue checks, compiler and
symbol observation, declaration signature audit, compile-only canonical-path
imports, native link validation, runtime validation, and safe-wrapper tests.
The R1 command is:

```text
cargo run -p slatec-tools --bin slatec-corpus -- generate-raw-api-inventory --offline
cargo run -p slatec-tools --bin slatec-corpus -- validate-raw-api-inventory --offline
```

Validation rejects a reviewed entry lacking its source hash, unique observed
symbol, executable argument order, ABI/profile evidence, unique canonical path,
raw and provider feature, complete docs, or required link/runtime status.

## Stability and transition policy

For `0.1.x`, reviewed canonical paths and routine names do not move. Features
are additive where practical and path corrections retain compatibility
re-exports. Evidence-proven ABI corrections may change a signature because an
incorrect FFI declaration is a safety bug. The ABI-shaped
`slatec_sys::generated` namespace remains available during 0.x, but is
transitional, unstable, and never the sole stable path for a promoted routine.

An excluded identity has a machine-readable reason. The R2 promotion queue is
evidence-ranked in `promotion-priority.json`; R3 is reserved for callback
drivers, R4 for exceptional return and CHARACTER/complex interfaces, and R5
for the publication freeze audit. `roots-family-report.json` is the required
pilot gap report; HWSCRT and POIS3D remain tracked reviewed drivers with their
selected closures and native regression evidence.

## BLAS R2A evidence and ABI boundary

The reviewed BLAS policy covers 121 historically user-callable subroutines
whose GNU Fortran declaration shapes and native symbols were validated in the
numeric-scalar, numeric-array, scalar-function, complex-argument, and
CHARACTER ABI batches. The policy is keyed to a manifest of selected provider
source hashes; any source drift stops regeneration before a reviewed path can
be emitted.

BLAS character options are native one-byte CHARACTER buffers followed by the
compiler-observed trailing hidden length values (`FortranCharacterLength`), in
visible argument order. `Complex32` documents the selected GNU Fortran
COMPLEX storage record. The three complex-return functions are deliberately
not declared because a correct Rust return ABI has not been established.
Routine documentation is generated from verified prologue facts plus a small
authored BLAS operation template, and its audit checks every exported routine,
argument, source link, ABI statement, and `# Safety` section.
