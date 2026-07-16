# Nonlinear equations and root finding

## Scope

This survey covers scalar roots, polynomial roots, systems of nonlinear equations and derivative-checking support. Nonlinear least squares is discussed primarily in [Optimization](optimization.md), although its implementation shares substantial machinery with nonlinear systems.

The SLATEC table of contents assigns scalar and polynomial roots to GAMS `F1`, systems of equations to `F2`, and derivative-checking service routines to `F3` ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

## Problem classes

- a scalar continuous function with a sign-changing bracket;
- scalar roots without a guaranteed smoothness model;
- all roots of a polynomial with real or complex coefficients;
- square systems \(F(x)=0\);
- systems with user-supplied or finite-difference Jacobians;
- derivative consistency checks for user callbacks.

## Major routine families

| Family | Representative routines | Algorithm or role | User-facing status | Precision pattern |
|---|---|---|---|---|
| bracketed scalar root | `FZERO`, `DFZERO` | Dekker-style combination of bisection and secant interpolation | user-callable | S/D |
| polynomial zeros | `RPQR79/CPQR79`, `RPZERO/CPZERO` | polynomial root algorithms for real or complex coefficients | user-callable | real/complex family; naming is not a simple S/D prefix pair |
| Powell hybrid systems | `SNSQ/DNSQ`, easy drivers `SNSQE/DNSQE` | Newton/scaled-gradient dogleg method with QR and Broyden rank-one updates | user-callable | S/D |
| square-system solver | `SOS/DSOS` | separate nonlinear system family | user-callable | S/D |
| derivative checking | `CHKDER/DCKDER` | compare supplied Jacobian/gradients with function behavior | user-callable service | S/D |
| QR and dogleg support | `QRFAC`, `QFORM`, `R1UPDT`, `R1MPYQ`, `DOGLEG` and D variants | subsidiary implementation machinery | subsidiary | S/D |

Sources: [`slatec-toc`](https://www.netlib.org/slatec/toc), [`DFZERO source`](https://www.netlib.org/slatec/src/dfzero.f), [`DNSQ source`](https://www.netlib.org/slatec/src/dnsq.f).

## Scalar bracketing: `DFZERO`

`DFZERO` accepts an external scalar function and interval endpoints, uses relative and absolute tolerances, and returns a status flag that distinguishes successful contraction, endpoint roots, possible singular behavior, lack of a sign change and excessive evaluations. Its prologue states that the method combines bisection with the secant rule and cites Dekker ([`DFZERO source`](https://www.netlib.org/slatec/src/dfzero.f)).

Important interface characteristics:

- the callback is a Fortran external function;
- `B` and `C` are input/output values;
- `R` is a suggested initial point;
- termination uses both relative and absolute tolerances;
- every return path is reported through `IFLAG`;
- machine precision is obtained from `D1MACH(4)`.

**Modern interpretation:** this is naturally wrapped as a bracket object and a result containing the final bracket, estimate, status and evaluation count if recoverable. A safe wrapper must not reduce all non-success flags to a generic exception, because the distinctions are numerically meaningful.

## Nonlinear systems: `DNSQ`

The `DNSQ` prologue says it combines the MINPACK `HYBRD` and `HYBRDJ` codes and implements a modification of Powell’s hybrid method. The user supplies function and optionally Jacobian subroutines. If no Jacobian is supplied, the routine forms a forward-difference approximation; band widths may be supplied to reduce work for banded Jacobians ([`DNSQ source`](https://www.netlib.org/slatec/src/dnsq.f); [`netlib-minpack`](https://www.netlib.org/minpack/)).

Its interface exposes:

- separate `FCN` and `JAC` callbacks;
- a mode selecting analytic or finite-difference Jacobians;
- mutable iterate and residual vectors;
- a column-major Jacobian with leading dimension;
- scaling vector and scaling mode;
- evaluation limits, print cadence and step-bound controls;
- packed upper-triangular QR output;
- four work arrays;
- function/Jacobian evaluation counts;
- a multi-valued `INFO` status.

The callback’s integer flag is also a control channel: a negative value can request termination, while zero may be used for printing callbacks. This is not equivalent to a simple pure Rust closure returning a vector.

## Algorithm families and assumptions

The `DNSQ` documentation describes a correction chosen as a convex combination of Newton and scaled-gradient directions, with Broyden rank-one Jacobian updates and occasional Jacobian recomputation. Convergence assumes reasonably behaved functions and consistency between a supplied Jacobian and function values. The derivative checker exists because incorrect Jacobians can produce misleading convergence ([`DNSQ source`](https://www.netlib.org/slatec/src/dnsq.f)).

Finite differences assume a scale for relative function errors through `EPSFCN`. Variable scaling can be automatic or supplied through `DIAG`. These controls should remain available in an advanced safe API.

Polynomial root routines are a different problem family and should not be forced into the callback-based scalar/system API.

## Provenance

`SNSQ/DNSQ` explicitly identify themselves as combinations of MINPACK routines. The canonical MINPACK collection covers nonlinear equations and nonlinear least squares and supplies the recognizable QR/dogleg/Levenberg-Marquardt support families ([`netlib-minpack`](https://www.netlib.org/minpack/)). `FZERO/DFZERO` cite Sandia reports and authors rather than MINPACK.

**Project policy:** preserve package/family provenance per routine. “Nonlinear equations” is a functional domain, not one historical package.

## Dependencies

Typical dependencies include:

- machine constants (`R1MACH`/`D1MACH`);
- Euclidean norm helpers;
- QR factorization and update routines;
- dogleg step construction;
- error handling;
- callbacks supplied by the application.

For systems solvers, the dependency closure can be substantial even though the public driver is one source file.

## Limitations and assumptions

- Scalar bracketing methods need a meaningful interval and are not general multiple-root detectors.
- A sign-change method can miss roots of even multiplicity.
- System solvers are local methods and depend on the starting point.
- A small step or progress test does not necessarily prove that the residual is zero.
- Finite-difference Jacobians can be unreliable for noisy, discontinuous or badly scaled functions.
- Dense QR storage gives quadratic memory and cubic Jacobian-processing costs for large systems.
- User callbacks may terminate or perform printing through mutable integer flags.
- Callback re-entry, panic unwinding and thread behavior have not been tested.

## Relation to modern libraries

Modern optimization libraries often place scalar root finding, multivariate roots, least squares and minimization behind result objects and uniform callback interfaces. SciPy, for example, exposes root finding and optimization in one high-level module while retaining multiple algorithms and structured results ([SciPy optimize](https://docs.scipy.org/doc/scipy/reference/optimize.html)). A modernized MINPACK project also exists, but it is not evidence that SLATEC’s incorporated sources are identical to that code ([modernized MINPACK](https://github.com/fortran-lang/minpack)).

**Project interpretation:** `slatec-rs` should offer domain-coherent results and options while naming the underlying historical algorithm explicitly.

## Preliminary FFI risks

| Risk | Detail | Mitigation proposal |
|---|---|---|
| callbacks | Fortran calls user functions with fixed signatures and mutable flags | generated `extern "C"`/compiler-compatible trampolines and scoped callback context |
| panic across FFI | Rust panic must not unwind through Fortran | catch panic in trampoline and convert to controlled termination |
| callback state | closures may capture Rust state | pass through thread-local or explicit registry only after reentrancy tests |
| Jacobian layout | callback writes a column-major matrix with leading dimension | checked mutable matrix view |
| dummy callback argument | `DNSQ` still accepts `JAC` when finite differences are selected | raw layer supplies a valid inert symbol; safe layer hides it |
| work arrays | many exact-length mutable arrays | allocate internally with checked formulas |
| output/status aliases | iterate and endpoints are mutated | result object records final state and original status |
| global error state | `XERMSG` may coexist with routine status | capture/report both without discarding either |

## Project proposals for safe Rust APIs

### Scalar roots

```text
BracketedRootOptions
BracketedRootResult {
    estimate,
    bracket,
    status,
}
```

The callback could be `FnMut(f64) -> f64`, with panic captured inside the trampoline. Method names should identify `fzero` compatibility rather than implying a generic best algorithm.

### Systems

```text
HybridSystemSolver
HybridOptions
HybridResult {
    x,
    residual,
    status,
    function_evaluations,
    jacobian_evaluations,
    qr_summary,
}
```

Offer separate constructors for:

- finite-difference dense Jacobian;
- finite-difference banded Jacobian;
- user-provided Jacobian.

The easy driver and expert driver can map to different option levels without exposing raw work arrays.

### Polynomial roots

Use separate coefficient and root-vector APIs, preserving whether coefficients and roots are real or complex. Do not merge these into callback-based root finding.

## Candidate crate boundaries

**Raw, tentative:**

- `slatec-minpack-sys` for verified MINPACK-derived system and least-squares families;
- `slatec-roots-sys` for scalar and polynomial families not belonging to MINPACK;
- shared support only when dependency analysis proves a stable boundary.

**Safe, tentative:**

- `slatec-roots` for scalar and polynomial roots;
- `slatec-nonlinear` for systems;
- or one `slatec-nonlinear` crate with explicit modules.

The package-oriented raw and function-oriented safe split is a project proposal.

## Open questions and experiments

- Can callbacks safely support nested or concurrent solver calls?
- Does the linked error subsystem use process-global mutable state?
- Which callback ABI is generated by each supported Fortran compiler?
- Can negative callback flags reliably transport Rust-side cancellation?
- Should printing callbacks be unsupported, captured or mapped to an observer closure?
- Are the `SOS/DSOS` family and polynomial solvers independently packageable?
- Which exact MINPACK revision was incorporated and what changed?
- What result/status normalization can be done without losing historical meaning?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`DFZERO source`](https://www.netlib.org/slatec/src/dfzero.f)
- [`DNSQ source`](https://www.netlib.org/slatec/src/dnsq.f)
- [`netlib-minpack`](https://www.netlib.org/minpack/)
- [SciPy optimize](https://docs.scipy.org/doc/scipy/reference/optimize.html)
