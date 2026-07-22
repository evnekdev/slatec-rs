# Safe callback-driver expansion

The hosted safe API extends the existing contained-callback runtime across the
reviewed `DPFQAD`, `SOS`/`DSOS`, and `*DRIV1`/`*DRIV2` families. Enable the
needed narrow safe feature plus exactly one backend:

```text
std,external-backend,quadrature-piecewise-polynomial
std,external-backend,nonlinear-systems
std,external-backend,ode-sdrive-expert
```

`integrate_piecewise_polynomial` combines `DPFQAD` with a checked
`PiecewisePolynomial<f64>`, allocates no caller-visible workspace, and returns
the exact `DPFQAD` completion state. `solve_scalar_equations` and its `_f32`
counterpart own the `SOS`/`DSOS` work arrays, expose a typed termination
report, and translate the native one-based equation number to a zero-based
Rust index.

`Driv1Session` owns ordinary real `SDRIV1`/`DDRIV1` continuation state.
`Driv2Session` owns the additional integer workspace and exposes typed Adams,
Gear, or automatic selection plus zero-based root events. The complex
counterparts use public `OdeComplex32`; conversion to the internal ABI type is
explicit and checked for size and alignment. All sessions permit only
same-direction continuation.

Every callback is installed only for its native call, runs while the
process-wide native lock is held, and is removed on success, error, or panic.
Panics never unwind through Fortran. Non-finite callback outputs and nested
callback-bearing SLATEC calls are rejected, and a callback failure makes its
stateful ODE session terminal. The lock serializes native execution; it is not
a claim of parallel native callback safety.

The deterministic wrapper/status/deferred records are
`generated/safe-api/callback-driver-*.json`. Runnable concise examples are in
[`examples/quadrature`](../../examples/quadrature),
[`examples/nonlinear`](../../examples/nonlinear), and
[`examples/ode`](../../examples/ode).
