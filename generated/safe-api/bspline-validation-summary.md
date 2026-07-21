# Safe B-spline construction and evaluation

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- The `bspline` feature exposes `BINTK`/`DBINTK` exact general-order interpolation with caller-supplied complete knots, `BVALU`/`DBVALU` values and derivatives, and `BSQAD`/`DBSQAD` definite integration, in f32 and f64. The additive `bspline-cubic-interpolation` feature additionally exposes source-accurate `BINT4`/`DBINT4` cubic construction.
- `BSpline::interpolate_with_knots` enforces finite strictly increasing nodes, finite values, `N >= K >= 1`, complete finite nondecreasing `T.len() = N + K`, endpoint placement, and the documented Schoenberg--Whitney condition before FFI. It never generates, sorts, inserts, or merges knots.
- `BSpline::interpolate_cubic` owns exact `T=NDATA+6`, `BCOEF=NDATA+2`, and `W=5*(NDATA+2)` storage; typed endpoint conditions distinguish first from second derivatives and typed knot placement distinguishes source options 1, 2, and 3. It verifies the returned cubic shape and every input node.
- Evaluation and integration have exact checked private workspace `3*K`. Integration additionally rejects `K > 20`, the reviewed `BSQAD`/`DBSQAD` limit.
- All native entry is serialized through the process-global runtime lock and scoped XERROR restoration where a reviewed path requires it. Basis vectors, weighted callbacks, tensor-product splines, smoothing, NURBS, arbitrary strides, adapters, and translated algorithms remain deferred.
