# Purpose

Integration over infinite intervals Standard fortran subroutine PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAGI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAGI](https://www.netlib.org/slatec/src/dqagi.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. INFINITY,BOUND) INFINITY,+INFINITY) Hopefully satisfying following claim for accuracy Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `BOUND`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Finite bound of integration range (has no meaning if interval is doubly-infinite) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `INF`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer indicating the kind of integration range involved 1 corresponds to  (BOUND,+INFINITY), 1            to  (-INFINITY,BOUND), INFINITY,+INFINITY). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `EPSABS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `EPSREL`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.1 or LENIW.LT.LIMIT*4. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because are set to zero.  Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and Double precision Approximation to the integral is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because are set to zero.  Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and not applicable or not stated by selected source not a workspace argument

## 7. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) are set to zero.  Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero.  Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. ON RETURN Integer 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 abnormal termination of the routine. The estimates for result and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of 6. 6. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is assumed that the requested tolerance cannot be achieved, and that the returned are set to ZERO, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. 6. form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and LAST otherwise ..., WORK(LIMIT+LAST) Contain the right end points, contain the contain the integral approximations over the subintervals, integral approximations over the subintervals, ..., WORK(LIMIT*3) contain the error estimates. (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is assumed that the requested tolerance cannot be achieved, and that the returned are set to ZERO, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. 6. form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and LAST otherwise ..., WORK(LIMIT+LAST) Contain the right end points, contain the contain the integral approximations over the subintervals, integral approximations over the subintervals, ..., WORK(LIMIT*3) contain the error estimates. not applicable or not stated by selected source not a workspace argument

## 11. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end not applicable or not stated by selected source not a workspace argument

## 12. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero.  Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B), contain the integral approximations over the subintervals, not stated by selected source not applicable or not stated by selected source

## 13. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and not applicable or not stated by selected source

## 14. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are set to ZERO, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and Double precision Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) Contain the right end points, contain the contain the integral approximations over the subintervals, integral approximations over the subintervals, ..., WORK(LIMIT*3) contain the error estimates. are set to ZERO, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and Double precision Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) Contain the right end points, contain the contain the integral approximations over the subintervals, integral approximations over the subintervals, ..., WORK(LIMIT*3) contain the error estimates. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `F`: not a workspace argument
- `BOUND`: not a workspace argument
- `INF`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LIMIT`: not a workspace argument
- `LENW`: not a workspace argument
- `LAST`: are set to zero.  Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B), contain the integral approximations over the subintervals,
- `IWORK`: Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and
- `WORK`: are set to ZERO, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and Double precision Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) Contain the right end points, contain the contain the integral approximations over the subintervals, integral approximations over the subintervals, ..., WORK(LIMIT*3) contain the error estimates.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqagi`
- Original SLATEC routine: `DQAGI`
- Native symbol: `dqagi_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQAGI](https://www.netlib.org/slatec/src/dqagi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
