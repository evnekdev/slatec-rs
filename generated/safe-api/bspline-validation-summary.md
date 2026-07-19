# Safe B-spline construction and evaluation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The partial `bspline` feature exposes existing scalar B-representations only: `BVALU`/`DBVALU` values and derivatives, plus `BSQAD`/`DBSQAD` definite integration, in f32 and f64.
- `BSpline` owns exact native knots, coefficients, and order. Validation enforces `T.len() = N + K`, `N >= K >= 1`, finite nondecreasing knots, knot multiplicity at most `K`, a positive basic domain, and finite coefficients. No sorting, duplicate merging, knot insertion, conversion, or fitting occurs.
- Evaluation and integration have exact checked private workspace `3*K`. Integration additionally rejects `K > 20`, the reviewed `BSQAD`/`DBSQAD` limit.
- `BVALU` interval search starts from local `INBV=1` every call. All native entry is serialized through the process-global runtime lock and scoped XERROR restoration.
- Interpolation construction, basis vectors, weighted callbacks, PP conversion, tensor-product splines, smoothing, NURBS, arbitrary strides, adapters, and translated algorithms remain deferred.
