# Safe tabulated-data operations

Enable `tabulated-data` together with exactly one native backend. The feature
provides a single owned representation for finite real samples with strictly
increasing abscissas:

```rust
use slatec::interpolation::tabulated::TabulatedData;
use slatec::quadrature::integrate_tabulated;

let samples = TabulatedData::<f64>::from_samples(
    vec![0.0_f64, 1.0, 2.0],
    vec![1.0, 4.0, 9.0],
)?;
let polynomial = samples.interpolating_polynomial()?;
let value = polynomial.evaluate(1.5)?;
let integral = integrate_tabulated(&samples, 0.0..=2.0)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Mathematical scope and provenance

`TabulatedData` is a checked input object, not a fitting or smoothing API. It
retains paired values exactly as supplied after validating equal lengths, at
least two samples, finite coordinates and values, and strictly increasing
abscissas. It never sorts, deduplicates, resamples, or retains a native
pointer.

`interpolating_polynomial` invokes SLATEC `POLINT`/`DPLINT` to create an owned
Newton-form global interpolant. `evaluate` and
`evaluate_with_derivatives` invoke `POLYVL`/`DPOLVL`; the latter returns first
through requested-order derivatives and represents derivatives above the
polynomial degree as zero. `taylor_coefficients_at` invokes `POLCOF`/`DPOLCF`
and returns coefficients about the supplied finite centre.

`integrate_tabulated` and `integrate_tabulated_f32` invoke `DAVINT` and
`AVINT`, respectively. They integrate arbitrary-spacing sampled data using
overlapping parabolas. Bounds must be finite, ordered, and inside the sampled
domain. With more than two source samples, at least three abscissas must occur
inside the requested interval; a two-sample data set uses the native trapezoid
special case. These checks avoid intentionally entering legacy fatal error
paths.

## Ownership, errors, and runtime policy

All interpolation coefficients, derivative buffers, Taylor buffers, and
native work arrays are private owned Rust allocations. No raw pointer,
Fortran workspace length, or native index is public. `TabulatedDataError`
reports malformed samples, non-finite query points, integer conversion,
allocation, and unexpected native-contract failures. `IntegrationError`
reports invalid bounds, insufficient in-interval samples, integer conversion,
or an unexpected original `IERR` value.

The operation has no callback and no continuation state. Each native call is
nevertheless guarded by the existing process-global native runtime lock and
does not reconfigure XERROR: the checked preconditions avoid every reviewed
legacy-error path. The lock reflects reachable legacy XERROR state and
unqualified external/provider runtime behavior; it is not a claim that these
numerical kernels are independently reentrant.

## Deliberate exclusions

`BINT4`/`DBINT4` need a separately reviewed cubic B-spline boundary and knot
policy. Low-level B-spline primitives are already better represented by the
checked `BSpline` and piecewise-polynomial APIs. Callback-bearing `GAUS8` and
`DGAUS8` would duplicate the existing options-based quadrature API. Those
choices, plus raw-to-safe coverage for all canonical raw routines, are
recorded in
[`generated/safe-api/raw-to-safe-coverage.md`](../../generated/safe-api/raw-to-safe-coverage.md)
and
[`generated/safe-api/next-expansion-selection.md`](../../generated/safe-api/next-expansion-selection.md).
