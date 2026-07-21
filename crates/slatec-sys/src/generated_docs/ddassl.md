# Purpose

Solves a differential/algebraic system `G(T, Y, YPRIME) = 0` with backward-differentiation formulas of orders one through five. This is the original double precision SLATEC DASSL driver `DDASSL`.

# Description

The first call starts a new problem. It requires consistent initial `T`, `Y`, and `YPRIME` unless `INFO(11)=1` asks DASSL to compute a consistent derivative. Subsequent calls continue the same problem, retain state in `RWORK` and `IWORK`, and advance in the original direction of `TOUT`. The verified source prologue is [DDASSL](https://www.netlib.org/slatec/src/ddassl.f).

# Arguments

- `RES`: required `DasslResidualF64` callback. It implements `RES(T, Y, YPRIME, DELTA, IRES, RPAR, IPAR)`, writes `DELTA[0..NEQ] = G(T, Y, YPRIME)`, and must not alter `T`, `Y`, or `YPRIME`. It receives `IRES=0`; it may set `IRES=-1` for an illegal input (DASSL retries) or `IRES=-2` to return with `IDID=-11`.
- `NEQ`: input number of equations; `NEQ >= 1`.
- `T`: input initial independent-variable value and output time reached; it must be mutable storage.
- `Y`: mutable length-`NEQ` vector of initial solution components and output solution at returned `T`.
- `YPRIME`: mutable length-`NEQ` vector of initial derivatives and output derivative approximation; it must initially satisfy `G` unless `INFO(11)=1`.
- `TOUT`: input target, different from `T`; forward and backward integration are allowed. The first step does not pass `TOUT`; an interval-mode later step may interpolate from beyond it unless `TSTOP` applies.
- `INFO`: mutable integer array of length at least 15; DASSL uses entries 1 through 11. `INFO(1)=0` starts a problem and `=1` acknowledges an interrupted continuation. `INFO(2)` selects scalar (0) or length-`NEQ` vector (1) `RTOL`/`ATOL`; `INFO(3)` selects interval (0) or intermediate-output (1) mode; `INFO(4)=1` selects `TSTOP=RWORK(1)`; `INFO(5)=0` requests numerical differentiation and `=1` requires `JAC`; `INFO(6)=0` is dense and `=1` is banded with `IWORK(1)=ML`, `IWORK(2)=MU`; `INFO(7)=1` sets `HMAX=RWORK(2)`; `INFO(8)=1` sets `H0=RWORK(3)`; `INFO(9)=1` sets `MAXORD=IWORK(3)`, where `1 <= MAXORD <= 5`; `INFO(10)=1` requests nonnegative constraints; `INFO(11)=1` computes an initially consistent `YPRIME`.
- `RTOL`, `ATOL`: input/output error tolerances. With `INFO(2)=0` both are scalars; with `INFO(2)=1` both are length-`NEQ` vectors. All entries must be nonnegative; DASSL may increase them for `IDID=-2`.
- `IDID`: output completion/status code; see **Status and error values**.
- `RWORK`: mutable f64 workspace of length `LRW`; it contains persistent continuation state.
- `LRW`: input declared `RWORK` length. Minimum is `40 + (MAXORD + 4)*NEQ + NEQ*NEQ` for dense storage; `40 + (MAXORD + 4)*NEQ + (2*ML + MU + 1)*NEQ` for banded user `JAC`; or that banded amount plus `2*(NEQ/(ML + MU + 1) + 1)` for banded finite-difference `JAC`.
- `IWORK`: mutable integer workspace of length `LIW`; it contains persistent continuation state and the documented band widths/order options.
- `LIW`: input declared `IWORK` length; `LIW >= 20 + NEQ`.
- `RPAR`, `IPAR`: caller-owned real and integer parameter arrays forwarded to `RES` and `JAC`; DASSL does not alter them and their lengths are defined by the caller/callback.
- `JAC`: optional `DasslJacobianF64` callback. With `INFO(5)=1`, it implements `JAC(T, Y, YPRIME, PD, CJ, RPAR, IPAR)` and writes `PD = dG/dY + CJ*dG/dYPRIME`; it must not alter `T`, `Y`, `YPRIME`, or `CJ`. Dense `PD` is column-major with first dimension `NEQ` and stores `PD(I,J)`; banded `PD` has first dimension `2*ML + MU + 1` and stores the entry at `PD(I - J + ML + MU + 1, J)`. With `INFO(5)=0`, it is ignored but the ABI argument remains required.

# Return value

This Fortran subroutine has no direct return value. Time, solution, derivative, tolerances, status, and solver state are returned through mutable arguments.

# Callback contract

`RES` and, when selected, `JAC` are synchronous and must remain valid for the complete native call. They must uphold the exact Rust callback ABI, preserve all callback vector extents, and **must not unwind** through Fortran. Neither callback may retain native pointers after returning.

# Status and error values

- `IDID=1`: an intermediate-output step completed; `TOUT` is not yet reached.
- `IDID=2`: integration reached `TSTOP` exactly.
- `IDID=3`: integration reached `TOUT`, possibly by interpolation after stepping beyond it.
- `IDID=-1`: about 500 steps were expended; set `INFO(1)=1` to continue deliberately.
- `IDID=-2`: tolerances were too stringent; `RTOL`/`ATOL` were increased for a possible continuation.
- `IDID=-3`: an `ATOL` component and its solution component are both zero.
- `IDID=-6`: repeated local error-test failures.
- `IDID=-7`: repeated corrector convergence failures.
- `IDID=-8`: singular partial-derivative matrix.
- `IDID=-9`: repeated corrector and error-test failures on the last step.
- `IDID=-10`: `RES` set `IRES=-1` and the corrector could not converge.
- `IDID=-11`: `RES` set `IRES=-2`; control returned to the caller.
- `IDID=-12`: DASSL failed to compute an initial `YPRIME`.
- `IDID=-33`: unrecoverable or invalid-input termination; the source error runtime reports the diagnostic.

# Workspace and array requirements

`Y`, `YPRIME`, and callback `DELTA` have length `NEQ`. Preserve `RWORK` and `IWORK` unchanged for continuation except documented option entries. On return `RWORK(3)` is the next attempted step, `RWORK(4)` the farthest integration time, and `RWORK(7)` the last successful step. `IWORK(7..=8)` report orders; `IWORK(11..=15)` report steps, residual calls, Jacobian evaluations, error-test failures, and convergence failures. For continuation do not change `NEQ`, `T`, `Y`, `YPRIME`, `RWORK`, `IWORK`, or the residual system; after `IDID=2` or `3`, choose a new `TOUT` without reversing direction.

# ABI notes

- Canonical Rust path: `slatec_sys::dassl::ddassl`
- Original SLATEC routine: `DDASSL`
- Native symbol: `ddassl_`
- Precision: double precision
- Exact Netlib source file: [DDASSL](https://www.netlib.org/slatec/src/ddassl.f)

# Safety

Every pointer must be non-null, correctly aligned, and valid for the exact scalar or array extent above. `Y`, `YPRIME`, `RWORK`, and `IWORK` are mutated and must not alias in a way that violates Rust's aliasing requirements. Preserve Fortran column-major `PD` layout, parameter-array and callback lifetime, persistent continuation state, and the no-unwind callback rule.
