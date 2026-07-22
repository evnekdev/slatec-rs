# Safe exact univariate B-spline interpolation construction

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Selected native constructors: `BINTK`/`DBINTK` for general-order exact interpolation and `BINT4`/`DBINT4` for fixed cubic interpolation. Both return ordinary owned `BSpline<f32>` or `BSpline<f64>` values.
- `BSpline::interpolate_with_knots(nodes, values, knots, order)` accepts an explicit complete knot sequence. `BSpline::interpolate_cubic(nodes, values, left_boundary, right_boundary, knot_placement)` encodes the source endpoint-condition and knot-placement choices without exposing writable native storage.
- General-order exact private workspace: coefficients `N`, factorization `Q=(2*K-1)*N`, scratch `WORK=2*K`. Cubic exact private workspace: `T=NDATA+6`, `BCOEF=NDATA+2`, `W=5*(NDATA+2)`. All arithmetic is checked and allocation is fallible.
- Both constructors preflight finite strictly increasing nodes and finite values; cubic construction additionally validates finite derivative conditions and explicit exterior knots. Returned splines are audited against every input node.
- Native calls remain `SerializedGlobal`; no callback, file I/O, source artifact, or retained factorization is introduced.
