# Safe nonlinear easy drivers

The `nonlinear-easy` feature exposes the original double-precision SLATEC
`DNSQE` and single-precision `SNSQE` easy drivers. They solve a square system

```text
F(x) = 0
```

using the driver's finite-difference Jacobian mode only (`IOPT = 2`). Rust does
not reimplement the Powell-hybrid algorithm and no user Jacobian is exposed.

## Requirements

This is a hosted API: it requires `std`, `alloc`, `nonlinear-easy`, one native
backend, and the validated GNU Fortran `x86_64-w64-mingw32` profile. It is not
allocation-free, `no_std`, or bare-metal compatible. The wrapper allocates the
documented native workspace `(3*N*N + 13*N)/2`, uses checked arithmetic and
Fortran-integer conversion, and fixes the Fortran `NPRINT` argument to zero.

## Callback and runtime policy

The callback receives a read-only current iterate and a mutable residual slice,
both exactly `initial.len()` long. The shared callback bridge verifies native
pointer/dimension consistency before constructing those slices, catches Rust
panics, rejects NaN and infinity residuals, and records the first failure. A
The historical negative-`IFLAG` native stop convention exits through SLATEC's
fatal error path on the validated profile, so cancellation is intentionally not
exposed by this safe API.

Native calls serialize through the process-global SLATEC runtime lock. A
callback cannot call quadrature, scalar roots, or another nonlinear solve; the
attempt is rejected deterministically rather than deadlocking. The lock handles
poisoning by recovering its inner state, so a contained callback failure does
not make later valid calls unavailable.

## Statuses and errors

`Converged`, `MaximumFunctionEvaluations`, `ToleranceTooSmall`, and
`SlowProgress` retain the reviewed `INFO=1..4` meanings in `NonlinearStatus`.
The last three still return a `NonlinearResult` so callers can inspect the final
iterate and residual. Invalid initial vectors or tolerances, callback panic,
non-finite residuals, nesting, and raw-contract violations are
returned as `NonlinearError`.

The native finite-difference callback budget is fixed at `200 * (N + 1)`;
there is no safe option for a limit that the easy driver does not actually
accept.

## Example

```rust,no_run
use slatec::nonlinear::{NonlinearOptions, solve_system};

let result = solve_system(
    &[0.8, 2.2],
    |x, residual| {
        residual[0] = x[0] + x[1] - 3.0;
        residual[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
    },
    NonlinearOptions::default(),
)?;
assert!((result.solution[0] - 1.0).abs() < 1.0e-8);
assert!((result.solution[1] - 2.0).abs() < 1.0e-8);
# Ok::<(), slatec::nonlinear::NonlinearError>(())
```

`generated/safe-api/fortran-argument-map.json` retains the full Rust-to-Fortran
mapping, including hidden `JAC`, `IOPT`, `WA`, and `LWA` arguments.
