# Safe B-spline evaluation and integration

The `bspline` feature provides `slatec::interpolation::bspline::BSpline<T>`
for a pre-existing scalar B-spline in the reviewed SLATEC B-representation.
It requires `std`, one explicit native backend, and the validated GNU MinGW
ABI profile:

```text
cargo add slatec --features std,external-backend,bspline
```

`BSpline::<f32>` calls `BVALU` and `BSQAD`; `BSpline::<f64>` calls `DBVALU`
and `DBSQAD`. The curve owns exactly the caller-supplied nondecreasing knot
array `T`, coefficient array `A`, and order `K`. If `A` has length `N`, `T`
must have length `N + K`, `N >= K`, and the basic closed domain is
`T[K - 1]..=T[N]` in Rust indexing. Knots and coefficients are finite; knots
are never sorted or merged; an equal-knot run may not exceed `K`.

`evaluate` and `derivative` are in-place native scalar evaluations. An
interior knot uses the native right limit, and the right endpoint uses the
left limit. `evaluate_into` preserves caller query order and writes directly
to the supplied output slice. Out-of-domain input is rejected by default;
`Extrapolation::Clamp` explicitly maps it to the nearest endpoint and never
asks SLATEC to extrapolate.

`integrate` calls `BSQAD`/`DBSQAD` with exact `3*K` private workspace. It
accepts reversed limits, preserving the native signed result, and supports
orders through 20 because that is the reviewed native quadrature limit.

Each native call also uses exactly `3*K` private work values, creates a fresh
native interval-search state, validates all native integer conversions, and
uses fallible allocation. The process-wide native-runtime lock is held across
the XERROR scope and native call. This preserves process-global XERROR state
and protects the package's saved quadrature tables; it is not a parallel
native-execution claim.

This feature intentionally excludes interpolation/fitting construction,
basis-function vectors, tensor-product splines, smoothing, B-spline-to-PP
conversion, NURBS/rational splines, arbitrary strides, external array adapters
and translated algorithms. PCHIP remains the separate
`interpolation::pchip` family.

The polynomial degree is `order - 1`; the constructor returns the order and
degree separately so that callers do not need to infer SLATEC's convention.
Repeated knots are valid up to the order. They lower continuity at the knot,
and a knot repeated exactly `order` times may be discontinuous. Consequently,
an exact query at an interior repeated knot uses the native right-hand limit.
This is a B-representation API, not a shape-preserving interpolator: PCHIP is
the appropriate separate API when monotonicity-preserving interpolation is the
goal.

Examples: `examples/bspline/from_parts.rs`, `derivatives.rs`, and
`integrate.rs`.
