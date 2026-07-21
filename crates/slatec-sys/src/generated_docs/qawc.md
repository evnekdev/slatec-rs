# Purpose

Computation of a Cauchy principal value Standard fortran subroutine Real version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `QAWC`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAWC](https://www.netlib.org/slatec/src/qawc.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Under limit of integration LIMIT.GE.1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Upper limit of integration LIMIT.GE.1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `C`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Parameter in the weight function, C.NE.A, C.NE.B. A or C = B, the routine will end with A or C = B or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `EPSABS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `EPSREL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.1 or LENW.LT.LIMIT*4. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `RESULT`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate or the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6 . 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine the estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 2 The occurrence of roundoff error is detec- ted, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because 6. 6. 6 . 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine the estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more sub- divisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 2 The occurrence of roundoff error is detec- ted, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because 6. 6. not applicable or not stated by selected source not a workspace argument

## 11. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval 6. form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), LAST otherwise ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval 6. form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), LAST otherwise ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. not applicable or not stated by selected source not a workspace argument

## 12. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end with Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end with not applicable or not stated by selected source not a workspace argument

## 13. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B), not stated by selected source not applicable or not stated by selected source

## 14. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), not applicable or not stated by selected source

## 15. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), Real Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), Real Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `F`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `C`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LIMIT`: not a workspace argument
- `LENW`: not a workspace argument
- `LAST`: are set to zero.  Except when LENW or LIMIT is invalid, IWORK(1), WORK(LIMIT*2+1) and Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B),
- `IWORK`: Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ... , form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2),
- `WORK`: are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), Real Vector of dimension at least LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qawc`
- Original SLATEC routine: `QAWC`
- Native symbol: `qawc_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QAWC](https://www.netlib.org/slatec/src/qawc.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
