# Safe scalar and single-precision polynomial root finding

`slatec::roots` is an opt-in safe facade over original SLATEC scalar and
single-precision polynomial-root routines. It does not implement a
root-finding algorithm in Rust.

The module is restricted to the validated
`ffi-profile-gnu-mingw-x86_64` profile: GNU Fortran targeting
`x86_64-w64-mingw32`. Cargo neither downloads nor compiles Fortran. Build the
selected native archive explicitly before executing native tests.

## Scalar API

```rust,no_run
use slatec::roots::{find_root, RootBracket, RootOptions, RootStatus};

let result = find_root(
    |x| x * x - 2.0,
    RootBracket { lower: 1.0, upper: 2.0 },
    RootOptions::default(),
)?;
assert!((result.estimate - 2.0_f64.sqrt()).abs() < 1.0e-9);
assert_eq!(result.status, RootStatus::Converged);
# Ok::<(), slatec::roots::RootError>(())
```

`find_root_f32` is the matching `FZERO` entry point. Use
`RootOptions::single_precision()` for practical single-precision defaults.

The wrapper requires finite, distinct endpoints and finite non-negative
relative and absolute tolerances. It evaluates both endpoints before entering
Fortran: an exact endpoint root returns immediately, and all other calls
require an opposite sign without multiplying the two values. Reversed
endpoints are accepted. An explicit initial guess must be finite and strictly
inside the interval; otherwise the wrapper returns `InvalidInitialGuess`.
When no initial guess is supplied, it passes the upper endpoint, following the
original routine's documented recommendation when no better interior estimate
is available.

`FZERO` has a fixed internal stopping limit of more than 500 function calls,
so the safe options deliberately do not pretend to provide a caller-controlled
evaluation limit. `RootResult::evaluations` counts invocations through the
Rust trampoline, including the safe endpoint checks.

## Completion and callback policy

`RootStatus` preserves the native `IFLAG` distinctions: convergence, a native
endpoint root, possible singular behavior, loss of a sign-changing interval,
and the fixed evaluation limit. The latter three are returned as an `Ok`
result with their numerical warning status; an invalid initial bracket is
rejected before native entry.

Root and quadrature calls share one scoped thread-local callback slot and the
same process-wide native-runtime lock. The trampoline catches panics and
records `NaN` or infinity rather than allowing either to cross a Fortran frame.
The wrapper reports the corresponding `RootError` after native control returns.
The RAII context always clears on Rust return paths. Parallel callback-bearing
calls serialize, and nested root or quadrature calls from either callback are
rejected deterministically. This is a safety and legacy-runtime policy, not a
claim of concurrent native execution.

## Polynomial roots

The additive `roots-polynomial` feature exposes owned `num_complex::Complex32`
results for real or complex coefficients in descending-power order. Use
`real_polynomial_roots` or `complex_polynomial_roots` for the iterative
`RPZERO`/`CPZERO` method, or their `_with_method` variants with
`PolynomialRootMethod::CompanionQr` for `RPQR79`/`CPQR79`.

Input coefficients are copied; native roots, bounds, and exact work arrays
are private. A zero leading coefficient, non-finite component, or dimension
overflow is rejected before FFI. The iterative drivers use automatic native
initial estimates and preserve their documented best roots on the `25*N`
iteration limit as `PolynomialRootStatus::IterationLimitReached`; error bounds
are absent in that partial-result status. The companion-QR drivers return an
error on nonconvergence because their source does not promise partial roots.

Only `f32`/`Complex32` is currently reviewed. The layout is checked against
the selected GNU Fortran default-COMPLEX ABI; there is no `f64`/`Complex64`
polynomial-root facade. Calls are process-serialized by the shared native
runtime lock. See `examples/roots/polynomial_roots.rs`.

## Remaining root-family boundaries

`SNSQ`,
`DNSQ`, `SNSQE`, `DNSQE`, `SOS`, `DSOS`, `CHKDER`, and `DCKDER` remain deferred
as nonlinear-system architecture work; this module exposes neither their
callbacks nor their dense work arrays.

## Native setup

```text
cargo test -p slatec --features roots-native-tests --target x86_64-pc-windows-gnu --test roots_native -- --test-threads=1
cargo test -p slatec --features roots-polynomial-native-tests --target x86_64-pc-windows-gnu --test polynomial_roots_native -- --test-threads=1
```

The offline `source-build` provider supplies the verified native family closure
after explicit cache acquisition. Source-only checks can select
`external-backend`. Objects, archives, executables, and detailed native evidence
remain ignored.
