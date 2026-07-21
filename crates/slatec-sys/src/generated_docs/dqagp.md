# Purpose

Computation of a definite integral Standard fortran subroutine Double precision version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `DQAGP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DQAGP](https://www.netlib.org/slatec/src/dqagp.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Double precision Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Lower limit of integration provided break point or integration LIMIT, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Upper limit of integration NPTS2)/2). DIMENSIONING PARAMETERS Double precision Upper limit of integration NPTS2)/2). DIMENSIONING PARAMETERS not applicable or not stated by selected source not a workspace argument

## 4. `NPTS2`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS.GE.2. 6. 2) 2) elements of which are the user provided break elements of which are the user provided break or break points are specified outside the integration range or 2). 2), the routine will end with not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `POINTS`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). Double precision If these points do not constitute an ascending sequence there will be an automatic sorting. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `EPSABS`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EPSREL`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Relative accuracy requested If  EPSABS.LE.0 28), 28)) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `RESULT`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Approximation to the integral are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `ABSERR`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. it is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. one can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. it must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because 6. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. it is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. one can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. it must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because 6. 6. not applicable or not stated by selected source not a workspace argument

## 12. `LENIW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. NPTS2)/2). DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK NPTS2)/2, NPTS2)/2, which is the maximum number of subintervals in the which is the maximum number of subintervals in the partition of the given integration interval (A,B), partition of the given integration interval (A,B), 2). 2), the routine will end with NPTS2. NPTS2, the routine will end NPTS2)/2. NPTS2)/2. NPTS2)/2). DIMENSIONING PARAMETERS Integer Dimensioning parameter for IWORK NPTS2)/2, NPTS2)/2, which is the maximum number of subintervals in the which is the maximum number of subintervals in the partition of the given integration interval (A,B), partition of the given integration interval (A,B), 2). 2), the routine will end with NPTS2. NPTS2, the routine will end NPTS2)/2. NPTS2)/2. not applicable or not stated by selected source not a workspace argument

## 13. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Dimensioning parameter for WORK NPTS2. NPTS2, the routine will end Integer Dimensioning parameter for WORK NPTS2. NPTS2, the routine will end not applicable or not stated by selected source not a workspace argument

## 14. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) contain the left end points of the subintervals in the partition of (A,B), not stated by selected source not applicable or not stated by selected source

## 15. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Contain the Contain the subdivision levels of the subintervals, i.e. subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) if (AA,BB) is a subinterval of (P1,P2) ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Contain the Contain the subdivision levels of the subintervals, i.e. subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) if (AA,BB) is a subinterval of (P1,P2) ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, not applicable or not stated by selected source

## 16. `WORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). and WORK(LIMIT*3+1) are set to zero. is set to A and WORK(LIMIT+1) ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Double precision Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the corresponding error estimates, ..., WORK(LIMIT*4+NPTS2) contain the integration limits and the break points sorted in an ascending sequence. and WORK(LIMIT*3+1) are set to zero. is set to A and WORK(LIMIT+1) ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Double precision Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the corresponding error estimates, ..., WORK(LIMIT*4+NPTS2) contain the integration limits and the break points sorted in an ascending sequence. not applicable or not stated by selected source

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
- `NPTS2`: not a workspace argument
- `POINTS`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `LENIW`: not a workspace argument
- `LENW`: not a workspace argument
- `LAST`: are set to zero.  Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) contain the left end points of the subintervals in the partition of (A,B),
- `IWORK`: Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Contain the Contain the subdivision levels of the subintervals, i.e. subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) if (AA,BB) is a subinterval of (P1,P2) ..., IWORK(LIMIT*2+NPTS2) have no significance for the user,
- `WORK`: and WORK(LIMIT*3+1) are set to zero. is set to A and WORK(LIMIT+1) ARRAYS form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise Double precision Vector of dimension at least LENW on return contain the left contain the left end points of the subintervals in the end points of the subintervals in the partition of (A,B), partition of (A,B), ..., WORK(LIMIT+LAST) contain the right end points, ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, ..., WORK(LIMIT*3+LAST) contain the corresponding error estimates, ..., WORK(LIMIT*4+NPTS2) contain the integration limits and the break points sorted in an ascending sequence.

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dqagp`
- Original SLATEC routine: `DQAGP`
- Native symbol: `dqagp_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DQAGP](https://www.netlib.org/slatec/src/dqagp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
