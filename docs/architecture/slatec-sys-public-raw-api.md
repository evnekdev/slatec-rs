# `slatec-sys` public raw API

## Purpose and evidence boundary

`slatec-sys` is the unsafe, provider-neutral Rust declaration layer for
feasible SLATEC program units. It never downloads, compiles, selects, or links
a Fortran implementation. Provider selection belongs to `slatec-src` or to the
final application.

The authoritative inventory has one record for each of the 1,517 retained
identities. It distinguishes 812 `canonical-public` routines from internal
subsidiaries, support units, and evidence-backed exclusions. Current evidence
is generated under [`generated/raw-api`](../../generated/raw-api/) and the
direct public inventory is
[`canonical-public-api.json`](../../generated/public-api/canonical-public-api.json).

## Supported ABI profile

The validated native profile is GNU Fortran on `x86_64-pc-windows-gnu`, selected
by `ffi-profile-gnu-mingw-x86_64`. `INTEGER` and `LOGICAL` use 32-bit storage,
`LOGICAL` is not Rust `bool`, hidden `CHARACTER` lengths use `usize`, and native
names use the observed lowercase-underscore convention. Complex return layout
and callback signatures are public only when selected-source and compiler
evidence agree. This profile is not a portability promise.

## One declaration owner and one public path

Every public native symbol has one authoritative Rust `extern` declaration in a
private implementation module. A mathematical module re-exports that one item:

```text
private authoritative declaration
            -> one canonical mathematical public path
```

The generated
[`ffi-declaration-ownership.json`](../../generated/public-api/ffi-declaration-ownership.json)
records declaration ownership, ABI fingerprints, features, and the one-path
invariant. Validation rejects duplicate link names, incompatible signatures,
public ABI-shaped declaration modules, and more than one public path for a
routine.

The crate is unpublished. Correcting a pre-release namespace removes the old
path directly; it does not retain a shadow API. Historical cleanup evidence is
kept outside the current contract under [`generated/history`](../../generated/history/).

## Canonical mathematical namespace

Public paths describe mathematics, storage, or problem structure:

```rust
slatec_sys::blas::{level1, level2, level3}
slatec_sys::linear_algebra::{dense, banded, packed, sparse, eigen}
slatec_sys::roots::{scalar, polynomial, complex}
slatec_sys::nonlinear
slatec_sys::least_squares
slatec_sys::optimization
slatec_sys::quadrature
slatec_sys::ode
slatec_sys::dae
slatec_sys::pde::fishpack
slatec_sys::fftpack
slatec_sys::interpolation
slatec_sys::approximation
slatec_sys::special
slatec_sys::statistics
slatec_sys::integral_equations
slatec_sys::runtime
```

Examples include:

```rust
slatec_sys::roots::scalar::fzero
slatec_sys::roots::complex::cpzero
slatec_sys::linear_algebra::eigen::imtql2
slatec_sys::pde::fishpack::pois3d
```

Private ABI or generator grouping modules are implementation details. They are
not stable or supported import paths.

## Public status and role model

Public reports use these terminal dispositions:

- `canonical-public` — callable raw routine with a canonical path and complete
  structural evidence;
- `internal-subsidiary` and `support-routine` — provider internals not exposed
  as user-callable raw functions;
- `historical-program` and `demonstration-program` — retained non-library
  program records;
- `catalogue-only`, `missing-symbol`, `unsupported-abi`, and
  `not-independently-callable` — explicit non-public outcomes.

Historical source classifications remain provenance, not an unresolved public
status. A historically older algorithm can still be `canonical-public` when
its source, ABI, symbol, provider closure, and documentation are sound.

Roles are recorded separately as historically user-callable driver, expert
public primitive, internal subsidiary, shared utility, runtime support, or not
independently callable. Drivers receive the primary family placement.

## Documentation contract

Every canonical public routine records a purpose, original name, precision,
mathematical operation, selected provider, source hash, native symbol, feature,
supported ABI profile, and ABI fingerprint. Its ordered argument records cover
Fortran and Rust types, shape, direction, semantic description, dimensions,
leading dimensions, workspaces, nullability, callback requirements, mutation,
aliasing, retention, state, status, and safety obligations.

Source-prologue facts remain distinct from executable-declaration facts. Unknown
semantic information is explicit; it is never guessed from a parameter name.
Narrow authored corrections are source-hash guarded and live in metadata.
Generated Rust and report files are never hand edited.

## Features and providers

Public `slatec-sys` features name mathematical families and expose declarations
only. `slatec-sys/all` is an additive declaration aggregate and never selects a
provider. `slatec-src` owns source/system/external provider selection and the
`links = "slatec"` namespace. Applications can instead provide native symbols
from a separately configured backend.

Feature/provider reconciliation rejects orphan declaration features, missing
provider closures, duplicate providers, and mismatched feature names.

## Validation and release policy

Validation is layered: catalogue and source-hash identity; compiler-observed
ABI and symbol; unique declaration ownership; canonical-path compile probes;
native link and representative runtime probes; documentation coverage; feature
reconciliation; deterministic regeneration; and package-content checks.

For the first published release, canonical paths and routine names establish
the compatibility baseline. Features should be additive where practical. An
evidence-proven unsafe signature correction remains permitted because an
incorrect ABI declaration is a safety and correctness defect. No generated
implementation path is stable merely because it compiles.

Release readiness requires every retained identity to reconcile, one canonical
path for every public routine, complete public argument and Safety contracts,
provider/package coherence, reproducible evidence, and the documented portable
and GNU MinGW validation matrices. It does not require every raw routine to
have a safe wrapper or a runtime test.
