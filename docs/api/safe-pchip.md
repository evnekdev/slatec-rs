# Safe PCHIP interpolation

Enable the hosted PCHIP feature with one explicit native backend:

```text
slatec = { version = "0.1", default-features = false, features = ["std", "source-build", "pchip"] }
```

`source-build` is cache-only and verifies every required Netlib source hash;
`external-backend` is for an application that supplies the reviewed native
symbols. The API is available for both `f32` and `f64`: `f32` calls `PCHIM`,
`PCHIC`, `PCHSP`, `PCHFE`, `PCHFD`, and `PCHIA`; `f64` calls their `D`-prefixed
counterparts.

## Curve representation

`PiecewiseCubicHermite<T>` owns three same-length vectors: strictly increasing
finite knots `x[i]`, values `f[i]`, and first derivatives `d[i]`.  It
represents the native piecewise cubic Hermite function interval-by-interval;
it is not a general spline, B-spline, or multidimensional interpolation API.

`from_derivatives` validates and retains supplied derivatives exactly.
`monotone` uses the default PCHIP construction. On monotone data each cubic
interval is monotone; where data changes direction PCHIM deliberately creates
an extremum and reports the count through `construction_report`. This is
shape-preserving local interpolation, not a promise that arbitrary input data
becomes globally monotone.

`monotone_with_conditions` exposes the audited PCHIC endpoint and switch
controls with `MonotoneEndpointCondition` and `SwitchPointPolicy`, never raw
integer code words. `spline` uses PCHSP's distinct cubic-spline derivative
construction with typed `EndpointCondition`: not-a-knot, first/second
derivative, and the historical three/four-point formulas. A zero prescribed
second derivative is the PCHSP natural condition.

No constructor sorts inputs, drops duplicate knots, merges values, or accepts
strides. Every native call uses contiguous `INCFD = 1`. PCHIC owns exact
scratch `2*(N-1)` and PCHSP owns exact scratch `2*N`; arithmetic and native
integer conversion are checked before allocation.

## Evaluation and extrapolation

`evaluate_into` has no output allocation and calls PCHFE. Its derivative
counterpart calls PCHFD and requires two same-length caller output buffers.
Points retain their original order; ordered points are merely faster for the
native search, not a correctness precondition.

The default policy rejects an out-of-domain query before FFI. With
`Extrapolation::Allow`, PCHFE/PCHFD extrapolate from the nearest endpoint
cubic and return `EvaluationReport::extrapolated_points`. `integrate` also
rejects out-of-domain limits by default. `integrate_with_policy` preserves
PCHIA's signed reversed-bound behavior and, when allowed, records separately
whether the lower and upper limits used endpoint-cubic extrapolation.

## Runtime and exclusions

The selected PCHIP numerical files contain DATA/SAVE constants, and their
checked failure paths reach SLATEC XERROR's global J4SAVE/XERSVE state. The
safe facade therefore holds the existing process-wide native lock for the
entire XERROR scope and native call. It is `SerializedGlobal` for the reviewed
GNU MinGW source backend and `BackendDependent` for external/system providers.

This feature intentionally has no ecosystem adapters, array abstraction,
arbitrary-stride support, B-splines, smoothing, hidden matrix conversion, or
translated interpolation algorithm.
