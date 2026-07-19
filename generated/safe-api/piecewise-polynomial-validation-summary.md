# Safe piecewise-polynomial interpolation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The `piecewise-polynomial` feature exposes owned real univariate PP form in f32 and f64: scalar and batch `PPVAL`/`DPPVAL` evaluation, derivative evaluation, and exact `PPQAD`/`DPPQAD` definite integration.
- A piece stores strictly increasing breakpoints and exact native right-Taylor derivative columns. At an interior breakpoint evaluation uses the right piece; the last endpoint uses the final piece. Extrapolation is rejected by default and optional clamping is Rust-side.
- `BSpline::to_piecewise_polynomial` uses exact `BSPPP`/`DBSPPP` with checked capacities `K*(N+3)`, `K*(N-K+1)`, and `N-K+2`. PCHIP conversion and PP-to-B-spline conversion remain deferred rather than translating coefficients.
- PP numeric paths have no reviewed mutable numerical COMMON, SAVE, DATA, callback, or I/O state, but reachable XERROR and provider/runtime uncertainty keep every native call under the process-global runtime lock.
