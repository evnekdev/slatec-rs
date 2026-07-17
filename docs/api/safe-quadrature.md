# Safe adaptive quadrature

The `slatec::quadrature` module is a safe, closure-based facade over the
original SLATEC/QUADPACK `QAG`, `QAGS`, `QAGI`, and `QAWC` Fortran routines in
single and double precision. No integration algorithm is implemented in Rust.

This API is available only for the validated `ffi-profile-gnu-mingw-x86_64`
profile: GNU Fortran targeting `x86_64-w64-mingw32`. The runtime-profile
machine constants, legacy errors, and FNLIB initialization must match the
selected-corpus snapshot. Cargo does not download or compile Fortran.

## API families

- `integrate` and `integrate_f32`: finite intervals with a selectable
  Gauss-Kronrod rule (`DQAG`/`QAG`).
- `integrate_singular` and `integrate_singular_f32`: finite intervals where an
  integrable endpoint singularity benefits from extrapolation (`DQAGS`/`QAGS`).
- `integrate_infinite` and `integrate_infinite_f32`: upper, lower, or two-sided
  infinite intervals (`DQAGI`/`QAGI`).
- `integrate_principal_value` and its single-precision counterpart: Cauchy
  principal values with the singular point strictly inside the interval
  (`DQAWC`/`QAWC`).

Weighted oscillatory, breakpoint-array, and expert workspace interfaces remain
deferred. The safe layer never exposes raw work arrays.

```rust,no_run
use slatec::quadrature::{integrate, IntegrationOptions};

let result = integrate(
    |x| x * x,
    0.0,
    1.0,
    IntegrationOptions::default(),
)?;
assert!((result.value - 1.0 / 3.0).abs() < 1.0e-10);
# Ok::<(), slatec::quadrature::IntegrationError>(())
```

## Callback and concurrency policy

The selected Fortran interfaces accept a function pointer but no user-data
pointer. Rust therefore installs a scoped, thread-local callback context while
holding a shared native-runtime lock. The trampoline uses `catch_unwind`; a
panic never crosses Fortran frames. A panic or non-finite callback result is
recorded, subsequent callback evaluations return a finite sentinel, and the
safe call reports `CallbackPanicked` or `CallbackFailed` after Fortran returns.

The context is cleared by an RAII guard on every Rust return path. Parallel
callers are serialized because the selected native runtime has process-global
error state. Nested callback-based integration is rejected before taking the
lock, avoiding deadlock. The integrand may call runtime-validated non-callback
SLATEC functions because the native lock is reentrant on its owning thread.

The sentinel strategy cannot force QUADPACK to return immediately. A failed
callback may therefore incur additional native evaluations before the Rust
error is returned, but its result is never presented as a successful integral.

## Validation and errors

The wrapper rejects non-finite finite bounds, invalid tolerances, a zero or
unrepresentable subdivision limit, and a principal-value point outside the
open interval. Reversed finite bounds are supported. Workspace allocation uses
checked `4 * limit` arithmetic and checked conversion to the profile's Fortran
`INTEGER`.

Native statuses are mapped without claiming more detail than QUADPACK reports:
maximum subdivisions, roundoff, bad integrand behavior, non-convergence, and
probable divergence or slow convergence. Native status 6 represents invalid
input; ordinary causes are prevented by the Rust checks. Callback NaN and
infinity are errors rather than integrand values.

`IntegrationOptions::default()` targets double precision. Single-precision
calls should use `IntegrationOptions::single_precision()` or request another
realistic tolerance; the safe layer rejects tolerances below the QAG-family
machine-precision rule.

## Native setup

Build the selected archive explicitly, then run native tests for the GNU target:

```text
cargo run -p slatec-tools --bin slatec-corpus -- build-native-ffi --offline
$env:SLATEC_NATIVE_LIB_DIR = "<ignored native archive directory>"
$env:SLATEC_GFORTRAN_RUNTIME_DIR = "<GNU Fortran runtime directory>"
cargo test -p slatec --features quadrature-native-tests --target x86_64-pc-windows-gnu --test quadrature_native
```

Source-only builds and tests require neither a Fortran compiler nor a native
archive. Objects, archives, executables, and detailed logs remain ignored.
