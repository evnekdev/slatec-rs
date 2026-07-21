# Purpose

Solves a square system of nonlinear algebraic equations by a modified Newton method with a forward-difference Jacobian. This is the original single precision SLATEC driver `SOS`.

# Description

`SOS` repeatedly evaluates the numbered equations through `FNC(X, K)`, forms or updates an upper-triangular linear system, and returns the current iterate and convergence status through mutable arguments. The selected, source-hash-verified prologue is [SOS](https://www.netlib.org/slatec/src/sos.f).

# Arguments

- `FNC`: required synchronous `SosEquationF32` callback. It receives a readable length-`NEQ` current or finite-difference-perturbed iterate `X` and one-based integer `K` in `1..=NEQ`, and returns the `K`th equation value. It must not mutate or retain either pointer and must not unwind.
- `NEQ`: input number of equations and unknowns; it must be positive.
- `X`: mutable length-`NEQ` `f32` solution vector. It supplies the initial estimate and is overwritten with the most recent iterate. During finite differences the native code temporarily changes components before calling `FNC`.
- `RTOLX`: input nonnegative relative iterate tolerance. The increment criterion is `abs(X(I)-XOLD(I)) <= RTOLX*abs(X(I)) + ATOLX`.
- `ATOLX`: input nonnegative absolute iterate tolerance. A positive value is useful when a solution component can be zero.
- `TOLF`: input nonnegative residual tolerance. Residual convergence requires every equation residual to be no greater than `TOLF`; scale equations so this test is meaningful.
- `IFLAG`: input/output control and status integer. Input `0` uses default controls; input `-1` reads optional controls from `IW(1..=2)`. On return it reports one of the status values below.
- `RW`: mutable `f32` workspace with at least `LRW` elements. The required minimum is `1 + 6*NEQ + NEQ*(NEQ + 1)/2`. `RW(1)` reports the residual norm on return.
- `LRW`: input declared length of `RW`; it must meet the stated minimum.
- `IW`: mutable integer workspace with at least `LIW` elements. `IW(1)=-1` requests iteration output when `IFLAG=-1`; `IW(2)` supplies a positive iteration limit in that mode; `IW(3)` reports the iteration count on return.
- `LIW`: input declared length of `IW`; it must be at least `3 + NEQ`.

# Return value

This Fortran subroutine has no direct return value. It returns the iterate, residual norm, iteration count, and termination state through `X`, `RW`, `IW`, and `IFLAG`.

# Callback contract

`FNC` is called synchronously, potentially many times and with finite-difference perturbations of `X`. It receives only `X` and the one-based equation index `K`: `NEQ` is not passed through the callback ABI, and there is no user-data/context pointer. The callback may only read the externally known supplied `X` extent for the duration of the call; it must not retain pointers, panic, or unwind across the native boundary. Stateful Rust use therefore requires an external scoped context mechanism in a future wrapper.

# Status and error values

| `IFLAG` | Meaning |
| ---: | --- |
| `1` | The iterate-increment test was satisfied. |
| `2` | The residual test was satisfied. |
| `3` | Both increment and residual tests were satisfied. |
| `4` | Numerical precision was inadequate or convergence was too slow. |
| `5` | The iteration limit was reached. |
| `6` | The iteration limit was reached and the iteration was not converging. |
| `7` | The iteration appeared to diverge. |
| `8` | The Jacobian-related triangular system was singular or nearly singular. |
| `9` | An input, optional-control, or workspace requirement was invalid. |

# Workspace and array requirements

`X` has exactly `NEQ` elements. `RW` has at least `1 + 6*NEQ + NEQ*(NEQ + 1)/2` elements and `IW` has at least `3 + NEQ` elements. The native routine mutates `X`, `RW`, and `IW`; preserve their allocation and do not create incompatible aliases.

# ABI notes

- Canonical Rust path: `slatec_sys::nonlinear::systems::sos`
- Original SLATEC routine: `SOS`
- Native symbol: `sos_`
- Precision: single precision
- Supported ABI profile: `ffi-profile-gnu-mingw-x86_64`
- Exact Netlib source file: [SOS](https://www.netlib.org/slatec/src/sos.f)

# Safety

Every scalar pointer must be non-null, correctly aligned, and valid for the required read or write access. `X`, `RW`, and `IW` must satisfy the exact lengths above and must not alias in a way that violates Rust or the native routine's mutation assumptions. `FNC` must have the stated GNU Fortran scalar-function ABI, remain valid through the call, neither retain pointers nor unwind, and tolerate the source-documented temporary perturbations of `X`. The caller is responsible for serializing use of legacy native runtime state.
