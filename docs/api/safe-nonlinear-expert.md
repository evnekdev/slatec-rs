# Safe expert nonlinear solvers

The `nonlinear-expert` feature exposes the original SLATEC `DNSQ` and `SNSQ`
Powell-hybrid drivers. It provides separate safe functions for native
finite-difference Jacobians and user-supplied dense analytic Jacobians. The
`nonlinear-jacobian-check` feature exposes the companion `DCKDER` and `CHKDER`
consistency checks.

## Solver controls

`ExpertNonlinearOptions` maps directly to reviewed expert arguments:

| Rust control | Fortran | Meaning |
| --- | --- | --- |
| `tolerance` | `XTOL` | relative change convergence tolerance |
| `maximum_function_evaluations` | `MAXFEV` | positive residual-call budget |
| `finite_difference_step` | `EPSFCN` | relative residual-error estimate |
| `step_bound_factor` | `FACTOR` | positive initial trust-region multiplier |
| `scaling` | `MODE`, `DIAG` | automatic or copied positive variable scales |
| `jacobian_structure` | `ML`, `MU` | dense or banded native finite differences |

`NPRINT` is fixed to zero. Observer callbacks and negative-`IFLAG`
cancellation remain deferred because they share the legacy mutable callback
control channel; negative `IFLAG` reaches the profile's fatal XERROR path.

## Jacobian callback

The analytic API uses `IOPT=1`. Its callback receives the current iterate, the
valid current residual, and `JacobianMut`, a checked logical `N × N` view over
Fortran column-major `FJAC(LDFJAC,N)`. An entry is stored at:

```text
row + column * leading_dimension
```

Every logical entry is initialized to NaN before the callback and must be
overwritten with a finite value. This detects incomplete writes as well as
explicit NaN or infinity. Banded controls apply only to `IOPT=2` native finite
differences; user Jacobians are dense.

The wrapper allocates exact checked arrays: `N*N` for `FJAC`, `N*(N+1)/2` for
packed `R`, and six `N`-element arrays (`DIAG`, `QTF`, `WA1` through `WA4`). QR
outputs remain internal.

## Runtime behavior

Expert solves require `std`, `alloc`, `nonlinear-expert`, one explicit backend,
and the validated GNU Fortran `x86_64-w64-mingw32` profile. The shared callback
runtime checks native dimensions, pointers, non-overlap, `LDFJAC`, and callback
flags before creating Rust slices. Residual and Jacobian panics are caught;
non-finite outputs are recorded; finite sentinels let Fortran return without
unwinding across FFI. Native calls serialize, and every nested quadrature,
root, easy nonlinear, or expert nonlinear callback call is rejected.

`INFO=1` through `INFO=5` retain distinct Rust statuses. Native `NFEV` and
`NJEV` are reported and cross-checked against the contained callback counts.

## Jacobian checking

`check_jacobian` and `check_jacobian_f32` run the exact two-stage `DCKDER` and
`CHKDER` protocol: mode 1 creates a neighboring point, Rust evaluates the
function there, and mode 2 returns a component score in `[0,1]`. The original
routine describes values above `0.5` as probably correct and values below
`0.5` as probably incorrect; cancellation can make a score inconclusive.

These helpers invoke Rust closures outside FFI and need only `alloc`, not
`std`. A closure panic is therefore an ordinary Rust panic, never a Fortran
unwind. The current native backend is still hosted GNU MinGW; this is not a
bare-metal validation claim.

## Example

```rust,no_run
use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_with_jacobian};

let result = solve_system_with_jacobian(
    &[0.8, 2.2],
    |x, f| {
        f[0] = x[0] + x[1] - 3.0;
        f[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
    },
    |x, _, mut j| {
        j.set(0, 0, 1.0).unwrap();
        j.set(0, 1, 1.0).unwrap();
        j.set(1, 0, 2.0 * x[0]).unwrap();
        j.set(1, 1, 2.0 * x[1]).unwrap();
    },
    ExpertNonlinearOptions::default(),
)?;
assert!((result.solution[0] - 1.0).abs() < 1.0e-8);
assert!((result.solution[1] - 2.0).abs() < 1.0e-8);
# Ok::<(), slatec::nonlinear::NonlinearError>(())
```
