# Differential equations

## Scope

This domain includes ordinary differential equations (ODEs), differential-algebraic equations (DAEs), boundary-value problems (BVPs), and selected elliptic PDE/integral-equation support. The SLATEC table of contents places these under GAMS `I` and distinguishes initial-value, boundary-value and PDE-related problem classes ([`slatec-toc`](https://www.netlib.org/slatec/toc)).

The opt-in `ode-sdrive-expert` feature exposes an owned real `f32`/`f64`
session over `SDRIV3`/`DDRIV3` for explicit initial-value problems only,
`y'(t) = f(t, y)`. It is documented in the [safe SDRIVE session guide]
(../api/safe-ode-sdrive-expert.md). The reviewed broader family selection and
deferrals remain recorded in the [ODE-family audit](ode-audit.md).

`SDRIV3`/`DDRIV3` calls are process-serialized. The source review found
initialized local `IER` storage in the transitive `SDSTP`/`DDSTP` step routines,
in addition to the scoped callback context and process-global XERROR control.
The reviewed GNU MinGW objects expose each `IER` as a writable local-linkage
`.bss` symbol. The sessions therefore make no parallel-native-execution claim.
The complete 330-source offline cache (including `DDCOR` and `DGBFA`) has been
hash-verified, compiled, and symbol-inspected; the native stress test proves
global serialization rather than reentrancy. See the [runtime concurrency and storage policy]
(../architecture/runtime-concurrency-and-storage-policy.md).

## Principal families

| Family | Representative routines | Method/problem | Precision |
|---|---|---|---|
| Runge–Kutta–Fehlberg | `DERKF/DDERKF` | explicit one-step nonstiff IVP solving | S/D |
| Adams methods | `DEABM/DDEABM` | variable-step multistep nonstiff IVPs | S/D |
| DRIVE family | `SDRIV1/DDRIV1/CDRIV1`, `SDRIV2/DDRIV2/CDRIV2`, `SDRIV3/DDRIV3/CDRIV3` | general IVPs with increasing control and stiff options | S/D/C |
| DEPAC/LSODE-related families | `DLSODE` and related drivers/subsidiaries | Adams or backward differentiation with dense/banded Jacobians | mainly S/D |
| DASSL | `SDASSL/DDASSL` | implicit DAEs \(G(t,y,y')=0\) using BDF orders 1–5 | S/D |
| BVP solvers | `BVPOR/DBVPOR`, `BVSUP/DBVSUP`, `BVSUD/DBVSUD` and support | boundary-value systems, shooting/superposition-style families | S/D |
| Separable elliptic PDE | FISHPACK routines such as `HWSCRT`, `GENBUN`, `BLKTRI`, `POIS3D` | finite-difference elliptic equations and block systems | mostly S, some complex support |
| Integral-equation/support families | routines classified in later `I` branches | problem-specific integral/differential systems | varies |

The exact source lineage of each ODE/BVP family requires routine-level provenance work. DASSL explicitly identifies Linda Petzold and Lawrence Livermore National Laboratory in its prologue ([`DDASSL source`](https://www.netlib.org/slatec/src/ddassl.f)).

## DASSL as a representative stateful interface

`DDASSL` solves \(G(t,y,y')=0\) using backward differentiation formulas of orders one through five. It requires residual and optional Jacobian callbacks, persistent real/integer work arrays, an `INFO` control array, scalar or vector tolerances, and an `IDID` result code. It supports continuation calls and optional intermediate output ([`DDASSL source`](https://www.netlib.org/slatec/src/ddassl.f)).

Important properties:

- initial \(y\) and \(y'\) are normally required to be consistent;
- the residual callback can report recoverable illegal values or request termination;
- dense and banded iteration matrices have different storage formulas;
- analytic or finite-difference Jacobians are selectable;
- the solver may step past `TOUT` and interpolate back;
- state in `RWORK` and `IWORK` must be preserved across calls;
- tolerances are input/output and can be relaxed when requested accuracy is unreasonable.

This is not safely represented as a stateless function taking only initial and final times.

## ODE algorithm families

Historical IVP routines span explicit and implicit methods:

- explicit Runge–Kutta–Fehlberg for nonstiff systems;
- Adams predictor–corrector/multistep methods;
- backward differentiation formulas for stiff systems and DAEs;
- dense or banded Newton iteration matrices;
- interpolation/dense output between internal steps;
- optional user Jacobians and error-weight vectors.

The correct method depends on stiffness, Jacobian structure, event/discontinuity handling and tolerance scaling.

## Boundary-value problems

BVP solvers generally require:

- differential-equation callbacks;
- boundary-condition callbacks;
- an initial mesh or shooting guess;
- continuation or nonlinear correction;
- matrix workspaces and status reporting.

Some families solve linear problems by superposition, while others handle nonlinear systems through iteration. Public safe APIs should distinguish these mathematical models rather than expose one generic BVP constructor.

## FISHPACK and PDE overlap

FISHPACK solves separable elliptic PDEs on structured grids and related block-tridiagonal systems. It uses cyclic reduction and Fourier-related machinery; `POIS3D`, for example, is described as using FFTPACK. Therefore the PDE domain depends on transforms and structured linear algebra even though FISHPACK and FFTPACK share a relocated Netlib directory ([`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/); [`netlib-fishpack`](https://www.netlib.org/fishpack/)).

The first reviewed safe PDE surface is intentionally narrower: the hosted
`fishpack-cartesian-2d` feature owns a single-precision Cartesian
Poisson/Helmholtz problem and calls only `HWSCRT`. It provides typed periodic,
Dirichlet, Neumann, and mixed edges; private checked workspace; explicit
`PERTRB`/non-uniqueness reporting; and process-global native serialization.
It does not expose generic FISHPACK, `GENBUN`, `BLKTRI`, `POIS3D`, or any
non-Cartesian geometry. See the [safe Cartesian FISHPACK guide]
(../api/safe-fishpack-cartesian-2d.md).

## Common callback and workspace patterns

- RHS callback \(f(t,y)\);
- residual callback \(G(t,y,y')\);
- dense or banded Jacobian callback;
- boundary-condition callback;
- optional problem parameter arrays (`RPAR`, `IPAR`);
- persistent real/integer state arrays;
- mode arrays controlling first call, continuation, tolerance form and output mode.

## Numerical limitations

- ODE/DAE methods control local error, not guaranteed global error.
- Stiffness detection and method selection can fail.
- DAE initial conditions may be inconsistent or the iteration matrix singular.
- Pure relative tolerance fails when a solution component is zero.
- Discontinuities require explicit stop/restart handling.
- BVP convergence depends strongly on guesses, mesh and conditioning.
- FISHPACK assumes specific separable operators, grids and boundary conditions.
- Historical integer and workspace limits can constrain large systems.

## Modern equivalents

Modern libraries such as SUNDIALS provide CVODE/IDA with richer event handling, sparse/iterative linear solver integration and standardized state objects. SciPy exposes `solve_ivp`, `solve_bvp` and DAE-related capabilities through contemporary Python interfaces. These are functional comparisons, not provenance matches.

## FFI considerations

| Risk | Consequence | Proposed mitigation |
|---|---|---|
| long-lived solver state | work arrays must survive continuation calls | owned solver-session object |
| callbacks | Fortran invokes Rust repeatedly | panic-safe scoped trampolines |
| mutable mode arrays | invalid combinations are easy | typed option builder and internal encoding |
| scalar/vector tolerances | ABI and length ambiguity | explicit enum with owned vectors |
| banded Jacobian layout | indexing errors corrupt memory | checked band-matrix callback view |
| continuation rules | changing forbidden fields invalidates state | state-machine API |
| global errors | thread-safety uncertain | serialize until tested; preserve raw error records |
| events/discontinuities | legacy solvers lack modern event abstraction | explicit stop points and restart protocol |

## Project proposals for safe Rust

Candidate top-level types:

```text
OdeProblem
DaeProblem
BvpProblem
OdeSession
DaeSession
StepOutcome
IntegrationStatus
```

The session should own work arrays and enforce first-call/continuation transitions. It should expose `advance_to`, `step`, and `restart` rather than permitting arbitrary mutation of `INFO`, `RWORK` and `IWORK`.

## Tentative crate ownership

- Raw: package/family closures such as `slatec-dassl-sys`, `slatec-depac-sys`, `slatec-bvp-sys`, `slatec-fishpack-sys`.
- Safe: `slatec-ode`, `slatec-dae`, `slatec-bvp`, and `slatec-pde` or modules in a broader `slatec-diffeq`.
- Shared callback and state machinery may be internal, but should not create public dependency cycles.

## Open questions

- Which exact ODE family names and revisions are present in the 4.1 archive?
- Are LSODE/DEPAC symbols modified relative to upstream ODEPACK or predecessor sources?
- Which solvers are reentrant and thread-safe?
- What common blocks or saved state exist below the public drivers?
- Which BVP families warrant separate safe APIs?
- How should dense output and event detection be layered without claiming native support?
- Can FISHPACK be separated cleanly from FFTPACK in the link graph?

## Sources

- [`slatec-toc`](https://www.netlib.org/slatec/toc)
- [`DDASSL source`](https://www.netlib.org/slatec/src/ddassl.f)
- [`slatec-fishfft`](https://www.netlib.org/slatec/fishfft/)
- [`netlib-fishpack`](https://www.netlib.org/fishpack/)
