# Numerical quadrature and cubature

## Scope

SLATEC places numerical differentiation and integration under GAMS class `H`. The substantial integration collection is primarily one-dimensional automatic quadrature, especially QUADPACK, supplemented by Gaussian rules and spline-related integration helpers. The Version 4.1 table of contents identifies `QPDOC` as documentation for QUADPACK and lists both user-facing drivers and subsidiary local rules ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`netlib-quadpack`](https://www.netlib.org/quadpack/)).

SLATEC does not appear to offer a comparably unified general multidimensional cubature package. Any “cubature” support must therefore be verified routine by routine rather than inferred from the breadth of the one-dimensional collection.

## Principal routine families

| Family | Representative routines | Problem class | Precision |
|---|---|---|---|
| General finite interval | `QAG/DQAG`, `QAGS/DQAGS` | adaptive integration of a user function over a finite interval | S/D |
| User-specified break points | `QAGP/DQAGP` | finite intervals with known local difficulty or singular points | S/D |
| Infinite intervals | `QAGI/DQAGI` | semi-infinite or doubly infinite intervals | S/D |
| Fourier tails | `QAWF/DQAWF` | sine/cosine weighted integrals over a semi-infinite interval | S/D |
| Finite oscillatory weights | `QAWO/DQAWO` | sine/cosine weighted finite-interval integrals | S/D |
| Algebraic/logarithmic endpoint weights | `QAWS/DQAWS` | endpoint singularities with prescribed weight form | S/D |
| Cauchy principal value | `QAWC/DQAWC` | weight \(1/(x-c)\) | S/D |
| Non-adaptive local rules | `QK15/DQK15` through `QK61/DQK61` | Gauss–Kronrod rules and error estimates | S/D |
| Specialized local rules | `QC25C`, `QC25F`, `QC25S`, `QK15I`, `QK15W` and D variants | weighted or transformed subinterval rules | S/D |
| Simpler adaptive Gaussian driver | `GAUS8/DGAUS8` | adaptive 8-point Legendre–Gauss integration | S/D |
| Spline integration helpers | `BFQAD/DBFQAD`, `PPQAD/DPPQAD`, `PFQAD/DPFQAD` | integrals involving spline representations | S/D |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`netlib-quadpack`](https://www.netlib.org/quadpack/).

QUADPACK commonly provides a simpler driver and an extended `E` routine. The extended routines expose subdivision arrays, error lists and additional diagnostics; the simpler drivers allocate or partition a user-provided workspace and return a narrower interface.

## Public versus subsidiary routines

Public routines select an integration strategy and manage adaptive subdivision. Subsidiary routines evaluate fixed-order rules, maintain error-ordered interval lists, extrapolate sequences and compute weighted moments. Examples include `QELG/DQELG`, `QPSRT/DQPSRT`, `QCHEB/DQCHEB` and the `QK*` rules.

A safe wrapper should normally expose the public drivers. Subsidiary rules may be useful to experts, but should not be promoted merely because they have callable symbols.

## Core algorithms

QUADPACK combines:

- Gauss–Kronrod embedded rules;
- adaptive interval subdivision;
- local absolute-error estimation;
- extrapolation for selected difficult integrals;
- transformed integration over infinite intervals;
- Clenshaw–Curtis/Chebyshev moment methods for some weighted cases;
- error-priority lists stored in integer work arrays.

The choice of driver encodes assumptions about singularities, oscillation and interval geometry. A generic `integrate()` API that hides the driver can easily select the wrong method.

## Callbacks, workspaces and output

Most drivers accept an external scalar function. Common outputs include:

- integral estimate;
- absolute error estimate;
- number of function evaluations;
- an integer status;
- interval endpoints, local results and local errors for extended drivers.

Workspaces often contain parallel arrays for left/right endpoints, subinterval results and error estimates, plus integer ordering information. Exact required lengths depend on a caller-supplied subdivision limit.

**FFI implication:** callback panics must not unwind through Fortran. A trampoline must catch failures and convert them to a controlled termination path, although many QUADPACK callback signatures do not provide a direct error flag. The safe layer may therefore need a scoped callback registry and post-call error recovery.

## Numerical limitations

- Error estimates are heuristic and can be unreliable for severe cancellation, discontinuity, non-integrable singularities or highly oscillatory functions.
- Asking for accuracy below machine capability leads to failure status rather than guaranteed precision.
- A small reported error does not prove that the integrand was correctly evaluated.
- Infinite-interval transformations can magnify endpoint behavior.
- Weighted drivers require the user to classify the weight correctly.
- The subdivision limit is a hard resource bound.
- Callback side effects can make adaptive re-evaluation surprising.

## Modern equivalents

Modern environments expose similar families through QUADPACK-derived interfaces. SciPy’s `quad` remains a high-level wrapper around QUADPACK, while other libraries offer tanh–sinh, adaptive cubature and arbitrary-precision alternatives. This does not make the historical SLATEC copy identical to current wrappers.

## Project proposals for safe Rust

Candidate API groups:

```text
FiniteInterval
InfiniteInterval
KnownBreakpoints
OscillatoryWeight
EndpointWeight
CauchyPrincipalValue
```

Each should return a result containing the raw QUADPACK status, estimate, error estimate, evaluation count and optional subdivision diagnostics.

A safe callback could be `FnMut(f64) -> Result<f64, E>`, implemented through a panic-safe trampoline. Because Fortran does not natively transport `E`, the wrapper must store callback errors externally and terminate conservatively.

## Tentative crate ownership

- Raw: `slatec-quadpack-sys`, with non-QUADPACK helpers either in `slatec-quadrature-support-sys` or the containing raw domain crate.
- Safe: `slatec-quadrature`.
- Spline integration remains owned by the interpolation safe crate, even if some routines are classified under integration.

## Open questions

- Which exact QUADPACK revision is in SLATEC 4.1?
- Are there SLATEC-local changes to error handling or machine constants?
- Which multidimensional integration routines, if any, warrant a separate cubature module?
- How can callback errors stop evaluation without invoking undefined behavior?
- Which extended diagnostics remain meaningful after safe workspace ownership?
- Can concurrent integrations safely coexist with the selected callback registry and error subsystem?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`netlib-quadpack`](https://www.netlib.org/quadpack/)
- [`quadpack-manual`](https://doi.org/10.1007/978-3-642-61786-7)
