# Safe polynomial fitting

Enable `approximation-polynomial-fitting` together with exactly one native
backend to use the checked `f32`/`f64` polynomial least-squares facade:

```rust
use slatec::interpolation::approximation::{PolynomialFit, PolynomialFitOptions};

let x = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
let y = [-1.0_f64, -0.5, 1.0, 3.5, 7.0];
let fit = PolynomialFit::<f64>::fit(&x, &y, PolynomialFitOptions::all_degrees(2))?;
assert!((fit.evaluate(0.5)? - 2.125).abs() < 1.0e-10);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

`PolynomialFit<f32>` and `PolynomialFit<f64>` own the orthogonal polynomial
representation from SLATEC `POLFIT`/`DPOLFT`. Inputs are finite paired samples;
their abscissas may be unordered or repeated. `fit_weighted` additionally
accepts one positive finite weight per sample. The public options map exactly
to the source-defined degree-selection modes: all degrees through a maximum,
a positive RMS tolerance, or an F test at 1%, 5%, or 10% significance.

`evaluate`, `evaluate_with_derivatives`, and `evaluate_into` use
`PVALUE`/`DP1VLU`. The retained single-precision program unit is `PVALUE`;
there is no `P1VLU` identity in the selected corpus. `power_coefficients`
uses `PCOEF`/`DPCOEF` and returns ascending powers about the origin.
`power_coefficients_at(origin)` returns the Taylor coefficients in ascending
powers of `(x - origin)` for any finite requested origin; this can provide a
better-scaled local representation.

When the RMS tolerance cannot be reached or the statistical test is
inconclusive, the source returns its documented best model. The safe facade
preserves it with `PolynomialFitStatus` rather than discarding usable output.

## Ownership and runtime policy

The fit copies every input and keeps the native `A` representation, fitted
values, derivative buffers, and coefficient-conversion storage private. No
caller workspace, raw pointer, or Fortran indexing convention reaches the
safe API. Every call is serialized by the existing process-wide native runtime
lock because legacy XERROR and provider/runtime state are reachable.

This is a fitting API, not an interpolation API. Use
`interpolation::tabulated::TabulatedData` for a global Newton polynomial
through strictly increasing distinct samples, or the checked PCHIP, B-spline,
and piecewise-polynomial families for their respective local representations.

## Explicitly deferred approximation work

`FC`/`DFC` and `EFC`/`DEFC` are stateful constrained B-spline fitting and
incremental-preprocessing workflows. Their persistent work arrays, optional
variance state, and caller-described constraint layout do not yet establish a
single coherent owned high-level contract. Their direct subsidiaries are
recorded as deferred or internal in the generated polynomial-fit audit; no
safe feature promises those workflows.
