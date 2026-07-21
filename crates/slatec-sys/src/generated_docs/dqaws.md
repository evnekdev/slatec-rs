# Purpose

Integration of functions having algebraico-logarithmic end point singularities Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAWS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAWS](https://www.netlib.org/slatec/src/dqaws.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Lower limit of integration 6. 1) or BETA.LE.(-1) or or INTEGR.LT.1 or INTEGR.GT.4 or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Upper limit of integration, B.GT.A 6. 1) or BETA.LE.(-1) or or INTEGR.LT.1 or INTEGR.GT.4 or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `ALFA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision 1) 1), the routine will end with 1) or BETA.LE.(-1) or or INTEGR.LT.1 or INTEGR.GT.4 or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `BETA`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision 1) 1), the routine will end with not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INTEGR`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Indicates which WEIGHT function is to be used = 1  (X-A)**ALFA*(B-X)**BETA = 2  (X-A)**ALFA*(B-X)**BETA*LOG(X-A) = 3  (X-A)**ALFA*(B-X)**BETA*LOG(B-X) = 4  (X-A)**ALFA*(B-X)**BETA*LOG(X-A)*LOG(B-X) If INTEGR.LT.1 or INTEGR.GT.4, the routine not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EPSABS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `EPSREL`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.2 or LENW.LT.LIMIT*4. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral are set to not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) are set to not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine The estimates for the integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of 6. 6. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand, in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump discontinuity or a local singularity of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because is invalid are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.2. 6. form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), LAST otherwise ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand, in order to determine the integration difficulties which prevent the requested tolerance from being achieved. In case of a jump discontinuity or a local singularity of algebraico-logarithmic type at one or more interior points of the integration range, one should proceed by splitting up the interval at these points and calling the integrator on the subranges. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because is invalid are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.2. 6. form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), LAST otherwise ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. not applicable or not stated by selected source not a workspace argument

## 14. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is invalid Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end is invalid Integer Dimensioning parameter for WORK must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end not applicable or not stated by selected source not a workspace argument

## 15. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the significant number of elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B), not stated by selected source not applicable or not stated by selected source

## 16. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). WORK(LIMIT*2+1) and Integer Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ..., form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), WORK(LIMIT*2+1) and Integer Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ..., form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), not applicable or not stated by selected source

## 17. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), Double precision Vector of dimension LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), Double precision Vector of dimension LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates. not applicable or not stated by selected source

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
- `ALFA`: not a workspace argument
- `BETA`: not a workspace argument
- `INTEGR`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LIMIT`: not a workspace argument
- `LENW`: not a workspace argument
- `LAST`: are set to Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the significant number of elements actually in the WORK ARRAYS. contain the left end points of the subintervals in the partition of (A,B),
- `IWORK`: WORK(LIMIT*2+1) and Integer Vector of dimension LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)), ..., form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2),
- `WORK`: are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS ARRAYS form a decreasing sequence with K = LAST if LAST.LE.(LIMIT/2+2), Double precision Vector of dimension LENW On return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the error estimates.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqaws`
- Original SLATEC routine: `DQAWS`
- Native symbol: `dqaws_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQAWS](https://www.netlib.org/slatec/src/dqaws.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
