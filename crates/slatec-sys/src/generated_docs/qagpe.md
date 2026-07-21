# Purpose

Computation of a definite integral Standard fortran subroutine Real version PARAMETERS ON ENTRY

# Description

This canonical unsafe binding exposes original SLATEC routine `QAGPE`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [QAGPE](https://www.netlib.org/slatec/src/qagpe.f).

# Arguments

## 1. `F`

callback `callback` argument; Fortran declaration `REAL`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `A`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Integral of F Real Lower limit of integration provided break point or integration limit, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L). LAST not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `B`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Upper limit of integration not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NPTS2`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. 6. 2) 2) elements of which are the user provided break elements of which are the user provided break or Break points are specified outside the integration range or 2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. If this happens for the subinterval numbered K, NDIN(K) is put to 1, otherwise not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `POINTS`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real If these POINTS do not constitute an ascending sequence there will be an automatic sorting. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `EPSABS`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user. Real Absolute accuracy requested and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `EPSREL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user. Real Relative accuracy requested If  EPSABS.LE.0 28), 28)) or LIMIT.LT.NPTS2. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `LIMIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.NPTS2 If LIMIT.LT.NPTS2, the routine will end with (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs At some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because LAST LAST otherwise otherwise Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.NPTS2 If LIMIT.LT.NPTS2, the routine will end with (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs At some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because LAST LAST otherwise otherwise not applicable or not stated by selected source not a workspace argument

## 9. `RESULT`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user. Real Approximation to the integral ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. ALIST(1) and not stated by selected source not applicable or not stated by selected source not a workspace argument

## 10. `ABSERR`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `NEVAL`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. Integer Number of integrand evaluations not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 6. 6. 6. ON RETURN Integer 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ALIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least LIMIT, the first Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 14. `BLIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are set to A and B respectively. Real Vector of dimension at least LIMIT, the first are set to A and B respectively. Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 15. `RLIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least LIMIT, the first Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 16. `ELIST`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least LIMIT, the first Real Vector of dimension at least LIMIT, the first not applicable or not stated by selected source not a workspace argument

## 17. `PTS`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). Real Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. Real Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. not applicable or not stated by selected source not a workspace argument

## 18. `IORD`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the not applicable or not stated by selected source not a workspace argument

## 19. `LEVEL`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as not applicable or not stated by selected source not a workspace argument

## 20. `NDIN`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), 0. Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), 0. not applicable or not stated by selected source not a workspace argument

## 21. `LAST`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) elements of which are the integral approximations on the subintervals elements of which are the moduli of the absolute error estimates on the subintervals LAST otherwise Integer Number of subintervals actually produced in the subdivisions process not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

# Status and error values

such that ELIST(IORD(1)), ..., ELIST(IORD(K))

# Workspace and array requirements

- `F`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `NPTS2`: not a workspace argument
- `POINTS`: not a workspace argument
- `EPSABS`: not a workspace argument
- `EPSREL`: not a workspace argument
- `LIMIT`: not a workspace argument
- `RESULT`: not a workspace argument
- `ABSERR`: not a workspace argument
- `NEVAL`: not a workspace argument
- `IER`: not a workspace argument
- `ALIST`: not a workspace argument
- `BLIST`: not a workspace argument
- `RLIST`: not a workspace argument
- `ELIST`: not a workspace argument
- `PTS`: not a workspace argument
- `IORD`: not a workspace argument
- `LEVEL`: not a workspace argument
- `NDIN`: not a workspace argument
- `LAST`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::callbacks::qagpe`
- Original SLATEC routine: `QAGPE`
- Native symbol: `qagpe_`
- ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [QAGPE](https://www.netlib.org/slatec/src/qagpe.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
