# Purpose

Computation of a definite integral Standard fortran subroutine Real version

# Description

This canonical unsafe binding exposes original SLATEC routine `QAG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAG](https://www.netlib.org/slatec/src/qag.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand Function F(X). The actual name for F needs to be Declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Lower limit of integration KRONROD PAIR is used with 7 - 15 POINTS If KEY.LT.2, 10 - 21 POINTS If KEY = 2, 15 - 31 POINTS If KEY = 3, 20 - 41 POINTS If KEY = 4, 25 - 51 POINTS If KEY = 5, 30 - 61 POINTS If KEY.GT.5. ON RETURN LIMIT.GE.1. LAST If not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Upper limit of integration LIMIT.GE.1. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `EPSABS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Absolute accuracy requested AND not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `EPSREL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `KEY`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer for choice of local integration rule not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `RESULT`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Approximation to the integral are set to zero. EXCEPT when LENW is invalid, IWORK(1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, Which should EQUAL or EXCEED ABS(I-RESULT) are set to zero. EXCEPT when LENW is invalid, IWORK(1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero. EXCEPT when LENW is invalid, IWORK(1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine The estimates for RESULT and ERROR are Less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of 6. 6. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (and taking the according dimension adjustments into account). HOWEVER, If this yield no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (I.E. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) One will probably gain from splitting up the interval at this point and calling the INTEGRATOR on the SUBRANGES. If possible, AN APPROPRIATE SPECIAL-PURPOSE INTEGRATOR should be used which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval 6. , WORK(LIMIT*3+IWORK(K)) LAST otherwise LAST otherwise contain the contain the right end points, right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. (and taking the according dimension adjustments into account). HOWEVER, If this yield no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (I.E. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) One will probably gain from splitting up the interval at this point and calling the INTEGRATOR on the SUBRANGES. If possible, AN APPROPRIATE SPECIAL-PURPOSE INTEGRATOR should be used which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval 6. , WORK(LIMIT*3+IWORK(K)) LAST otherwise LAST otherwise contain the contain the right end points, right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. not applicable or not stated by selected source not a workspace argument

## 12. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for work must be at least LIMIT*4. IF LENW.LT.LIMIT*4, the routine will end with Integer Dimensioning parameter for work must be at least LIMIT*4. IF LENW.LT.LIMIT*4, the routine will end with not applicable or not stated by selected source not a workspace argument

## 13. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero. EXCEPT when LENW is invalid, IWORK(1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. LAST otherwise contain the left end points of the subintervals in the partition of contain the right end points, not stated by selected source not applicable or not stated by selected source

## 14. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least limit, the first K elements of which contain pointers to the error estimates over the subintervals, such that , WORK(LIMIT*3+IWORK(K)) Integer Vector of dimension at least limit, the first K elements of which contain pointers to the error estimates over the subintervals, such that , WORK(LIMIT*3+IWORK(K)) not applicable or not stated by selected source

## 15. `WORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and to B. DIMENSIONING PARAMETERS ARRAYS , WORK(LIMIT*3+IWORK(K)) Real Vector of dimension at least LENW on return contain the left end contain the left end points of the subintervals in the partition of points of the subintervals in the partition of contain the contain the right end points, right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and to B. DIMENSIONING PARAMETERS ARRAYS , WORK(LIMIT*3+IWORK(K)) Real Vector of dimension at least LENW on return contain the left end contain the left end points of the subintervals in the partition of points of the subintervals in the partition of contain the contain the right end points, right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. not applicable or not stated by selected source

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
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `KEY`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LIMIT`: not a workspace argument
- `LENW`: not a workspace argument
- `LAST`: are set to zero. EXCEPT when LENW is invalid, IWORK(1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. LAST otherwise contain the left end points of the subintervals in the partition of contain the right end points,
- `IWORK`: Integer Vector of dimension at least limit, the first K elements of which contain pointers to the error estimates over the subintervals, such that , WORK(LIMIT*3+IWORK(K))
- `WORK`: and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and to B. DIMENSIONING PARAMETERS ARRAYS , WORK(LIMIT*3+IWORK(K)) Real Vector of dimension at least LENW on return contain the left end contain the left end points of the subintervals in the partition of points of the subintervals in the partition of contain the contain the right end points, right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::qag`
- Original SLATEC routine: `QAG`
- Native symbol: `qag_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [QAG](https://www.netlib.org/slatec/src/qag.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
