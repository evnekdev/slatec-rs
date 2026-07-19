# Safe B-spline construction and evaluation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The partial hosted `bspline` feature exposes `BINTK`/`DBINTK` exact general-order interpolation with caller-supplied complete knots, `BVALU`/`DBVALU` values and derivatives, and `BSQAD`/`DBSQAD` definite integration, in f32 and f64.
- `BSpline::interpolate_with_knots` enforces finite strictly increasing nodes, finite values, `N >= K >= 1`, complete finite nondecreasing `T.len() = N + K`, endpoint placement, and the documented Schoenberg--Whitney condition before FFI. It never generates, sorts, inserts, or merges knots.
- Construction owns only the returned knots, coefficients, and order. It uses exact checked private `Q=(2*K-1)*N` and `WORK=2*K` allocations, drops factorization state before returning, and verifies that the output reproduces all input nodes.
- Evaluation and integration have exact checked private workspace `3*K`. Integration additionally rejects `K > 20`, the reviewed `BSQAD`/`DBSQAD` limit.
- All native entry is serialized through the process-global runtime lock and scoped XERROR restoration. BINT4/DBINT4 fixed-cubic construction, basis vectors, weighted callbacks, tensor-product splines, smoothing, NURBS, arbitrary strides, adapters, and translated algorithms remain deferred.
