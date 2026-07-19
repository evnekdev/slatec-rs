# Safe exact univariate B-spline interpolation construction

- Snapshot: `complete-slatec-05078ebcb649b50e4435`.
- Selected native family: `BINTK`/`DBINTK`, one general-order exact interpolation constructor pair. It solves the total-positive banded collocation system without pivoting and returns ordinary owned `BSpline<f32>` or `BSpline<f64>` values.
- Public constructor: `BSpline::interpolate_with_knots(nodes, values, knots, order)`. Knots are caller supplied because the selected driver does not generate a knot policy.
- Exact private workspace: coefficients `N`, factorization `Q=(2*K-1)*N`, scratch `WORK=2*K`. All arithmetic is checked and allocation is fallible.
- Solvability: strictly increasing nodes, endpoint placement, and Schoenberg--Whitney support are preflighted. The driver has no INFO output; an invalid verified result maps to `SingularInterpolationSystem` after XERROR restoration.
- `BINT4`/`DBINT4` remain deferred: their fixed cubic order, endpoint-condition switches, and automatic knot-placement policies are materially different.
- Native calls remain `SerializedGlobal`; no callback, file I/O, source artifact, or retained factorization is introduced.
