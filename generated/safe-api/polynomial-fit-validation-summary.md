# Safe polynomial fitting

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- `approximation-polynomial-fitting` exposes an immutable f32/f64 weighted least-squares representation over `POLFIT`/`DPOLFT`, with source-defined all-degree, RMS-tolerance, and F-test selection.
- `PVALUE`/`DP1VLU` evaluate values and derivatives; `PCOEF`/`DPCOEF` return ordinary origin coefficients or a Taylor expansion about any finite requested origin. The single-precision evaluator is `PVALUE`; `P1VLU` is not a retained SLATEC identity.
- Fitting is intentionally distinct from the checked global interpolation representation: sample abscissas may be unordered or repeated. All native storage is private and every call holds the process-global runtime lock.
- `FC`/`DFC` and `EFC`/`DEFC` remain explicitly deferred as stateful constrained B-spline workflows; no provider or safe feature is promised for them.
