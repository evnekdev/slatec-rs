# Safe exact B-spline interpolation, evaluation, and integration

The `bspline` feature provides `slatec::interpolation::bspline::BSpline<T>`
for a scalar B-spline in the reviewed SLATEC B-representation. It can validate
already constructed native-format data and construct an exact interpolant from
nodes, ordinates, and an explicit complete knot sequence.
It requires `std`, one explicit native backend, and the validated GNU MinGW
ABI profile:

```text
cargo add slatec --features std,external-backend,bspline
```

`BSpline::<f32>` calls `BINTK`, `BVALU`, and `BSQAD`; `BSpline::<f64>` calls
`DBINTK`, `DBVALU`, and `DBSQAD`. The curve owns exactly its nondecreasing
knot array `T`, coefficient array `A`, and order `K`. If `A` has length `N`,
`T` has length `N + K`, `N >= K`, and the basic closed domain is
`T[K - 1]..=T[N]` in Rust indexing. Knots and coefficients are finite; knots
are never sorted or merged; an equal-knot run may not exceed `K`.

## Exact interpolation construction

`BSpline::interpolate_with_knots(nodes, values, knots, order)` uses the
reviewed general-order `BINTK`/`DBINTK` pair to construct coefficients such
that

```text
s(nodes[i]) = values[i],  i = 0 .. N - 1
```

to the native precision. This is exact interpolation, not smoothing,
least-squares fitting, or PCHIP construction. `order` is degree plus one, so
order four constructs a cubic B-spline. The selected SLATEC driver requires a
complete caller-supplied knot sequence; the API therefore deliberately does
not choose or synthesize a knot policy. It does not silently choose natural,
not-a-knot, periodic, or clamped boundary conditions.

Nodes and values are borrowed only for construction. Nodes must be finite and
strictly increasing, values must be finite, `N >= order >= 1`, and `knots`
must be a finite nondecreasing sequence of exactly `N + order` entries. The
constructor copies the knots unchanged, allocates native output coefficients,
and returns an ordinary owned `BSpline` with no retained nodes or factorization
state.

For a valid collocation system, the complete sequence must meet the native
endpoint placement rules and the Schoenberg--Whitney support condition. In
zero-based notation, interior nodes require

```text
knots[i] < nodes[i] < knots[i + order].
```

The first lower or last upper equality is allowed only when all `order`
endpoint knots equal that endpoint node. These conditions are checked before
native entry. A failed factorization has no native `INFO` output; a non-finite
or independently non-interpolating result is rejected as
`BSplineError::SingularInterpolationSystem`.

Construction uses private, fallibly allocated work arrays of exactly
`(2 * order - 1) * N` values for factorization and `2 * order` values for
basis scratch. Checked arithmetic guards every buffer formula. The
factorization is dropped after the constructor returns.

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

This feature intentionally excludes BINT4/DBINT4 fixed-cubic special
construction, fitting, basis-function vectors, tensor-product splines,
smoothing, NURBS/rational splines, arbitrary strides, external array adapters
and translated algorithms.
When the hosted `piecewise-polynomial` feature is also enabled,
`BSpline::to_piecewise_polynomial` performs the reviewed exact
`BSPPP`/`DBSPPP` conversion. PCHIP remains the separate
`interpolation::pchip` family.

The polynomial degree is `order - 1`; the constructor returns the order and
degree separately so that callers do not need to infer SLATEC's convention.
Repeated knots are valid up to the order. They lower continuity at the knot,
and a knot repeated exactly `order` times may be discontinuous. Consequently,
an exact query at an interior repeated knot uses the native right-hand limit.
This is a B-representation API, not a shape-preserving interpolator: PCHIP is
the appropriate separate API when monotonicity-preserving interpolation is the
goal.

Examples: `examples/interpolation/bspline_interpolate.rs`,
`examples/interpolation/bspline_interpolate_and_convert.rs`, and
`examples/bspline/from_parts.rs`.
