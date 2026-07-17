# Optimization and nonlinear least squares

## Scope

This survey covers SLATEC’s constrained optimization, linear programming, bounded and constrained linear least squares, nonlinear least squares and related fitting support. The SLATEC table of contents places optimization under GAMS `G`, while nonlinear least squares appears under approximation class `K1B`; users are explicitly directed to search related classes rather than expect one contiguous package ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Problem classes

- sparse linear programming;
- bounded linear least squares;
- equality- and inequality-constrained linear least squares;
- nonnegative least squares;
- unconstrained nonlinear least squares;
- nonlinear least squares with user or finite-difference Jacobians;
- covariance estimation after nonlinear fitting;
- supporting linear algebra and derivative checking.

## Major routine families

| Family | Representative routines | Mathematical role | Provenance/confidence | Precision |
|---|---|---|---|---|
| sparse linear programming | `SPLP/DSPLP` | linear objectives and sparse constraint matrices | SLATEC-incorporated family; exact external lineage needs review | S/D |
| bounded/constrained least squares | `SBOLS/DBOLS`, `SBOCLS/DBOCLS` | bounds and linear equality constraints around least squares | Lawson–Hanson-related incorporated family | S/D |
| equality/inequality least squares | `LSEI/DLSEI` | constrained linear least squares, optional covariance | incorporated least-squares family | S/D |
| nonnegative/equality constrained | `WNNLS/DWNNLS` | weighted/nonnegative constrained least squares | Lawson–Hanson family | S/D |
| nonlinear least squares | `SNLS1/DNLS1`, `SNLS1E/DNLS1E` | modified Levenberg–Marquardt, expert and easy drivers | combines MINPACK `LMDER` and `LMDIF` | S/D |
| covariance after fit | `SCOV/DCOV` | covariance estimate after `NLS1` success | SLATEC fitting support | S/D |
| common subsidiaries | `QRFAC`, `LMPAR`, `QRSOLV`, `RWUPDT`, `FDJAC3`, `ENORM` and D variants | QR, LM parameter, updates, finite differences and norms | MINPACK-derived | S/D |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`DNLS1 source`](https://www.netlib.org/slatec/src/dnls1.f), [`DSPLP source`](https://www.netlib.org/slatec/src/dsplp.f), [`netlib-minpack`](https://www.netlib.org/minpack/).

## Nonlinear least squares: `DNLS1`

`DNLS1` minimizes a sum of squares of \(M\) nonlinear functions in \(N\) variables using a modification of the Levenberg–Marquardt algorithm. Its source prologue states that it combines the MINPACK `LMDER` and `LMDIF` codes. The user may provide a Jacobian or request forward differences ([`DNLS1 source`](https://www.netlib.org/slatec/src/dnls1.f)).

The expert interface follows the same historical pattern as `DNSQ`:

- callback-driven residual and optional Jacobian evaluation;
- mutable parameter and residual arrays;
- leading dimension for the Jacobian;
- multiple convergence tolerances;
- function-evaluation limit;
- variable scaling controls;
- print cadence;
- status and evaluation counters;
- QR-related outputs;
- caller-provided work arrays.

The `E` driver reduces configuration but does not erase the underlying local-convergence and scaling assumptions.

## Sparse linear programming: `DSPLP`

`DSPLP` is described as solving linear programming problems with up to a few thousand constraints and variables while taking advantage of sparse constraint storage. Its interface includes user matrix-handling callbacks and extensive integer/real workspaces, and its subsidiary list contains many `SPLP`-specific routines ([`slatec-toc`](https://www.netlib.org/slatec/toc); [`DSPLP source`](https://www.netlib.org/slatec/src/dsplp.f)).

**Modern interpretation:** this is not a thin function suitable for a single `&[f64]` wrapper. It is a stateful solver protocol with sparse input conventions, work arrays, scaling/control options and diagnostic outputs. A safe API requires a dedicated problem builder and verified storage conversion.

## Constrained linear least squares

`SBOLS/DBOLS`, `SBOCLS/DBOCLS`, `LSEI/DLSEI` and `WNNLS/DWNNLS` overlap mathematically but impose different constraint models:

- variable bounds;
- bounds on linear expressions `C x` (the `SBOCLS`/`DBOCLS` model);
- linear equality constraints;
- linear inequality constraints;
- nonnegativity on selected variables;
- optional covariance or residual information.

They should not be represented by one ambiguous “least_squares” function. Safe APIs should encode the constraint model in types or builders.

## Algorithms and dependencies

Major algorithmic ingredients include:

- Levenberg–Marquardt trust-region/damping logic;
- QR factorization with column operations;
- triangular solves;
- finite-difference Jacobians;
- active-set or elimination methods for constrained least squares;
- sparse linear-programming basis updates and matrix-access callbacks;
- norm, scaling and machine-precision support.

Nonlinear least squares shares subsidiaries with nonlinear systems. Constrained linear least squares shares dense linear algebra routines. Sparse LP has a large dedicated closure.

## Precision variants

The principal drivers are paired single/double routines. Complex nonlinear least squares is not represented as a simple complex variant in the table of contents. A safe API must not imply generic scalar support where only real S/D routines exist.

Mixed integer and floating-point workspaces are common. Fortran integer ABI and conversion from Rust `usize` require checked bounds.

## Limitations and assumptions

- nonlinear least squares is local and starting-point dependent;
- convergence tests may be satisfied without a globally optimal fit;
- user Jacobians must agree with residual callbacks;
- covariance estimates depend on model and rank assumptions;
- constrained least-squares families accept different matrix layouts and may overwrite inputs;
- sparse LP scale limits are historical, not modern capacity guarantees;
- no uniform result schema exists across the families;
- some solvers communicate through callbacks and mutable control flags;
- rank deficiency and ill-conditioning must be surfaced rather than hidden.

## Relation to modern libraries

Current high-level optimization libraries commonly provide a unified result object while retaining separate algorithms for local/global minimization, linear programming, constrained optimization, nonlinear least squares and roots. SciPy’s current optimize API is one example of this functional organization ([SciPy optimize](https://docs.scipy.org/doc/scipy/reference/optimize.html)).

**Modern interpretation:** SLATEC’s algorithms should be exposed by precise historical method names and capabilities, not marketed as a comprehensive modern optimization suite. Modern libraries generally offer more algorithms, sparse Jacobian abstractions, automatic differentiation integration and richer constraint modeling.

## Preliminary FFI risks

| Risk | Example | Proposed mitigation |
|---|---|---|
| residual/Jacobian callbacks | `DNLS1` callback mode and mutable flags | panic-safe trampolines and explicit callback contracts |
| sparse matrix callbacks | `DSPLP` user matrix access protocol | build a tested adapter around owned sparse storage |
| overwritten matrices | constrained least-squares drivers may transform input | ownership-taking builders or documented mutable views |
| mixed work arrays | real and integer workspace formulas | internal checked allocation |
| leading dimensions | Jacobian and constraint matrices | validated column-major views |
| status diversity | solver-specific `MODE`, `INFO`, rank and error outputs | solver-specific result enums plus raw code |
| process-global error state | routines may also call `XERMSG` | serialize or isolate until thread-safety is proved |
| callback cancellation | negative flags or package-specific controls | map a Rust cancellation error to documented termination paths only |
| aliasing | historical APIs may reuse work/input arrays | conservative non-overlap checks |

## Project proposals for safe Rust APIs

### Nonlinear least squares

```text
NonlinearLeastSquaresProblem
LevenbergMarquardtOptions
LeastSquaresResult {
    parameters,
    residuals,
    cost,
    status,
    evaluations,
    covariance: Option<_>,
}
```

Provide separate residual-only and residual-plus-Jacobian constructors. Make finite-difference configuration explicit.

### Linear constrained least squares

Use separate problem builders:

```text
BoundedLeastSquares
EqualityInequalityLeastSquares
NonnegativeLeastSquares
BoundedConstrainedLeastSquares
```

A facade may later unify shared result fields, but it should not erase method-specific capabilities.

### Linear programming

A dedicated sparse-LP builder should own converted storage and callbacks. It should expose solver status, objective, primal variables and available diagnostics without inventing unsupported dual data.

## Candidate crate boundaries

**Raw, tentative:**

- `slatec-minpack-sys` for verified MINPACK-derived nonlinear least-squares and nonlinear-system sources;
- `slatec-lsq-sys` for Lawson–Hanson-style linear least-squares families if provenance/dependency analysis supports it;
- `slatec-splp-sys` for the sparse LP closure.

**Safe, tentative:**

- `slatec-least-squares`;
- `slatec-optimization`;
- or modules inside a broader `slatec-opt` crate.

A separate safe nonlinear-equations crate may share an internal callback-support crate, but public cyclic dependencies should be avoided.

## Open questions and experiments

- What exact MINPACK version and modifications are present in `NLS1`?
- Which constrained least-squares routines are directly derived from Lawson and Hanson distributions?
- What sparse storage and callback invariants does `SPLP` require?
- Can `SPLP` safely support concurrent solves?
- Which inputs are overwritten and which outputs remain valid on partial failure?
- How should rank-deficiency and covariance warnings map to Rust enums?
- Can common callback infrastructure serve both `DNSQ` and `DNLS1` without global state?
- Which routines should remain expert-only because a safe abstraction would otherwise conceal essential controls?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`DNLS1 source`](https://www.netlib.org/slatec/src/dnls1.f)
- [`DSPLP source`](https://www.netlib.org/slatec/src/dsplp.f)
- [`netlib-minpack`](https://www.netlib.org/minpack/)
- [`netlib-opt`](https://www.netlib.org/opt/)
- [SciPy optimize](https://docs.scipy.org/doc/scipy/reference/optimize.html)
