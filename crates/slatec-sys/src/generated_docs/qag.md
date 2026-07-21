# Purpose

Approximates a definite integral over `(A, B)` with an adaptive Gauss-Kronrod rule. This is the original single precision QUADPACK driver `QAG`.

# Description

`QAG` repeatedly evaluates `F` and subdivides `(A, B)` until the requested absolute or relative accuracy is met, or it reports the limiting condition through `IER`. The selected source is [QAG](https://www.netlib.org/slatec/src/qag.f).

# Arguments

## `F`

**Direction:** `callback`. The synchronous `IntegrandF32` integrand receives one readable `f32` abscissa and returns its function value. It must remain valid through the call, must not retain the pointer, and **must not unwind** through Fortran.

## `A`

**Direction:** `input`. Lower integration limit.

## `B`

**Direction:** `input`. Upper integration limit.

## `EPSABS`

**Direction:** `input`. Requested absolute accuracy.

## `EPSREL`

**Direction:** `input`. Requested relative accuracy. When `EPSABS <= 0`, it must be at least `max(50 * relative_machine_accuracy, 0.5e-28)` in the routine precision, otherwise `IER=6`.

## `KEY`

**Direction:** `input`. Selects the local Gauss-Kronrod pair: `< 2` selects 7/15 points; `2`, `3`, `4`, and `5` select 10/21, 15/31, 20/41, and 25/51 points; `> 5` selects 30/61 points.

## `RESULT`

**Direction:** `output`. Approximation to the requested integral.

## `ABSERR`

**Direction:** `output`. Estimate of the absolute error; it is intended to satisfy `ABSERR >= abs(I - RESULT)`.

## `NEVAL`

**Direction:** `output`. Number of integrand evaluations.

## `IER`

**Direction:** `status-output`. Completion and error indicator; see **Status and error values**.

## `LIMIT`

**Direction:** `input`. Maximum number of subintervals; `LIMIT >= 1`. It is also the required minimum length of `IWORK`.

## `LENW`

**Direction:** `input`. Declared length of `WORK`; `LENW >= 4 * LIMIT`.

## `LAST`

**Direction:** `output`. Number of subintervals produced. It determines the number of significant entries in the workspace segments.

## `IWORK`

**Direction:** `workspace-output`. Integer array of at least `LIMIT` elements. Its first `K` entries order subinterval error estimates, where `K=LAST` when `LAST <= LIMIT/2 + 2`, otherwise `K=LIMIT + 1 - LAST`.

## `WORK`

**Direction:** `workspace-output`. `f32` array of at least `LENW` elements. On return its four `LIMIT`-strided segments hold left endpoints, right endpoints, subintegral estimates, and error estimates for the first `LAST` subintervals.

# Return value

This Fortran subroutine has no direct return value. It returns the integral estimate, error estimate, evaluation count, status, and subdivision state through mutable arguments.

# Callback contract

`F` is invoked synchronously using the reviewed `IntegrandF32` ABI. It may read its one scalar input only for the duration of the call and **must not panic or unwind** across the native boundary.

# Status and error values

| `IER` | Meaning |
| ---: | --- |
| `0` | Normal, reliable completion; the requested accuracy is assumed achieved. |
| `1` | The maximum number of subdivisions was reached. Increasing `LIMIT` may help. |
| `2` | Roundoff prevented the requested tolerance from being achieved. |
| `3` | Extremely bad integrand behavior occurred within the integration interval. |
| `6` | Invalid input: tolerance, `LIMIT`, or `LENW` constraints were violated. `RESULT`, `ABSERR`, `NEVAL`, and `LAST` are zeroed as documented by the source. |

# Workspace and array requirements

`IWORK` length is at least `LIMIT`; `WORK` length is at least `LENW`, with `LENW >= 4 * LIMIT`. `WORK(1..LAST)`, `WORK(LIMIT+1..LIMIT+LAST)`, `WORK(2*LIMIT+1..2*LIMIT+LAST)`, and `WORK(3*LIMIT+1..3*LIMIT+LAST)` respectively hold the left endpoints, right endpoints, subintegral estimates, and error estimates.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qag`
- Original SLATEC routine: `QAG`
- Native symbol: `qag_`
- Precision: single precision
- Exact Netlib source file: [QAG](https://www.netlib.org/slatec/src/qag.f)

# Safety

All scalar pointers and the `IWORK`/`WORK` arrays must be non-null, aligned, and valid for the documented extent. The output and workspace arguments are mutated; do not create aliases that violate Rust or the native routine's assumptions. Preserve the required workspace layout, callback lifetime, and no-unwind rule.
