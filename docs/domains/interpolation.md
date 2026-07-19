# Interpolation, approximation and splines

## Scope

SLATEC classifies interpolation under GAMS `E` and approximation/fitting under `K`, with regression overlap in `L8`. The collection includes PCHIP, B-spline/B-representation routines, piecewise-polynomial representations, polynomial interpolation and fitting, and evaluation/integration utilities ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Principal families

| Family | Representative routines | Purpose | Provenance |
|---|---|---|---|
| PCHIP derivative construction | `PCHIM/DPCHIM`, `PCHIC/DPCHIC` | monotone or user-controlled piecewise cubic Hermite derivatives | PCHIP |
| PCHIP evaluation | `PCHFE/DPCHFE`, `PCHFD/DPCHFD`, `CHFEV/DCHFEV` | values and derivatives | PCHIP |
| PCHIP integration | `PCHIA/DPCHIA`, `PCHID/DPCHID` | integrate Hermite representation | PCHIP |
| B-spline interpolation | `BINT4/DBINT4`, `BINTK/DBINTK` | construct B-representation interpolants | BSPLINE collection |
| B-spline evaluation | `BVALU/DBVALU`, `BSPEV/DBSPEV` | evaluate values/derivatives | BSPLINE |
| Representation conversion | `BSPPP/DBSPPP`, `PPVAL/DPPVAL`, `PPQAD/DPPQAD` | B-representation to piecewise polynomial, evaluation, and integration | BSPLINE |
| Spline fitting | `FC/DFC`, `EFC/DEFC` | weighted and constrained B-spline fitting | incorporated fitting family |
| Polynomial interpolation/fitting | `POLINT/DPOLINT`, `POLFIT/DPOLFT`, `PVALUE/DP1VLU` | interpolate or least-squares fit polynomials | mixed SLATEC families |
| Interval location | `INTRV/DINTRV` and helpers | locate knot/subinterval | subsidiary utility |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`slatec-pchip`](https://www.netlib.org/slatec/pchip/).

## PCHIP design

PCHIP separates construction from evaluation. `PCHIM` computes derivatives for a shape-preserving monotone cubic Hermite interpolant; `PCHIC` offers more boundary and switch-point control. Evaluation routines consume knots, values and derivatives. This separation maps naturally to an immutable Rust interpolant object.

PCHIP is not synonymous with all SLATEC interpolation. B-splines, smoothing/fitting and multidimensional routines have separate provenance and storage.

## Reviewed B-spline subset

The opt-in `bspline` feature now exposes a univariate real B-representation
with exact general-order construction through `BINTK`/`DBINTK` when callers
provide a complete valid knot sequence. It owns nondecreasing knots,
coefficients, and order; evaluates values and native derivatives; and performs
definite integration without translating a spline algorithm. The basic domain
is `T[K-1]..=T[N]`; outside points are rejected by default or may be explicitly
clamped to an endpoint limit. It does not fit data, generate knots, insert
knots, form basis vectors, sort inputs, or convert storage except through the
separate reviewed PP conversion.

The native closure reaches XERROR and `BSQAD`/`DBSQAD` retain initialized
quadrature tables, so this subset remains globally serialized. See
[`safe-bspline.md`](../api/safe-bspline.md) for the exact native contract.

## Reviewed piecewise-polynomial subset

The hosted `piecewise-polynomial` feature now exposes an owned real univariate
PP representation through `PPVAL`/`DPPVAL`, `PPQAD`/`DPPQAD`, and exact
`BSPPP`/`DBSPPP` B-spline conversion. Its breakpoints are strictly increasing
and each column stores right Taylor derivatives. Evaluation uses the right
piece at interior breaks, integration is exact for the represented pieces,
and extrapolation is rejected by default or explicitly clamped in Rust.
PCHIP conversion and PP-to-B-spline conversion remain deferred: neither is
silently reconstructed in Rust. See
[`safe-piecewise-polynomial.md`](../api/safe-piecewise-polynomial.md).

## B-representation and PP representation

B-spline routines often use:

- knot sequence;
- spline order;
- B-spline coefficients;
- interval-search state;
- work arrays for divided differences or factorization.

Piecewise-polynomial (PP) form stores local coefficients per interval and can be more direct for repeated evaluation or integration. Conversion changes representation and storage requirements.

## Precision variants

Most major families have single/double pairs. Complex interpolation is not a general parallel family. Integer arrays carry dimensions, orders, knot intervals and status. Character/logical options appear in some control interfaces.

## Public and subsidiary routines

Public routines build, fit, evaluate, differentiate or integrate an interpolant. Subsidiary routines locate intervals, compute divided differences, solve banded systems and manipulate knot sequences. A safe API should hide the latter unless they provide independently useful low-level operations.

## Algorithms and dependencies

- monotone Hermite slope selection;
- cubic Hermite evaluation;
- B-spline basis evaluation and knot insertion/representation conversion;
- banded linear systems for interpolation conditions;
- constrained and weighted least squares for fitting;
- interval searches and sorting;
- quadrature over polynomial pieces.

Dependencies cross into dense/banded linear algebra, optimization/least squares, sorting and quadrature.

## Numerical limitations

- monotonicity preservation trades smoothness/shape choices against unconstrained cubic splines;
- duplicate or unordered abscissae are invalid for many constructors;
- clustered knots can cause conditioning problems;
- extrapolation behavior is routine-specific;
- high-degree global polynomial interpolation can be unstable;
- fitting results depend on weights, constraints and knot choice;
- derivative estimates can amplify noise.

## Modern equivalents

SciPy offers `PchipInterpolator`, B-splines and many spline constructors. Rust crates provide several cubic and spline abstractions, but often differ in boundary conditions, multidimensional support and error semantics. A compatibility wrapper should document the precise SLATEC/PCHIP algorithm rather than claiming equivalence from a shared method name.

## FFI and safe-wrapper considerations

| Risk | Mitigation proposal |
|---|---|
| strict ordering and length invariants | validated constructors |
| multiple representations | separate `Pchip`, `BSpline`, `PiecewisePolynomial` types |
| mutable work arrays | internal allocation |
| interval-search cached state | keep per-object mutable cursor only if thread-safe, otherwise use pure search |
| extrapolation flags | explicit enum/result |
| banded fitting matrices | checked matrix/storage conversion |
| one-based interval outputs | convert in safe layer |
| status flags and partial outputs | preserve raw status and document output validity |

## Project proposals

```text
Pchip<T>
BSpline<T>
PiecewisePolynomial<T>
PolynomialFit<T>
SplineFitOptions
```

Construction should own or copy coefficients needed for safe reuse. Evaluation should accept scalar and slice forms. Integration and derivatives should be methods when supported by the underlying representation.

## Tentative crate ownership

- Raw: `slatec-pchip-sys`, `slatec-bspline-sys`, and a smaller approximation/fitting raw closure if justified.
- Safe: `slatec-interpolation`, possibly with `pchip`, `bspline`, `polynomial` and `fit` modules.
- Least-squares solvers remain owned by linear algebra/optimization even when used internally.

## Open questions

- What exact BSPLINE package provenance and version are present?
- Which routines retain cached interval state or saved variables?
- Which extrapolation behaviors should safe wrappers expose?
- Are multidimensional interpolation families large enough for a separate module?
- Can fitting routines share safe matrix types with the linear-algebra crate without cyclic dependencies?
- Which PCHIP files differ from the standalone Netlib PCHIP collection?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`slatec-pchip`](https://www.netlib.org/slatec/pchip/)
