# Safe SDRIVE expert ODE sessions

`slatec::ode` is an opt-in safe facade over the original SLATEC `SDRIV3`
(single precision) and `DDRIV3` (double precision) expert drivers. Enable it
with:

```text
cargo add slatec --no-default-features --features std,external-backend,ode-sdrive-expert
```

The only supported public model is the real explicit initial-value problem

```text
y'(t) = f(t, y),  y(t0) = y0.
```

This is an ODE integrator, not a least-squares solver, nonlinear-programming
interface, DAE solver, or general matrix API.

## Session and continuation model

`OdeSession<f32, F, E>` owns the state, callback, time, `SDRIV3` work arrays,
and continuation flag. `OdeSession<f64, F, E>` does the same for `DDRIV3`.
The session is not cloneable and does not expose native work arrays. It may
advance only in the direction established by the first `integrate_to` call.
The first and subsequent calls use `NTASK=3`, exact-target mode.

The supported native controls are `MINT=1` Adams or `MINT=2` BDF,
`MITER=0` functional iteration, `IMPL=0`, and `NROOT=0`. This preserves an
RHS-only scope. Roots/events, Jacobians, finite-difference Jacobian control,
mass matrices, implicit forms, DAEs, dense output, and interpolation remain
deferred.

## Callback containment

The Rust RHS is `FnMut(T, &[T], &mut [T]) -> Result<(), E>`. It can retain
mutable captured state. The native callback is `F(N,T,Y,YDOT)`. The trampoline
constructs exactly `N`-element slices, catches panics, rejects non-finite
derivatives, stores a callback error, and sets native `N=0` for every callback
failure. No Rust unwind crosses Fortran. Such a failure makes that session
terminal; create a fresh session after handling the error.

Callbacks are installed in thread-local state only while the process-wide
native runtime lock is held. This prevents callback-context cross-talk and
rejects callback-nested solver calls. Sessions are not `Sync`; independent
calls are deliberately serialized while XERROR state is scoped and restored.
The reviewed `SDRIV3`/`DDRIV3` closure has no mandatory external-file
protocol, although its legacy error paths contain formatted `WRITE`
diagnostics. `SDSTP`/`DDSTP` do contain initialized `IER` locals. The reviewed
GNU MinGW build emits each as a local-linkage writable `ier.0` `.bss` symbol,
which is shared within the loaded image rather than thread-local. The native
closure remains serialized even apart from callback and XERROR state.
Level-one XERROR diagnostics are scoped to `XSETF(0)`, so ordinary recoverable
native returns do not print; unreviewed fatal native contracts are not made
recoverable and permanently fail the session. The reviewed acquisition cache
contains the complete hash-verified closure, including `DDCOR` and `DGBFA`;
all selected objects were inspected and native continuation, recovery, and
serialized-concurrency tests pass.

## Tolerances and workspace

Relative tolerance maps to scalar native `EPS`. `OdeTolerance::Scalar` uses
`IERROR=3` and `max(abs(y_i), EWT(1))`; `OdeTolerance::Vector` uses `IERROR=4`
and `max(abs(y_i), EWT(i))`. Inputs must be finite, nonnegative/positive where
required, and vector tolerances must match the state dimension.

For the restricted controls, the session allocates exactly
`(MXORD + 4) * N + 250` real values and 50 integer values with checked
arithmetic. Those arrays retain opaque continuation history and are internally
owned.

`ReachedTarget`, `ExcessWork`, and `ToleranceAdjusted` preserve meaningful
time/state and permit same-direction continuation. Callback errors, callback
panics, non-finite derivative output, and unreviewed native terminal statuses
leave the session unusable.

## Storage interoperability and backend

The core API uses `Vec<T>`, `&[T]`, and `&mut [T]`; it neither introduces an
owned vector/matrix ecosystem nor depends on `nalgebra`, `ndarray`, or `faer`.
Optional adapters can be added separately later. This safe family requires
`std`, an explicit native backend, and the validated GNU MinGW profile. The
source backend is offline-only and does not download or commit SLATEC source.

See the checked examples in [`examples/ode`](../../examples/ode) and the
deterministic `generated/safe-api/ode-sdrive-*.json` records for exact raw,
workspace, status, callback, concurrency, and source-closure evidence.
